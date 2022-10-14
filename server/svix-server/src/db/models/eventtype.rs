// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::{
    core::types::{BaseId, EventTypeId, EventTypeName, OrganizationId},
    json_wrapper,
};
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Schema(pub HashMap<String, Json>);
json_wrapper!(Schema);

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
}
