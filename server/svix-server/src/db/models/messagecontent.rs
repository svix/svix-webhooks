// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue::Set};

use crate::core::types::MessageId;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "messagecontent")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: MessageId,
    pub created_at: DateTimeWithTimeZone,
    pub payload: Vec<u8>,
    pub expiration: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::message::Entity",
        from = "Column::Id",
        to = "super::message::Column::Id"
    )]
    Message,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(msg_id: MessageId, payload: Vec<u8>) -> Self {
        let timestamp = Utc::now();
        Self {
            id: Set(msg_id),
            created_at: Set(timestamp.into()),
            payload: Set(payload),
            ..ActiveModelTrait::default()
        }
    }
}

impl Entity {
    pub fn secure_find_by_id_in(ids: Vec<MessageId>) -> Select<Entity> {
        Self::find().filter(Column::Id.is_in(ids))
    }
}
