// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::core::types::{
    ApplicationId, BaseId, EndpointHeaders, EndpointId, EndpointIdOrUid, EndpointSecret,
    EndpointUid, EventChannelSet, EventTypeNameSet, ExpiringSigningKeys,
};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::prelude::*, Condition};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "endpoint")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: EndpointId,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub app_id: ApplicationId,
    pub key: EndpointSecret,
    pub url: String,
    pub description: String,
    pub event_types_ids: Option<EventTypeNameSet>,
    pub version: i32,
    pub rate_limit: Option<i32>,
    pub deleted: bool,
    pub disabled: bool,
    pub first_failure_at: Option<DateTimeWithTimeZone>,
    pub uid: Option<EndpointUid>,
    pub old_keys: Option<ExpiringSigningKeys>,
    pub channels: Option<EventChannelSet>,
    pub headers: Option<EndpointHeaders>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::application::Entity",
        from = "Column::AppId",
        to = "super::application::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Application,
    #[sea_orm(has_many = "super::messagedestination::Entity")]
    Messagedestination,
}

impl Related<super::application::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Application.def()
    }
}

impl Related<super::messagedestination::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messagedestination.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(EndpointId::new(None, None)),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
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
    pub fn secure_find(app_id: ApplicationId) -> Select<Entity> {
        Self::find()
            .filter(Column::AppId.eq(app_id))
            .filter(Column::Deleted.eq(false))
    }

    pub fn secure_find_by_id(app_id: ApplicationId, id: EndpointId) -> Select<Entity> {
        Self::secure_find(app_id).filter(Column::Id.eq(id))
    }

    pub fn secure_find_by_id_or_uid(
        app_id: ApplicationId,
        id_or_uid: EndpointIdOrUid,
    ) -> Select<Entity> {
        Self::secure_find(app_id).filter(
            Condition::any()
                .add(Column::Id.eq(id_or_uid.to_owned()))
                .add(Column::Uid.eq(id_or_uid)),
        )
    }
}
