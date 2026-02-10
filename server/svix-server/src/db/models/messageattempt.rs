// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::Utc;
use sea_orm::{ActiveValue::Set, Order, QueryOrder, QuerySelect, entity::prelude::*};

use crate::core::types::{
    BaseId, EndpointId, MessageAttemptId, MessageAttemptTriggerType, MessageEndpointId, MessageId,
    MessageStatus,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "messageattempt")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: MessageAttemptId,
    pub created_at: DateTimeWithTimeZone,
    pub msg_id: MessageId,
    pub msg_dest_id: Option<MessageEndpointId>,
    pub endp_id: EndpointId,
    pub url: String,
    pub status: MessageStatus,
    pub response_status_code: i16,
    #[sea_orm(column_type = "Text")]
    pub response: String,
    pub ended_at: Option<DateTimeWithTimeZone>,
    pub trigger_type: MessageAttemptTriggerType,
    /// Response duration in milliseconds
    pub response_duration_ms: i64,
    pub next_attempt: Option<DateTimeWithTimeZone>,
    pub attempt_number: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::message::Entity",
        from = "Column::MsgId",
        to = "super::message::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Message,
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
            id: Set(MessageAttemptId::new(timestamp.into(), None)),
            created_at: Set(timestamp.into()),
            ..ActiveModelTrait::default()
        }
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

pub trait Query: QuerySelect + QueryFilter + QueryOrder + Sized {
    fn after_id(self, id: MessageAttemptId) -> Self {
        self.filter(Column::Id.gte(id))
    }

    fn after_id_exclusive(self, id: MessageAttemptId) -> Self {
        self.filter(Column::Id.gt(id))
    }

    fn before_id(self, id: MessageAttemptId) -> Self {
        self.filter(Column::Id.lte(id))
    }

    fn after(self, t: DateTimeUtc) -> Self {
        self.after_id(MessageAttemptId::start_id(t))
    }

    fn before(self, t: DateTimeUtc) -> Self {
        self.before_id(MessageAttemptId::start_id(t))
    }

    fn with_status(self, status: MessageStatus) -> Self {
        self.filter(Column::Status.eq(status))
    }

    fn oldest_first(self) -> Self {
        self.order_by(Column::Id, Order::Asc)
    }

    /// Only return the last attempt per message (every attempt will be part of a separate message)
    fn latest_per_msg(self) -> Self {
        self.distinct_on([Column::MsgId])
            .order_by_desc(Column::MsgId)
            .order_by_desc(Column::Id)
    }
}

impl Query for Select<Entity> {}
