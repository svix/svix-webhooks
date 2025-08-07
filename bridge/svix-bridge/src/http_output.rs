use anyhow::Context;
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use indexmap::IndexMap;
use serde::Deserialize;
use svix_bridge_types::{async_trait, BoxError, ForwardRequest, ReceiverOutput};
use url::Url;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum HttpOutputOpts {
    // Single-variant enum so we can require the "type": "http" field in deserialization
    #[serde(rename = "http")]
    Inner {
        url: Url,
        #[serde(default)]
        headers: IndexMap<String, String>,
    },
}

pub(crate) struct HttpOutput {
    client: reqwest::Client,
    url: Url,
    headers: HeaderMap,
    name: String,
}

impl HttpOutputOpts {
    pub(crate) fn into_receiver_output(
        self,
        name: String,
    ) -> anyhow::Result<Box<dyn ReceiverOutput>> {
        let Self::Inner { url, headers } = self;
        let headers = headers
            .into_iter()
            .map(|(k, v)| {
                Ok((
                    HeaderName::try_from(k.as_str())
                        .with_context(|| format!("invalid header name `{k}`"))?,
                    HeaderValue::try_from(v.as_str())
                        .with_context(|| format!("invalid header value `{v}`"))?,
                ))
            })
            .collect::<anyhow::Result<_>>()?;

        let client = reqwest::Client::new();
        Ok(Box::new(HttpOutput {
            client,
            url,
            headers,
            name,
        }))
    }
}

#[async_trait]
impl ReceiverOutput for HttpOutput {
    fn name(&self) -> &str {
        &self.name
    }

    async fn handle(&self, request: ForwardRequest) -> Result<(), BoxError> {
        self.client
            .post(self.url.clone())
            .headers(self.headers.clone())
            .json(&request.payload)
            .send()
            .await?;
        Ok(())
    }
}
