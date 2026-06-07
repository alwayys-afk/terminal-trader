use chrono::NaiveDate;

use super::lots::{OperationKind, TaxLotStore};

/// A row in the unified timeline shown on the operations page.
#[derive(Debug, Clone)]
pub enum TimelineEntry {
    /// Index into lot_store.applied_transactions.
    Transaction { txn_index: usize },
    /// Index into lot_store.operations.
    Operation { op_index: usize },
}

impl TimelineEntry {
    pub fn date(&self, store: &TaxLotStore) -> NaiveDate {
        match self {
            TimelineEntry::Transaction { txn_index } => store.applied_transactions[*txn_index].date,
            TimelineEntry::Operation { op_index } => store.operations[*op_index].date,
        }
    }
}

/// Which field of an operation is being edited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpEditField {
    /// Multiplier: symbol, NameChange: old_symbol
    Symbol1,
    /// NameChange only: new_symbol
    Symbol2,
    /// Multiplier only: old_qty
    OldQty,
    /// Multiplier only: new_qty
    NewQty,
}

/// Which field of a transaction is being edited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TxnEditField {
    Symbol,
    Quantity,
    Price,
    LotMethod,
}

#[derive(Default)]
pub struct AddBuffer {
    pub date: String,
    pub kind: String,
    // Multiplier fields:
    pub symbol: String,
    pub old_qty: String,
    pub new_qty: String,
    // NameChange fields:
    pub old_symbol: String,
    pub new_symbol: String,
}

/// Buffer for adding a new transaction.
#[derive(Default)]
pub struct TxnAddBuffer {
    pub date: String,
    pub action: String,
    pub symbol: String,
    pub quantity: String,
    pub price: String,
}

// TODO: Collapse the modal flags (editing, adding, editing_txn, adding_txn, confirm_delete)
// into a single Mode enum to make illegal state combinations unrepresentable.
#[derive(Default)]
pub struct OperationsState {
    pub timeline: Vec<TimelineEntry>,
    pub selected: usize,
    // Operation editing.
    pub editing: Option<OpEditField>,
    pub edit_buffer: String,
    pub adding: bool,
    pub add_step: usize,
    pub add_buffer: AddBuffer,
    // Transaction editing.
    pub editing_txn: Option<TxnEditField>,
    pub adding_txn: bool,
    pub txn_add_step: usize,
    pub txn_add_buffer: TxnAddBuffer,
    /// When true, the next `d` press (or `y`) confirms the pending delete.
    pub confirm_delete: bool,
    /// Pending field edits accumulated during Tab-cycling (txn editing).
    pub pending_txn_edits: std::collections::HashMap<TxnEditField, String>,
    /// Whether focus is on the reconciliation panel (right side).
    pub recon_focus: bool,
    /// Selected row in the reconciliation panel.
    pub recon_selected: usize,
    /// Total number of rows in the reconciliation panel (set during render).
    pub recon_count: usize,
}

impl OperationsState {
    /// Rebuild the timeline by merging the already-sorted transactions and operations lists.
    pub fn rebuild_timeline(&mut self, store: &TaxLotStore) {
        let mut entries =
            Vec::with_capacity(store.applied_transactions.len() + store.operations.len());

        let mut ti = 0;
        let mut oi = 0;
        while ti < store.applied_transactions.len() && oi < store.operations.len() {
            // Operations apply before transactions on the same date.
            if store.operations[oi].date <= store.applied_transactions[ti].date {
                entries.push(TimelineEntry::Operation { op_index: oi });
                oi += 1;
            } else {
                entries.push(TimelineEntry::Transaction { txn_index: ti });
                ti += 1;
            }
        }
        for oi in oi..store.operations.len() {
            entries.push(TimelineEntry::Operation { op_index: oi });
        }
        for ti in ti..store.applied_transactions.len() {
            entries.push(TimelineEntry::Transaction { txn_index: ti });
        }

        self.timeline = entries;
        if self.selected >= self.timeline.len() && !self.timeline.is_empty() {
            self.selected = self.timeline.len() - 1;
        }
    }
}

/// Helper: get the primary symbol for display from an OperationKind.
pub fn op_symbol_text(kind: &OperationKind) -> String {
    match kind {
        OperationKind::Multiplier { symbol, .. } => symbol.clone(),
        OperationKind::NameChange {
            old_symbol,
            new_symbol,
        } => {
            format!("{} -> {}", old_symbol, new_symbol)
        }
    }
}

/// Helper: get qty display text from an OperationKind.
pub fn op_qty_text(kind: &OperationKind) -> String {
    match kind {
        OperationKind::Multiplier {
            old_qty, new_qty, ..
        } => {
            format!("{:.0} -> {:.0}", old_qty, new_qty)
        }
        OperationKind::NameChange { .. } => "--".to_string(),
    }
}
