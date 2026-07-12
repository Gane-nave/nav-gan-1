//! Screen reader support — ARIA labels, semantic announcements, live regions.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// ARIA role for a UI element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AriaRole {
    Navigation,
    Main,
    Button,
    Alert,
    Status,
    Timer,
    List,
    ListItem,
    Heading,
    Region,
    Img,
    Link,
    Dialog,
    Progressbar,
    Log,
}

/// ARIA live region politeness level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LivePoliteness {
    Off,
    Polite,
    Assertive,
}

/// An accessible UI element with ARIA properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleElement {
    pub id: String,
    pub role: AriaRole,
    pub label: String,
    pub description: Option<String>,
    pub live: LivePoliteness,
    pub hidden: bool,
    pub expanded: Option<bool>,
    pub level: Option<u32>,
    pub value_now: Option<f64>,
    pub value_min: Option<f64>,
    pub value_max: Option<f64>,
}

impl AccessibleElement {
    /// Create a new accessible element.
    pub fn new(id: &str, role: AriaRole, label: &str) -> Self {
        Self {
            id: id.to_string(),
            role,
            label: label.to_string(),
            description: None,
            live: LivePoliteness::Off,
            hidden: false,
            expanded: None,
            level: None,
            value_now: None,
            value_min: None,
            value_max: None,
        }
    }

    /// Set description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Set live region politeness.
    pub fn with_live(mut self, live: LivePoliteness) -> Self {
        self.live = live;
        self
    }

    /// Set hidden state.
    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Set progress bar values.
    pub fn with_progress(mut self, now: f64, min: f64, max: f64) -> Self {
        self.role = AriaRole::Progressbar;
        self.value_now = Some(now);
        self.value_min = Some(min);
        self.value_max = Some(max);
        self
    }

    /// Validate the element has required ARIA properties.
    pub fn validate(&self) -> Vec<AccessibilityIssue> {
        let mut issues = Vec::new();

        if self.label.is_empty() && !self.hidden {
            issues.push(AccessibilityIssue {
                element_id: self.id.clone(),
                severity: IssueSeverity::Error,
                rule: "aria-label-required".to_string(),
                message: format!(
                    "Element '{}' with role {:?} is missing a label",
                    self.id, self.role
                ),
            });
        }

        if self.role == AriaRole::Heading && self.level.is_none() {
            issues.push(AccessibilityIssue {
                element_id: self.id.clone(),
                severity: IssueSeverity::Warning,
                rule: "heading-level".to_string(),
                message: format!("Heading '{}' is missing aria-level", self.id),
            });
        }

        if self.role == AriaRole::Progressbar && self.value_now.is_none() {
            issues.push(AccessibilityIssue {
                element_id: self.id.clone(),
                severity: IssueSeverity::Warning,
                rule: "progressbar-value".to_string(),
                message: format!("Progress bar '{}' is missing aria-valuenow", self.id),
            });
        }

        if self.role == AriaRole::Img && self.description.is_none() && !self.hidden {
            issues.push(AccessibilityIssue {
                element_id: self.id.clone(),
                severity: IssueSeverity::Warning,
                rule: "img-description".to_string(),
                message: format!("Image '{}' should have a description", self.id),
            });
        }

        issues
    }

    /// Generate a screen reader announcement string.
    pub fn announce(&self) -> String {
        let mut parts = Vec::new();

        match self.role {
            AriaRole::Heading => {
                if let Some(level) = self.level {
                    parts.push(format!("Heading level {}", level));
                }
            }
            AriaRole::Button => parts.push("Button".to_string()),
            AriaRole::Link => parts.push("Link".to_string()),
            AriaRole::Alert => parts.push("Alert".to_string()),
            AriaRole::Progressbar => {
                if let (Some(now), Some(max)) = (self.value_now, self.value_max) {
                    let pct = (now / max * 100.0) as u32;
                    parts.push(format!("Progress {}%", pct));
                }
            }
            _ => {}
        }

        parts.push(self.label.clone());

        if let Some(desc) = &self.description {
            parts.push(desc.clone());
        }

        parts.join(", ")
    }
}

/// Accessibility issue found during validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityIssue {
    pub element_id: String,
    pub severity: IssueSeverity,
    pub rule: String,
    pub message: String,
}

/// Issue severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

/// Screen reader announcement queue — manages ordered announcements.
pub struct AnnouncementQueue {
    queue: VecDeque<Announcement>,
    max_size: usize,
}

