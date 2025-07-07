// Modified version of the file openapi-generator would usually put in
// apis/request.rs

use std::{collections::HashMap, time::Duration};

use http1::header::{HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use http_body_util::{BodyExt as _, Full};
use hyper::body::Bytes;
use itertools::Itertools as _;
use rand::Rng;
use serde::de::DeserializeOwned;

use crate::{error::Error, models, Configuration};

#[allow(dead_code)]
pub(crate) enum Auth {
    None,
    Bearer,
}

/// If the authorization type is unspecified then it will be automatically
/// detected based on the configuration. This functionality is useful when the
/// OpenAPI definition does not include an authorization scheme.
#[derive(Clone)]
pub(crate) struct Request {
    method: http1::Method,
    path: &'static str,
    query_params: HashMap<&'static str, String>,
    no_return_type: bool,
    path_params: HashMap<&'static str, String>,
    header_params: HashMap<&'static str, String>,
    // TODO: multiple body params are possible technically, but not supported here.
    serialized_body: Option<String>,
}

impl Request {
    pub fn new(method: http1::Method, path: &'static str) -> Self {
        Request {
            method,
            path,
            query_params: HashMap::new(),
            path_params: HashMap::new(),
            header_params: HashMap::new(),
            serialized_body: None,
            no_return_type: false,
        }
    }

    pub fn with_body_param<T: serde::Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(serde_json::to_string(&param).unwrap());
        self
    }

    pub fn with_optional_header_param(
        mut self,
        basename: &'static str,
        param: Option<String>,
    ) -> Self {
        if let Some(value) = param {
            self.header_params.insert(basename, value);
        }
        self
    }

    pub fn with_query_param(mut self, basename: &'static str, param: impl QueryParamValue) -> Self {
        self.query_params.insert(basename, param.encode());
        self
    }

    pub fn with_optional_query_param<T: QueryParamValue>(
        mut self,
        basename: &'static str,
        param: Option<T>,
    ) -> Self {
        if let Some(value) = param {
            self.query_params.insert(basename, value.encode());
        }
        self
    }

    pub fn with_path_param(mut self, basename: &'static str, param: String) -> Self {
        self.path_params.insert(basename, param);
        self
    }

    pub fn returns_nothing(mut self) -> Self {
        self.no_return_type = true;
        self
    }

    pub async fn execute<T: DeserializeOwned>(self, conf: &Configuration) -> Result<T, Error> {
        match self.execute_with_backoff(conf, conf.num_retries).await? {
            // This is a hack; if there's no_ret_type, T is (), but serde_json gives an
            // error when deserializing "" into (), so deserialize 'null' into it
            // instead.
            // An alternate option would be to require T: Default, and then return
            // T::default() here instead since () implements that, but then we'd
            // need to impl default for all models.
            None => Ok(serde_json::from_str("null").expect("serde null value")),
            Some(bytes) => Ok(serde_json::from_slice(&bytes).map_err(Error::generic)?),
        }
    }

    async fn execute_with_backoff(
        mut self,
        conf: &Configuration,
        mut retries: u32,
    ) -> Result<Option<Bytes>, Error> {
        let no_return_type = self.no_return_type;
        if self.method == http1::Method::POST && !self.header_params.contains_key("idempotency-key")
        {
            self.header_params
                .insert("idempotency-key", format!("auto_{}", uuid::Uuid::new_v4()));
        }

        let mut request = self.build_request(conf)?;
        request
            .headers_mut()
            .insert("svix-req-id", rand::rng().random::<u32>().into());

        let mut retry_count = 0;
        const MAX_BACKOFF: Duration = Duration::from_secs(5);
        let mut backoff = Duration::from_millis(20);

        let execute_request = async |request| {
            let response = conf.client.request(request).await.map_err(Error::generic)?;

            let status = response.status();
            if !status.is_success() {
                Err(Error::from_response(status, response.into_body()).await)
            } else if no_return_type {
                Ok(None)
            } else {
                let bytes = response
                    .into_body()
                    .collect()
                    .await
                    .map_err(Error::generic)?
                    .to_bytes();
                Ok(Some(bytes))
            }
        };

        loop {
            let request_fut = execute_request(request.clone());
            let res = if let Some(duration) = conf.timeout {
                tokio::time::timeout(duration, request_fut)
                    .await
                    .map_err(Error::generic)?
            } else {
                request_fut.await
            };

            match res {
                Ok(result) => return Ok(result),
                e @ Err(Error::Validation(_)) => return e,
                Err(Error::Http(err)) if err.status.as_u16() < 500 => return Err(Error::Http(err)),
                e @ Err(_) => {
                    if retries == 0 {
                        return e;
                    }
                }
            }

            tokio::time::sleep(backoff).await;
            retry_count += 1;
            retries -= 1;
            request
                .headers_mut()
                .insert("svix-retry-count", retry_count.into());
            backoff = MAX_BACKOFF.min(backoff * 2);
        }
    }

    fn build_request(self, conf: &Configuration) -> Result<http1::Request<Full<Bytes>>, Error> {
        let mut path = self.path.to_owned();
        for (k, v) in self.path_params {
            // replace {id} with the value of the id path param
            path = path.replace(&format!("{{{k}}}"), &v);
        }

        let mut uri = format!("{}{}", conf.base_path, path);

        let mut query_string = ::url::form_urlencoded::Serializer::new("".to_owned());
        for (key, val) in self.query_params {
            query_string.append_pair(key, &val);
        }

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = http1::Uri::try_from(uri).map_err(Error::generic)?;
        let mut req_builder = http1::Request::builder().uri(uri).method(self.method);

        // Detect the authorization type if it hasn't been set.
        let auth = if conf.bearer_access_token.is_some() {
            Auth::Bearer
        } else {
            Auth::None
        };
        match auth {
            Auth::Bearer => {
                if let Some(token) = &conf.bearer_access_token {
                    let value =
                        HeaderValue::try_from(format!("Bearer {token}")).map_err(Error::generic)?;
                    req_builder = req_builder.header(AUTHORIZATION, value);
                }
            }
            Auth::None => {}
        }

        if let Some(user_agent) = &conf.user_agent {
            let value = HeaderValue::try_from(user_agent).map_err(Error::generic)?;
            req_builder = req_builder.header(USER_AGENT, value);
        }

        for (k, v) in self.header_params {
            // Return invalid header values back to the caller.
            // Previously, these would trigger the unwrap below this loop.
            let v = HeaderValue::try_from(v).map_err(Error::generic)?;
            req_builder = req_builder.header(k, v);
        }

        let req_headers = req_builder.headers_mut().unwrap();
        if let Some(body) = self.serialized_body {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            req_headers.insert(CONTENT_LENGTH, body.len().into());
            Ok(req_builder.body(Full::from(body)).map_err(Error::generic)?)
        } else {
            Ok(req_builder.body(Full::default()).map_err(Error::generic)?)
        }
    }
}

pub(crate) trait QueryParamValue {
    fn encode(&self) -> String;
}

macro_rules! impl_query_param_value {
    ($ty:ty) => {
        impl QueryParamValue for $ty {
            fn encode(&self) -> String {
                self.to_string()
            }
        }
    };
}

impl_query_param_value!(bool);
impl_query_param_value!(i32);
impl_query_param_value!(String);
impl_query_param_value!(models::BackgroundTaskStatus);
impl_query_param_value!(models::BackgroundTaskType);
impl_query_param_value!(models::MessageStatus);
impl_query_param_value!(models::Ordering);
impl_query_param_value!(models::StatusCodeClass);

impl QueryParamValue for Vec<String> {
    fn encode(&self) -> String {
        self.iter().format(",").to_string()
    }
}
