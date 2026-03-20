/// Runbook executor: parse, validate, execute, log, report
/// Phase 1083

#[derive(Debug, Clone)]
pub struct RunbookExec {
    pub parse_ok: bool,
    pub validate_ok: bool,
    pub execute_ok: bool,
    pub log_ok: bool,
    pub report_ok: bool,
}

impl Default for RunbookExec {
    fn default() -> Self {
        Self::new()
    }
}

impl RunbookExec {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            validate_ok: true,
            execute_ok: true,
            log_ok: true,
            report_ok: true,
        }
    }

    pub fn preparation_ok(&self) -> bool {
        self.parse_ok && self.validate_ok
    }

    pub fn execution_ok(&self) -> bool {
        self.execute_ok && self.log_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.preparation_ok() && self.execution_ok()
    }

    pub fn needs_review(&self) -> bool {
        !self.parse_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preparation() {
        let c = RunbookExec::new();
        assert!(c.preparation_ok());
    }

    #[test]
    fn test_execution() {
        let c = RunbookExec::new();
        assert!(c.execution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RunbookExec::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_review() {
        let c = RunbookExec::new();
        assert!(!c.needs_review());
    }

    #[test]
    fn test_parse() {
        let mut c = RunbookExec::new();
        c.parse_ok = false;
        assert!(c.needs_review());
    }

    #[test]
    fn test_health() {
        let c = RunbookExec::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
