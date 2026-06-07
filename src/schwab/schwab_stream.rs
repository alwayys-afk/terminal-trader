use std::collections::{HashSet, VecDeque};

use anyhow::{Result, anyhow};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, connect_async};
use tracing::{debug, info, trace, warn};

use crate::schwab::SchwabClient;
use crate::schwab::streamer_types::{
    AcctActivityTick, DataFrame, LevelOneEquityTick, ServiceName, StreamerInbound, StreamerRequest,
};

type WsConn = tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>;

pub enum StreamEvent {
    EquityTicks(Vec<LevelOneEquityTick>),
    AccountActivity(Vec<AcctActivityTick>),
}

pub struct SchwabStream {
    read: SplitStream<WsConn>,
    write: SplitSink<WsConn, Message>,
    customer_id: String,
    correl_id: String,
    pending: VecDeque<StreamEvent>,
}

impl SchwabStream {
    /// Connect to the Schwab streamer WebSocket and perform the LOGIN handshake.
    pub async fn connect(client: &SchwabClient) -> Result<Self> {
        let info = client.get_streamer_info().await?;
        let ws_url = info
            .streamer_socket_url
            .ok_or_else(|| anyhow!("no streamerSocketUrl"))?;
        let customer_id = info
            .schwab_client_customer_id
            .ok_or_else(|| anyhow!("no schwabClientCustomerId"))?;
        let correl_id = info
            .schwab_client_correl_id
            .ok_or_else(|| anyhow!("no schwabClientCorrelId"))?;
        let channel = info
            .schwab_client_channel
            .ok_or_else(|| anyhow!("no schwabClientChannel"))?;
        let function_id = info
            .schwab_client_function_id
            .ok_or_else(|| anyhow!("no schwabClientFunctionId"))?;

        info!("schwab_stream: connecting to {ws_url}");
        let (ws, _) = connect_async(&ws_url)
            .await
            .map_err(|e| anyhow!("WebSocket connect: {e}"))?;
        info!("schwab_stream: connected");
        let (mut write, mut read) = ws.split();

        // LOGIN
        let login = StreamerRequest::login(
            &customer_id,
            &correl_id,
            &client.access_token().await?,
            &channel,
            &function_id,
        );
        write
            .send(Message::Text(
                serde_json::to_string(&login)
                    .map_err(|e| anyhow!("LOGIN serialize: {e}"))?
                    .into(),
            ))
            .await
            .map_err(|e| anyhow!("LOGIN send: {e}"))?;

        // Wait for LOGIN response
        loop {
            match read.next().await {
                Some(Ok(Message::Text(t))) => {
                    let msg: StreamerInbound = match serde_json::from_str(&t) {
                        Ok(v) => v,
                        Err(e) => {
                            debug!("schwab_stream: login: unparseable message: {e}");
                            continue;
                        }
                    };
                    if let StreamerInbound::Response { response } = msg {
                        for r in response {
                            if r.content.code != 0 {
                                let msg = r.content.msg.as_deref().unwrap_or("unknown");
                                return Err(anyhow!(
                                    "LOGIN failed: code={} msg={msg}",
                                    r.content.code
                                ));
                            }
                        }
                        info!("schwab_stream: LOGIN ok");
                        break;
                    }
                }
                Some(Err(e)) => return Err(anyhow!("WS error during login: {e}")),
                None => return Err(anyhow!("WS closed during login")),
                _ => {}
            }
        }

        Ok(Self {
            read,
            write,
            customer_id,
            correl_id,
            pending: VecDeque::new(),
        })
    }

    pub async fn subscribe_equities(&mut self, symbols: &HashSet<String>) -> Result<()> {
        let req =
            StreamerRequest::subs_equities(&self.customer_id, &self.correl_id, symbols.iter());
        self.send_request(&req)
            .await
            .map_err(|e| anyhow!("LEVELONE_EQUITIES SUBS: {e}"))
    }

