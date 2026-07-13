//! Adversarial tests for gane-rope.

use gane_rope::Rope;

#[test]
fn adversarial_large_insert() {
    let big = "x".repeat(10_000);
    let rope = Rope::from_str(&big, 64);
    assert_eq!(rope.len(), 10_000);
    assert!(rope.chunk_count() > 1);
}

#[test]
fn adversarial_many_small_inserts() {
    let mut rope = Rope::new(16);
    for i in 0..1000 {
        rope.append(&format!("{}", i % 10));
    }
    assert_eq!(rope.len(), 1000);
    assert_eq!(rope.char_count(), 1000);
}

#[test]
fn adversarial_insert_delete_cycle() {
    let mut rope = Rope::from_str("hello", 64);
    for _ in 0..100 {
        rope.append(" world");
        rope.delete(5, rope.len());
    }
    assert_eq!(rope.collect_string(), "hello");
}

#[test]
fn adversarial_split_concat_roundtrip() {
    let original = "the quick brown fox jumps over the lazy dog";
    let rope = Rope::from_str(original, 8);
    let (left, right) = rope.split_at(10);
    let merged = Rope::concat(&left, &right);
    assert_eq!(merged.collect_string(), original);
}

#[test]
fn adversarial_replace_all_empty_result() {
    let mut rope = Rope::from_str("aaa", 64);
    rope.replace_all("a", "");
    assert!(rope.is_empty());
}

#[test]
fn adversarial_replace_all_grow() {
    let mut rope = Rope::from_str("ab", 64);
    rope.replace_all("a", "xxxxx");
    assert_eq!(rope.collect_string(), "xxxxxb");
}

#[test]
fn adversarial_empty_operations() {
    let mut rope = Rope::new(64);
    assert!(rope.is_empty());
    assert_eq!(rope.find("x"), None);
    rope.delete(0, 0); // no-op
    assert!(rope.is_empty());
    rope.insert(0, "");
    assert_eq!(rope.len(), 0);
}

#[test]
fn adversarial_byte_boundary() {
    // UTF-8 multi-byte characters
    let rope = Rope::from_str("héllo wörld", 4);
    assert_eq!(rope.char_count(), 11);
    assert!(rope.len() > 11); // multi-byte chars
    let found = rope.find("wörld");
    assert!(found.is_some());
}
