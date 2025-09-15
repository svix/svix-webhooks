// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue::Set};

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
pub enum Relation {}

#[axum::async_trait]
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

    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
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
