//! Session lifecycle management.
//!
//! Provides session creation, validation, renewal, and expiration for
//! authenticated users. Sessions are stored in-memory with automatic
//! cleanup of expired entries.

use chrono::{DateTime, Duration, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("session not found")]
    NotFound,
    #[error("session has expired")]
    Expired,
    #[error("session has been revoked")]
    Revoked,
    #[error("maximum sessions exceeded for user: {0}")]
    MaxSessionsExceeded(String),
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A user session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session ID.
    pub id: Uuid,
    /// User or service identifier.
    pub subject: String,
    /// Assigned role for this session.
    pub role: String,
    /// When the session was created.
    pub created_at: DateTime<Utc>,
    /// When the session expires.
    pub expires_at: DateTime<Utc>,
    /// When the session was last accessed.
    pub last_accessed_at: DateTime<Utc>,
    /// Whether the session has been revoked.
    pub revoked: bool,
    /// Client IP address (if known).
    pub client_ip: Option<String>,
    /// User agent string (if known).
    pub user_agent: Option<String>,
}

// ---------------------------------------------------------------------------
// Session manager
// ---------------------------------------------------------------------------

/// Configuration for session management.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Session lifetime.
    pub session_lifetime: Duration,
    /// Maximum idle time before expiration.
    pub idle_timeout: Duration,
    /// Maximum concurrent sessions per user.
    pub max_sessions_per_user: usize,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            session_lifetime: Duration::hours(24),
            idle_timeout: Duration::hours(1),
            max_sessions_per_user: 5,
        }
    }
}

/// Thread-safe session store.
pub struct SessionManager {
    sessions: RwLock<HashMap<Uuid, Session>>,
    config: SessionConfig,
}

