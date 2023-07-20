use aide::openapi::{self, OpenApi, Parameter, ReferenceOr};
use schemars::{visit::Visitor, JsonSchema};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn initialize_openapi() -> OpenApi {
    aide::gen::on_error(|error| {
        tracing::error!("Aide generation error: {error}");
    });
    // Extract schemas to `#/components/schemas/` instead of using inline schemas.
    aide::gen::extract_schemas(true);
    // Have aide attempt to infer the `Content-Type` of responses based on the
    // handlers' return types.
    aide::gen::infer_responses(true);
    aide::gen::inferred_empty_response_status(204);

    aide::gen::in_context(|ctx| ctx.schema = schemars::gen::SchemaSettings::openapi3().into());

    let tag_groups = serde_json::json![[
        {
            "name": "General",
            "tags": ["Application", "Event Type"]
        },
        {
            "name": "Application specific",
            "tags": ["Authentication", "Endpoint", "Message", "Message Attempt", "Integration"]
        },
        {
            "name": "Utility",
            "tags": ["Health"]
        },
        {
            "name": "Webhooks",
            "tags": ["Webhooks"]
        }
    ]];

    let webhooks = serde_json::to_value(webhooks::webhooks())
        .expect("failed to serialize webhook documentation");

    const DESCRIPTION: &str = include_str!("../description.md");

    OpenApi {
        info: openapi::Info {
            title: "Svix API".to_owned(),
            version: VERSION.to_owned(),
            extensions: indexmap::indexmap! {
                "x-logo".to_string() => serde_json::json!({
                    "url": "https://www.svix.com/static/img/brand-padded.svg",
                    "altText": "Svix Logo",
                }),
            },
            description: Some(DESCRIPTION.to_string()),
            ..Default::default()
        },
        tags: vec![
            openapi::Tag {
                name: "Application".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Message".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Message Attempt".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Endpoint".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Integration".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Event Type".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Authentication".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Health".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Webhooks".to_owned(),
                ..openapi::Tag::default()
            },
        ],
        extensions: indexmap::indexmap! {
            "x-tagGroups".to_owned() => tag_groups,
            "x-webhooks".to_owned() => webhooks,
        },
        ..Default::default()
    }
}

/// Replaces OpenAPI 3.1 style `"foo": true` schemas with OpenAPI 3.0 style
/// `"foo": {"type": "object"}` schemas.
fn replace_true_schemas(mut openapi: OpenApi) -> OpenApi {
    use schemars::schema::{InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec};

    // Checks if it's a plain boolean schema, and if yes replaces it with a
    // `{"type": "object"}` schema. If it's an object then it will descend into
    // its properties and `additionalProperties` too.
    fn visit_schema(schema: &mut Schema) {
        match schema {
            Schema::Bool(true) => {
                *schema = Schema::Object(SchemaObject {
                    instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
                    ..Default::default()
                })
            }
            Schema::Bool(false) => {
                tracing::warn!("unexpected `false` schema encountered");
            }
            Schema::Object(obj) => {
                // When examples are added to a `"foo": bool` schema it gets
                // expanded into an object, i.e. `"foo": {"example": ...}`, but
                // no "type" field is set on it. Although the OpenAPI spec does
                // not specifically say that the type field is mandatory, in
                // practice a lack of the "type" field should only ever occur
                // when a `true` schema gets replaced because it is somehow
                // modified (e.g. example added), or because it's a "$ref"
                // object.
                // If it's not a reference, then we must add the "type" field
                // back with the value "object" so code generators work correctly.
                if obj.instance_type.is_none() && obj.reference.is_none() {
                    obj.instance_type = Some(SingleOrVec::Single(Box::new(InstanceType::Object)));
                }

                obj.object.as_mut().map(visit_object_validation);
            }
        }
    }

    fn visit_object_validation(obj: &mut Box<ObjectValidation>) {
        if let Some(additional_props) = &mut obj.additional_properties {
            visit_schema(additional_props.as_mut())
        }
        for (_, schema) in obj.properties.iter_mut() {
            visit_schema(schema)
        }
    }

    if let Some(ref mut components) = openapi.components {
        for (_, schema) in components.schemas.iter_mut() {
            visit_schema(&mut schema.json_schema)
        }
    }

    openapi
}

