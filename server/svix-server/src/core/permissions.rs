use aide::OperationInput;
use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::request::Parts,
};

use crate::{
    ctx,
    db::models::{application, applicationmetadata},
    error::{Error, HttpError, Result},
    AppState,
};

use super::{
    security::{permissions_from_bearer, AccessLevel, Permissions},
    types::{ApplicationId, ApplicationIdOrUid, FeatureFlagSet, OrganizationId},
};

pub struct ReadAll {
    pub org_id: OrganizationId,
    pub feature_flags: AllowedFeatureFlags,
}

#[async_trait]
impl FromRequestParts<AppState> for ReadAll {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        let permissions = ctx!(permissions_from_bearer(parts, state).await)?;
        let org_id = permissions.org_id();
        let feature_flags = match permissions.access_level {
            AccessLevel::Organization(_) => AllowedFeatureFlags::All,
            AccessLevel::Application(_, _) => AllowedFeatureFlags::Some(permissions.feature_flags),
        };
        Ok(Self {
            org_id,
            feature_flags,
        })
    }
}

impl OperationInput for ReadAll {}

pub struct Organization {
    pub org_id: OrganizationId,
}

impl OperationInput for Organization {}

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
impl FromRequestParts<AppState> for Organization {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
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
impl FromRequestParts<AppState> for Application {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        let permissions = permissions_from_bearer(parts, state).await?;

        let Path(ApplicationPathParams { app_id }) =
            ctx!(Path::<ApplicationPathParams>::from_request_parts(parts, state).await)?;
        let app = ctx!(
            application::Entity::secure_find_by_id_or_uid(permissions.org_id(), app_id.to_owned(),)
                .one(&state.db)
                .await
        )?
        .ok_or_else(|| HttpError::not_found(None, None))?;

        permissions.check_app_is_permitted(&app.id)?;

        Ok(Self { app })
    }
}

impl OperationInput for Application {}

// Organization level privileges, with the requested application
pub struct OrganizationWithApplication {
    pub app: application::Model,
}

impl OperationInput for OrganizationWithApplication {}

#[async_trait]
impl FromRequestParts<AppState> for OrganizationWithApplication {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        let Organization { org_id } = ctx!(Organization::from_request_parts(parts, state).await)?;

        let Path(ApplicationPathParams { app_id }) =
            ctx!(Path::<ApplicationPathParams>::from_request_parts(parts, state).await)?;
        let app = ctx!(
            application::Entity::secure_find_by_id_or_uid(org_id, app_id.to_owned(),)
                .one(&state.db)
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

impl OperationInput for ApplicationWithMetadata {}

#[async_trait]
impl FromRequestParts<AppState> for ApplicationWithMetadata {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        let permissions = permissions_from_bearer(parts, state).await?;

        let Path(ApplicationPathParams { app_id }) =
            ctx!(Path::<ApplicationPathParams>::from_request_parts(parts, state).await)?;
        let (app, metadata) = ctx!(
            application::Model::fetch_with_metadata(
                &state.db,
                permissions.org_id(),
                app_id.to_owned()
            )
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

/// Denotes what features gated by feature flags the requester has access to.
pub enum AllowedFeatureFlags {
    /// Requester has access to all features regardless of flags. No checking
    /// of feature flags should be done for this request.
    All,
    /// Requester has access to a limited set of features. The set may be empty.
    Some(FeatureFlagSet),
}
