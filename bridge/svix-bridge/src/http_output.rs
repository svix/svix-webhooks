use serde::Deserialize;
use svix_bridge_types::{async_trait, BoxError, ForwardRequest, ReceiverOutput};
use url::Url;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum HttpOutputOpts {
    // Single-variant enum so we can require the "type": "http" field in deserialization
    #[serde(rename = "http")]
    Inner { url: Url },
}

pub(crate) struct HttpOutput {
    client: reqwest::Client,
    url: Url,
    name: String,
}

impl HttpOutputOpts {
    pub(crate) fn into_receiver_output(self, name: String) -> Box<dyn ReceiverOutput> {
        let Self::Inner { url } = self;
        let client = reqwest::Client::new();
        Box::new(HttpOutput { client, url, name })
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
            .json(&request.payload)
            .send()
            .await?;
        Ok(())
    }
}
