use std::{collections::HashSet, time::Duration};

use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use cel::{Value, Context, Program, objects::{Map, Key}};
use std::sync::Arc;

#[cfg(debug_assertions)]
use std::time::Instant;
#[cfg(debug_assertions)]
use tracing::{debug, warn};

use super::types::EventTypeName;

/// Maximum length for CEL expressions to prevent abuse
const MAX_CEL_EXPRESSION_LENGTH: usize = 2048;

/// Maximum complexity for CEL expressions (approximate token count)
const MAX_CEL_COMPLEXITY: usize = 50;

/// Warning threshold for CEL expression execution time (milliseconds)
#[cfg(debug_assertions)]
const CEL_EXECUTION_WARNING_THRESHOLD_MS: u128 = 10;

/// Error threshold for CEL expression execution time (milliseconds)
#[cfg(debug_assertions)]
const CEL_EXECUTION_ERROR_THRESHOLD_MS: u128 = 100;


/// CEL expression execution performance metrics
/// Only available when debug mode is enabled
#[cfg(debug_assertions)]
#[derive(Debug, Clone)]
pub struct CelExecutionMetrics {
    pub expression: String,
    pub execution_time_ms: u128,
    pub complexity: usize,
    pub success: bool,
    pub error_message: Option<String>,
}

#[cfg(debug_assertions)]
impl CelExecutionMetrics {
    pub fn new(expression: String, execution_time_ms: u128, complexity: usize, success: bool, error_message: Option<String>) -> Self {
        Self {
            expression,
            execution_time_ms,
            complexity,
            success,
            error_message,
        }
    }
    
    /// Log performance metrics with appropriate level based on execution time
    pub fn log_performance(&self) {
        if self.execution_time_ms >= CEL_EXECUTION_ERROR_THRESHOLD_MS {
            warn!(
                "CEL expression execution exceeded error threshold: {}ms (threshold: {}ms) - expression: '{}', complexity: {}, success: {}",
                self.execution_time_ms,
                CEL_EXECUTION_ERROR_THRESHOLD_MS,
                self.expression,
                self.complexity,
                self.success
            );
        } else if self.execution_time_ms >= CEL_EXECUTION_WARNING_THRESHOLD_MS {
            warn!(
                "CEL expression execution exceeded warning threshold: {}ms (threshold: {}ms) - expression: '{}', complexity: {}, success: {}",
                self.execution_time_ms,
                CEL_EXECUTION_WARNING_THRESHOLD_MS,
                self.expression,
                self.complexity,
                self.success
            );
        } else {
            debug!(
                "CEL expression executed: {}ms - expression: '{}', complexity: {}, success: {}",
                self.execution_time_ms,
                self.expression,
                self.complexity,
                self.success
            );
        }
        
        if let Some(ref error) = self.error_message {
            warn!("CEL expression execution failed: {}", error);
        }
    }
}

use crate::{
    core::{
        cache::{kv_def, Cache, CacheBehavior, CacheKey, CacheValue},
        types::{
            ApplicationId, ApplicationUid, EndpointHeaders, EndpointId, EndpointSecretInternal,
            EventChannelSet, EventTypeNameSet, ExpiringSigningKeys, MessageAttemptTriggerType,
            OrganizationId,
        },
    },
    db::models::{application, endpoint},
    error::{Error, Result},
};

/// Convert JSON value to CEL value
fn json_to_cel_value(json: &JsonValue) -> Result<Value, String> {
    match json {
        JsonValue::Null => Ok(Value::Null),
        JsonValue::Bool(b) => Ok(Value::Bool(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Int(i))
            } else if let Some(u) = n.as_u64() {
                Ok(Value::UInt(u))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Float(f))
            } else {
                Err("Invalid number format".to_string())
            }
        }
        JsonValue::String(s) => Ok(Value::String(Arc::new(s.clone()))),
        JsonValue::Array(arr) => {
            let mut cel_array = Vec::new();
            for item in arr {
                cel_array.push(json_to_cel_value(item)?);
            }
            Ok(Value::List(Arc::new(cel_array)))
        }
        JsonValue::Object(obj) => {
            let mut cel_map = std::collections::HashMap::new();
            for (k, v) in obj {
                cel_map.insert(Key::String(Arc::new(k.clone())), json_to_cel_value(v)?);
            }
            Ok(Value::Map(Map { map: Arc::new(cel_map) }))
        }
    }
}

/// Calculate the approximate complexity of a CEL expression
/// This is a simple heuristic based on counting operators, function calls, and field accesses
fn calculate_cel_complexity(expr: &str) -> usize {
    let mut complexity = 0;
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        match chars[i] {
            // Logical operators
            '&' if i + 1 < chars.len() && chars[i + 1] == '&' => {
                complexity += 2;
                i += 2; // Skip both characters
                continue;
            }
            '|' if i + 1 < chars.len() && chars[i + 1] == '|' => {
                complexity += 2;
                i += 2; // Skip both characters
                continue;
            }
            // Comparison operators
            '=' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                complexity += 2;
                i += 2; // Skip both characters
                continue;
            }
            '!' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                complexity += 2;
                i += 2; // Skip both characters
                continue;
            }
            '<' | '>' => complexity += 1,
            // Arithmetic operators
            '+' | '-' | '*' | '/' | '%' => complexity += 1,
            // Field access and function calls
            '.' => complexity += 1,
            '(' => complexity += 1,
            '[' => complexity += 1,
            // String literals (approximate)
            '\'' | '"' => {
                complexity += 1;
                // Skip to end of string
                let quote_char = chars[i];
                i += 1;
                while i < chars.len() && chars[i] != quote_char {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        i += 1; // Skip escaped character
                    }
                    i += 1;
                }
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    
    complexity
}

