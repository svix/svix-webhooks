// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;

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
    pub msg_dest_id: MessageEndpointId,
    pub endp_id: EndpointId,
    pub url: String,
    pub status: MessageStatus,
    pub response_status_code: i16,
    #[sea_orm(column_type = "Text")]
    pub response: String,
    pub ended_at: Option<DateTimeWithTimeZone>,
    pub trigger_type: MessageAttemptTriggerType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::messagedestination::Entity",
        from = "Column::MsgDestId",
        to = "super::messagedestination::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Messagedestination,
}

impl Related<super::messagedestination::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messagedestination.def()
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
