use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::request::Parts,
    Extension,
};
use sea_orm::DatabaseConnection;

use crate::{
    ctx,
    db::models::{application, applicationmetadata},
    error::{Error, HttpError, Result},
};

use super::{
    security::{permissions_from_bearer, AccessLevel, Permissions},
    types::{ApplicationId, ApplicationIdOrUid, OrganizationId},
};

pub struct ReadAll {
    pub org_id: OrganizationId,
}

#[async_trait]
impl<S> FromRequestParts<S> for ReadAll
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let permissions = ctx!(permissions_from_bearer(parts, state).await)?;
        let org_id = permissions.org_id();
        Ok(Self { org_id })
    }
}

pub struct Organization {
    pub org_id: OrganizationId,
}

impl Permissions {
    fn check_app_is_permitted(&self, app_id: &ApplicationId) -> Result<()> {
        if let Some(ref permitted_app_id) = self.app_id() {
            if permitted_app_id != app_id {
                return Err(HttpError::not_found(None, None).into());
            }
        }
        Ok(())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Organization
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let permissions = permissions_from_bearer(parts, state).await?;

        let org_id = match permissions.access_level {
            AccessLevel::Organization(org_id) => org_id,
            _ => return Err(HttpError::permission_denied(None, None).into()),
        };

        Ok(Self { org_id })
    }
}

pub struct Application {
    pub app: application::Model,
}

#[async_trait]
impl<S> FromRequestParts<S> for Application
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let permissions = permissions_from_bearer(parts, state).await?;

        let Path(ApplicationPathParams { app_id }) =
            ctx!(Path::<ApplicationPathParams>::from_request_parts(parts, state).await)?;
        let Extension(ref db) =
            ctx!(Extension::<DatabaseConnection>::from_request_parts(parts, state).await)?;
        let app = ctx!(
            application::Entity::secure_find_by_id_or_uid(permissions.org_id(), app_id.to_owned(),)
                .one(db)
                .await
        )?
        .ok_or_else(|| HttpError::not_found(None, None))?;

        permissions.check_app_is_permitted(&app.id)?;

        Ok(Self { app })
    }
}

// Organization level privileges, with the requested application
pub struct OrganizationWithApplication {
    pub app: application::Model,
}

#[async_trait]
impl<S> FromRequestParts<S> for OrganizationWithApplication
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let Organization { org_id } = ctx!(Organization::from_request_parts(parts, state).await)?;

        let Path(ApplicationPathParams { app_id }) =
            ctx!(Path::<ApplicationPathParams>::from_request_parts(parts, state).await)?;
        let Extension(ref db) =
            ctx!(Extension::<DatabaseConnection>::from_request_parts(parts, state).await)?;
        let app = ctx!(
            application::Entity::secure_find_by_id_or_uid(org_id, app_id.to_owned(),)
                .one(db)
                .await
        )?
        .ok_or_else(|| HttpError::not_found(None, None))?;
        Ok(OrganizationWithApplication { app })
    }
}

pub struct ApplicationWithMetadata {
    pub app: application::Model,
    pub metadata: applicationmetadata::Model,
}

#[async_trait]
impl<S> FromRequestParts<S> for ApplicationWithMetadata
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let permissions = permissions_from_bearer(parts, state).await?;

        let Path(ApplicationPathParams { app_id }) =
            ctx!(Path::<ApplicationPathParams>::from_request_parts(parts, state).await)?;
        let Extension(ref db) =
            ctx!(Extension::<DatabaseConnection>::from_request_parts(parts, state).await)?;
        let (app, metadata) = ctx!(
            application::Model::fetch_with_metadata(db, permissions.org_id(), app_id.to_owned())
                .await
        )?
        .ok_or_else(|| HttpError::not_found(None, None))?;

        permissions.check_app_is_permitted(&app.id)?;

        Ok(Self { app, metadata })
    }
}

#[derive(serde::Deserialize)]
struct ApplicationPathParams {
    app_id: ApplicationIdOrUid,
}