/// Validates a CEL expression by attempting to compile it
/// Returns Ok(()) if the expression is valid, Err(String) with error message if invalid
pub fn validate_cel_expression(expr: &str) -> Result<(), String> {
    if expr.trim().is_empty() {
        return Ok(());
    }

    // Check expression length limit
    if expr.len() > MAX_CEL_EXPRESSION_LENGTH {
        return Err(format!(
            "CEL expression too long: {} characters (max: {})",
            expr.len(),
            MAX_CEL_EXPRESSION_LENGTH
        ));
    }

    // Check expression complexity limit
    let complexity = calculate_cel_complexity(expr);
    if complexity > MAX_CEL_COMPLEXITY {
        return Err(format!(
            "CEL expression too complex: {} complexity score (max: {})",
            complexity,
            MAX_CEL_COMPLEXITY
        ));
    }

    // Use panic-safe compilation for validation
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Program::compile(expr)
    }));
    
    match result {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_e)) => Err(format!("CEL compilation error")),
        Err(_) => Err(format!("CEL compilation panic")),
    }
}

/// The information cached during the creation of a message. Includes a [`Vec`] of all endpoints
/// associated with the given application and organization ID.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateMessageApp {
    pub id: ApplicationId,
    pub uid: Option<ApplicationUid>,
    pub org_id: OrganizationId,
    pub rate_limit: Option<u16>,
    endpoints: Vec<CreateMessageEndpoint>,
    deleted: bool,
}

impl CreateMessageApp {
    /// Fetch all requisite information for creating a [`CreateMessageApp`] from the PostgreSQL
    /// database
    async fn fetch_from_pg_by_model(
        db: &DatabaseTransaction,
        app: application::Model,
    ) -> Result<CreateMessageApp> {
        let endpoints = endpoint::Entity::secure_find(app.id.clone())
            .all(db)
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;

        Ok(CreateMessageApp {
            id: app.id,
            uid: app.uid,
            org_id: app.org_id,
            rate_limit: app
                .rate_limit
                .map(|v| v.try_into())
                .transpose()
                .map_err(|_| Error::validation("Application rate limit out of bounds"))?,
            endpoints,
            deleted: app.deleted,
        })
    }

    /// Fetches all information for creating a [`CreateMessageApp`] from the Redis cache if it
    /// exists or from PostgreSQL otherwise. If the RedisCache is Some, but does not contain the
    /// requisite information, fetch it from PostgreSQL and insert the data into the cache.
    pub async fn layered_fetch(
        cache: &Cache,
        pg: &DatabaseConnection,
        app: Option<application::Model>,
        org_id: OrganizationId,
        app_id: ApplicationId,
        ttl: Duration,
    ) -> Result<Option<CreateMessageApp>> {
        let cache_key = AppEndpointKey::new(&org_id, &app_id);

        // First check Redis
        if let Ok(Some(cma)) = cache.get::<CreateMessageApp>(&cache_key).await {
            if cma.deleted {
                return Ok(None);
            } else {
                return Ok(Some(cma));
            }
        }

        // Then check PostgreSQL
        let db = pg.begin().await?;
        // Fetch the [`application::Model`] either given or from the ID
        let app = if let Some(app) = app {
            app
        } else if let Some(app) = application::Entity::secure_find_by_id(org_id, app_id)
            .one(&db)
            .await?
        {
            app
        } else {
            return Ok(None);
        };

        // Fetch the actual [`CreateMessageApp`]
        let out = Self::fetch_from_pg_by_model(&db, app).await?;

        // Insert it into Redis
        let _ = cache.set(&cache_key, &out, ttl).await;

        if out.deleted {
            return Ok(None);
        }

        Ok(Some(out))
    }

