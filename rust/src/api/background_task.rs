// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct BackgroundTaskListOptions {
    /// Filter the response based on the status.
    pub status: Option<BackgroundTaskStatus>,

    /// Filter the response based on the type.
    pub task: Option<BackgroundTaskType>,

    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

pub struct BackgroundTask<'a> {
    cfg: &'a Configuration,
}

impl<'a> BackgroundTask<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List background tasks executed in the past 90 days.
    pub async fn list(
        &self,
        options: Option<BackgroundTaskListOptions>,
    ) -> Result<ListResponseBackgroundTaskOut> {
        let BackgroundTaskListOptions {
            status,
            task,
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/background-task")
            .with_optional_query_param("status", status)
            .with_optional_query_param("task", task)
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg)
            .await
    }

    /// Get a background task by ID.
    pub async fn get(&self, task_id: String) -> Result<BackgroundTaskOut> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/background-task/{task_id}")
            .with_path_param("task_id", task_id)
            .execute(self.cfg)
            .await
    }
}
