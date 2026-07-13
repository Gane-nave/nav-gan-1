//! Test harness — test suite runner, assertions, and result reporting.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Test case status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    TimedOut,
}

/// Result of a single test case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseResult {
    pub name: String,
    pub status: TestStatus,
    pub duration_ms: u64,
    pub message: Option<String>,
    pub assertions_checked: u32,
}

/// A test suite — a collection of test cases.
pub struct TestSuite {
    name: String,
    cases: Vec<Box<dyn TestCase>>,
    timeout: Duration,
}

/// Trait for a test case.
pub trait TestCase: Send {
    /// Get the test case name.
    fn name(&self) -> &str;

    /// Run the test case. Returns Ok(()) on success or Err(message) on failure.
    fn run(&self) -> Result<u32, String>;
}

impl TestSuite {
    /// Create a new test suite.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cases: Vec::new(),
            timeout: Duration::from_secs(30),
        }
    }

    /// Set the per-test timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Add a test case.
    pub fn add_case(&mut self, case: Box<dyn TestCase>) {
        self.cases.push(case);
    }

    /// Run all test cases and return results.
    pub fn run(&self) -> TestSuiteResult {
        let suite_start = Instant::now();
        let mut results = Vec::new();

        for case in &self.cases {
            let start = Instant::now();
            let (status, message, assertions) = match case.run() {
                Ok(assertions) => (TestStatus::Passed, None, assertions),
                Err(msg) => (TestStatus::Failed, Some(msg), 0),
            };
            let duration = start.elapsed();

            let status = if duration > self.timeout {
                TestStatus::TimedOut
            } else {
                status
            };

            results.push(TestCaseResult {
                name: case.name().to_string(),
                status,
                duration_ms: duration.as_millis() as u64,
                message,
                assertions_checked: assertions,
            });
        }

        TestSuiteResult {
            suite_name: self.name.clone(),
            results,
            total_duration_ms: suite_start.elapsed().as_millis() as u64,
        }
    }

    /// Get the suite name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the number of test cases.
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }
}

/// Result of running an entire test suite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteResult {
    pub suite_name: String,
    pub results: Vec<TestCaseResult>,
    pub total_duration_ms: u64,
}

impl TestSuiteResult {
    /// Get the number of passed tests.
    pub fn passed(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == TestStatus::Passed)
            .count()
    }

    /// Get the number of failed tests.
    pub fn failed(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == TestStatus::Failed)
            .count()
    }

    /// Get the number of skipped tests.
    pub fn skipped(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == TestStatus::Skipped)
            .count()
    }

    /// Check if all tests passed.
    pub fn all_passed(&self) -> bool {
        self.results
            .iter()
            .all(|r| r.status == TestStatus::Passed || r.status == TestStatus::Skipped)
    }

    /// Get a summary string.
    pub fn summary(&self) -> String {
        format!(
            "{}: {} passed, {} failed, {} skipped ({} ms)",
            self.suite_name,
            self.passed(),
            self.failed(),
            self.skipped(),
            self.total_duration_ms
        )
    }

    /// Get failed test details.
    pub fn failures(&self) -> Vec<&TestCaseResult> {
        self.results
            .iter()
            .filter(|r| r.status == TestStatus::Failed)
            .collect()
    }
}

/// A simple function-based test case.
pub struct FnTestCase {
    name: String,
    test_fn: Box<dyn Fn() -> Result<u32, String> + Send>,
}

impl FnTestCase {
    /// Create a new function-based test case.
    pub fn new<F>(name: &str, test_fn: F) -> Self
    where
        F: Fn() -> Result<u32, String> + Send + 'static,
    {
        Self {
            name: name.to_string(),
            test_fn: Box::new(test_fn),
        }
    }
}

impl TestCase for FnTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> Result<u32, String> {
        (self.test_fn)()
    }
}

/// Assertion helper — collects assertion results.
pub struct AssertionCollector {
    checked: u32,
    failures: Vec<String>,
}

impl AssertionCollector {
    /// Create a new assertion collector.
    pub fn new() -> Self {
        Self {
            checked: 0,
            failures: Vec::new(),
        }
    }

    /// Assert that two values are equal.
    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(
        &mut self,
        actual: &T,
        expected: &T,
        context: &str,
    ) {
        self.checked += 1;
        if actual != expected {
            self.failures
                .push(format!("{context}: expected {expected:?}, got {actual:?}"));
        }
    }

