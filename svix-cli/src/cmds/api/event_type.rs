// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args, Clone)]
pub struct EventTypeListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    #[arg(long)]
    pub order: Option<Ordering>,
    /// When `true` archived (deleted but not expunged) items are included in the response.
    #[arg(long)]
    pub include_archived: Option<bool>,
    /// When `true` the full item (including the schema) is included in the response.
    #[arg(long)]
    pub with_content: Option<bool>,
}

impl From<EventTypeListOptions> for svix::api::EventTypeListOptions {
    fn from(value: EventTypeListOptions) -> Self {
        let EventTypeListOptions {
            limit,
            iterator,
            order,
            include_archived,
            with_content,
        } = value;
        Self {
            limit,
            iterator,
            order,
            include_archived,
            with_content,
        }
    }
}

#[derive(Args, Clone)]
pub struct EventTypeCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EventTypeCreateOptions> for svix::api::EventTypeCreateOptions {
    fn from(value: EventTypeCreateOptions) -> Self {
        let EventTypeCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EventTypeImportOpenapiOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EventTypeImportOpenapiOptions> for svix::api::EventTypeImportOpenapiOptions {
    fn from(value: EventTypeImportOpenapiOptions) -> Self {
        let EventTypeImportOpenapiOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EventTypeDeleteOptions {
    /// By default event types are archived when "deleted". Passing this to `true` deletes them entirely.
    #[arg(long)]
    pub expunge: Option<bool>,
}

impl From<EventTypeDeleteOptions> for svix::api::EventTypeDeleteOptions {
    fn from(value: EventTypeDeleteOptions) -> Self {
        let EventTypeDeleteOptions { expunge } = value;
        Self { expunge }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct EventTypeArgs {
    #[command(subcommand)]
    pub command: EventTypeCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum EventTypeCommands {
    /// Return the list of event types.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix event-type list\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"archived\": false,
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"deprecated\": true,
    \"description\": \"A user has signed up\",
    \"featureFlag\": \"...\",
    \"featureFlags\": [\"cool-new-feature\"],
    \"groupName\": \"user\",
    \"name\": \"user.signup\",
    \"schemas\": {
      \"1\": {
        \"description\": \"An invoice was paid by a user\",
        \"properties\": {
          \"invoiceId\": {
            \"description\": \"The invoice id\",
            \"type\": \"string\"
          },
          \"userId\": {
            \"description\": \"The user id\",
            \"type\": \"string\"
          }
        },
        \"required\": [\"invoiceId\",\"userId\"],
        \"title\": \"Invoice Paid Event\",
        \"type\": \"object\"
      }
    },
    \"updatedAt\": \"2030-01-01T00:00:00Z\"
  }],
  \"done\": true,
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\"
}\n")]
    List {
        #[clap(flatten)]
        options: EventTypeListOptions,
    },
    /// Create new or unarchive existing event type.
    ///
    /// Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
    /// Endpoints filtering on the event type before archival will continue to filter on it.
    /// This operation does not preserve the description and schemas.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix event-type create {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"archived\": false,
  \"deprecated\": true,
  \"description\": \"A user has signed up\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"groupName\": \"user\",
  \"name\": \"user.signup\",
  \"schemas\": {
    \"1\": {
      \"description\": \"An invoice was paid by a user\",
      \"properties\": {
        \"invoiceId\": {
          \"description\": \"The invoice id\",
          \"type\": \"string\"
        },
        \"userId\": {
          \"description\": \"The user id\",
          \"type\": \"string\"
        }
      },
      \"required\": [\"invoiceId\",\"userId\"],
      \"title\": \"Invoice Paid Event\",
      \"type\": \"object\"
    }
  }
}\n\nExample response:
{
  \"archived\": false,
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"deprecated\": true,
  \"description\": \"A user has signed up\",
  \"featureFlag\": \"...\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"groupName\": \"user\",
  \"name\": \"user.signup\",
  \"schemas\": {
    \"1\": {
      \"description\": \"An invoice was paid by a user\",
      \"properties\": {
        \"invoiceId\": {
          \"description\": \"The invoice id\",
          \"type\": \"string\"
        },
        \"userId\": {
          \"description\": \"The user id\",
          \"type\": \"string\"
        }
      },
      \"required\": [\"invoiceId\",\"userId\"],
      \"title\": \"Invoice Paid Event\",
      \"type\": \"object\"
    }
  },
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    Create {
        event_type_in: crate::json::JsonOf<EventTypeIn>,
        #[clap(flatten)]
        options: EventTypeCreateOptions,
    },
    /// Given an OpenAPI spec, create new or update existing event types.
    ///
    /// If an existing `archived` event type is updated, it will be unarchived.
    /// The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
    /// top-level.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix event-type import-openapi {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"dryRun\": true,
  \"replaceAll\": true,
  \"spec\": {
    \"info\": {
      \"title\": \"Webhook Example\",
      \"version\": \"1.0.0\"
    },
    \"openapi\": \"3.1.0\",
    \"webhooks\": {
      \"pet.new\": {
        \"post\": {
          \"requestBody\": {
            \"content\": {
              \"application/json\": {
                \"schema\": {
                  \"properties\": {
                    \"id\": {
                      \"format\": \"int64\",
                      \"type\": \"integer\"
                    },
                    \"name\": {
                      \"type\": \"string\"
                    },
                    \"tag\": {
                      \"type\": \"string\"
                    }
                  },
                  \"required\": [\"id\",\"name\"]
                }
              }
            },
            \"description\": \"Information about a new pet in the system\"
          },
          \"responses\": {
            \"200\": {
              \"description\": \"Return a 200 status to indicate that the data was received successfully\"
            }
          }
        }
      }
    }
  },
  \"specRaw\": \"\\n# Both YAML and JSON are supported\\nopenapi: 3.1.0\\ninfo:\\n  title: Webhook Example\\n  version: 1.0.0\\n# Since OAS 3.1.0 the paths element isn\\u0027t necessary. Now a valid OpenAPI Document can describe only paths, webhooks, or even only reusable components\\nwebhooks:\\n  # Each webhook needs a name\\n  \\\"pet.new\\\":\\n    # This is a Path Item Object, the only difference is that the request is initiated by the API provider\\n    post:\\n      requestBody:\\n        description: Information about a new pet in the system\\n        content:\\n          application/json:\\n            schema:\\n              $ref: \\\"#/components/schemas/Pet\\\"\\n      responses:\\n        \\\"200\\\":\\n          description: Return a 200 status to indicate that the data was received successfully\\n\\ncomponents:\\n  schemas:\\n    Pet:\\n      required:\\n        - id\\n        - name\\n      properties:\\n        id:\\n          type: integer\\n          format: int64\\n        name:\\n          type: string\\n        tag:\\n          type: string\\n\"
}\n\nExample response:
{
  \"data\": {
    \"modified\": [\"...\"],
    \"to_modify\": [{
      \"deprecated\": true,
      \"description\": \"...\",
      \"featureFlags\": [\"...\"],
      \"groupName\": \"user\",
      \"name\": \"user.signup\",
      \"schemas\": {
        \"description\": \"An invoice was paid by a user\",
        \"properties\": {
          \"invoiceId\": {
            \"description\": \"The invoice id\",
            \"type\": \"string\"
          },
          \"userId\": {
            \"description\": \"The user id\",
            \"type\": \"string\"
          }
        },
        \"required\": [\"invoiceId\",\"userId\"],
        \"title\": \"Invoice Paid Event\",
        \"type\": \"object\"
      }
    }]
  }
}\n")]
    ImportOpenapi {
        event_type_import_open_api_in: Option<crate::json::JsonOf<EventTypeImportOpenApiIn>>,
        #[clap(flatten)]
        options: EventTypeImportOpenapiOptions,
    },
    /// Get an event type.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix event-type get example.event\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"archived\": false,
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"deprecated\": true,
  \"description\": \"A user has signed up\",
  \"featureFlag\": \"...\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"groupName\": \"user\",
  \"name\": \"user.signup\",
  \"schemas\": {
    \"1\": {
      \"description\": \"An invoice was paid by a user\",
      \"properties\": {
        \"invoiceId\": {
          \"description\": \"The invoice id\",
          \"type\": \"string\"
        },
        \"userId\": {
          \"description\": \"The user id\",
          \"type\": \"string\"
        }
      },
      \"required\": [\"invoiceId\",\"userId\"],
      \"title\": \"Invoice Paid Event\",
      \"type\": \"object\"
    }
  },
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    Get { event_type_name: String },
    /// Create or update an event type.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix event-type upsert example.event {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"archived\": false,
  \"deprecated\": true,
  \"description\": \"A user has signed up\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"groupName\": \"user\",
  \"schemas\": {
    \"1\": {
      \"description\": \"An invoice was paid by a user\",
      \"properties\": {
        \"invoiceId\": {
          \"description\": \"The invoice id\",
          \"type\": \"string\"
        },
        \"userId\": {
          \"description\": \"The user id\",
          \"type\": \"string\"
        }
      },
      \"required\": [\"invoiceId\",\"userId\"],
      \"title\": \"Invoice Paid Event\",
      \"type\": \"object\"
    }
  }
}\n\nExample response:
{
  \"archived\": false,
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"deprecated\": true,
  \"description\": \"A user has signed up\",
  \"featureFlag\": \"...\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"groupName\": \"user\",
  \"name\": \"user.signup\",
  \"schemas\": {
    \"1\": {
      \"description\": \"An invoice was paid by a user\",
      \"properties\": {
        \"invoiceId\": {
          \"description\": \"The invoice id\",
          \"type\": \"string\"
        },
        \"userId\": {
          \"description\": \"The user id\",
          \"type\": \"string\"
        }
      },
      \"required\": [\"invoiceId\",\"userId\"],
      \"title\": \"Invoice Paid Event\",
      \"type\": \"object\"
    }
  },
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    Upsert {
        event_type_name: String,
        event_type_update: crate::json::JsonOf<EventTypeUpdate>,
    },
    /// Archive an event type.
    ///
    /// Endpoints already configured to filter on an event type will continue to do so after archival.
    /// However, new messages can not be sent with it and endpoints can not filter on it.
    /// An event type can be unarchived with the
    /// [create operation](#operation/create_event_type_api_v1_event_type__post).
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix event-type delete example.event\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Delete {
        event_type_name: String,
        #[clap(flatten)]
        options: EventTypeDeleteOptions,
    },
    /// Partially update an event type.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix event-type patch example.event {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"archived\": true,
  \"deprecated\": true,
  \"description\": \"...\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"groupName\": \"user\",
  \"schemas\": {
    \"description\": \"An invoice was paid by a user\",
    \"properties\": {
      \"invoiceId\": {
        \"description\": \"The invoice id\",
        \"type\": \"string\"
      },
      \"userId\": {
        \"description\": \"The user id\",
        \"type\": \"string\"
      }
    },
    \"required\": [\"invoiceId\",\"userId\"],
    \"title\": \"Invoice Paid Event\",
    \"type\": \"object\"
  }
}\n\nExample response:
{
  \"archived\": false,
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"deprecated\": true,
  \"description\": \"A user has signed up\",
  \"featureFlag\": \"...\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"groupName\": \"user\",
  \"name\": \"user.signup\",
  \"schemas\": {
    \"1\": {
      \"description\": \"An invoice was paid by a user\",
      \"properties\": {
        \"invoiceId\": {
          \"description\": \"The invoice id\",
          \"type\": \"string\"
        },
        \"userId\": {
          \"description\": \"The user id\",
          \"type\": \"string\"
        }
      },
      \"required\": [\"invoiceId\",\"userId\"],
      \"title\": \"Invoice Paid Event\",
      \"type\": \"object\"
    }
  },
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    Patch {
        event_type_name: String,
        event_type_patch: Option<crate::json::JsonOf<EventTypePatch>>,
    },
}

impl EventTypeCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.event_type().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                event_type_in,
                options,
            } => {
                let resp = client
                    .event_type()
                    .create(event_type_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ImportOpenapi {
                event_type_import_open_api_in,
                options,
            } => {
                let resp = client
                    .event_type()
                    .import_openapi(
                        event_type_import_open_api_in
                            .unwrap_or_default()
                            .into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { event_type_name } => {
                let resp = client.event_type().get(event_type_name).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Upsert {
                event_type_name,
                event_type_update,
            } => {
                let resp = client
                    .event_type()
                    .upsert(event_type_name, event_type_update.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete {
                event_type_name,
                options,
            } => {
                client
                    .event_type()
                    .delete(event_type_name, Some(options.into()))
                    .await?;
            }
            Self::Patch {
                event_type_name,
                event_type_patch,
            } => {
                let resp = client
                    .event_type()
                    .patch(
                        event_type_name,
                        event_type_patch.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
