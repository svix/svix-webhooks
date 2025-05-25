#[derive(Debug, Clone, PartialEq, Default)]
pub struct SvixOptions{
   pub(crate) wanted_server_url: Option<string>,
   pub initial_retry_delay_millis: Option<i64>,
   pub num_retries: Option<i32>
}