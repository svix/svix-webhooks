use svix::api::{Svix, SvixOptions};

pub struct TestClient {
    pub client: Svix,
}

pub struct TestClientBuilder {
    token: Option<String>,
    url: Option<String>,
    retries: Option<u32>,
}

impl TestClientBuilder {
    pub fn new() -> Self {
        Self {
            token: None,
            url: None,
            retries: None,
        }
    }

    #[allow(unused)]
    pub fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    pub fn build(self) -> TestClient {
        let token = self.token.unwrap_or_else(|| {
            std::env::var("SVIX_TOKEN").expect("SVIX_TOKEN is required to run this test")
        });
        let url = self.url.unwrap_or_else(|| {
            std::env::var("SVIX_SERVER_URL").expect("SVIX_SERVER_URL is required to run this test")
        });
        let client = Svix::new(
            token.clone(),
            Some(SvixOptions {
                server_url: Some(url.clone()),
                num_retries: self.retries,
                ..Default::default()
            }),
        );

        TestClient { client }
    }
}
