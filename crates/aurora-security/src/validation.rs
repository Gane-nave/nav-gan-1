//! Input validation and sanitization.
//!
//! Validates and sanitizes API inputs to prevent injection attacks,
//! buffer overflows, and malformed data from reaching the system.

use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("field '{field}' is required")]
    Required { field: String },
    #[error("field '{field}' exceeds max length {max} (got {actual})")]
    TooLong {
        field: String,
        max: usize,
        actual: usize,
    },
    #[error("field '{field}' is below min length {min} (got {actual})")]
    TooShort {
        field: String,
        min: usize,
        actual: usize,
    },
    #[error("field '{field}' contains invalid characters")]
    InvalidCharacters { field: String },
    #[error("field '{field}' value {value} is out of range [{min}, {max}]")]
    OutOfRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },
    #[error("field '{field}' contains potentially dangerous content")]
    DangerousContent { field: String },
    #[error("invalid coordinate: latitude must be [-90, 90], got {0}")]
    InvalidLatitude(f64),
    #[error("invalid coordinate: longitude must be [-180, 180], got {0}")]
    InvalidLongitude(f64),
    #[error("invalid UUID format: {0}")]
    InvalidUuid(String),
}

// ---------------------------------------------------------------------------
// Validators
// ---------------------------------------------------------------------------

