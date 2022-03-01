// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::cfg::Configuration;
use crate::core::types::{
    EndpointHeaders, EndpointSecret, MessageAttemptTriggerType, MessageId, MessageStatus,
};
use crate::db::models::{application, endpoint, message, messageattempt, messagedestination};
use crate::error::{Error, Result};
use crate::queue::{MessageTask, QueueTask, TaskQueueConsumer, TaskQueueProducer};
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderName};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::time::{sleep, Duration};

use std::{iter, str::FromStr};

const USER_AGENT: &str = concat!("Svix-Webhooks/", env!("CARGO_PKG_VERSION"));

/// Generates a set of headers for any one webhook event
fn generate_msg_headers(
    cfg: &Configuration,
    body: &str,
    msg_id: &MessageId,
    configured_headers: Option<EndpointHeaders>,
    endpoint_signing_keys: Vec<&EndpointSecret>,
    _endpoint_url: &str,
) -> HeaderMap {
    let timestamp = Utc::now().timestamp();
    let to_sign = format!("{}.{}.{}", msg_id, timestamp, body);
    let signatures = endpoint_signing_keys
        .into_iter()
        .map(|x| hmac_sha256::HMAC::mac(to_sign.as_bytes(), &x.0[..]));
    let signatures_str = signatures
        .map(|x| format!("v1,{}", base64::encode(x)))
        .collect::<Vec<String>>()
        .join(" ");
    let mut headers = HeaderMap::new();
    let id = msg_id.0.parse().unwrap();
    let timestamp = timestamp.to_string().parse().unwrap();
    let signatures_str = signatures_str.parse().unwrap();
    if cfg.whitelabel_headers {
        headers.insert("webhook-id", id);
        headers.insert("webhook-timestamp", timestamp);
        headers.insert("webhook-signature", signatures_str);
    } else {
        headers.insert("svix-id", id);
        headers.insert("svix-timestamp", timestamp);
        headers.insert("svix-signature", signatures_str);
    }

    if let Some(configured_headers) = configured_headers {
        for (k, v) in configured_headers.0 {
            if let (Ok(k), Ok(v)) = (HeaderName::from_str(&k), v.parse()) {
                headers.insert(k, v);
            } else {
                tracing::error!("Invalid HeaderName or HeaderValues for `{}: {}`", k, v);
            }
        }
    }

    headers
}

