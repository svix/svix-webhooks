use axum::{
    async_trait,
    extract::{FromRequest, Path, RequestParts},
    Extension,
};
use sea_orm::DatabaseConnection;

use crate::{
    ctx,
    db::models::application,
    error::{Error, HttpError, Result},
};

use super::{
    security::{permissions_from_bearer, AccessLevel},
    types::{ApplicationIdOrUid, OrganizationId},
};

pub struct Organization {
    pub org_id: OrganizationId,
}

#[async_trait]
impl<B> FromRequest<B> for Organization
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let permissions = permissions_from_bearer(req).await?;

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
impl<B> FromRequest<B> for Application
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let permissions = permissions_from_bearer(req).await?;

        let Path(ApplicationPathParams { app_id }) =
            ctx!(Path::<ApplicationPathParams>::from_request(req).await)?;
        let Extension(ref db) = ctx!(Extension::<DatabaseConnection>::from_request(req).await)?;
        let app = ctx!(
            application::Entity::secure_find_by_id_or_uid(permissions.org_id(), app_id.to_owned(),)
                .one(db)
                .await
        )?
        .ok_or_else(|| HttpError::not_found(None, None))?;

        if let Some(permitted_app_id) = permissions.app_id() {
            if permitted_app_id != app.id {
                return Err(HttpError::not_found(None, None).into());
            }
        }

        Ok(Self { app })
    }
}

// Organization level privileges, with the requested application
pub struct OrganizationWithApplication {
    pub app: application::Model,
}

#[async_trait]
impl<B> FromRequest<B> for OrganizationWithApplication
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let Organization { org_id } = ctx!(Organization::from_request(req).await)?;

        let Path(ApplicationPathParams { app_id }) =
            ctx!(Path::<ApplicationPathParams>::from_request(req).await)?;
        let Extension(ref db) = ctx!(Extension::<DatabaseConnection>::from_request(req).await)?;
        let app = ctx!(
            application::Entity::secure_find_by_id_or_uid(org_id, app_id.to_owned(),)
                .one(db)
                .await
        )?
        .ok_or_else(|| HttpError::not_found(None, None))?;
        Ok(OrganizationWithApplication { app })
    }
}

#[derive(serde::Deserialize)]
struct ApplicationPathParams {
    app_id: ApplicationIdOrUid,
}
