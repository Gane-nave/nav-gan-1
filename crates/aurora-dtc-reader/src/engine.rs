/// DTC reader: diagnostic trouble codes, freeze frame, pending codes
/// Phase 189

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DtcCategory {
    Powertrain,
    Chassis,
    Body,
    Network,
}

#[derive(Debug, Clone)]
pub struct DiagnosticCode {
    pub code: String,
    pub category: DtcCategory,
    pub description: String,
    pub is_pending: bool,
}

impl DiagnosticCode {
    pub fn severity(&self) -> u8 {
        match self.category {
            DtcCategory::Powertrain => 3,
            DtcCategory::Chassis => 3,
            DtcCategory::Body => 1,
            DtcCategory::Network => 2,
        }
    }

    pub fn is_critical(&self) -> bool {
        self.severity() >= 3 && !self.is_pending
    }
}

#[derive(Debug, Clone, Default)]
pub struct DtcReader {
    pub codes: Vec<DiagnosticCode>,
}

impl DtcReader {
    pub fn new() -> Self {
        Self { codes: Vec::new() }
    }

    pub fn active_count(&self) -> usize {
        self.codes.iter().filter(|c| !c.is_pending).count()
    }

    pub fn pending_count(&self) -> usize {
        self.codes.iter().filter(|c| c.is_pending).count()
    }

    pub fn has_critical(&self) -> bool {
        self.codes.iter().any(|c| c.is_critical())
    }

    pub fn clear_all(&mut self) {
        self.codes.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_reader() {
        let r = DtcReader::new();
        assert_eq!(r.active_count(), 0);
    }

    #[test]
    fn test_severity() {
        let c = DiagnosticCode {
            code: "P0301".into(),
            category: DtcCategory::Powertrain,
            description: "Misfire".into(),
            is_pending: false,
        };
        assert_eq!(c.severity(), 3);
    }

    #[test]
    fn test_critical() {
        let c = DiagnosticCode {
            code: "P0301".into(),
            category: DtcCategory::Powertrain,
            description: "Misfire".into(),
            is_pending: false,
        };
        assert!(c.is_critical());
    }

    #[test]
    fn test_pending_not_critical() {
        let c = DiagnosticCode {
            code: "P0301".into(),
            category: DtcCategory::Powertrain,
            description: "Misfire".into(),
            is_pending: true,
        };
        assert!(!c.is_critical());
    }

    #[test]
    fn test_clear() {
        let mut r = DtcReader::new();
        r.codes.push(DiagnosticCode {
            code: "B0001".into(),
            category: DtcCategory::Body,
            description: "Test".into(),
            is_pending: false,
        });
        r.clear_all();
        assert_eq!(r.active_count(), 0);
    }

    #[test]
    fn test_pending_count() {
        let mut r = DtcReader::new();
        r.codes.push(DiagnosticCode {
            code: "P0100".into(),
            category: DtcCategory::Powertrain,
            description: "MAF".into(),
            is_pending: true,
        });
        assert_eq!(r.pending_count(), 1);
    }
}
