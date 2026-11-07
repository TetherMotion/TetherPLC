use crate::model::NodeId;

/// Monotonic per-POU allocator. Ids are never reused after delete —
/// diagnostics, symmap entries and breakpoints key on them, so an
/// undo-restore must get a NEW id (doc 06 §6).
#[derive(Debug, Clone)]
pub struct IdAlloc {
    next: NodeId,
}

impl IdAlloc {
    pub fn new() -> Self {
        IdAlloc { next: 1 }
    }

    /// Resume allocation above every id already present in the network
    /// (load path — the file may carry ids allocated by another session).
    pub fn over<'a>(&mut self, ids: impl Iterator<Item = &'a NodeId>) -> &mut Self {
        for &id in ids {
            if id >= self.next {
                self.next = id + 1;
            }
        }
        self
    }

    pub fn alloc(&mut self) -> NodeId {
        let id = self.next;
        self.next += 1;
        id
    }

    pub fn peek(&self) -> NodeId {
        self.next
    }
}

impl Default for IdAlloc {
    fn default() -> Self {
        Self::new()
    }
}
