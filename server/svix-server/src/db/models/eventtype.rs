// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::{
    core::types::{
        BaseId, EventTypeId, EventTypeName, FeatureFlag, FeatureFlagSet, OrganizationId,
    },
    json_wrapper,
};
use chrono::Utc;
use jsonschema::{Draft, JSONSchema};
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

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

    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
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
                .add(Column::FeatureFlag.is_in(flags.into_iter()))
                .add(Column::FeatureFlag.is_null()),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default, JsonSchema)]
pub struct Schema(HashMap<String, Json>);
json_wrapper!(Schema);

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
