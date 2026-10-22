// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use async_trait::async_trait;
use chrono::Utc;
use jsonschema::Draft;
use schemars::JsonSchema;
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    core::types::{
        BaseId, EventTypeId, EventTypeName, FeatureFlag, FeatureFlagSet, OrganizationId,
    },
    json_wrapper,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "eventtype")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: EventTypeId,
    pub org_id: OrganizationId,
    pub description: String,
    pub deleted: bool,
    pub deprecated: bool,
    pub schemas: Option<Schema>,
    pub name: EventTypeName,
    pub feature_flag: Option<FeatureFlag>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let timestamp = Utc::now();
        Self {
            id: Set(EventTypeId::new(timestamp.into(), None)),
            created_at: Set(timestamp.into()),
            updated_at: Set(timestamp.into()),
            deleted: Set(false),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Utc::now().into());
        Ok(self)
    }
}

impl Entity {
    pub fn secure_find(org_id: OrganizationId) -> Select<Entity> {
        Self::find().filter(Column::OrgId.eq(org_id))
    }

    pub fn secure_find_by_name(org_id: OrganizationId, name: EventTypeName) -> Select<Entity> {
        Self::secure_find(org_id).filter(Column::Name.eq(name))
    }

    pub fn filter_feature_flags(query: Select<Self>, flags: FeatureFlagSet) -> Select<Self> {
        query.filter(
            sea_orm::Condition::any()
                .add(Column::FeatureFlag.is_in(flags))
                .add(Column::FeatureFlag.is_null()),
        )
    }
}

pub fn schema_example() -> serde_json::Value {
    serde_json::json!({
        "description": "An invoice was paid by a user",
        "properties": {
            "invoiceId": {
                "description": "The invoice id",
                "type": "string"
            },
            "userId": {
                "description": "The user id",
                "type": "string"
            }
        },
        "required": [
            "invoiceId",
            "userId"
        ],
        "title": "Invoice Paid Event",
        "type": "object"
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Schema(HashMap<String, Json>);
json_wrapper!(Schema);

impl Schema {
    pub fn validate(&self) -> Result<(), validator::ValidationError> {
        let opts = schema_validator_options();
        match self.0.values().find_map(|schema| opts.build(schema).err()) {
            Some(error) => Err(validator::ValidationError::new("invalid_schema")
                .with_message(error.to_string().into())),
            None => Ok(()),
        }
    }

    pub fn example(&self) -> Option<&serde_json::Value> {
        self.0
            .get("1")
            .and_then(|version| match version {
                serde_json::Value::Object(obj) => obj.get("examples"),
                _ => None,
            })
            .and_then(|examples| match examples {
                serde_json::Value::Array(arr) => arr.iter().next(),
                _ => None,
            })
    }
}

impl JsonSchema for Schema {
    fn schema_name() -> String {
        stringify!(Schema).to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut schema = gen.subschema_for::<HashMap<String, Json>>();

        if let schemars::schema::Schema::Object(obj) = &mut schema {
            obj.extensions
                .insert("example".to_string(), schema_example());
        }

        schema
    }

    fn is_referenceable() -> bool {
        false
    }
}

/// Refuses every `$ref` outside the schema itself, so tenant schemas cannot make the server fetch URLs or read files.
struct NoExternalRefs;

impl jsonschema::Retrieve for NoExternalRefs {
    fn retrieve(
        &self,
        uri: &jsonschema::Uri<String>,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(format!("external `$ref` is not allowed: {uri}").into())
    }
}

/// Draft 7 options for compiling tenant-supplied schemas. Use this instead of `jsonschema::Validator::options()`.
pub fn schema_validator_options() -> jsonschema::ValidationOptions<'static> {
    jsonschema::Validator::options()
        .with_draft(Draft::Draft7)
        .with_retriever(NoExternalRefs)
}

#[cfg(test)]
mod tests {
    use super::Schema;

    fn schema_with_ref(external_ref: &str) -> Schema {
        serde_json::from_value(serde_json::json!({
            "1": {
                "type": "object",
                "properties": { "a": { "$ref": external_ref } }
            }
        }))
        .unwrap()
    }

    #[test]
    fn test_external_refs_are_not_retrieved() {
        let manifest = concat!("file://", env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
        for external_ref in ["http://127.0.0.1:1/schema.json", manifest] {
            let err = schema_with_ref(external_ref)
                .validate()
                .expect_err("external $ref must be rejected");
            assert!(
                err.to_string().contains("external `$ref` is not allowed"),
                "unexpected error for {external_ref}: {err}"
            );
        }
    }

    #[test]
    fn test_internal_refs_still_resolve() {
        let schemas: Schema = serde_json::from_value(serde_json::json!({
            "1": {
                "definitions": { "id": { "type": "string" } },
                "type": "object",
                "properties": { "a": { "$ref": "#/definitions/id" } }
            }
        }))
        .unwrap();

        schemas.validate().unwrap();
    }
}
