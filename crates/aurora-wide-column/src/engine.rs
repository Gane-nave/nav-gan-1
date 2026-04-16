/// Wide column: family, row, cell, tombstone, compact
/// Phase 1049

#[derive(Debug, Clone)]
pub struct WideColumn {
    pub family_ok: bool,
    pub row_ok: bool,
    pub cell_ok: bool,
    pub tombstone_ok: bool,
    pub compact_ok: bool,
}

impl Default for WideColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl WideColumn {
    pub fn new() -> Self {
        Self {
            family_ok: true,
            row_ok: true,
            cell_ok: true,
            tombstone_ok: true,
            compact_ok: true,
        }
    }

    pub fn storage_ok(&self) -> bool {
        self.family_ok && self.row_ok && self.cell_ok
    }

    pub fn maintenance_ok(&self) -> bool {
        self.tombstone_ok && self.compact_ok
    }

    pub fn all_ok(&self) -> bool {
        self.storage_ok() && self.maintenance_ok()
    }

    pub fn needs_gc(&self) -> bool {
        !self.tombstone_ok || !self.compact_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.family_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let c = WideColumn::new();
        assert!(c.storage_ok());
    }

    #[test]
    fn test_maintenance() {
        let c = WideColumn::new();
        assert!(c.maintenance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WideColumn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_gc() {
        let c = WideColumn::new();
        assert!(!c.needs_gc());
    }

    #[test]
    fn test_tombstone() {
        let mut c = WideColumn::new();
        c.tombstone_ok = false;
        assert!(c.needs_gc());
    }

    #[test]
    fn test_health() {
        let c = WideColumn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
