use aide::{
    axum::{ApiRouter, routing::post_with},
    transform::TransformPathItem,
};
use axum::extract::State;
use svix_server_derive::aide_annotate;

use crate::{AppState, core::permissions, error::Result, v1::utils::NoContent};

/// Redrive DLQ
#[aide_annotate(op_id = "v1.admin.redrive-dlq")]
pub async fn redrive_dlq(
    State(AppState { queue_tx, .. }): State<AppState>,
    _: permissions::Organization,
) -> Result<NoContent> {
    if let Err(e) = queue_tx.redrive_dlq().await {
        tracing::warn!(error = ?e, "DLQ redrive failed");
    }
    Ok(NoContent)
}

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/admin/redrive-dlq",
        post_with(redrive_dlq, redrive_dlq_operation),
        move |op: TransformPathItem<'_>| op.tag("Admin".as_ref()).hidden(true),
    )
}