    pub fn filtered_endpoints(
        &self,
        trigger_type: MessageAttemptTriggerType,
        event_type: &EventTypeName,
        channels: Option<&EventChannelSet>,
        payload: Option<&JsonValue>,
    ) -> Vec<CreateMessageEndpoint> {
        self.endpoints
            .iter()
            .filter(|endpoint| {
                // No disabled or deleted endpoints ever
                !endpoint.disabled && !endpoint.deleted
                    // Manual attempt types go through regardless
                    && (trigger_type == MessageAttemptTriggerType::Manual
                        || (
                            // If an endpoint has event types and it matches ours, or has no event types
                            endpoint
                                .event_types_ids
                                .as_ref()
                                .map(|x| x.0.contains(event_type))
                                .unwrap_or(true)
                            // If an endpoint has no channels accept all messages, otherwise only if their channels overlap.
                            // A message with no channels doesn't match an endpoint with channels.
                            && endpoint
                                .channels
                                .as_ref()
                                .map(|x| {
                                    !x.0.is_disjoint(
                                        channels.map(|x| &x.0).unwrap_or(&HashSet::new()),
                                    )
                                })
                                .unwrap_or(true)
                        ))
            })
            .filter(|endpoint| {

                tracing::debug!(
                    "endpoint.filter: {:?}, payload: {:?}",
                    endpoint.filter,
                    payload
                );

                match (&endpoint.filter, payload) {
                    (Some(expr), Some(payload_json)) => {
                        // Start performance monitoring (only in debug mode)
                        #[cfg(debug_assertions)]
                        let start_time = Instant::now();
                        #[cfg(debug_assertions)]
                        let complexity = calculate_cel_complexity(expr);
                        #[cfg(debug_assertions)]
                        let mut success = false;
                        #[cfg(debug_assertions)]
                        let mut error_message = None;
                        
                        let payload_value = match json_to_cel_value(payload_json) {
                            Ok(v) => v,
                            Err(e) => {
                                tracing::debug!("JSON to CEL conversion error: {:?}", e);
                                return false;
                            }
                        };

                        tracing::debug!(
                            "payload_value: {:?}",
                            payload_value
                        );

                        let result = (|| -> bool {
                            let mut context = Context::default();
                            
                            if let Some(payload_obj) = payload_json.as_object() {
                                for (key, value) in payload_obj {
                                    match json_to_cel_value(value) {
                                        Ok(cel_value) => {
                                            if let Err(e) = context.add_variable(key, cel_value) {
                                                #[cfg(debug_assertions)]
                                                {
                                                    error_message = Some(format!("Failed to add variable '{}': {}", key, e));
                                                }
                                                tracing::debug!("Failed to add variable '{}': {:?}", key, e);
                                                return false;
                                            }
                                        }
                                        Err(e) => {
                                            #[cfg(debug_assertions)]
                                            {
                                                error_message = Some(format!("Failed to convert field '{}' to CEL value: {}", key, e));
                                            }
                                            tracing::debug!("Failed to convert field '{}' to CEL value: {:?}", key, e);
                                            return false;
                                        }
                                    }
                                }
                            } else {
                                #[cfg(debug_assertions)]
                                {
                                    error_message = Some(format!("Payload is not a JSON object"));
                                }
                                tracing::debug!("Payload is not a JSON object");
                                return false;
                            }
                            
                            let program = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                Program::compile(expr)
                            })) {
                                Ok(Ok(program)) => program,
                                Ok(Err(e)) => {
                                    #[cfg(debug_assertions)]
                                    {
                                        error_message = Some(format!("Failed to compile CEL expression: {:?}", e));
                                    }
                                    tracing::debug!("CEL compile error: {:?}", e);
                                    return false;
                                }
                                Err(_) => {
                                    #[cfg(debug_assertions)]
                                    {
                                        error_message = Some(format!("CEL compilation panic"));
                                    }
                                    tracing::debug!("CEL compilation panic");
                                    return false;
                                }
                            };
                            
                            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                program.execute(&context)
                            })) {
                                Ok(Ok(val)) => {
                                    #[cfg(debug_assertions)]
                                    {
                                        success = true;
                                    }
                                    match val {
                                        Value::Bool(b) => b,
                                        _ => false,
                                    }
                                }
                                Ok(Err(_e)) => {
                                    #[cfg(debug_assertions)]
                                    {
                                        error_message = Some(format!("Failed to execute CEL expression: {}", _e));
                                    }
                                    false
                                }
                                Err(_) => {
                                    #[cfg(debug_assertions)]
                                    {
                                        error_message = Some(format!("CEL execution panic"));
                                    }
                                    tracing::debug!("CEL execution panic");
                                    false
                                }
                            }
                        })();
                        
                        // Record performance metrics (only in debug mode)
                        #[cfg(debug_assertions)]
                        {
                            let execution_time = start_time.elapsed();
                            let execution_time_ms = execution_time.as_millis();
                            
                            let metrics = CelExecutionMetrics::new(
                                expr.clone(),
                                execution_time_ms,
                                complexity,
                                success,
                                error_message,
                            );
                            
                            // Log performance metrics
                            metrics.log_performance();
                        }
                        
                        result
                    }
                    _ => true,
                }
            })
            .cloned()
            .collect()
    }
}

/// The information for each individual endpoint cached with the creation of a message.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateMessageEndpoint {
    pub id: EndpointId,
    pub url: String,
    pub key: EndpointSecretInternal,
    pub event_types_ids: Option<EventTypeNameSet>,
    pub channels: Option<EventChannelSet>,
    pub filter: Option<String>,
    pub rate_limit: Option<u16>,
    // Same type as the `DateTimeWithTimeZone from SeaORM used in the endpoint model
    pub first_failure_at: Option<DateTime<FixedOffset>>,
    pub headers: Option<EndpointHeaders>,
    pub disabled: bool,
    pub deleted: bool,
    // outside of this module, valid_signing_keys should be used instead
    old_signing_keys: Option<ExpiringSigningKeys>,
}