/// A queued announcement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub text: String,
    pub politeness: LivePoliteness,
    pub priority: u32,
}

impl AnnouncementQueue {
    /// Create a new announcement queue.
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            max_size,
        }
    }

    /// Add an announcement.
    pub fn push(&mut self, text: &str, politeness: LivePoliteness, priority: u32) {
        if self.queue.len() >= self.max_size {
            // Remove lowest priority item
            if let Some(min_idx) = self
                .queue
                .iter()
                .enumerate()
                .min_by_key(|(_, a)| a.priority)
                .map(|(i, _)| i)
            {
                if priority > self.queue[min_idx].priority {
                    self.queue.remove(min_idx);
                } else {
                    return;
                }
            }
        }

        let announcement = Announcement {
            text: text.to_string(),
            politeness,
            priority,
        };

        // Assertive announcements go to front
        if politeness == LivePoliteness::Assertive {
            self.queue.push_front(announcement);
        } else {
            self.queue.push_back(announcement);
        }
    }

    /// Pop the next announcement.
    pub fn pop(&mut self) -> Option<Announcement> {
        self.queue.pop_front()
    }

    /// Number of pending announcements.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Clear all announcements.
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessible_element_creation() {
        let el = AccessibleElement::new("nav-btn", AriaRole::Button, "Start Navigation");
        assert_eq!(el.id, "nav-btn");
        assert_eq!(el.role, AriaRole::Button);
        assert_eq!(el.label, "Start Navigation");
        assert!(!el.hidden);
    }

    #[test]
    fn test_element_with_progress() {
        let el = AccessibleElement::new("eta-progress", AriaRole::Status, "ETA Progress")
            .with_progress(75.0, 0.0, 100.0);
        assert_eq!(el.role, AriaRole::Progressbar);
        assert_eq!(el.value_now, Some(75.0));
    }

    #[test]
    fn test_validate_missing_label() {
        let el = AccessibleElement::new("btn", AriaRole::Button, "");
        let issues = el.validate();
        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule, "aria-label-required");
    }

    #[test]
    fn test_validate_heading_no_level() {
        let el = AccessibleElement::new("h1", AriaRole::Heading, "Title");
        let issues = el.validate();
        assert!(issues.iter().any(|i| i.rule == "heading-level"));
    }

    #[test]
    fn test_validate_hidden_no_label_ok() {
        let el = AccessibleElement::new("spacer", AriaRole::Region, "").with_hidden(true);
        let issues = el.validate();
        assert!(issues.is_empty());
    }

    #[test]
    fn test_announce_button() {
        let el = AccessibleElement::new("btn", AriaRole::Button, "Start");
        assert_eq!(el.announce(), "Button, Start");
    }

    #[test]
    fn test_announce_progress() {
        let el = AccessibleElement::new("p", AriaRole::Progressbar, "Loading")
            .with_progress(50.0, 0.0, 100.0);
        assert_eq!(el.announce(), "Progress 50%, Loading");
    }

    #[test]
    fn test_announcement_queue() {
        let mut queue = AnnouncementQueue::new(10);
        queue.push("Turn left", LivePoliteness::Polite, 1);
        queue.push("Recalculating", LivePoliteness::Assertive, 5);

        // Assertive goes to front
        let first = queue.pop().unwrap();
        assert_eq!(first.text, "Recalculating");
        assert_eq!(first.politeness, LivePoliteness::Assertive);

        let second = queue.pop().unwrap();
        assert_eq!(second.text, "Turn left");
    }

    #[test]
    fn test_announcement_queue_overflow() {
        let mut queue = AnnouncementQueue::new(2);
        queue.push("A", LivePoliteness::Polite, 1);
        queue.push("B", LivePoliteness::Polite, 2);
        queue.push("C", LivePoliteness::Polite, 3); // Should evict A (lowest priority)

        assert_eq!(queue.len(), 2);
        let first = queue.pop().unwrap();
        assert_eq!(first.text, "B");
    }

    #[test]
    fn test_announcement_queue_overflow_low_priority_rejected() {
        let mut queue = AnnouncementQueue::new(2);
        queue.push("A", LivePoliteness::Polite, 5);
        queue.push("B", LivePoliteness::Polite, 5);
        queue.push("C", LivePoliteness::Polite, 1); // Lower priority, rejected

        assert_eq!(queue.len(), 2);
    }
}