    pub async fn unsubscribe_equities(&mut self, symbols: &HashSet<String>) -> Result<()> {
        let req =
            StreamerRequest::unsubs_equities(&self.customer_id, &self.correl_id, symbols.iter());
        self.send_request(&req)
            .await
            .map_err(|e| anyhow!("LEVELONE_EQUITIES UNSUBS: {e}"))
    }

    pub async fn subscribe_acct_activity(&mut self) -> Result<()> {
        let req = StreamerRequest::subs_acct_activity(&self.customer_id, &self.correl_id);
        self.send_request(&req)
            .await
            .map_err(|e| anyhow!("ACCT_ACTIVITY SUBS: {e}"))
    }

    /// Returns the next meaningful event. Heartbeats and response frames are
    /// consumed internally. Returns `Ok(None)` on clean disconnect.
    pub async fn next_event(&mut self) -> Result<Option<StreamEvent>> {
        if let Some(event) = self.pending.pop_front() {
            return Ok(Some(event));
        }

        loop {
            match self.read.next().await {
                Some(Ok(Message::Text(t))) => {
                    let inbound: StreamerInbound = match serde_json::from_str(&t) {
                        Ok(v) => v,
                        Err(e) => {
                            debug!("schwab_stream: unparseable message: {e}");
                            continue;
                        }
                    };

                    match inbound {
                        StreamerInbound::Notify { .. } => {
                            trace!("schwab_stream: heartbeat");
                            continue;
                        }
                        StreamerInbound::Response { .. } => {
                            debug!("schwab_stream: response (post-login)");
                            continue;
                        }
                        StreamerInbound::Data { data } => {
                            for frame in data {
                                if let Some(event) = Self::parse_frame(frame) {
                                    self.pending.push_back(event);
                                }
                            }
                            if let Some(event) = self.pending.pop_front() {
                                return Ok(Some(event));
                            }
                            continue;
                        }
                    }
                }
                Some(Ok(Message::Close(frame))) => {
                    info!("schwab_stream: server sent Close: {:?}", frame);
                    return Ok(None);
                }
                None => {
                    warn!("schwab_stream: stream ended");
                    return Ok(None);
                }
                Some(Err(e)) => return Err(anyhow!("WS error: {e}")),
                _ => continue,
            }
        }
    }

    async fn send_request(&mut self, req: &StreamerRequest) -> Result<()> {
        let json = serde_json::to_string(req).map_err(|e| anyhow!("serialize: {e}"))?;
        self.write
            .send(Message::Text(json.into()))
            .await
            .map_err(|e| anyhow!("send: {e}"))
    }

    fn parse_frame(frame: DataFrame) -> Option<StreamEvent> {
        match frame.service {
            ServiceName::LeveloneEquities => {
                trace!("schwab_stream: LEVELONE_EQUITIES tick");
                let ticks: Vec<LevelOneEquityTick> = frame
                    .content
                    .into_iter()
                    .filter_map(|v| match serde_json::from_value(v) {
                        Ok(t) => Some(t),
                        Err(e) => {
                            warn!("schwab_stream: failed to parse equity tick: {e}");
                            None
                        }
                    })
                    .collect();
                if ticks.is_empty() {
                    None
                } else {
                    Some(StreamEvent::EquityTicks(ticks))
                }
            }
            ServiceName::AcctActivity => {
                let ticks: Vec<AcctActivityTick> = frame
                    .content
                    .into_iter()
                    .filter_map(
                        |v| match serde_json::from_value::<AcctActivityTick>(v.clone()) {
                            Ok(t) => Some(t),
                            Err(e) => {
                                warn!(
                                    "schwab_stream: failed to parse ACCT_ACTIVITY tick: {e}, raw={v}"
                                );
                                None
                            }
                        },
                    )
                    .collect();
                Some(StreamEvent::AccountActivity(ticks))
            }
            svc => {
                trace!("schwab_stream: unhandled service={:?}", svc);
                None
            }
        }
    }
}
