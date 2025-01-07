use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct BackgroundTaskListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub order: Option<Ordering>,
    pub status: Option<BackgroundTaskStatus>,
    pub task: Option<BackgroundTaskType>,
}

pub struct BackgroundTask<'a> {
    cfg: &'a Configuration,
}

impl<'a> BackgroundTask<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        options: Option<BackgroundTaskListOptions>,
    ) -> Result<ListResponseBackgroundTaskOut> {
        let BackgroundTaskListOptions {
            iterator,
            limit,
            order,
            status,
            task,
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

    pub async fn get(&self, task_id: String) -> Result<BackgroundTaskOut> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/background-task/{task_id}")
            .with_path_param("task_id", task_id)
            .execute(self.cfg)
            .await
    }
}
