// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT
use crate::core::types::EndpointId;

use crate::core::types::metadata::Metadata;
use crate::{ctx, error};

use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ConnectionTrait, TryIntoModel};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "endpointmetadata")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: EndpointId,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub data: Metadata,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::endpoint::Entity",
        from = "Column::Id",
        to = "super::endpoint::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Endpoint,
}

impl Related<super::endpoint::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Endpoint.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(Utc::now().into());
        Ok(self)
    }
}

impl ActiveModel {
    pub fn new(endp_id: EndpointId, metadata: impl Into<Option<Metadata>>) -> Self {
        let id = Set(endp_id);
        let data = Set(metadata.into().unwrap_or_default());
        let timestamp = Utc::now();
        Self {
            id,
            data,
            created_at: Set(timestamp.into()),
            updated_at: Set(timestamp.into()),
        }
    }

    /// Upserts the record if it's new or updated, AND data is nonempty. Otherwise the record is
    /// ignored or destroyed as appropriate.
    pub async fn upsert_or_delete(self, db: &impl ConnectionTrait) -> error::Result<Model> {
        let data = self.data.clone().take().unwrap_or_default();

        if data.is_empty() {
            let model = ctx!(self.clone().try_into_model())?;
            ctx!(self.delete(db).await)?;
            return Ok(model);
        }

        ctx!(Entity::upsert(self).exec_with_returning(db).await)
    }
}

impl Entity {
    pub fn upsert(am: ActiveModel) -> sea_orm::Insert<ActiveModel> {
        Self::insert(am).on_conflict(
            OnConflict::column(Column::Id)
                .update_columns([Column::Data, Column::UpdatedAt])
                .to_owned(),
        )
    }
}
