//! Adversarial tests for gane-wal.

use gane_wal::WriteAheadLog;

#[test]
fn adversarial_large_log() {
    let mut wal = WriteAheadLog::new();
    for i in 0..10_000 {
        wal.append("SET", &format!("key{}", i), Some(&format!("val{}", i)));
    }
    assert_eq!(wal.len(), 10_000);
    assert_eq!(wal.last_lsn(), 10_000);
    assert!(wal.bytes_written() > 0);
}

#[test]
fn adversarial_checkpoint_and_truncate() {
    let mut wal = WriteAheadLog::new();
    for i in 0..100 {
        wal.append("SET", &format!("k{}", i), Some("v"));
    }
    wal.checkpoint(50);
    let removed = wal.truncate_checkpointed();
    assert_eq!(removed, 50);
    assert_eq!(wal.len(), 50);
    // Remaining entries should be LSN 51..100
    let replay = wal.replay_since_checkpoint();
    assert_eq!(replay.len(), 50);
}

#[test]
fn adversarial_same_key_many_writes() {
    let mut wal = WriteAheadLog::new();
    for i in 0..1000 {
        wal.append("SET", "shared-key", Some(&format!("v{}", i)));
    }
    let entries = wal.entries_for_key("shared-key");
    assert_eq!(entries.len(), 1000);
    let latest = wal.latest_for_key("shared-key").unwrap();
    assert_eq!(latest.value.as_deref(), Some("v999"));
}

#[test]
fn adversarial_checkpoint_beyond_lsn() {
    let mut wal = WriteAheadLog::new();
    wal.append("SET", "a", Some("1"));
    wal.checkpoint(100); // beyond last LSN
    assert_eq!(wal.checkpoint_lsn(), 100);
    assert_eq!(wal.uncheckpointed_count(), 0);
}

#[test]
fn adversarial_replay_empty() {
    let wal = WriteAheadLog::new();
    let replay = wal.replay_since_checkpoint();
    assert!(replay.is_empty());
}

#[test]
fn adversarial_clear_and_reuse() {
    let mut wal = WriteAheadLog::new();
    for i in 0..50 {
        wal.append("SET", &format!("k{}", i), Some("v"));
    }
    wal.clear();
    assert!(wal.is_empty());
    assert_eq!(wal.last_lsn(), 0);
    // Can still append
    let lsn = wal.append("SET", "new", Some("val"));
    assert_eq!(lsn, 1);
}

#[test]
fn adversarial_delete_operations() {
    let mut wal = WriteAheadLog::new();
    wal.append("SET", "x", Some("100"));
    wal.append("DEL", "x", None);
    let latest = wal.latest_for_key("x").unwrap();
    assert_eq!(latest.op, "DEL");
    assert!(latest.value.is_none());
}

#[test]
fn adversarial_monotonic_timestamps() {
    let mut wal = WriteAheadLog::new();
    for _ in 0..100 {
        wal.append("SET", "k", Some("v"));
    }
    let entries = wal.entries();
    for i in 1..entries.len() {
        assert!(entries[i].timestamp > entries[i - 1].timestamp);
        assert!(entries[i].lsn > entries[i - 1].lsn);
    }
}