impl SessionManager {
    pub fn new(config: SessionConfig) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            config,
        }
    }

    /// Create a new session for a subject.
    pub fn create_session(
        &self,
        subject: &str,
        role: &str,
        client_ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Session, SessionError> {
        let mut sessions = self.sessions.write();

        // Check max sessions per user
        let active_count = sessions
            .values()
            .filter(|s| s.subject == subject && !s.revoked && s.expires_at > Utc::now())
            .count();

        if active_count >= self.config.max_sessions_per_user {
            return Err(SessionError::MaxSessionsExceeded(subject.to_string()));
        }

        let now = Utc::now();
        let session = Session {
            id: Uuid::new_v4(),
            subject: subject.to_string(),
            role: role.to_string(),
            created_at: now,
            expires_at: now + self.config.session_lifetime,
            last_accessed_at: now,
            revoked: false,
            client_ip,
            user_agent,
        };

        sessions.insert(session.id, session.clone());
        Ok(session)
    }

    /// Validate and refresh a session by ID.
    pub fn validate_session(&self, session_id: Uuid) -> Result<Session, SessionError> {
        let mut sessions = self.sessions.write();
        let session = sessions
            .get_mut(&session_id)
            .ok_or(SessionError::NotFound)?;

        if session.revoked {
            return Err(SessionError::Revoked);
        }

        let now = Utc::now();
        if session.expires_at < now {
            return Err(SessionError::Expired);
        }

        // Check idle timeout
        if now - session.last_accessed_at > self.config.idle_timeout {
            return Err(SessionError::Expired);
        }

        // Touch the session
        session.last_accessed_at = now;

        Ok(session.clone())
    }

    /// Revoke a session.
    pub fn revoke_session(&self, session_id: Uuid) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write();
        let session = sessions
            .get_mut(&session_id)
            .ok_or(SessionError::NotFound)?;
        session.revoked = true;
        Ok(())
    }

    /// Revoke all sessions for a subject.
    pub fn revoke_all_sessions(&self, subject: &str) -> usize {
        let mut sessions = self.sessions.write();
        let mut count = 0;
        for session in sessions.values_mut() {
            if session.subject == subject && !session.revoked {
                session.revoked = true;
                count += 1;
            }
        }
        count
    }

    /// Remove expired and revoked sessions.
    pub fn cleanup(&self) -> usize {
        let mut sessions = self.sessions.write();
        let now = Utc::now();
        let before = sessions.len();
        sessions.retain(|_, s| !s.revoked && s.expires_at > now);
        before - sessions.len()
    }

    /// Count active sessions.
    pub fn active_session_count(&self) -> usize {
        let now = Utc::now();
        self.sessions
            .read()
            .values()
            .filter(|s| !s.revoked && s.expires_at > now)
            .count()
    }

    /// List active sessions for a subject.
    pub fn sessions_for_subject(&self, subject: &str) -> Vec<Session> {
        let now = Utc::now();
        self.sessions
            .read()
            .values()
            .filter(|s| s.subject == subject && !s.revoked && s.expires_at > now)
            .cloned()
            .collect()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(SessionConfig::default())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_manager() -> SessionManager {
        SessionManager::new(SessionConfig {
            session_lifetime: Duration::hours(1),
            idle_timeout: Duration::minutes(30),
            max_sessions_per_user: 3,
        })
    }

    #[test]
    fn create_and_validate_session() {
        let mgr = test_manager();
        let session = mgr.create_session("user-1", "admin", None, None).unwrap();
        assert_eq!(session.subject, "user-1");
        assert_eq!(session.role, "admin");

        let validated = mgr.validate_session(session.id).unwrap();
        assert_eq!(validated.subject, "user-1");
    }

    #[test]
    fn validate_nonexistent_session() {
        let mgr = test_manager();
        assert!(matches!(
            mgr.validate_session(Uuid::new_v4()),
            Err(SessionError::NotFound)
        ));
    }

    #[test]
    fn revoke_session() {
        let mgr = test_manager();
        let session = mgr.create_session("user-1", "viewer", None, None).unwrap();
        mgr.revoke_session(session.id).unwrap();
        assert!(matches!(
            mgr.validate_session(session.id),
            Err(SessionError::Revoked)
        ));
    }

    #[test]
    fn revoke_all_sessions() {
        let mgr = test_manager();
        mgr.create_session("user-1", "viewer", None, None).unwrap();
        mgr.create_session("user-1", "viewer", None, None).unwrap();
        mgr.create_session("user-2", "viewer", None, None).unwrap();

        let count = mgr.revoke_all_sessions("user-1");
        assert_eq!(count, 2);
        assert_eq!(mgr.active_session_count(), 1);
    }

    #[test]
    fn max_sessions_per_user() {
        let mgr = test_manager();
        mgr.create_session("user-1", "viewer", None, None).unwrap();
        mgr.create_session("user-1", "viewer", None, None).unwrap();
        mgr.create_session("user-1", "viewer", None, None).unwrap();
        assert!(matches!(
            mgr.create_session("user-1", "viewer", None, None),
            Err(SessionError::MaxSessionsExceeded(_))
        ));

        // Different user can still create sessions
        assert!(mgr.create_session("user-2", "viewer", None, None).is_ok());
    }

    #[test]
    fn cleanup_removes_revoked() {
        let mgr = test_manager();
        let s1 = mgr.create_session("user-1", "viewer", None, None).unwrap();
        mgr.create_session("user-2", "viewer", None, None).unwrap();
        mgr.revoke_session(s1.id).unwrap();

        let removed = mgr.cleanup();
        assert_eq!(removed, 1);
        assert_eq!(mgr.active_session_count(), 1);
    }

    #[test]
    fn sessions_for_subject() {
        let mgr = test_manager();
        mgr.create_session("user-1", "viewer", None, None).unwrap();
        mgr.create_session("user-1", "operator", None, None)
            .unwrap();
        mgr.create_session("user-2", "viewer", None, None).unwrap();

        let user1_sessions = mgr.sessions_for_subject("user-1");
        assert_eq!(user1_sessions.len(), 2);
    }

    #[test]
    fn session_stores_metadata() {
        let mgr = test_manager();
        let session = mgr
            .create_session(
                "user-1",
                "admin",
                Some("192.168.1.1".to_string()),
                Some("Mozilla/5.0".to_string()),
            )
            .unwrap();

        assert_eq!(session.client_ip.as_deref(), Some("192.168.1.1"));
        assert_eq!(session.user_agent.as_deref(), Some("Mozilla/5.0"));
    }

    #[test]
    fn expired_session_cannot_validate() {
        let mgr = SessionManager::new(SessionConfig {
            session_lifetime: Duration::seconds(-1),
            idle_timeout: Duration::hours(1),
            max_sessions_per_user: 5,
        });
        let session = mgr.create_session("user-1", "viewer", None, None).unwrap();
        assert!(matches!(
            mgr.validate_session(session.id),
            Err(SessionError::Expired)
        ));
    }
}