impl CreateMessageEndpoint {
    pub fn valid_signing_keys(&self) -> Vec<&EndpointSecretInternal> {
        match self.old_signing_keys {
            Some(ref old_keys) => std::iter::once(&self.key)
                .chain(
                    old_keys
                        .0
                        .iter()
                        .filter(|x| x.expiration > Utc::now())
                        .map(|x| &x.key),
                )
                .collect(),
            None => vec![&self.key],
        }
    }
}

impl TryFrom<endpoint::Model> for CreateMessageEndpoint {
    type Error = Error;

    fn try_from(m: endpoint::Model) -> Result<CreateMessageEndpoint> {
        Ok(CreateMessageEndpoint {
            id: m.id,
            url: m.url,
            key: m.key,
            old_signing_keys: m.old_keys,
            event_types_ids: m.event_types_ids,
            channels: m.channels,
            filter: m.filter,
            rate_limit: m
                .rate_limit
                .map(|v| v.try_into())
                .transpose()
                .map_err(|_| Error::validation("Endpoint rate limit out of bounds"))?,
            first_failure_at: m.first_failure_at,
            headers: m.headers,
            disabled: m.disabled,
            deleted: m.deleted,
        })
    }
}

kv_def!(AppEndpointKey, CreateMessageApp);
impl AppEndpointKey {
    // FIXME: Rewrite doc comment when AppEndpointValue members are known
    /// Returns a key for fetching all cached endpoints for a given organization and application.
    pub fn new(org: &OrganizationId, app: &ApplicationId) -> AppEndpointKey {
        AppEndpointKey(format!("SVIX_CACHE_APP_v3_{org}_{app}"))
    }
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use chrono::Utc;

    use super::{CreateMessageApp, CreateMessageEndpoint};
    use crate::core::{
        cryptography::Encryption,
        types::{
            EndpointId, EndpointSecret, EndpointSecretInternal, ExpiringSigningKey,
            ExpiringSigningKeys,
        },
    };

    #[test]
    fn test_valid_signing_keys() {
        let key = EndpointSecretInternal::from_endpoint_secret(
            EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
            &Encryption::new_noop(),
        )
        .unwrap();

        let unexpired_old_key = ExpiringSigningKey {
            key: key.clone(),
            expiration: Utc::now()
                + chrono::Duration::hours(ExpiringSigningKeys::OLD_KEY_EXPIRY_HOURS),
        };
        let expired_old_key = ExpiringSigningKey {
            key: key.clone(),
            expiration: Utc::now()
                - chrono::Duration::hours(ExpiringSigningKeys::OLD_KEY_EXPIRY_HOURS),
        };
        let old_signing_keys = Some(ExpiringSigningKeys(vec![
            unexpired_old_key,
            expired_old_key,
        ]));

        let cme = CreateMessageEndpoint {
            id: EndpointId::from("Test".to_string()),
            url: "".to_string(),
            key,
            old_signing_keys,
            event_types_ids: None,
            channels: None,
            filter: None,
            rate_limit: None,
            first_failure_at: None,
            headers: None,
            disabled: false,
            deleted: false,
        };

        let keys = cme.valid_signing_keys();

        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn test_filtered_endpoints_with_cel_filter() {
        use crate::core::types::{ApplicationId, EventTypeName, MessageAttemptTriggerType, OrganizationId};
        use serde_json::json;

        // Create test endpoints
        let endpoint_with_filter = CreateMessageEndpoint {
            id: EndpointId::from("filtered_endpoint".to_string()),
            url: "https://example.com/filtered".to_string(),
            key: EndpointSecretInternal::from_endpoint_secret(
                EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                &Encryption::new_noop(),
            ).unwrap(),
            old_signing_keys: None,
            event_types_ids: None,
            channels: None,
            filter: Some("age >= 18".to_string()),
            rate_limit: None,
            first_failure_at: None,
            headers: None,
            disabled: false,
            deleted: false,
        };

        let endpoint_without_filter = CreateMessageEndpoint {
            id: EndpointId::from("unfiltered_endpoint".to_string()),
            url: "https://example.com/unfiltered".to_string(),
            key: EndpointSecretInternal::from_endpoint_secret(
                EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                &Encryption::new_noop(),
            ).unwrap(),
            old_signing_keys: None,
            event_types_ids: None,
            channels: None,
            filter: None,
            rate_limit: None,
            first_failure_at: None,
            headers: None,
            disabled: false,
            deleted: false,
        };

        let create_message_app = CreateMessageApp {
            id: ApplicationId::from("test_app".to_string()),
            uid: None,
            org_id: OrganizationId::from("test_org".to_string()),
            rate_limit: None,
            endpoints: vec![endpoint_with_filter, endpoint_without_filter],
            deleted: false,
        };

        // Test with payload that should pass filter (age >= 18)
        let adult_payload = json!({"name": "John", "age": 25});
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("user.created".to_string()),
            None,
            Some(&adult_payload),
        );

        // Expect both endpoints: one passes filter, one has no filter
        assert_eq!(filtered_endpoints.len(), 2);

        // Test with payload that should fail filter (age < 18)
        let minor_payload = json!({"name": "Jane", "age": 16});
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("user.created".to_string()),
            None,
            Some(&minor_payload),
        );
        
