//! Option chain page: calls | strike | puts grid for one expiration, with
//! expiration navigation, weekly/monthly/date filters, and moneyness filters.
//! Live LEVELONE_OPTIONS ticks for the selected row overlay the snapshot.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::app::{AppState, ChainExpiryFilter, MoneynessFilter};
use crate::schwab::{ExpiryKind, OptionContract, PutCall};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, underlying: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header: underlying, spot, fetched-at
            Constraint::Length(1), // expiration strip
            Constraint::Min(0),    // grid
            Constraint::Length(1), // status / keys
        ])
        .split(area);

    let Some((chain, fetched_at)) = state.chains.get(underlying) else {
        f.render_widget(
            Paragraph::new(format!("  Loading option chain for {underlying}..."))
                .style(Style::default().fg(Color::DarkGray)),
            chunks[2],
        );
        render_status(f, chunks[3], state);
        return;
    };

    let view = &state.chain_view;

    // Header line: underlying, spot, filters, snapshot age.
    let age = fetched_at.elapsed().as_secs();
    let mut header_spans = vec![
        Span::styled(
            format!(" {} ", chain.underlying),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("${:.2}", chain.underlying_price),
            Style::default().fg(Color::White),
        ),
    ];
    let mut filters: Vec<String> = Vec::new();
    match view.expiry_filter {
        Some(ChainExpiryFilter::Weekly) => filters.push("weekly".into()),
        Some(ChainExpiryFilter::Monthly) => filters.push("monthly".into()),
        Some(ChainExpiryFilter::Date(d)) => filters.push(d.format("%-m/%-d/%y").to_string()),
        None => {}
    }
    match view.moneyness {
        Some(MoneynessFilter::Atm) => filters.push("ATM".into()),
        Some(MoneynessFilter::Itm) => filters.push("ITM".into()),
        Some(MoneynessFilter::Otm) => filters.push("OTM".into()),
        None => {}
    }
    if !filters.is_empty() {
        header_spans.push(Span::styled(
            format!("   filter: {}", filters.join(" + ")),
            Style::default().fg(Color::Yellow),
        ));
    }
    header_spans.push(Span::styled(
        format!("   snapshot {age}s ago (r refresh)"),
        Style::default().fg(Color::DarkGray),
    ));
    f.render_widget(Paragraph::new(Line::from(header_spans)), chunks[0]);

    // Expiration strip: neighbors around the selection.
    let dates = view.visible_expirations(chain);
    if dates.is_empty() {
        f.render_widget(
            Paragraph::new("  No expirations match the filter (x clears filters)")
                .style(Style::default().fg(Color::DarkGray)),
            chunks[2],
        );
        render_status(f, chunks[3], state);
        return;
    }
    let sel_exp = view.expiration_idx.min(dates.len() - 1);
    let strip_start = sel_exp.saturating_sub(2);
    let mut strip_spans = vec![Span::styled(" exp: ", Style::default().fg(Color::DarkGray))];
    if strip_start > 0 {
        strip_spans.push(Span::styled("… ", Style::default().fg(Color::DarkGray)));
    }
    for (i, date) in dates.iter().enumerate().skip(strip_start).take(6) {
        let kind = chain
            .expirations
            .get(date)
            .map(|s| s.kind())
            .unwrap_or(ExpiryKind::Other);
        let tag = match kind {
            ExpiryKind::Weekly => "w",
            ExpiryKind::Monthly => "m",
            ExpiryKind::Quarterly => "q",
            ExpiryKind::EndOfMonth => "e",
            ExpiryKind::Other => "-",
        };
        let dte = (*date - chrono::Local::now().date_naive()).num_days();
        let label = format!("{} ({tag},{dte}d)  ", date.format("%-m/%-d"));
        let style = if i == sel_exp {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        strip_spans.push(Span::styled(label, style));
    }
    if strip_start + 6 < dates.len() {
        strip_spans.push(Span::styled("…", Style::default().fg(Color::DarkGray)));
    }
    f.render_widget(Paragraph::new(Line::from(strip_spans)), chunks[1]);

    // Grid for the selected expiration.
    let Some(slice) = chain.expirations.get(&dates[sel_exp]) else {
        return;
    };
    let rows_data = view.rows(slice, chain.underlying_price);

    let call_focus = view.side == PutCall::Call;
    let side_style = |focused: bool| {
        if focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };
    let header = Row::new(vec![
        Cell::from("vol").style(side_style(call_focus)),
        Cell::from("oi").style(side_style(call_focus)),
        Cell::from("delta").style(side_style(call_focus)),
        Cell::from("bid").style(side_style(call_focus)),
        Cell::from("ask").style(side_style(call_focus)),
        Cell::from("strike").style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("bid").style(side_style(!call_focus)),
        Cell::from("ask").style(side_style(!call_focus)),
        Cell::from("delta").style(side_style(!call_focus)),
        Cell::from("oi").style(side_style(!call_focus)),
        Cell::from("vol").style(side_style(!call_focus)),
    ])
    .height(1);

    let spot = chain.underlying_price;
    let sel_row = view.strike_idx.min(rows_data.len().saturating_sub(1));
    let table_rows: Vec<Row> = rows_data
        .iter()
        .map(|row| {
            // Live ticks (selected row is subscribed) overlay snapshot values.
            let call = side_cells(state, row.call, true);
            let put = side_cells(state, row.put, false);
            let strike_style = if (row.strike - spot).abs()
                == rows_data
                    .iter()
                    .map(|r| (r.strike - spot).abs())
                    .fold(f64::INFINITY, f64::min)
            {
                // Closest-to-spot strike gets an ATM marker color.
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let mut cells = call;
            cells.push(Cell::from(crate::schwab::format_strike(row.strike)).style(strike_style));
            cells.extend(put);
            Row::new(cells)
        })
        .collect();

    let widths = [
        Constraint::Length(7), // call vol
        Constraint::Length(7), // call oi
        Constraint::Length(6), // call delta
        Constraint::Length(8), // call bid
        Constraint::Length(8), // call ask
        Constraint::Length(8), // strike
        Constraint::Length(8), // put bid
        Constraint::Length(8), // put ask
        Constraint::Length(6), // put delta
        Constraint::Length(7), // put oi
        Constraint::Length(7), // put vol
    ];

    let title = format!(
        " {} {}  {} ",
        chain.underlying,
        dates[sel_exp].format("%-m/%-d/%y"),
        if call_focus { "CALLS" } else { "PUTS" },
    );
    let table = Table::new(table_rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title))
        .column_spacing(1)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    let mut table_state = TableState::default();
    if !rows_data.is_empty() {
        table_state.select(Some(sel_row));
    }
    f.render_stateful_widget(table, chunks[2], &mut table_state);

    render_status(f, chunks[3], state);
}