    /// Assert that a condition is true.
    pub fn assert_true(&mut self, condition: bool, context: &str) {
        self.checked += 1;
        if !condition {
            self.failures
                .push(format!("{context}: expected true, got false"));
        }
    }

    /// Assert that a value is within a tolerance.
    pub fn assert_approx(&mut self, actual: f64, expected: f64, tolerance: f64, context: &str) {
        self.checked += 1;
        if (actual - expected).abs() > tolerance {
            self.failures.push(format!(
                "{context}: expected {expected} ± {tolerance}, got {actual}"
            ));
        }
    }

    /// Assert that a value is in a range.
    pub fn assert_in_range(&mut self, value: f64, min: f64, max: f64, context: &str) {
        self.checked += 1;
        if value < min || value > max {
            self.failures
                .push(format!("{context}: expected {value} in [{min}, {max}]"));
        }
    }

    /// Finish and return the result.
    pub fn finish(self) -> Result<u32, String> {
        if self.failures.is_empty() {
            Ok(self.checked)
        } else {
            Err(self.failures.join("; "))
        }
    }

    /// Get the number of assertions checked so far.
    pub fn checked(&self) -> u32 {
        self.checked
    }

    /// Get the number of failures so far.
    pub fn failure_count(&self) -> usize {
        self.failures.len()
    }
}

impl Default for AssertionCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_test_case_pass() {
        let case = FnTestCase::new("simple_pass", || Ok(1));
        assert_eq!(case.name(), "simple_pass");
        assert_eq!(case.run().unwrap(), 1);
    }

    #[test]
    fn test_fn_test_case_fail() {
        let case = FnTestCase::new("simple_fail", || Err("broken".to_string()));
        assert!(case.run().is_err());
    }

    #[test]
    fn test_test_suite_all_pass() {
        let mut suite = TestSuite::new("unit_tests");
        suite.add_case(Box::new(FnTestCase::new("a", || Ok(2))));
        suite.add_case(Box::new(FnTestCase::new("b", || Ok(3))));

        let result = suite.run();
        assert!(result.all_passed());
        assert_eq!(result.passed(), 2);
        assert_eq!(result.failed(), 0);
    }

    #[test]
    fn test_test_suite_with_failure() {
        let mut suite = TestSuite::new("mixed");
        suite.add_case(Box::new(FnTestCase::new("pass", || Ok(1))));
        suite.add_case(Box::new(FnTestCase::new(
            "fail",
            || Err("oops".to_string()),
        )));

        let result = suite.run();
        assert!(!result.all_passed());
        assert_eq!(result.passed(), 1);
        assert_eq!(result.failed(), 1);
        assert_eq!(result.failures().len(), 1);
        assert_eq!(result.failures()[0].name, "fail");
    }

    #[test]
    fn test_suite_summary() {
        let mut suite = TestSuite::new("demo");
        suite.add_case(Box::new(FnTestCase::new("t1", || Ok(1))));
        let result = suite.run();
        let summary = result.summary();
        assert!(summary.contains("demo"));
        assert!(summary.contains("1 passed"));
        assert!(summary.contains("0 failed"));
    }

    #[test]
    fn test_assertion_collector_all_pass() {
        let mut ac = AssertionCollector::new();
        ac.assert_eq(&1, &1, "int equality");
        ac.assert_true(true, "bool check");
        ac.assert_approx(3.15, 3.15, 0.01, "approx check");
        ac.assert_in_range(5.0, 0.0, 10.0, "range");

        assert_eq!(ac.checked(), 4);
        assert_eq!(ac.failure_count(), 0);
        assert!(ac.finish().is_ok());
    }

    #[test]
    fn test_assertion_collector_with_failures() {
        let mut ac = AssertionCollector::new();
        ac.assert_eq(&1, &2, "wrong int");
        ac.assert_true(false, "wrong bool");
        ac.assert_approx(10.0, 5.0, 0.1, "wrong float");
        ac.assert_in_range(100.0, 0.0, 10.0, "out of range");

        assert_eq!(ac.checked(), 4);
        assert_eq!(ac.failure_count(), 4);
        let result = ac.finish();
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("wrong int"));
        assert!(msg.contains("wrong bool"));
    }

    #[test]
    fn test_test_case_result_serialization() {
        let result = TestCaseResult {
            name: "test_serialize".to_string(),
            status: TestStatus::Passed,
            duration_ms: 42,
            message: None,
            assertions_checked: 5,
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("test_serialize"));
        let deserialized: TestCaseResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.status, TestStatus::Passed);
    }
}
