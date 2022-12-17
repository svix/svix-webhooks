// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::core::types::{
    ApplicationId, ApplicationIdOrUid, ApplicationUid, BaseId, OrganizationId,
};
use crate::{ctx, error};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::prelude::*, Condition};
use sea_orm::{ConnectionTrait, QueryOrder, QuerySelect};

use super::applicationmetadata;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
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
    #[sea_orm(has_one = "super::applicationmetadata::Entity")]
    Metadata,
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

impl Related<super::applicationmetadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Metadata.def()
    }
}

impl Model {
    pub async fn fetch_or_create_metadata(
        &self,
        db: &impl ConnectionTrait,
    ) -> error::Result<applicationmetadata::ActiveModel> {
        let query = applicationmetadata::Entity::secure_find(self.id.clone());
        let metadata = ctx!(query.one(db).await)?
            .map(applicationmetadata::ActiveModel::from)
            .unwrap_or_else(|| applicationmetadata::ActiveModel::new(self.id.clone(), None));

        Ok(metadata)
    }

    pub async fn fetch_many_with_metadata(
        db: &DatabaseConnection,
        org_id: OrganizationId,
        limit: u64,
        after_id: impl Into<Option<ApplicationId>>,
    ) -> error::Result<impl Iterator<Item = (Self, applicationmetadata::Model)>> {
        let mut query = Entity::secure_find(org_id)
            .order_by_asc(Column::Id)
            .limit(limit);

        if let Some(id) = after_id.into() {
            query = query.filter(Column::Id.gt(id))
        }

        let results = ctx!(
            query
                .find_also_related(applicationmetadata::Entity)
                .all(db)
                .await
        )?;

        Ok(results.into_iter().map(|(app, metadata)| {
            let metadata =
                metadata.unwrap_or_else(|| applicationmetadata::Model::new(app.id.clone()));
            (app, metadata)
        }))
    }

    pub async fn fetch_with_metadata(
        db: &DatabaseConnection,
        org_id: OrganizationId,
        id_or_uid: ApplicationIdOrUid,
    ) -> error::Result<Option<(Self, applicationmetadata::Model)>> {
        let result = ctx!(
            Entity::secure_find_by_id_or_uid(org_id, id_or_uid)
                .find_also_related(applicationmetadata::Entity)
                .one(db)
                .await
        )?;
        Ok(result.map(|(app, metadata)| {
            let metadata =
                metadata.unwrap_or_else(|| applicationmetadata::Model::new(app.id.clone()));
            (app, metadata)
        }))
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(Utc::now().into());
        Ok(self)
    }
}

impl ActiveModel {
    pub fn new(org_id: OrganizationId) -> Self {
        let timestamp = Utc::now();
        Self {
            id: Set(ApplicationId::new(timestamp.into(), None)),
            org_id: Set(org_id),
            created_at: Set(timestamp.into()),
            updated_at: Set(timestamp.into()),
            deleted: Set(false),
            ..ActiveModelTrait::default()
        }
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