/// Render one side's cells for a strike row, preferring live ticks from the
/// quote cache over the chain snapshot.
fn side_cells(
    state: &AppState,
    contract: Option<&OptionContract>,
    call: bool,
) -> Vec<Cell<'static>> {
    let Some(c) = contract else {
        return vec![Cell::from(""); 5];
    };
    let (mut bid, mut ask) = (c.bid, c.ask);
    let mut volume = c.volume;
    if let Some(q) = state.stream.quotes.get(&c.osi_symbol) {
        if q.bid_price > 0.0 {
            bid = q.bid_price;
        }
        if q.ask_price > 0.0 {
            ask = q.ask_price;
        }
        if q.volume > 0 {
            volume = q.volume;
        }
    }
    let dim = Style::default().fg(Color::DarkGray);
    let vol_cell = Cell::from(format!("{volume}")).style(dim);
    let oi_cell = Cell::from(format!("{}", c.open_interest)).style(dim);
    let delta_cell = Cell::from(format!("{:+.2}", c.delta)).style(dim);
    let bid_cell = Cell::from(format!("{bid:.2}"));
    let ask_cell = Cell::from(format!("{ask:.2}"));
    // Column order mirrors around the strike column, matching the header:
    // calls read vol oi delta bid ask; puts read bid ask delta oi vol.
    if call {
        vec![vol_cell, oi_cell, delta_cell, bid_cell, ask_cell]
    } else {
        vec![bid_cell, ask_cell, delta_cell, oi_cell, vol_cell]
    }
}

fn render_status(f: &mut Frame, area: Rect, state: &AppState) {
    let text = format!(
        " {}  |  ←→ exp  ↑↓ strike  tab side  b buy  s sell  w/m weekly/monthly  a/i/o ATM/ITM/OTM  x clear  r refresh  Esc back",
        state.status,
    );
    f.render_widget(
        Paragraph::new(text).style(Style::default().fg(Color::DarkGray)),
        area,
    );
}