/// Validate that a string field is not empty.
pub fn require_non_empty(field: &str, value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        Err(ValidationError::Required {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validate string length is within bounds.
pub fn validate_length(
    field: &str,
    value: &str,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let len = value.len();
    if len < min {
        return Err(ValidationError::TooShort {
            field: field.to_string(),
            min,
            actual: len,
        });
    }
    if len > max {
        return Err(ValidationError::TooLong {
            field: field.to_string(),
            max,
            actual: len,
        });
    }
    Ok(())
}

/// Validate that a string contains only alphanumeric characters, hyphens,
/// and underscores (safe identifier format).
pub fn validate_identifier(field: &str, value: &str) -> Result<(), ValidationError> {
    if value
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        Ok(())
    } else {
        Err(ValidationError::InvalidCharacters {
            field: field.to_string(),
        })
    }
}

/// Validate latitude is in [-90, 90].
pub fn validate_latitude(lat: f64) -> Result<(), ValidationError> {
    if !(-90.0..=90.0).contains(&lat) || lat.is_nan() {
        Err(ValidationError::InvalidLatitude(lat))
    } else {
        Ok(())
    }
}

/// Validate longitude is in [-180, 180].
pub fn validate_longitude(lon: f64) -> Result<(), ValidationError> {
    if !(-180.0..=180.0).contains(&lon) || lon.is_nan() {
        Err(ValidationError::InvalidLongitude(lon))
    } else {
        Ok(())
    }
}

/// Validate a numeric value is within a range.
pub fn validate_range_f64(
    field: &str,
    value: f64,
    min: f64,
    max: f64,
) -> Result<(), ValidationError> {
    if value < min || value > max || value.is_nan() {
        Err(ValidationError::OutOfRange {
            field: field.to_string(),
            value: value.to_string(),
            min: min.to_string(),
            max: max.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validate a numeric value is within a range (integer).
pub fn validate_range_i64(
    field: &str,
    value: i64,
    min: i64,
    max: i64,
) -> Result<(), ValidationError> {
    if value < min || value > max {
        Err(ValidationError::OutOfRange {
            field: field.to_string(),
            value: value.to_string(),
            min: min.to_string(),
            max: max.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Check for potentially dangerous content (script injection, SQL injection patterns).
pub fn sanitize_text(field: &str, value: &str) -> Result<String, ValidationError> {
    let dangerous_patterns = [
        "<script",
        "javascript:",
        "onclick",
        "onerror",
        "onload",
        "'; DROP",
        "\" OR 1=1",
        "UNION SELECT",
        "../",
        "..\\",
    ];

    let lower = value.to_lowercase();
    for pattern in &dangerous_patterns {
        if lower.contains(&pattern.to_lowercase()) {
            return Err(ValidationError::DangerousContent {
                field: field.to_string(),
            });
        }
    }

    // Return sanitized (trimmed) value
    Ok(value.trim().to_string())
}

/// Validate a UUID string format.
pub fn validate_uuid(value: &str) -> Result<(), ValidationError> {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 5 {
        return Err(ValidationError::InvalidUuid(value.to_string()));
    }
    let expected_lens = [8, 4, 4, 4, 12];
    for (part, expected) in parts.iter().zip(expected_lens.iter()) {
        if part.len() != *expected || !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ValidationError::InvalidUuid(value.to_string()));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_non_empty_valid() {
        assert!(require_non_empty("name", "hello").is_ok());
    }

    #[test]
    fn require_non_empty_rejects_empty() {
        assert!(matches!(
            require_non_empty("name", ""),
            Err(ValidationError::Required { .. })
        ));
        assert!(matches!(
            require_non_empty("name", "   "),
            Err(ValidationError::Required { .. })
        ));
    }

    #[test]
    fn validate_length_ok() {
        assert!(validate_length("f", "hello", 1, 10).is_ok());
    }

    #[test]
    fn validate_length_too_short() {
        assert!(matches!(
            validate_length("f", "a", 3, 10),
            Err(ValidationError::TooShort { .. })
        ));
    }

    #[test]
    fn validate_length_too_long() {
        assert!(matches!(
            validate_length("f", "hello world", 1, 5),
            Err(ValidationError::TooLong { .. })
        ));
    }

    #[test]
    fn validate_identifier_ok() {
        assert!(validate_identifier("id", "my-key_123").is_ok());
    }

    #[test]
    fn validate_identifier_rejects_special() {
        assert!(matches!(
            validate_identifier("id", "my key!"),
            Err(ValidationError::InvalidCharacters { .. })
        ));
    }

    #[test]
    fn validate_latitude_ok() {
        assert!(validate_latitude(0.0).is_ok());
        assert!(validate_latitude(90.0).is_ok());
        assert!(validate_latitude(-90.0).is_ok());
    }

    #[test]
    fn validate_latitude_rejects_out_of_range() {
        assert!(matches!(
            validate_latitude(91.0),
            Err(ValidationError::InvalidLatitude(_))
        ));
        assert!(matches!(
            validate_latitude(f64::NAN),
            Err(ValidationError::InvalidLatitude(_))
        ));
    }

    #[test]
    fn validate_longitude_ok() {
        assert!(validate_longitude(0.0).is_ok());
        assert!(validate_longitude(180.0).is_ok());
        assert!(validate_longitude(-180.0).is_ok());
    }

    #[test]
    fn validate_longitude_rejects_out_of_range() {
        assert!(matches!(
            validate_longitude(181.0),
            Err(ValidationError::InvalidLongitude(_))
        ));
    }

    #[test]
    fn validate_range_f64_ok() {
        assert!(validate_range_f64("speed", 50.0, 0.0, 300.0).is_ok());
    }

    #[test]
    fn validate_range_f64_rejects() {
        assert!(matches!(
            validate_range_f64("speed", -1.0, 0.0, 300.0),
            Err(ValidationError::OutOfRange { .. })
        ));
    }

    #[test]
    fn validate_range_i64_ok() {
        assert!(validate_range_i64("port", 3000, 1, 65535).is_ok());
    }

    #[test]
    fn validate_range_i64_rejects() {
        assert!(matches!(
            validate_range_i64("port", 0, 1, 65535),
            Err(ValidationError::OutOfRange { .. })
        ));
    }

    #[test]
    fn sanitize_text_ok() {
        let result = sanitize_text("msg", "  hello world  ").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn sanitize_text_rejects_script() {
        assert!(matches!(
            sanitize_text("msg", "hello <script>alert(1)</script>"),
            Err(ValidationError::DangerousContent { .. })
        ));
    }

    #[test]
    fn sanitize_text_rejects_sql_injection() {
        assert!(matches!(
            sanitize_text("msg", "'; DROP TABLE users;--"),
            Err(ValidationError::DangerousContent { .. })
        ));
    }

    #[test]
    fn sanitize_text_rejects_path_traversal() {
        assert!(matches!(
            sanitize_text("path", "../../etc/passwd"),
            Err(ValidationError::DangerousContent { .. })
        ));
    }

    #[test]
    fn validate_uuid_ok() {
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
    }

    #[test]
    fn validate_uuid_rejects_invalid() {
        assert!(matches!(
            validate_uuid("not-a-uuid"),
            Err(ValidationError::InvalidUuid(_))
        ));
        assert!(matches!(
            validate_uuid("550e8400-e29b-41d4-a716"),
            Err(ValidationError::InvalidUuid(_))
        ));
    }
}
