use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use sea_orm::{entity::prelude::*, QueryOrder};
use sea_orm::{DatabaseConnection, QuerySelect};

use super::RecoverIn;
use crate::{
    core::{
        permissions,
        types::{BaseId, MessageAttemptTriggerType, MessageEndpointId, MessageStatus},
    },
    ctx,
    db::models::{application, endpoint, messagedestination},
    error::{HttpError, Result, ValidationErrorItem},
    queue::{MessageTask, TaskQueueProducer},
    v1::utils::{ApplicationEndpointPath, EmptyResponse, ValidatedJson},
    AppState,
};

async fn bulk_recover_failed_messages(
    db: DatabaseConnection,
    queue_tx: TaskQueueProducer,
    app: application::Model,
    endp: endpoint::Model,
    since: DateTime<Utc>,
) -> Result<()> {
    const RECOVER_LIMIT: u64 = 10_000;
    const BATCH_SIZE: u64 = 100;
    let mut iterator: Option<MessageEndpointId> = None;
    let mut num_done: u64 = 0;

    loop {
        let mut query = messagedestination::Entity::secure_find_by_endpoint(endp.id.clone())
            .filter(messagedestination::Column::Id.gte(MessageEndpointId::start_id(since)))
            .filter(messagedestination::Column::Status.eq(MessageStatus::Fail))
            .order_by_asc(messagedestination::Column::Id)
            .limit(RECOVER_LIMIT);

        if let Some(iterator) = iterator {
            query = query.filter(messagedestination::Column::Id.gt(iterator))
        }

        let items = ctx!(query.all(&db).await)?;
        let cur_len = items.len() as u64;
        iterator = items.last().map(|x| x.id.clone());

        for msg_dest in items {
            queue_tx
                .send(
                    MessageTask::new_task(
                        msg_dest.msg_id,
                        app.id.clone(),
                        msg_dest.endp_id,
                        MessageAttemptTriggerType::Manual,
                    ),
                    None,
                )
                .await?;
        }

        num_done += cur_len;
        if cur_len < BATCH_SIZE || num_done > RECOVER_LIMIT {
            break;
        }
    }

    Ok(())
}

pub(super) const RECOVER_FAILED_WEBHOOKS_DESCRIPTION: &str =
    "Resend all failed messages since a given time.";

pub(super) async fn recover_failed_webhooks(
    State(AppState {
        ref db, queue_tx, ..
    }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(data): ValidatedJson<RecoverIn>,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    // Add five minutes so that people can easily just do `now() - two_weeks` without having to worry about clock sync
    let timeframe = chrono::Duration::days(14);
    let timeframe = timeframe + chrono::Duration::minutes(5);

    if data.since < Utc::now() - timeframe {
        return Err(HttpError::unprocessable_entity(vec![ValidationErrorItem {
            loc: vec!["body".to_owned(), "since".to_owned()],
            msg: "Cannot recover messages more than 14 days old.".to_owned(),
            ty: "value_error".to_owned(),
        }])
        .into());
    }

    let endp = ctx!(
        endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
            .one(db)
            .await
    )?
    .ok_or_else(|| HttpError::not_found(None, None))?;

    let db = db.clone();
    let queue_tx = queue_tx.clone();
    tokio::spawn(
        async move { bulk_recover_failed_messages(db, queue_tx, app, endp, data.since).await },
    );

    Ok((StatusCode::ACCEPTED, Json(EmptyResponse {})))
}
