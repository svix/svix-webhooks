// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;

use crate::core::types::{BaseId, EndpointId, MessageEndpointId, MessageId, MessageStatus};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "messagedestination")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: MessageEndpointId,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub msg_id: MessageId,
    pub endp_id: EndpointId,
    pub status: MessageStatus,
    pub next_attempt: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::endpoint::Entity",
        from = "Column::EndpId",
        to = "super::endpoint::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Endpoint,
    #[sea_orm(
        belongs_to = "super::message::Entity",
        from = "Column::MsgId",
        to = "super::message::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Message,
    #[sea_orm(has_many = "super::messageattempt::Entity")]
    Messageattempt,
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

impl Related<super::messageattempt::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messageattempt.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let timestamp = Utc::now();
        Self {
            id: Set(MessageEndpointId::new(timestamp.into(), None)),
            created_at: Set(timestamp.into()),
            updated_at: Set(timestamp.into()),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(Utc::now().into());
        Ok(self)
    }
}

impl Entity {
    pub fn secure_find_by_msg(msg_id: MessageId) -> Select<Entity> {
        Self::find().filter(Column::MsgId.eq(msg_id))
    }

    pub fn secure_find_by_endpoint(endp_id: EndpointId) -> Select<Entity> {
        Self::find().filter(Column::EndpId.eq(endp_id))
    }
}
