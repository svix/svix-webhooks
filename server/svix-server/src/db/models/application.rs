// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::core::types::{
    ApplicationId, ApplicationIdOrUid, ApplicationUid, BaseId, OrganizationId,
};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::prelude::*, Condition};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "application")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: ApplicationId,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub org_id: OrganizationId,
    pub uid: Option<ApplicationUid>,
    pub name: String,
    pub rate_limit: Option<i32>,
    pub deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::endpoint::Entity")]
    Endpoint,
    #[sea_orm(has_many = "super::message::Entity")]
    Message,
}

impl Related<super::endpoint::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Endpoint.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let timestamp = Utc::now();
        Self {
            id: Set(ApplicationId::new(timestamp.into(), None)),
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
        Self::find()
            .filter(Column::OrgId.eq(org_id))
            .filter(Column::Deleted.eq(false))
    }

    pub fn secure_find_by_id(org_id: OrganizationId, id: ApplicationId) -> Select<Entity> {
        Self::secure_find(org_id).filter(Column::Id.eq(id))
    }

    pub fn secure_find_by_id_or_uid(
        org_id: OrganizationId,
        id_or_uid: ApplicationIdOrUid,
    ) -> Select<Entity> {
        Self::secure_find(org_id).filter(
            Condition::any()
                .add(Column::Id.eq(id_or_uid.to_owned()))
                .add(Column::Uid.eq(id_or_uid)),
        )
    }
}
