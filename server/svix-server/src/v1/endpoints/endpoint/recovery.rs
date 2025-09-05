use axum::extract::{Path, State};
use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use sea_orm::{entity::prelude::*, QueryOrder, QuerySelect, QueryTrait};
use serde::{Deserialize, Serialize};
use svix_server_derive::aide_annotate;

use super::RecoverIn;
use crate::{
    core::{
        permissions,
        types::{
            BaseId, MessageAttemptId, MessageAttemptTriggerType, MessageStatus,
            QueueBackgroundTaskId,
        },
    },
    db::models::{application, endpoint, messageattempt},
    error::{HttpError, Result, ValidationErrorItem},
    queue::{MessageTask, TaskQueueProducer},
    v1::utils::{ApplicationEndpointPath, JsonStatus, ValidatedJson},
    AppState,
};

async fn bulk_recover_failed_messages(
    db: DatabaseConnection,
    queue_tx: TaskQueueProducer,
    app: application::Model,
    endp: endpoint::Model,
    since: DateTime<Utc>,
    until: DateTime<Utc>,
) -> Result<()> {
    const RECOVER_LIMIT: u64 = 10_000;
    const BATCH_SIZE: u64 = 100;
    let mut iterator: Option<MessageAttemptId> = None;
    let mut num_done: u64 = 0;

    loop {
        let mut query = messageattempt::Entity::secure_find_by_endpoint(endp.id.clone())
            .filter(messageattempt::Column::Id.gte(MessageAttemptId::start_id(since)))
            .filter(messageattempt::Column::Id.lt(MessageAttemptId::start_id(until)))
            .filter(messageattempt::Column::Status.eq(MessageStatus::Fail))
            .distinct_on([messageattempt::Column::MsgId])
            .order_by_asc(messageattempt::Column::MsgId)
            .filter(
                messageattempt::Column::MsgId.not_in_subquery(
                    <messageattempt::Entity as EntityTrait>::find()
                        .select_only()
                        .column(messageattempt::Column::MsgId)
                        .filter(messageattempt::Column::EndpId.eq(endp.id.clone()))
                        .filter(messageattempt::Column::Status.eq(MessageStatus::Success))
                        .filter(messageattempt::Column::Id.gte(MessageAttemptId::start_id(since)))
                        .filter(messageattempt::Column::Id.lt(MessageAttemptId::start_id(until)))
                        .into_query(),
                ),
            )
            .limit(BATCH_SIZE);

        if let Some(iterator) = iterator {
            query = query.filter(messageattempt::Column::Id.gt(iterator))
        }

        let items = query.all(&db).await?;
        let cur_len = items.len() as u64;
        iterator = items.last().map(|x| x.id.clone());

        for msg_dest in items {
            queue_tx
                .send(
                    &MessageTask::new_task(
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
        if cur_len < BATCH_SIZE || num_done >= RECOVER_LIMIT {
            break;
        }
    }

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundTaskStatus {
    Running,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum BackgroundTaskType {
    #[serde(rename = "endpoint.recover")]
    Recover,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RecoverOut {
    id: QueueBackgroundTaskId,
    status: BackgroundTaskStatus,
    task: BackgroundTaskType,
}

/// Resend all failed messages since a given time.
#[aide_annotate(op_id = "v1.endpoint.recover")]
pub(super) async fn recover_failed_webhooks(
    State(AppState {
        ref db, queue_tx, ..
    }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(RecoverIn { since, until }): ValidatedJson<RecoverIn>,
) -> Result<JsonStatus<202, RecoverOut>> {
    let until = until.unwrap_or_else(Utc::now);

    // Add five minutes so that people can easily just do `now() - two_weeks` without having to worry about clock sync
    let max_timeframe = Duration::days(14) + Duration::minutes(5);

    if since < until - max_timeframe {
        return Err(HttpError::unprocessable_entity(vec![ValidationErrorItem {
            loc: vec!["body".to_owned(), "since".to_owned()],
            msg: format!(
                "Cannot recover more than {} days of messages",
                max_timeframe.num_days()
            ),
            ty: "value_error".to_owned(),
        }])
        .into());
    }

    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let db = db.clone();
    let queue_tx = queue_tx.clone();
    tokio::spawn(async move {
        if let Err(e) = bulk_recover_failed_messages(db, queue_tx, app, endp, since, until).await {
            tracing::error!("Error recovering failed messages: {}", e);
        }
    });

    Ok(JsonStatus(RecoverOut {
        id: QueueBackgroundTaskId::new(None, None),
        status: BackgroundTaskStatus::Running,
        task: BackgroundTaskType::Recover,
    }))
}
