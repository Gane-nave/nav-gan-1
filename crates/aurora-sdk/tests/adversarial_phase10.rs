use aurora_sdk::client::*;
use aurora_sdk::extension::*;

#[test]
fn adversarial_sdk_client_builder_and_extensions() {
    // Build client with api_key
    let mut client = ClientBuilder::new()
        .api_key("test-key-123")
        .endpoint("https://api.aurora-nav.dev")
        .build()
        .unwrap();

    assert_eq!(client.state(), ClientState::Disconnected);
    client.connect().unwrap();
    assert_eq!(client.state(), ClientState::Connected);

    // Make a request
    let req = SdkRequest::new("position", serde_json::json!({"lat": 32.0}));
    let resp = client.request(req).unwrap();
    assert_eq!(resp.status, ResponseStatus::Success);
    assert_eq!(client.request_count(), 1);

    // Health check
    let health = client.health();
    assert_eq!(health.state, ClientState::Connected);
    assert_eq!(health.request_count, 1);

    // Disconnect
    client.disconnect();
    assert_eq!(client.state(), ClientState::Disconnected);

    // Extension priority ordering
    let mut reg = ExtensionRegistry::new();
    reg.register(Box::new(TimestampExtension)); // priority 50
    reg.register(Box::new(CorrelationIdExtension)); // priority 100

    let list = reg.list();
    assert_eq!(list[0].name, "correlation-id", "priority 100 must be first");
    assert_eq!(list[1].name, "timestamp", "priority 50 must be second");

    // Verify extensions actually transform requests
    let req = SdkRequest::new("test", serde_json::json!({}));
    let processed = reg.apply_request_extensions(req);
    assert!(
        processed.params.get("correlation_id").is_some(),
        "correlation_id must be added"
    );
    assert!(
        processed.params.get("sdk_timestamp").is_some(),
        "sdk_timestamp must be added"
    );
}