/// Adds the `Idempotency-Key` header parameter to all `POST` operations in the schema.
fn add_idempotency_to_post(mut openapi: OpenApi) -> OpenApi {
    // The header's value can be any valid string
    let string_schema = aide::gen::in_context(|ctx| String::json_schema(&mut ctx.schema));

    let s = openapi::SchemaObject {
        json_schema: string_schema,
        external_docs: None,
        example: None,
    };

    let idempotency_key_data = openapi::ParameterData {
        name: "idempotency-key".to_string(),
        description: Some("The request's idempotency key".to_string()),
        required: false,
        deprecated: None,
        format: openapi::ParameterSchemaOrContent::Schema(s),
        example: None,
        examples: indexmap::indexmap! {},
        explode: None,
        extensions: indexmap::indexmap! {},
    };

    if let Some(paths) = openapi.paths.as_mut() {
        for (_, op) in paths.paths.iter_mut() {
            match op {
                openapi::ReferenceOr::Reference { reference, .. } => {
                    // References to operations should never appear in our
                    // schema since all our operations are unique, and we
                    // don't reference any 3rd party/external operations.
                    tracing::warn!(
                        "Unexpected operation reference encountered in OpenAPI schema: {reference}"
                    );
                }
                openapi::ReferenceOr::Item(op) => {
                    if let Some(post) = op.post.as_mut() {
                        post.parameters.push(ReferenceOr::Item(Parameter::Header {
                            parameter_data: idempotency_key_data.clone(),
                            style: openapi::HeaderStyle::Simple,
                        }))
                    }
                }
            }
        }
    }

    openapi
}

/// Remove schemas from `components.schemas` of the spec which are under normal
/// circumstances not referenced. At the moment these are struct schemas used
/// by query parameters and path placeholders.
fn remove_unneeded_schemas(mut openapi: OpenApi) -> OpenApi {
    if let Some(components) = openapi.components.as_mut() {
        components.schemas.retain(|name, _| {
            !(name.ends_with("Path")
                || name.ends_with("QueryParams")
                || name.starts_with("Pagination"))
        });
    };

    openapi
}

/// Replaces the `examples` property of a schema with a singular `example`
/// property.
/// OpenAPI <=3.0 used `example` as an extension, >=3.1 standardized `examples`.
fn replace_multiple_examples(mut openapi: OpenApi) -> OpenApi {
    let mut visitor = schemars::visit::SetSingleExample {
        retain_examples: false,
    };

    if let Some(components) = openapi.components.as_mut() {
        for (_, schema_object) in components.schemas.iter_mut() {
            visitor.visit_schema(&mut schema_object.json_schema);
        }
    }

    openapi
}

/// Applies a list of hacks to the finished OpenAPI spec to make it usable with
/// our tooling.
pub fn postprocess_spec(mut openapi: OpenApi) -> OpenApi {
    let hacks = [
        add_idempotency_to_post,
        remove_unneeded_schemas,
        replace_true_schemas,
        replace_multiple_examples,
    ];

    for hack in hacks {
        openapi = hack(openapi);
    }

    openapi
}

/// This module documents operational webhooks. To document one define a data
/// struct first, use the [`webhook_event`] macro to generate a wrapping struct
/// for it, then include it in the [`webhooks`] function call.
mod webhooks {
    use std::collections::HashMap;

    use aide::openapi;
    use schemars::JsonSchema;

    use crate::core::operational_webhooks::{
        EndpointDisabledEventData, EndpointEvent, MessageAttemptEvent,
    };

