use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::app::{AppState, Page};
use crate::schwab::{Instruction, OrderStatus, OrderType};
use crate::tui::command::ListNav;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title / tab bar
            Constraint::Min(0),    // table
            Constraint::Length(1), // status bar
        ])
        .split(area);

    render_title(f, chunks[0], state);
    render_table(f, chunks[1], state);
    render_status(f, chunks[2], state);
}

fn render_title(f: &mut Frame, area: Rect, state: &AppState) {
    let refresh_text = state.refresh_text();

    let active = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let inactive = Style::default().fg(Color::DarkGray);

    let (pos_style, ord_style) = match state.current_page {
        Page::Positions | Page::LotDetail(_) | Page::Operations => (active, inactive),
        Page::Orders => (inactive, active),
    };

    let mut spans = vec![
        Span::styled("[1] ", inactive),
        Span::styled("positions", pos_style),
        Span::styled("  [2] ", inactive),
        Span::styled("ORDERS", ord_style),
    ];
    spans.push(Span::styled("  [3] ", inactive));
    spans.push(Span::styled("operations", inactive));
    spans.push(Span::styled(refresh_text, inactive));

    let title = Paragraph::new(Line::from(spans));
    f.render_widget(title, area);
}

fn render_table(f: &mut Frame, area: Rect, state: &AppState) {
    let header = Row::new([
        "Symbol", "Side", "Type", "Qty", "Filled", "Price", "Fill Px", "Last", "Status", "Time",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .height(1);

    let rows: Vec<Row> = state
        .orders
        .items()
        .iter()
        .map(|order| {
            let status_color = match order.status {
                OrderStatus::Filled => Color::Green,
                OrderStatus::Canceled | OrderStatus::Rejected | OrderStatus::Expired => Color::Red,
                OrderStatus::Working | OrderStatus::PendingActivation | OrderStatus::Queued => {
                    Color::Cyan
                }
                _ => Color::White,
            };

            let last = state
                .stream
                .quotes
                .get(&order.symbol)
                .map(|q| format!("{:.2}", q.last_price))
                .unwrap_or_else(|| "--".to_string());

            let fill_px = order
                .fill_price
                .map(|p| format!("{p:.2}"))
                .unwrap_or_else(|| "--".to_string());

            Row::new(vec![
                Cell::from(order.symbol.clone()),
                Cell::from(format_instruction(order.instruction)),
                Cell::from(format_order_type(order.order_type)),
                Cell::from(format!("{:.0}", order.quantity)),
                Cell::from(format!("{:.0}", order.filled_quantity)),
                Cell::from(format_price(order)),
                Cell::from(fill_px),
                Cell::from(last),
                Cell::from(format_status(order.status)).style(Style::default().fg(status_color)),
                Cell::from(format_time(&order.entered_time)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(16), // Symbol
        Constraint::Length(8),  // Side
        Constraint::Length(8),  // Type
        Constraint::Length(6),  // Qty
        Constraint::Length(6),  // Filled
        Constraint::Length(10), // Price
        Constraint::Length(8),  // Fill Px
        Constraint::Length(8),  // Last
        Constraint::Length(14), // Status
        Constraint::Length(11), // Time
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Orders "))
        .column_spacing(1)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    let mut table_state = TableState::default();
    if !state.orders.items().is_empty() {
        table_state.select(Some(state.orders.selected_idx()));
    }

    f.render_stateful_widget(table, area, &mut table_state);
}

fn render_status(f: &mut Frame, area: Rect, state: &AppState) {
    let copy_hint = if state.status.starts_with("Error:") {
        "  y copy"
    } else {
        ""
    };
    let text = format!(
        " {}  |  ↑↓ nav  g/G jump  x cancel  e edit  r repeat  R dup  o opp  p pos  : cmd  1 pos  3 ops  q quit{copy_hint}",
        state.status
    );
    let status = Paragraph::new(text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(status, area);
}

// ── Formatters ────────────────────────────────────────────────────────────────

fn format_instruction(instr: Instruction) -> &'static str {
    match instr {
        Instruction::Buy => "BUY",
        Instruction::Sell => "SELL",
        Instruction::BuyToCover => "BUY COV",
        Instruction::SellShort => "SHORT",
        Instruction::BuyToOpen => "BTO",
        Instruction::BuyToClose => "BTC",
        Instruction::SellToOpen => "STO",
        Instruction::SellToClose => "STC",
        Instruction::Exchange => "EXCH",
        Instruction::SellShortExempt => "SHT EX",
    }
}

fn format_order_type(ot: OrderType) -> &'static str {
    match ot {
        OrderType::Market => "MKT",
        OrderType::Limit => "LMT",
        OrderType::Stop => "STP",
        OrderType::StopLimit => "STP LMT",
        OrderType::TrailingStop => "TRAIL",
        OrderType::TrailingStopLimit => "TRAIL L",
        OrderType::MarketOnClose => "MOC",
        OrderType::LimitOnClose => "LOC",
        OrderType::Exercise => "EXER",
        OrderType::NetDebit => "NET DB",
        OrderType::NetCredit => "NET CR",
        OrderType::NetZero => "NET 0",
        OrderType::Cabinet => "CAB",
        OrderType::NonMarketable => "NON-MKT",
        OrderType::Unknown => "UNK",
    }
}

fn format_status(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Working => "WORKING",
        OrderStatus::Filled => "FILLED",
        OrderStatus::Canceled => "CANCELED",
        OrderStatus::PendingCancel => "PEND CANCEL",
        OrderStatus::Rejected => "REJECTED",
        OrderStatus::Expired => "EXPIRED",
        OrderStatus::Replaced => "REPLACED",
        OrderStatus::PendingReplace => "PEND REPLACE",
        OrderStatus::Queued => "QUEUED",
        OrderStatus::New => "NEW",
        OrderStatus::Accepted => "ACCEPTED",
        OrderStatus::PendingActivation => "PEND ACTIVE",
        OrderStatus::PendingAcknowledgement => "PEND ACK",
        OrderStatus::PendingRecall => "PEND RECALL",
        OrderStatus::AwaitingParentOrder => "AWAIT PARENT",
        OrderStatus::AwaitingCondition => "AWAIT COND",
        OrderStatus::AwaitingStopCondition => "AWAIT STOP",
        OrderStatus::AwaitingManualReview => "AWAIT REVIEW",
        OrderStatus::AwaitingUrOut => "AWAIT UR OUT",
        OrderStatus::AwaitingReleaseTime => "AWAIT REL",
        OrderStatus::Unknown => "UNKNOWN",
    }
}

fn format_price(order: &crate::schwab::Order) -> String {
    match order.order_type {
        OrderType::Market | OrderType::MarketOnClose => "MKT".to_string(),
        OrderType::Stop => order
            .stop_price
            .map(|p| format!("{p:.2}"))
            .unwrap_or_else(|| "--".to_string()),
        OrderType::StopLimit => {
            let stop = order
                .stop_price
                .map(|p| format!("{p:.2}"))
                .unwrap_or_else(|| "--".to_string());
            let lim = order
                .price
                .map(|p| format!("{p:.2}"))
                .unwrap_or_else(|| "--".to_string());
            format!("{stop}/{lim}")
        }
        _ => order
            .price
            .map(|p| format!("{p:.2}"))
            .unwrap_or_else(|| "--".to_string()),
    }
}

fn format_time(dt: &chrono::DateTime<chrono::Utc>) -> String {
    use chrono::TimeZone;
    let eastern = chrono_tz::America::New_York;
    eastern
        .from_utc_datetime(&dt.naive_utc())
        .format("%m/%d %H:%M")
        .to_string()
}
