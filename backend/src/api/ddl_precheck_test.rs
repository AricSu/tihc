use axum_test::TestServer;
use serde_json::json;
use crate::api::create_router;
use microkernel::platform::ServiceRegistry;
use std::sync::Arc;

#[tokio::test]
async fn test_ddl_precheck_endpoint() {
    // Create test server
    let registry = Arc::new(ServiceRegistry::new());
    let app = create_router(registry);
    let server = TestServer::new(app).unwrap();

    // Test safe DDL operation
    let response = server
        .post("/ddl/precheck")
        .json(&json!({
            "sql": "CREATE DATABASE test_db",
            "collation_enabled": true
        }))
        .await;

    assert_eq!(response.status_code(), 200);
    
    let body: serde_json::Value = response.json();
    assert!(body.get("is_lossy").is_some());
    assert!(body.get("risk_level").is_some());
    assert!(body.get("recommendations").is_some());
}

#[tokio::test]
async fn test_ddl_precheck_validation() {
    let registry = Arc::new(ServiceRegistry::new());
    let app = create_router(registry);
    let server = TestServer::new(app).unwrap();

    // Test empty SQL
    let response = server
        .post("/ddl/precheck")
        .json(&json!({
            "sql": "",
            "collation_enabled": true
        }))
        .await;

    assert_eq!(response.status_code(), 200);
    
    let body: serde_json::Value = response.json();
    assert_eq!(body.get("is_lossy").unwrap(), true);
    assert!(body.get("error").is_some());
}
