// Modified version of the file openapi-generator would usually put in
// apis/request.rs

use std::collections::HashMap;

use http1::header::{HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use http_body_util::{BodyExt as _, Full};
use hyper::body::Bytes;
use serde::de::DeserializeOwned;

use crate::{error::Error, Configuration};

#[allow(dead_code)]
pub(crate) enum Auth {
    None,
    Bearer,
}

/// If the authorization type is unspecified then it will be automatically
/// detected based on the configuration. This functionality is useful when the
/// OpenAPI definition does not include an authorization scheme.
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

    pub fn with_query_param(mut self, basename: &'static str, param: String) -> Self {
        self.query_params.insert(basename, param);
        self
    }

    pub fn with_optional_query_param<T: ToString>(
        mut self,
        basename: &'static str,
        param: Option<T>,
    ) -> Self {
        if let Some(value) = param {
            self.query_params.insert(basename, value.to_string());
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
        match self.execute_inner(conf).await? {
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

    async fn execute_inner(self, conf: &Configuration) -> Result<Option<Bytes>, Error> {
        let mut path = self.path.to_owned();
        for (k, v) in self.path_params {
            // replace {id} with the value of the id path param
            path = path.replace(&format!("{{{k}}}"), &v);
        }

        let mut uri = format!("{}{}", conf.base_path, path);

        // Work around rustc issue - we need to make sure that `query_string` is
        // not captured by the outer `async` generator. Using
        // `drop(query_string)` is insufficient, so we create a new scope
        {
            let mut query_string = ::url::form_urlencoded::Serializer::new("".to_owned());
            for (key, val) in self.query_params {
                query_string.append_pair(key, &val);
            }

            let query_string_str = query_string.finish();
            if !query_string_str.is_empty() {
                uri += "?";
                uri += &query_string_str;
            }
        }

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
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {token}"));
                }
            }
            Auth::None => {}
        }

        if let Some(user_agent) = &conf.user_agent {
            req_builder = req_builder.header(
                USER_AGENT,
                HeaderValue::from_str(user_agent).map_err(Error::generic)?,
            );
        }

        for (k, v) in self.header_params {
            req_builder = req_builder.header(k, v);
        }

        let req_headers = req_builder.headers_mut().unwrap();
        let request = if let Some(body) = self.serialized_body {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            req_headers.insert(CONTENT_LENGTH, body.len().into());
            req_builder.body(Full::from(body)).map_err(Error::generic)?
        } else {
            req_builder.body(Full::default()).map_err(Error::generic)?
        };

        let execute_request = async {
            let response = conf.client.request(request).await.map_err(Error::generic)?;

            let status = response.status();
            if !status.is_success() {
                Err(Error::from_response(status, response.into_body()).await)
            } else if self.no_return_type {
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

        if let Some(duration) = conf.timeout {
            tokio::time::timeout(duration, execute_request)
                .await
                .map_err(Error::generic)?
        } else {
            execute_request.await
        }
    }
}
