// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use chrono::Utc;
use jsonschema::{Draft, JSONSchema};
use schemars::JsonSchema;
use sea_orm::{entity::prelude::*, ActiveValue::Set};
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

#[axum::async_trait]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default)]
pub struct Schema(HashMap<String, Json>);
json_wrapper!(Schema);

impl Schema {
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

impl<'de> Deserialize<'de> for Schema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner: HashMap<String, Json> = Deserialize::deserialize(deserializer)?;

        // JSONSchema doesn't implement (De)Serialize, so we have to
        // manually enforce the values are valid JSON schemas

        let mut opts = JSONSchema::options();
        opts.with_draft(Draft::Draft7);

        if let Some(error) = inner
            .values()
            .filter_map(|schema| opts.compile(schema).err())
            .next()
        {
            return Err(serde::de::Error::custom(error));
        }

        Ok(Self(inner))
    }
}
