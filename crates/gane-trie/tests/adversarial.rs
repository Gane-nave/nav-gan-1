use gane_trie::trie::PrefixTrie;

#[test]
fn adversarial_prefix_search_autocomplete_remove() {
    let mut trie = PrefixTrie::new();

    // Insert 10 keys with common prefixes
    trie.insert("apple", "v_apple");
    trie.insert("app", "v_app");
    trie.insert("application", "v_application");
    trie.insert("apply", "v_apply");
    trie.insert("ape", "v_ape");
    trie.insert("banana", "v_banana");
    trie.insert("band", "v_band");
    trie.insert("ban", "v_ban");
    trie.insert("bat", "v_bat");
    trie.insert("bar", "v_bar");
    assert_eq!(trie.len(), 10);

    // Prefix search "app" should return apple, app, application, apply
    let app_keys = trie.keys_with_prefix("app");
    assert_eq!(app_keys.len(), 4);
    assert!(app_keys.contains(&"app".to_string()));
    assert!(app_keys.contains(&"apple".to_string()));
    assert!(app_keys.contains(&"application".to_string()));
    assert!(app_keys.contains(&"apply".to_string()));

    // Prefix search "ban" should return banana, band, ban
    let ban_keys = trie.keys_with_prefix("ban");
    assert_eq!(ban_keys.len(), 3);
    assert!(ban_keys.contains(&"banana".to_string()));
    assert!(ban_keys.contains(&"band".to_string()));
    assert!(ban_keys.contains(&"ban".to_string()));

    // Prefix search "ba" should return banana, band, ban, bat, bar
    let ba_keys = trie.keys_with_prefix("ba");
    assert_eq!(ba_keys.len(), 5);

    // Autocomplete with limit — should return sorted subset
    let auto = trie.autocomplete("app", 2);
    assert_eq!(auto.len(), 2);
    assert!(auto[0] <= auto[1]);

    // Autocomplete with limit larger than results
    let auto_all = trie.autocomplete("app", 100);
    assert_eq!(auto_all.len(), 4);

    // Remove "app" — prefix search should still find apple, application, apply
    assert!(trie.remove("app").is_some());
    assert_eq!(trie.len(), 9);
    let app_after = trie.keys_with_prefix("app");
    assert_eq!(app_after.len(), 3);
    assert!(!app_after.contains(&"app".to_string()));
    assert!(app_after.contains(&"apple".to_string()));

    // Remove "apple" — prefix search should find application, apply
    assert!(trie.remove("apple").is_some());
    assert_eq!(trie.len(), 8);
    let app_after2 = trie.keys_with_prefix("app");
    assert_eq!(app_after2.len(), 2);

    // Verify get returns None for removed keys
    assert!(trie.get("app").is_none());
    assert!(trie.get("apple").is_none());

    // Verify remaining keys still accessible
    assert_eq!(trie.get("application").unwrap(), "v_application");
    assert_eq!(trie.get("apply").unwrap(), "v_apply");
    assert_eq!(trie.get("banana").unwrap(), "v_banana");

    // has_prefix should still work
    assert!(trie.has_prefix("app"));
    assert!(trie.has_prefix("ban"));
    assert!(!trie.has_prefix("cat"));

    // Remove all "ba" keys — verify prefix search returns empty
    assert!(trie.remove("banana").is_some());
    assert!(trie.remove("band").is_some());
    assert!(trie.remove("ban").is_some());
    assert!(trie.remove("bat").is_some());
    assert!(trie.remove("bar").is_some());
    let ba_after = trie.keys_with_prefix("ba");
    assert_eq!(ba_after.len(), 0);
    assert!(!trie.has_prefix("ba"));

    // Hit rate tracking
    let rate = trie.hit_rate();
    assert!((0.0..=1.0).contains(&rate));

    // Stats
    assert_eq!(trie.total_inserts(), 10);
    assert_eq!(trie.total_removes(), 7);
}
