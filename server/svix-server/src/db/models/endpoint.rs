// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue::Set, Condition, IntoActiveModel, QuerySelect};

use super::endpointmetadata;
use crate::{
    core::types::{
        ApplicationId, BaseId, EndpointHeaders, EndpointId, EndpointIdOrUid,
        EndpointSecretInternal, EndpointUid, EventChannelSet, EventTypeNameSet,
        ExpiringSigningKeys,
    },
    error,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "endpoint")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: EndpointId,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub app_id: ApplicationId,
    pub key: EndpointSecretInternal,
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
    /// Optional CEL filter expression applied to payloads at dispatch time
    pub filter: Option<String>,
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
    #[sea_orm(has_one = "super::endpointmetadata::Entity")]
    Metadata,
}

impl Related<super::application::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Application.def()
    }
}

impl Related<super::endpointmetadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Metadata.def()
    }
}

#[axum::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Utc::now().into());
        Ok(self)
    }
}

impl ActiveModel {
    pub fn new(app_id: ApplicationId, key: EndpointSecretInternal) -> Self {
        let timestamp = Utc::now();
        Self {
            id: Set(EndpointId::new(timestamp.into(), None)),
            app_id: Set(app_id),
            created_at: Set(timestamp.into()),
            updated_at: Set(timestamp.into()),
            deleted: Set(false),
            key: Set(key),
            ..ActiveModelTrait::default()
        }
    }

    pub async fn fetch_with_metadata(
        db: &impl ConnectionTrait,
        app_id: ApplicationId,
        endp_id: EndpointIdOrUid,
    ) -> error::Result<Option<(Self, endpointmetadata::ActiveModel)>> {
        let Some((endp, metadata)) = Entity::secure_find_by_id_or_uid(app_id, endp_id)
            .find_also_related(endpointmetadata::Entity)
            .one(db)
            .await?
        else {
            return Ok(None);
        };

        let metadata = metadata
            .map(IntoActiveModel::into_active_model)
            .unwrap_or_else(|| endpointmetadata::ActiveModel::new(endp.id.clone(), None));

        Ok(Some((endp.into_active_model(), metadata)))
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

    pub fn secure_find_by_ids(
        app_id: ApplicationId,
        ids: impl IntoIterator<Item = EndpointId>,
        want_deleted: bool,
    ) -> Select<Entity> {
        let mut q = Self::find()
            .filter(Column::AppId.eq(app_id))
            .filter(Column::Id.is_in(ids));

        if !want_deleted {
            q = q.filter(Column::Deleted.eq(false))
        }

        q.distinct()
    }
}