/// Dispatches one webhook
async fn dispatch(
    cfg: Configuration,
    db: &DatabaseConnection,
    queue_tx: &TaskQueueProducer,
    msg_task: MessageTask,
) -> Result<()> {
    tracing::trace!("Dispatch: {} {}", &msg_task.msg_id, &msg_task.endpoint_id);

    let app_id = &msg_task.app_id;
    let endp_id = &msg_task.endpoint_id;
    let (endp, _app): (endpoint::Model, application::Model) = if let Some((endp, app)) =
        endpoint::Entity::secure_find_by_id(app_id.clone(), endp_id.clone())
            .filter(endpoint::Column::Disabled.eq(false))
            .find_also_related(application::Entity)
            .one(db)
            .await?
    {
        (endp, app.unwrap())
    } else {
        tracing::warn!(
            "Unexpected: app or endpoint don't exist (or deleted) {} {}",
            app_id,
            endp_id
        );
        return Ok(());
    };

    let msg = if let Some(msg) = message::Entity::find_by_id(msg_task.msg_id.clone())
        .one(db)
        .await?
    {
        msg
    } else {
        tracing::warn!("Unexpected: message doesn't exist {}", msg_task.msg_id);
        return Ok(());
    };

    // FIXME: no unwrap
    let body = serde_json::to_string(&msg.payload).unwrap();
    let headers = {
        let keys: Vec<&EndpointSecret> = if let Some(ref old_keys) = endp.old_keys {
            iter::once(&endp.key)
                .chain(old_keys.0.iter().map(|x| &x.key))
                .collect()
        } else {
            vec![&endp.key]
        };

        let mut headers =
            generate_msg_headers(&cfg, &body, &msg_task.msg_id, endp.headers, keys, &endp.url);
        headers.insert("user-agent", USER_AGENT.to_string().parse().unwrap());
        headers
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&endp.url)
        .headers(headers)
        .timeout(Duration::from_secs(cfg.worker_request_timeout as u64))
        .json(&msg.payload)
        .send()
        .await;

    let msg_dest = messagedestination::Entity::secure_find_by_msg(msg.id.clone())
        .filter(messagedestination::Column::EndpId.eq(msg_task.endpoint_id.clone()))
        .one(db)
        .await?
        .ok_or_else(|| {
            Error::Generic(format!(
                "Msg dest not found {} {}",
                msg.id, msg_task.endpoint_id
            ))
        })?;

    if (msg_dest.status != MessageStatus::Pending && msg_dest.status != MessageStatus::Sending)
        && (msg_task.trigger_type != MessageAttemptTriggerType::Manual)
    {
        // TODO: it happens when this message destination is "resent". This leads to 2 queue tasks with the same message destination
        tracing::warn!(
            "MessageDestination {} is not pending (it's {:?}).",
            msg_dest.id,
            msg_dest.status
        );
        return Ok(());
    }

    let attempt = messageattempt::ActiveModel {
        msg_id: Set(msg.id.clone()),
        endp_id: Set(endp.id.clone()),
        msg_dest_id: Set(msg_dest.id.clone()),
        url: Set(endp.url.clone()),

        ended_at: Set(Some(Utc::now().into())),
        trigger_type: Set(msg_task.trigger_type),
        ..Default::default()
    };
    let attempt = match res {
        Ok(res) => {
            let status_code = res.status().as_u16() as i16;
            let status = if res.status().is_success() {
                MessageStatus::Success
            } else {
                MessageStatus::Fail
            };
            let body = res
                .text()
                .await
                .unwrap_or_else(|_| "BODY_NOT_TEXT".to_string());
            let attempt = messageattempt::ActiveModel {
                response_status_code: Set(status_code),
                response: Set(body),
                status: Set(status),

                ..attempt
            };
            Ok(attempt)
        }
        Err(err) => {
            let attempt = messageattempt::ActiveModel {
                response_status_code: Set(0),
                response: Set("".to_owned()),
                status: Set(MessageStatus::Fail),

                ..attempt
            };
            Err((attempt, err))
        }
    };

    match attempt {
        Ok(attempt) => {
            let _attempt = attempt.insert(db).await?;

            let msg_dest = messagedestination::ActiveModel {
                status: Set(MessageStatus::Success),
                next_attempt: Set(None),
                ..msg_dest.into()
            };
            let msg_dest = msg_dest.update(db).await?;
            tracing::trace!("Worker success: {} {}", &msg_dest.id, &endp.id,);
        }
        Err((attempt, err)) => {
            let _attempt = attempt.insert(db).await?;

            let attempt_count = msg_task.attempt_count as usize;
            if msg_task.trigger_type == MessageAttemptTriggerType::Manual {
                tracing::debug!("Manual retry failed");
            } else if attempt_count < cfg.retry_schedule.len() {
                tracing::debug!(
                    "Worker failure retrying for attempt {}: {} {} {}",
                    attempt_count,
                    err,
                    &msg_dest.id,
                    &endp.id
                );
                let duration = cfg.retry_schedule[attempt_count];
                let msg_dest = messagedestination::ActiveModel {
                    next_attempt: Set(Some(
                        (Utc::now() + chrono::Duration::from_std(duration).unwrap()).into(),
                    )),
                    ..msg_dest.into()
                };
                let _msg_dest = msg_dest.update(db).await?;

                queue_tx
                    .send(
                        QueueTask::MessageV1(MessageTask {
                            attempt_count: msg_task.attempt_count + 1,
                            ..msg_task
                        }),
                        Some(duration),
                    )
                    .await?;
            } else {
                tracing::debug!(
                    "Worker failure attempts exhausted: {} {} {}",
                    err,
                    &msg_dest.id,
                    &endp.id
                );
                let msg_dest = messagedestination::ActiveModel {
                    status: Set(MessageStatus::Fail),
                    next_attempt: Set(None),
                    ..msg_dest.into()
                };
                let _msg_dest = msg_dest.update(db).await?;
            }
        }
    }
    Ok(())
}

/// Listens on the message queue for new tasks
pub async fn worker_loop(
    cfg: Configuration,
    pool: DatabaseConnection,
    queue_tx: TaskQueueProducer,
    mut queue_rx: TaskQueueConsumer,
) -> Result<()> {
    loop {
        let pool = pool.clone();
        match queue_rx.receive().await {
            Ok(delivery) => {
                let queue_task = delivery.task.clone();
                let queue_tx = queue_tx.clone();
                let cfg = cfg.clone();
                match queue_task {
                    QueueTask::MessageV1(msg) => {
                        tokio::spawn(async move {
                            if let Err(err) = dispatch(cfg, &pool, &queue_tx, msg).await {
                                tracing::error!("Error executing task: {}", err);
                                queue_tx.nack(delivery).await.unwrap();
                            } else {
                                // No unwrap
                                queue_tx.ack(delivery).await.unwrap();
                            }
                        });
                    }
                };
            }
            Err(err) => {
                tracing::error!("Error receiving task: {}", err);
                sleep(Duration::from_millis(10)).await;
            }
        }
    }
}
