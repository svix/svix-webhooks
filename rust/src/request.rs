// Modified version of the file openapi-generator would usually put in
// apis/request.rs

use std::{collections::HashMap, time::Duration};

use http1::header::{HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use http_body_util::{BodyExt as _, Full};
use hyper::body::Bytes;
use itertools::Itertools as _;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
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
        match self.execute_with_backoff(conf).await? {
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

    async fn execute_with_backoff(mut self, conf: &Configuration) -> Result<Option<Bytes>, Error> {
        let no_return_type = self.no_return_type;
        if self.method == http1::Method::POST && !self.header_params.contains_key("idempotency-key")
        {
            self.header_params
                .insert("idempotency-key", format!("auto_{}", uuid::Uuid::new_v4()));
        }

        const MAX_BACKOFF: Duration = Duration::from_secs(5);

        let retry_schedule = match &conf.retry_schedule {
            Some(schedule) => schedule,
            None => &std::iter::successors(Some(Duration::from_millis(20)), |last_backoff| {
                Some(MAX_BACKOFF.min(*last_backoff * 2))
            })
            .take(conf.num_retries as usize)
            .collect(),
        };
        let mut retries = retry_schedule.iter();

        let mut request = self.build_request(conf)?;
        request
            .headers_mut()
            .insert("svix-req-id", rand::rng().random::<u32>().into());

        let mut retry_count = 0;

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

            let next_backoff = retries.next().copied();

            match res {
                Ok(result) => return Ok(result),
                e @ Err(Error::Validation(_)) => return e,
                Err(Error::Http(err)) if err.status.as_u16() < 500 => return Err(Error::Http(err)),
                e @ Err(_) => {
                    if next_backoff.is_none() {
                        return e;
                    }
                }
            }

            tokio::time::sleep(next_backoff.expect("next_backoff is always Some")).await;
            retry_count += 1;

            request
                .headers_mut()
                .insert("svix-retry-count", retry_count.into());
        }
    }

    fn build_request(self, conf: &Configuration) -> Result<http1::Request<Full<Bytes>>, Error> {
        const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
        const PATH: &AsciiSet = &FRAGMENT.add(b'#').add(b'?').add(b'{').add(b'}');
        const PATH_SEGMENT: &AsciiSet = &PATH.add(b'/').add(b'%');

        let mut path = self.path.to_owned();
        for (k, v) in self.path_params {
            // replace {id} with the value of the id path param
            let percent_encoded_path_param_value =
                utf8_percent_encode(&v, PATH_SEGMENT).to_string();
            path = path.replace(&format!("{{{k}}}"), &percent_encoded_path_param_value);
        }

        let mut uri = format!("{}{}", conf.base_path, path);

        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
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

        let mut request = if let Some(body) = self.serialized_body {
            let req_headers = req_builder.headers_mut().unwrap();
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            req_headers.insert(CONTENT_LENGTH, body.len().into());
            req_builder.body(Full::from(body)).map_err(Error::generic)?
        } else {
            req_builder.body(Full::default()).map_err(Error::generic)?
        };

        let request_headers = request.headers_mut();

        // Detect the authorization type if it hasn't been set.
        let auth = if conf.bearer_access_token.is_some() {
            Auth::Bearer
        } else {
            Auth::None
        };
        match auth {
            Auth::Bearer => {
                if let Some(token) = &conf.bearer_access_token {
                    let value = format!("Bearer {token}")
                        .try_into()
                        .map_err(Error::generic)?;
                    request_headers.insert(AUTHORIZATION, value);
                }
            }
            Auth::None => {}
        }

        if let Some(user_agent) = &conf.user_agent {
            let value = user_agent.try_into().map_err(Error::generic)?;
            request_headers.insert(USER_AGENT, value);
        }

        for (k, v) in self.header_params {
            let v = v.try_into().map_err(Error::generic)?;
            request_headers.insert(k, v);
        }

        Ok(request)
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
impl_query_param_value!(models::ConnectorProduct);
impl_query_param_value!(models::MessageStatus);
impl_query_param_value!(models::Ordering);
impl_query_param_value!(models::StatusCodeClass);

impl QueryParamValue for Vec<String> {
    fn encode(&self) -> String {
        self.iter().format(",").to_string()
    }
}
