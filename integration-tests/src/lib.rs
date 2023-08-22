mod bom;
mod provider;
mod spog;
mod vex;

pub mod runner;

pub use bom::*;
pub use provider::*;
pub use spog::*;
pub use vex::*;

use core::future::Future;
use reqwest::{StatusCode, Url};
use serde_json::Value;
use spog_api::DEFAULT_CRDA_PAYLOAD_LIMIT;
use std::{net::TcpListener, time::Duration};
use tokio::{select, time::timeout};
use trustification_auth::{
    authenticator::config::{AuthenticatorConfig, SingleAuthenticatorClientConfig},
    client::TokenInjector,
    swagger_ui::SwaggerUiOidcConfig,
};
use trustification_event_bus::{EventBusConfig, EventBusType};
use trustification_index::IndexConfig;
use trustification_infrastructure::InfrastructureConfig;
use trustification_storage::StorageConfig;

const STORAGE_ENDPOINT: &str = "http://localhost:9000";
const KAFKA_BOOTSTRAP_SERVERS: &str = "localhost:9092";
const SSO_ENDPOINT: &str = "http://localhost:8090/realms/chicken";

/// Static client secret for testing, configured in `deploy/compose/container_files/init-sso/data/client-*.json`
const SSO_TESTING_CLIENT_SECRET: &str = "R8A6KFeyxJsMDBhjfHbpZTIF0GWt43HP";

pub async fn assert_within_timeout<F: Future>(t: Duration, f: F) {
    let result = timeout(t, f).await;
    assert!(
        result.is_ok(),
        "Unable to perform operation successfully within timeout"
    );
}

pub async fn wait_for_event<F: Future>(t: Duration, events: &EventBusConfig, bus_name: &str, id: &str, f: F) {
    let bus = events.create(&prometheus::Registry::new()).await.unwrap();
    let consumer = bus.subscribe("test-client", &[bus_name]).await.unwrap();
    assert_within_timeout(t, async {
        f.await;
        loop {
            if let Ok(Some(event)) = consumer.next().await {
                let payload = event.payload().unwrap();
                if let Ok(v) = serde_json::from_slice::<Value>(payload) {
                    let key = v["key"].as_str().unwrap();
                    if key.ends_with(id) {
                        break;
                    }
                } else {
                    let key = std::str::from_utf8(payload).unwrap();
                    if key.ends_with(id) {
                        break;
                    }
                }
            } else {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    })
    .await;
}

pub async fn get_response(url: &Url, exp_status: reqwest::StatusCode, context: &ProviderContext) -> Option<Value> {
    let response = reqwest::Client::new()
        .get(url.to_owned())
        .inject_token(&context.provider_manager)
        .await
        .unwrap()
        .send()
        .await
        .unwrap();
    assert_eq!(
        exp_status,
        response.status(),
        "Expected response code does not match with actual response"
    );
    if matches!(exp_status, StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND) {
        None
    } else {
        response.json().await.unwrap()
    }
}

// Return a unique ID
pub fn id(prefix: &str) -> String {
    let uuid = uuid::Uuid::new_v4();
    format!("{prefix}-{uuid}")
}

pub trait Urlifier {
    fn base_url(&self) -> &Url;
    fn urlify<S: Into<String>>(&self, path: S) -> Url {
        self.base_url().join(&path.into()).unwrap()
    }
}

fn testing_oidc() -> AuthenticatorConfig {
    AuthenticatorConfig {
        disabled: false,
        clients: SingleAuthenticatorClientConfig {
            client_ids: vec![
                "frontend".to_string(),
                "testing-user".to_string(),
                "testing-manager".to_string(),
            ],
            issuer_url: SSO_ENDPOINT.to_string(),
            ..Default::default()
        },
    }
}

fn testing_swagger_ui_oidc() -> SwaggerUiOidcConfig {
    SwaggerUiOidcConfig {
        swagger_ui_oidc_issuer_url: Some(SSO_ENDPOINT.to_string()),
        swagger_ui_oidc_client_id: "frontend".to_string(),
    }
}