        // Expect only the unfiltered endpoint
        assert_eq!(filtered_endpoints.len(), 1);

        // Test with no payload
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("user.created".to_string()),
            None,
            None,
        );
        
        // Both endpoints should be included (no payload means no filtering)
        assert_eq!(filtered_endpoints.len(), 2);
    }

    #[test]
    fn test_cel_filter_edge_cases() {
        use crate::core::types::{ApplicationId, EventTypeName, MessageAttemptTriggerType, OrganizationId};
        use serde_json::json;

        // Test with complex nested JSON payload
        let complex_payload = json!({
            "user": {
                "profile": {
                    "age": 25,
                    "name": "John",
                    "active": true,
                    "balance": 1000.50
                },
                "settings": {
                    "notifications": false,
                    "theme": "dark"
                }
            },
            "metadata": {
                "source": "mobile",
                "version": "1.0.0"
            }
        });

        let endpoints = vec![
            // Test nested object access
            CreateMessageEndpoint {
                id: EndpointId::from("nested_filter".to_string()),
                url: "https://example.com/nested".to_string(),
                key: EndpointSecretInternal::from_endpoint_secret(
                    EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                    &Encryption::new_noop(),
                ).unwrap(),
                old_signing_keys: None,
                event_types_ids: None,
                channels: None,
                filter: Some("user.profile.age >= 18".to_string()),
                rate_limit: None,
                first_failure_at: None,
                headers: None,
                disabled: false,
                deleted: false,
            },
            // Test string operations
            CreateMessageEndpoint {
                id: EndpointId::from("string_filter".to_string()),
                url: "https://example.com/string".to_string(),
                key: EndpointSecretInternal::from_endpoint_secret(
                    EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                    &Encryption::new_noop(),
                ).unwrap(),
                old_signing_keys: None,
                event_types_ids: None,
                channels: None,
                filter: Some("user.profile.name.startsWith('J')".to_string()),
                rate_limit: None,
                first_failure_at: None,
                headers: None,
                disabled: false,
                deleted: false,
            },
            // Test boolean operations
            CreateMessageEndpoint {
                id: EndpointId::from("bool_filter".to_string()),
                url: "https://example.com/bool".to_string(),
                key: EndpointSecretInternal::from_endpoint_secret(
                    EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                    &Encryption::new_noop(),
                ).unwrap(),
                old_signing_keys: None,
                event_types_ids: None,
                channels: None,
                filter: Some("user.profile.active == true".to_string()),
                rate_limit: None,
                first_failure_at: None,
                headers: None,
                disabled: false,
                deleted: false,
            },
            // Test complex expression with multiple conditions
            CreateMessageEndpoint {
                id: EndpointId::from("complex_filter".to_string()),
                url: "https://example.com/complex".to_string(),
                key: EndpointSecretInternal::from_endpoint_secret(
                    EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                    &Encryption::new_noop(),
                ).unwrap(),
                old_signing_keys: None,
                event_types_ids: None,
                channels: None,
                filter: Some("user.profile.age >= 18 && user.profile.balance >= 500.0 && user.profile.active == true".to_string()),
                rate_limit: None,
                first_failure_at: None,
                headers: None,
                disabled: false,
                deleted: false,
            },
        ];

        let create_message_app = CreateMessageApp {
            id: ApplicationId::from("test_app".to_string()),
            uid: None,
            org_id: OrganizationId::from("test_org".to_string()),
            rate_limit: None,
            endpoints,
            deleted: false,
        };

        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("user.created".to_string()),
            None,
            Some(&complex_payload),
        );

        // All filters should pass for this complex payload
        assert_eq!(filtered_endpoints.len(), 4);
    }

    #[test]
    fn test_cel_filter_error_handling() {
        use crate::core::types::{ApplicationId, EventTypeName, MessageAttemptTriggerType, OrganizationId};
        use serde_json::json;

        let endpoints = vec![
            // Test invalid CEL expression
            CreateMessageEndpoint {
                id: EndpointId::from("invalid_filter".to_string()),
                url: "https://example.com/invalid".to_string(),
                key: EndpointSecretInternal::from_endpoint_secret(
                    EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                    &Encryption::new_noop(),
                ).unwrap(),
                old_signing_keys: None,
                event_types_ids: None,
                channels: None,
                filter: Some("invalid syntax here".to_string()),
                rate_limit: None,
                first_failure_at: None,
                headers: None,
                disabled: false,
                deleted: false,
            },
            // Test missing field access
            CreateMessageEndpoint {
                id: EndpointId::from("missing_field".to_string()),
                url: "https://example.com/missing".to_string(),
                key: EndpointSecretInternal::from_endpoint_secret(
                    EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                    &Encryption::new_noop(),
                ).unwrap(),
                old_signing_keys: None,
                event_types_ids: None,
                channels: None,
                filter: Some("nonexistent.field >= 18".to_string()),
                rate_limit: None,
                first_failure_at: None,
                headers: None,
                disabled: false,
                deleted: false,
            },
            // Test type mismatch
            CreateMessageEndpoint {
                id: EndpointId::from("type_mismatch".to_string()),
                url: "https://example.com/type".to_string(),
                key: EndpointSecretInternal::from_endpoint_secret(
                    EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                    &Encryption::new_noop(),
                ).unwrap(),
                old_signing_keys: None,
                event_types_ids: None,
                channels: None,
                filter: Some("age >= 18".to_string()), // age is number, not string
                rate_limit: None,
                first_failure_at: None,
                headers: None,
                disabled: false,
                deleted: false,
            },
        ];

        let create_message_app = CreateMessageApp {
            id: ApplicationId::from("test_app".to_string()),
            uid: None,
            org_id: OrganizationId::from("test_org".to_string()),
            rate_limit: None,
            endpoints,
            deleted: false,
        };

        let simple_payload = json!({"age": 25, "name": "John"});
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("user.created".to_string()),
            None,
            Some(&simple_payload),
        );

        // Only the endpoint without filter should pass (the one with invalid filter should be filtered out)
        assert_eq!(filtered_endpoints.len(), 1);
    }

    #[test]
    fn test_cel_filter_array_access() {
        use crate::core::types::{ApplicationId, EventTypeName, MessageAttemptTriggerType, OrganizationId};
        use serde_json::json;

        // Test with array payload
        let array_payload = json!({
            "users": [
                {"name": "John", "age": 25},
                {"name": "Jane", "age": 16},
                {"name": "Bob", "age": 30}
            ],
            "count": 3
        });

        let endpoints = vec![
            // Test array element access
            CreateMessageEndpoint {
                id: EndpointId::from("array_filter".to_string()),
                url: "https://example.com/array".to_string(),
                key: EndpointSecretInternal::from_endpoint_secret(
                    EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                    &Encryption::new_noop(),
                ).unwrap(),
                old_signing_keys: None,
                event_types_ids: None,
                channels: None,
                filter: Some("users[0].age >= 18".to_string()), // First user's age
                rate_limit: None,
                first_failure_at: None,
                headers: None,
                disabled: false,
                deleted: false,
            },
            // Test array length check
            CreateMessageEndpoint {
                id: EndpointId::from("array_length".to_string()),
                url: "https://example.com/length".to_string(),
                key: EndpointSecretInternal::from_endpoint_secret(
                    EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                    &Encryption::new_noop(),
                ).unwrap(),
                old_signing_keys: None,
                event_types_ids: None,
                channels: None,
                filter: Some("count >= 2.0".to_string()),
                rate_limit: None,
                first_failure_at: None,
                headers: None,
                disabled: false,
                deleted: false,
            },
        ];

        let create_message_app = CreateMessageApp {
            id: ApplicationId::from("test_app".to_string()),
            uid: None,
            org_id: OrganizationId::from("test_org".to_string()),
            rate_limit: None,
            endpoints,
            deleted: false,
        };

        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("user.created".to_string()),
            None,
            Some(&array_payload),
        );

        // Both filters should pass
        assert_eq!(filtered_endpoints.len(), 2);
    }

    #[test]
    fn test_cel_filter_real_integration_web3_scenarios() {
        use crate::core::types::{ApplicationId, EventTypeName, MessageAttemptTriggerType, OrganizationId};
        use serde_json::json;

        // Create endpoints for Web3 scenarios
        let defi_endpoint = CreateMessageEndpoint {
            id: EndpointId::from("defi_endpoint".to_string()),
            url: "https://defi.com/webhook".to_string(),
            key: EndpointSecretInternal::from_endpoint_secret(
                EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                &Encryption::new_noop(),
            ).unwrap(),
            old_signing_keys: None,
            event_types_ids: None,
            channels: None,
            filter: Some("amount >= 1000.0 && token == 'USDC'".to_string()),
            rate_limit: None,
            first_failure_at: None,
            headers: None,
            disabled: false,
            deleted: false,
        };
        
        let nft_endpoint = CreateMessageEndpoint {
            id: EndpointId::from("nft_endpoint".to_string()),
            url: "https://nft.com/webhook".to_string(),
            key: EndpointSecretInternal::from_endpoint_secret(
                EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                &Encryption::new_noop(),
            ).unwrap(),
            old_signing_keys: None,
            event_types_ids: None,
            channels: None,
            filter: Some("price >= 1.0 && collection == 'BoredApe'".to_string()),
            rate_limit: None,
            first_failure_at: None,
            headers: None,
            disabled: false,
            deleted: false,
        };

        let create_message_app = CreateMessageApp {
            id: ApplicationId::from("test_app".to_string()),
            uid: None,
            org_id: OrganizationId::from("test_org".to_string()),
            rate_limit: None,
            endpoints: vec![defi_endpoint, nft_endpoint],
            deleted: false,
        };

        // Test DeFi transaction that should pass
        let defi_payload = json!({
            "amount": 1500.0,
            "token": "USDC",
            "chain": "ethereum"
        });
        
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("transaction.created".to_string()),
            None,
            Some(&defi_payload),
        );

        // Only DeFi endpoint should pass
        assert_eq!(filtered_endpoints.len(), 1);
        assert_eq!(filtered_endpoints[0].id, EndpointId::from("defi_endpoint".to_string()));

        // Test NFT transaction that should pass
        let nft_payload = json!({
            "price": 5.5,
            "collection": "BoredApe",
            "tokenId": 1234
        });
        
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("nft.sold".to_string()),
            None,
            Some(&nft_payload),
        );

        // Only NFT endpoint should pass
        assert_eq!(filtered_endpoints.len(), 1);
        assert_eq!(filtered_endpoints[0].id, EndpointId::from("nft_endpoint".to_string()));

        // Test transaction that should fail both filters
        let low_value_payload = json!({
            "amount": 500.0,  // Below DeFi threshold
            "price": 0.5,     // Below NFT threshold
            "token": "USDC",
            "collection": "BoredApe"
        });
        
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("transaction.created".to_string()),
            None,
            Some(&low_value_payload),
        );

        // No endpoints should pass
        assert_eq!(filtered_endpoints.len(), 0);
    }

    #[test]
    fn test_cel_filter_real_integration_complex_conditions() {
        use crate::core::types::{ApplicationId, EventTypeName, MessageAttemptTriggerType, OrganizationId};
        use serde_json::json;

        // Create endpoint with complex multi-condition filter
        let complex_endpoint = CreateMessageEndpoint {
            id: EndpointId::from("complex_endpoint".to_string()),
            url: "https://example.com/complex".to_string(),
            key: EndpointSecretInternal::from_endpoint_secret(
                EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                &Encryption::new_noop(),
            ).unwrap(),
            old_signing_keys: None,
            event_types_ids: None,
            channels: None,
            filter: Some("balance >= 1000.0 && status == 'active' && verified == true".to_string()),
            rate_limit: None,
            first_failure_at: None,
            headers: None,
            disabled: false,
            deleted: false,
        };

        let create_message_app = CreateMessageApp {
            id: ApplicationId::from("test_app".to_string()),
            uid: None,
            org_id: OrganizationId::from("test_org".to_string()),
            rate_limit: None,
            endpoints: vec![complex_endpoint],
            deleted: false,
        };

        // Test payload that should pass all conditions
        let valid_payload = json!({
            "balance": 1500.0,
            "status": "active",
            "verified": true
        });
        
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("user.created".to_string()),
            None,
            Some(&valid_payload),
        );

        // Should pass the complex filter
        assert_eq!(filtered_endpoints.len(), 1);
        assert_eq!(filtered_endpoints[0].id, EndpointId::from("complex_endpoint".to_string()));

        // Test payload that fails one condition (balance too low)
        let invalid_balance_payload = json!({
            "balance": 500.0,  // Below threshold
            "status": "active",
            "verified": true
        });
        
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("user.created".to_string()),
            None,
            Some(&invalid_balance_payload),
        );

        // Should fail the complex filter
        assert_eq!(filtered_endpoints.len(), 0);

        // Test payload that fails another condition (status inactive)
        let invalid_status_payload = json!({
            "balance": 1500.0,
            "status": "inactive",  // Wrong status
            "verified": true
        });
        
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("user.created".to_string()),
            None,
            Some(&invalid_status_payload),
        );

        // Should fail the complex filter
        assert_eq!(filtered_endpoints.len(), 0);
    }

    #[test]
    fn test_cel_filter_real_integration_performance_large_payload() {
        use crate::core::types::{ApplicationId, EventTypeName, MessageAttemptTriggerType, OrganizationId};
        use serde_json::json;

        // Create endpoint with deep nested filter
        let deep_nested_endpoint = CreateMessageEndpoint {
            id: EndpointId::from("deep_nested_endpoint".to_string()),
            url: "https://example.com/deep".to_string(),
            key: EndpointSecretInternal::from_endpoint_secret(
                EndpointSecret::Symmetric(STANDARD.decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
                &Encryption::new_noop(),
            ).unwrap(),
            old_signing_keys: None,
            event_types_ids: None,
            channels: None,
            filter: Some("level1.level2.level3.level4.level5.value == 42.0".to_string()),
            rate_limit: None,
            first_failure_at: None,
            headers: None,
            disabled: false,
            deleted: false,
        };

        let create_message_app = CreateMessageApp {
            id: ApplicationId::from("test_app".to_string()),
            uid: None,
            org_id: OrganizationId::from("test_org".to_string()),
            rate_limit: None,
            endpoints: vec![deep_nested_endpoint],
            deleted: false,
        };

        // Test with large nested payload
        let large_payload = json!({
            "level1": {
                "level2": {
                    "level3": {
                        "level4": {
                            "level5": {
                                "value": 42.0,
                                "name": "deep_value"
                            }
                        }
                    }
                }
            },
            "metadata": {
                "source": "test",
                "timestamp": "2024-01-01T00:00:00Z"
            }
        });
        
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("deep.test".to_string()),
            None,
            Some(&large_payload),
        );

        // Should pass the deep nested filter
        assert_eq!(filtered_endpoints.len(), 1);
        assert_eq!(filtered_endpoints[0].id, EndpointId::from("deep_nested_endpoint".to_string()));

        // Test with wrong value
        let wrong_value_payload = json!({
            "level1": {
                "level2": {
                    "level3": {
                        "level4": {
                            "level5": {
                                "value": 100.0,  // Wrong value
                                "name": "deep_value"
                            }
                        }
                    }
                }
            }
        });
        
        let filtered_endpoints = create_message_app.filtered_endpoints(
            MessageAttemptTriggerType::Scheduled,
            &EventTypeName::from("deep.test".to_string()),
            None,
            Some(&wrong_value_payload),
        );

        // Should fail the deep nested filter
        assert_eq!(filtered_endpoints.len(), 0);
    }

    #[test]
    fn test_validate_cel_expression() {
        use super::validate_cel_expression;
        // Test valid expressions
        assert!(validate_cel_expression("age >= 18").is_ok());
        assert!(validate_cel_expression("status == 'active'").is_ok());
        assert!(validate_cel_expression("verified == true").is_ok());
        assert!(validate_cel_expression("balance >= 1000.0 && status == 'active'").is_ok());
        assert!(validate_cel_expression("").is_ok());
        assert!(validate_cel_expression("   ").is_ok());

        assert!(validate_cel_expression("age >= ").is_err());
        assert!(validate_cel_expression("age >= 18 &&").is_err());
    }

    #[test]
    fn test_validate_cel_expression_length_limit() {
        use super::validate_cel_expression;
        
        // Test valid length
        let short_expr = "age >= 18";
        assert!(validate_cel_expression(short_expr).is_ok());
        
        // Test length limit
        let long_expr = "a".repeat(2500); // Create a string longer than 2048
        let result = validate_cel_expression(&long_expr);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("CEL expression too long"));
        assert!(error.contains("max: 2048"));
    }

    #[test]
    fn test_validate_cel_expression_complexity_limit() {
        use super::validate_cel_expression;
        
        // Test valid complexity
        let simple_expr = "age >= 18 && status == 'active'";
        assert!(validate_cel_expression(simple_expr).is_ok());
        
        // Test complexity limit with many operators (but keep under length limit)
        let complex_expr = (0..25)
            .map(|i| format!("f{}>=1.0&&f{}=='v{}'", i, i, i))
            .collect::<Vec<_>>()
            .join("&&");
        
        let result = validate_cel_expression(&complex_expr);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("CEL expression too complex"));
        assert!(error.contains("max: 50"));
    }

    #[test]
    fn test_calculate_cel_complexity() {
        use super::calculate_cel_complexity;
        
        // Test simple expressions
        assert_eq!(calculate_cel_complexity("age >= 18"), 1); // >= operator
        assert_eq!(calculate_cel_complexity("name == 'test'"), 4); // ==, ., and string
        assert_eq!(calculate_cel_complexity("age >= 18 && status == 'active'"), 7); // >=, &&, ==, ., string
        
        // Test complex expressions
        let complex = "a.b.c >= 1.0 && x.y.z == 'test' && p.q == true";
        let complexity = calculate_cel_complexity(complex);
        assert!(complexity > 10); // Should have high complexity
        
        // Test empty expression
        assert_eq!(calculate_cel_complexity(""), 0);
        assert_eq!(calculate_cel_complexity("   "), 0);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn test_cel_execution_metrics() {
        use super::{CelExecutionMetrics, CEL_EXECUTION_WARNING_THRESHOLD_MS, CEL_EXECUTION_ERROR_THRESHOLD_MS};
        
        // Test successful execution metrics
        let metrics = CelExecutionMetrics::new(
            "age >= 18".to_string(),
            5, // 5ms execution time
            3, // complexity
            true, // success
            None, // no error
        );
        
        assert_eq!(metrics.expression, "age >= 18");
        assert_eq!(metrics.execution_time_ms, 5);
        assert_eq!(metrics.complexity, 3);
        assert!(metrics.success);
        assert!(metrics.error_message.is_none());
        
        // Test warning threshold metrics
        let warning_metrics = CelExecutionMetrics::new(
            "complex.expression".to_string(),
            CEL_EXECUTION_WARNING_THRESHOLD_MS + 5, // Above warning threshold
            20,
            true,
            None,
        );
        
        assert!(warning_metrics.execution_time_ms >= CEL_EXECUTION_WARNING_THRESHOLD_MS);
        
        // Test error threshold metrics
        let error_metrics = CelExecutionMetrics::new(
            "very.complex.expression".to_string(),
            CEL_EXECUTION_ERROR_THRESHOLD_MS + 10, // Above error threshold
            50,
            false,
            Some("Compilation failed".to_string()),
        );
        
        assert!(error_metrics.execution_time_ms >= CEL_EXECUTION_ERROR_THRESHOLD_MS);
        assert!(!error_metrics.success);
        assert!(error_metrics.error_message.is_some());
    }
}