    /// Documents the webhook specified by the type `T`.
    fn document_webhook<T: Webhook>() -> (String, openapi::PathItem) {
        let type_name = std::any::type_name::<T>()
            .split("::")
            .last()
            .expect("Last element of split can't be empty")
            .to_string();

        let body_schema =
            aide::gen::in_context(|ctx| ctx.schema.subschema_for::<T>().into_object());

        let body_media = openapi::MediaType {
            schema: Some(openapi::SchemaObject {
                json_schema: body_schema.into(),
                external_docs: None,
                example: None,
            }),
            ..Default::default()
        };

        let body = openapi::RequestBody {
            content: indexmap::indexmap! {
                "application/json".to_string() => body_media,
            },
            ..Default::default()
        };

        let success_response = openapi::Response {
            description:
                "Return any 2XX status to indicate that the data was received successfully"
                    .to_string(),
            ..Default::default()
        };

        (
            type_name.clone(),
            openapi::PathItem {
                post: Some(openapi::Operation {
                    description: Some(T::description().to_string()),
                    operation_id: Some(type_name.clone()),
                    request_body: Some(openapi::ReferenceOr::Item(body)),
                    responses: Some(openapi::Responses {
                        responses: indexmap::indexmap! {
                            openapi::StatusCode::Range(2) => openapi::ReferenceOr::Item(success_response),
                        },
                        ..Default::default()
                    }),
                    summary: Some(type_name),
                    tags: vec!["Webhooks".to_string()],
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }

    trait Webhook: JsonSchema {
        fn description() -> &'static str;
    }

    /// Generates a struct that wraps the webhook payload, adding the discriminant.
    macro_rules! webhook_event {
        ($name:ident, $payload_name:ident, $discriminant:expr, $description:literal) => {
            #[allow(unused)]
            struct $name {
                data: $payload_name,
                type_: String,
            }

            impl JsonSchema for $name {
                fn schema_name() -> String {
                    stringify!($name).to_string()
                }

                fn json_schema(
                    gen: &mut schemars::gen::SchemaGenerator,
                ) -> schemars::schema::Schema {
                    #[derive(::schemars::JsonSchema)]
                    #[allow(unused)]
                    struct $name {
                        data: $payload_name,
                        #[serde(rename = "type")]
                        type_: String,
                    }

                    let mut schema = $name::json_schema(gen).into_object();

                    schema
                        .object()
                        .properties
                        .get_mut("type")
                        .map(|type_property| {
                            if let ::schemars::schema::Schema::Object(obj) = type_property {
                                let discriminant_value =
                                    serde_json::Value::String($discriminant.to_string());
                                obj.enum_values = Some(vec![discriminant_value.clone()]);
                                obj.metadata().default = Some(discriminant_value);
                            }
                        });
                    schema.metadata().description = Some($description.to_string());

                    schema.into()
                }
            }

            impl Webhook for $name {
                fn description() -> &'static str {
                    $description
                }
            }
        };
    }

    #[derive(JsonSchema)]
    #[allow(unused)]
    struct EndpointCreatedEventData {
        #[serde(flatten)]
        common_: EndpointEvent,
    }

    #[derive(JsonSchema)]
    #[allow(unused)]
    struct EndpointDeletedEventData {
        #[serde(flatten)]
        common_: EndpointEvent,
    }

    #[derive(JsonSchema)]
    #[allow(unused)]
    struct EndpointUpdatedEventData {
        #[serde(flatten)]
        common_: EndpointEvent,
    }

    #[derive(JsonSchema)]
    #[allow(unused)]
    struct MessageAttemptExhaustedEventData {
        #[serde(flatten)]
        common_: MessageAttemptEvent,
    }

    #[derive(JsonSchema)]
    #[allow(unused)]
    struct MessageAttemptFailingEventData {
        #[serde(flatten)]
        common_: MessageAttemptEvent,
    }

    webhook_event!(
        EndpointCreatedEvent,
        EndpointCreatedEventData,
        "endpoint.created",
        "Sent when an endpoint is created."
    );
    webhook_event!(
        EndpointDeletedEvent,
        EndpointDeletedEventData,
        "endpoint.deleted",
        "Sent when an endpoint is deleted."
    );
    webhook_event!(
        EndpointUpdatedEvent,
        EndpointUpdatedEventData,
        "endpoint.updated",
        "Sent when an endpoint is updated."
    );
    webhook_event!(
        EndpointDisabledEvent,
        EndpointDisabledEventData,
        "endpoint.disabled",
        "Sent when an endpoint has been automatically disabled after continuous failures."
    );
    webhook_event!(
        MessageAttemptExhaustedEvent,
        MessageAttemptExhaustedEventData,
        "message.attempt.exhausted",
        "Sent when a message delivery has failed (all of the retry attempts have been exhausted)."
    );
    webhook_event!(
        MessageAttemptFailingEvent,
        MessageAttemptFailingEventData,
        "message.attempt.failing",
        "Sent after a message has been failing for a few times.\nIt's sent on the fourth failure. It complements `message.attempt.exhausted` which is sent after the last failure."
    );

    /// Generates documentation for operational webhooks in the Redoc `x-webhooks`
    /// format. For more info see https://redocly.com/docs/api-reference-docs/specification-extensions/x-webhooks/
    pub(super) fn webhooks() -> HashMap<String, openapi::PathItem> {
        HashMap::from([
            document_webhook::<EndpointCreatedEvent>(),
            document_webhook::<EndpointDeletedEvent>(),
            document_webhook::<EndpointDisabledEvent>(),
            document_webhook::<EndpointUpdatedEvent>(),
            document_webhook::<MessageAttemptExhaustedEvent>(),
            document_webhook::<MessageAttemptFailingEvent>(),
        ])
    }
}
