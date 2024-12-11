use crate::{apis::background_tasks_api, error::Result, models::*, Configuration};

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
        background_tasks_api::list_background_tasks(
            self.cfg,
            background_tasks_api::ListBackgroundTasksParams {
                status,
                task,
                limit,
                iterator,
                order,
            },
        )
        .await
    }

    pub async fn get(&self, task_id: String) -> Result<BackgroundTaskOut> {
        background_tasks_api::get_background_task(
            self.cfg,
            background_tasks_api::GetBackgroundTaskParams { task_id },
        )
        .await
    }
}
