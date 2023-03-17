use std::{
    collections::HashMap,
    future::Future,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    pin::Pin,
    str::FromStr,
    sync::Arc,
    task::Poll,
    time::{Duration, Instant},
};

use axum::headers::{authorization::Credentials, Authorization};
use bytes::Bytes;
use futures::{future::BoxFuture, FutureExt};
use http::header::HeaderName;
use http::{HeaderMap, HeaderValue, Method, Response, StatusCode, Version};
use hyper::{
    client::connect::{dns::Name, HttpConnector},
    ext::HeaderCaseMap,
    Body, Client, Uri,
};
use hyper_openssl::HttpsConnector;
use ipnet::IpNet;
use openssl::ssl::{SslConnector, SslMethod};
use serde::Serialize;
use thiserror::Error;
use tokio::sync::Mutex;
use tower::Service;
use trust_dns_resolver::{
    error::ResolveError, lookup_ip::LookupIpIntoIter, AsyncResolver, TokioConnection,
    TokioConnectionProvider, TokioHandle,
};

pub type CaseSensitiveHeaderMap = HashMap<String, HeaderValue>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failure response: {0}")]
    FailureStatus(StatusCode),

    #[error("requests to this IP range are blocked (see the server configuration)")]
    BlockedIp,
    #[error("error resolving name: {0}")]
    ResolveError(#[from] ResolveError),

    #[error("request timed out")]
    TimedOut,

    #[error("error forming request: {0}")]
    InvalidHttpRequest(http::Error),
    #[error("error making request: {0}")]
    FailedRequest(hyper::Error),
}

pub struct WebhookClient {
    client: Client<HttpsConnector<NonLocalConnector>, Body>,
    whitelist_nets: Arc<Vec<IpNet>>,
}

impl WebhookClient {
    pub fn new(
        whitelist_nets: Option<Arc<Vec<IpNet>>>,
        whitelist_names: Option<Arc<Vec<String>>>,
    ) -> Self {
        let whitelist_nets = whitelist_nets.unwrap_or_else(|| Arc::new(Vec::new()));
        let whitelist_names = whitelist_names.unwrap_or_else(|| Arc::new(Vec::new()));

        let mut connector = HttpConnector::new_with_resolver(NonLocalDnsResolver::new(
            whitelist_nets.clone(),
            whitelist_names,
        ));
        connector.enforce_http(false);

        // Openssl is required here -- in practice, rustls does not support many
        // ciphers that we encounter on a regular basis:
        let ssl = SslConnector::builder(SslMethod::tls()).expect("SslConnector build failed");
        let https = HttpsConnector::with_connector(NonLocalConnector { connector }, ssl)
            .expect("HttpsConnector build failed");

        let client: Client<_, hyper::Body> = Client::builder()
            .http1_ignore_invalid_headers_in_responses(true)
            .http1_title_case_headers(true)
            .build(https);

        Self {
            client,
            whitelist_nets,
        }
    }

    pub async fn execute(&self, request: Request) -> Result<Response<Body>, Error> {
        self.execute_inner(request, true).await
    }

    pub fn execute_inner(
        &self,
        request: Request,
        retry: bool,
    ) -> BoxFuture<Result<Response<Body>, Error>> {
        async move {
            let org_req = request.clone();
            if let Some(auth) = request.uri.authority() {
                if let Ok(ip) = auth.host().parse::<IpAddr>() {
                    if !is_allowed(ip)
                        && !self
                            .whitelist_nets
                            .iter()
                            .any(|subnet| subnet.contains(&ip))
                    {
                        return Err(Error::BlockedIp);
                    }
                }
            }

            let mut req = if let Some(body) = request.body {
                hyper::Request::builder()
                    .method(request.method)
                    .uri(request.uri)
                    .version(request.version)
                    .body(Body::from(body))
                    .map_err(Error::InvalidHttpRequest)?
            } else {
                hyper::Request::builder()
                    .method(request.method)
                    .uri(request.uri)
                    .version(request.version)
                    .body(Body::empty())
                    .map_err(Error::InvalidHttpRequest)?
            };

            *req.headers_mut() = request.headers;

            if let Some(header_names) = request.header_names {
                req.extensions_mut().insert(header_names);
            }

            let start = Instant::now();
            let res = if let Some(timeout) = request.timeout {
                match tokio::time::timeout(timeout, self.client.request(req)).await {
                    Ok(Ok(resp)) => Ok(resp),
                    Ok(Err(e)) => Err({
                        if e.to_string()
                            .contains("requests to this IP range are blocked")
                        {
                            Error::BlockedIp
                        } else {
                            Error::FailedRequest(e)
                        }
                    }),
                    Err(_to) => Err(Error::TimedOut),
                }
            } else {
                self.client.request(req).await.map_err(|e| {
                    if e.to_string()
                        .contains("requests to this IP range are blocked")
                    {
                        Error::BlockedIp
                    } else {
                        Error::FailedRequest(e)
                    }
                })
            };

            if !retry {
                return res;
            }

            match res {
                Err(ref e) => match e {
                    Error::FailedRequest(e) if start.elapsed() < Duration::from_millis(1000) => {
                        tracing::info!("Insta-retrying: {}", e);
                        self.execute_inner(org_req, false).await
                    }
                    _ => res,
                },
                res => res,
            }
        }
        .boxed()
    }
}

#[derive(Clone)]
pub struct Request {
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    header_names: Option<HeaderCaseMap>,
    body: Option<Vec<u8>>,
    timeout: Option<Duration>,
    version: Version,
}

pub struct RequestBuilder {
    method: Option<Method>,
    uri: Option<Uri>,
    accept: Option<HeaderValue>,
    user_agent: Option<HeaderValue>,
    headers: Option<HeaderMap>,
    header_names: Option<HeaderCaseMap>,
    body: Option<Vec<u8>>,
    version: Option<Version>,
    timeout: Option<Duration>,
    basic_auth: Option<Vec<u8>>,

    // Derived from body
    content_type: Option<HeaderValue>,
}

#[derive(Debug)]
pub struct RequestBuildError(pub Vec<BuildError>);

impl std::fmt::Display for RequestBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str = self.0.iter().fold(String::default(), |acc, err| {
            if acc.is_empty() {
                format!("Build failed: {err}")
            } else {
                format!("{acc}; {err}")
            }
        });
        write!(f, "{err_str}")
    }
}

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("uri missing")]
    UriMissing,
    #[error("version missing")]
    VersionMissing,
}

fn decode_or_log(s: &str) -> String {
    urlencoding::decode(s)
        .map(|x| x.into_owned())
        .unwrap_or_else(|_| {
            tracing::error!("URL decoding failed");
            s.to_owned()
        })
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            method: None,
            uri: None,
            accept: None,
            user_agent: None,
            headers: None,
            header_names: None,
            body: None,
            version: None,
            timeout: None,
            content_type: None,
            basic_auth: None,
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn uri(mut self, uri: url::Url) -> Self {
        let basic_auth = if uri.password().is_some() || !uri.username().is_empty() {
            let username = decode_or_log(uri.username());
            let password = uri.password().map(decode_or_log).unwrap_or_default();

            Some(
                Authorization::basic(&username, &password)
                    .0
                    .encode()
                    .as_bytes()
                    .to_vec(),
            )
        } else {
            None
        };
        self.basic_auth = basic_auth;

        let uri =
            Uri::from_str(uri.as_str()).expect("If it's a valid url::Url, it's also a valid Uri");
        self.uri = Some(uri);
        self
    }

    pub fn uri_str(self, uri: &str) -> Result<Self, url::ParseError> {
        let uri = url::Url::from_str(uri)?;
        Ok(self.uri(uri))
    }

    fn build_headers(
        headers: CaseSensitiveHeaderMap,
    ) -> (hyper::HeaderMap, hyper::ext::HeaderCaseMap) {
        let mut hdr_map = hyper::HeaderMap::with_capacity(headers.len());
        let mut case_sensitive_hdrs: hyper::HeaderMap<Bytes> =
            hyper::HeaderMap::with_capacity(headers.len());
        for (k, v) in headers.into_iter() {
            match HeaderName::from_str(&k) {
                Ok(key) => {
                    hdr_map.insert(key.clone(), v);
                    case_sensitive_hdrs.insert(key, Bytes::copy_from_slice(k.as_bytes()));
                }
                Err(e) => {
                    tracing::error!("Failured to parse header {} {}", k, e);
                }
            }
        }
        (hdr_map, case_sensitive_hdrs.into())
    }

    pub fn headers(mut self, headers: CaseSensitiveHeaderMap) -> Self {
        let (hdrs, case_map) = Self::build_headers(headers);
        self.headers = Some(hdrs);
        self.header_names = Some(case_map);
        self
    }

    pub fn body(mut self, body: Vec<u8>, content_type: HeaderValue) -> Self {
        self.body = Some(body);
        self.content_type = Some(content_type);
        self
    }

    pub fn json_body<T: Serialize>(self, body: T) -> Result<Self, serde_json::Error> {
        let body = serde_json::to_vec(&body)?;
        Ok(self.body(body, HeaderValue::from_static("application/json")))
    }

    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn user_agent(mut self, user_agent: HeaderValue) -> Self {
        self.user_agent = Some(user_agent);
        self
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestBuilder {
    fn validate(&self) -> Result<(), RequestBuildError> {
        let mut errs: Vec<BuildError> = Vec::new();
        if self.uri.is_none() {
            errs.push(BuildError::UriMissing);
        }
        if self.version.is_none() {
            errs.push(BuildError::VersionMissing);
        }

        if !errs.is_empty() {
            Err(RequestBuildError(errs))
        } else {
            Ok(())
        }
    }

    pub fn build(self) -> Result<Request, RequestBuildError> {
        self.validate()?;

        let custom_headers = self.headers.unwrap_or_default();

        let uri = self.uri.unwrap();
        let authority = uri.authority().expect("Missing authority");
        let host = match authority.port() {
            Some(port) => format!("{}:{}", authority.host(), port),
            None => authority.host().to_string(),
        };

        let mut headers = HeaderMap::with_capacity(3 + custom_headers.len());

        // Ensure that host header is first -- even though this is technically
        // not required by HTTP spec, some clients fail if it's not first:
        headers.insert(http::header::HOST, HeaderValue::from_str(&host).unwrap());
        headers.insert(
            http::header::ACCEPT,
            self.accept.unwrap_or(HeaderValue::from_static("*/*")),
        );
        headers.insert(
            http::header::CONTENT_TYPE,
            self.content_type
                .unwrap_or(HeaderValue::from_static("application/json")),
        );

        headers.extend(custom_headers.into_iter());

        if let Some(user_agent) = self.user_agent {
            headers.insert(http::header::USER_AGENT, user_agent);
        }

        if let Some(auth_header) = self.basic_auth {
            if !headers.contains_key(http::header::AUTHORIZATION) {
                headers.insert(
                    http::header::AUTHORIZATION,
                    HeaderValue::from_bytes(&auth_header).unwrap(),
                );
            }
        }

        Ok(Request {
            method: self.method.unwrap_or(Method::POST),
            uri,
            headers,
            header_names: self.header_names,
            body: self.body,
            timeout: self.timeout,
            version: self.version.unwrap(),
        })
    }
}

#[derive(Clone, Debug)]
struct NonLocalConnector {
    connector: HttpConnector<NonLocalDnsResolver>,
}

impl Service<Uri> for NonLocalConnector {
    type Response = <hyper::client::HttpConnector as Service<Uri>>::Response;
    type Error = <hyper::client::HttpConnector as Service<Uri>>::Error;

    type Future = <hyper::client::HttpConnector<NonLocalDnsResolver> as Service<Uri>>::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.connector.poll_ready(cx)
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        self.connector.call(req)
    }
}

#[derive(Clone, Debug)]
struct NonLocalDnsResolver {
    state: Arc<Mutex<DnsState>>,
    whitelist_nets: Arc<Vec<IpNet>>,
    whitelist_names: Arc<Vec<String>>,
}

#[derive(Clone, Debug)]
enum DnsState {
    Init,
    Ready(Arc<AsyncResolver<TokioConnection, TokioConnectionProvider>>),
}

impl NonLocalDnsResolver {
    pub fn new(whitelist_nets: Arc<Vec<IpNet>>, whitelist_names: Arc<Vec<String>>) -> Self {
        NonLocalDnsResolver {
            state: Arc::new(Mutex::new(DnsState::Init)),
            whitelist_nets,
            whitelist_names,
        }
    }
}

impl Service<Name> for NonLocalDnsResolver {
    type Response = SocketAddrs;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, name: Name) -> Self::Future {
        let resolver = self.clone();
        let whitelist_nets = self.whitelist_nets.clone();
        let whitelist_names = self.whitelist_names.clone();

        Box::pin(async move {
            let mut lock = resolver.state.lock().await;

            let resolver = match &*lock {
                DnsState::Init => {
                    let resolver = new_resolver().await?;
                    *lock = DnsState::Ready(resolver.clone());
                    resolver
                }

                DnsState::Ready(resolver) => resolver.clone(),
            };

            drop(lock);

            let whitelisted_name = whitelist_names
                .iter()
                .any(|whitelisted| whitelisted == name.as_str());

            let lookup = resolver.lookup_ip(name.as_str()).await?;

            if lookup.iter().all(|ip| {
                !is_allowed(ip)
                    && !whitelist_nets.iter().any(|subnet| subnet.contains(&ip))
                    && !whitelisted_name
            }) {
                Err(Error::BlockedIp)
            } else {
                Ok(SocketAddrs {
                    iter: lookup.into_iter(),
                    whitelist_nets,
                    whitelisted_name,
                })
            }
        })
    }
}

pub struct SocketAddrs {
    iter: LookupIpIntoIter,
    whitelist_nets: Arc<Vec<IpNet>>,
    whitelisted_name: bool,
}

impl Iterator for SocketAddrs {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(ip_addr) => {
                    if is_allowed(ip_addr)
                        || self
                            .whitelist_nets
                            .iter()
                            .any(|subnet| subnet.contains(&ip_addr))
                        || self.whitelisted_name
                    {
                        return Some(SocketAddr::from((ip_addr, 0)));
                    }
                }

                None => return None,
            }
        }
    }
}

async fn new_resolver(
) -> Result<Arc<AsyncResolver<TokioConnection, TokioConnectionProvider>>, ResolveError> {
    Ok(Arc::new(AsyncResolver::from_system_conf(TokioHandle)?))
}

fn is_allowed(addr: IpAddr) -> bool {
    match addr {
        IpAddr::V4(addr) => {
            !addr.is_private()
                && !addr.is_loopback()
                && !addr.is_link_local()
                && !addr.is_broadcast()
                && !addr.is_documentation()
                && !is_shared(addr)
                && !is_reserved(addr)
                && !is_benchmarking(addr)
                && !starts_with_zero(addr)
        }
        IpAddr::V6(addr) => {
            !addr.is_multicast()
                && !addr.is_loopback()
                && !is_unicast_link_local(addr)
                && !is_unique_local(addr)
                && !addr.is_unspecified()
                && !is_documentation_v6(addr)
        }
    }
}

/// Util functions copied from the unstable standard library near identically
fn is_shared(addr: Ipv4Addr) -> bool {
    addr.octets()[0] == 100 && (addr.octets()[1] & 0b1100_0000 == 0b0100_0000)
}

fn is_reserved(addr: Ipv4Addr) -> bool {
    (addr.octets()[0] == 192 && addr.octets()[1] == 0 && addr.octets()[2] == 0)
        || (addr.octets()[0] & 240 == 240 && !addr.is_broadcast())
}

fn is_benchmarking(addr: Ipv4Addr) -> bool {
    addr.octets()[0] == 198 && (addr.octets()[1] & 0xfe) == 18
}

fn starts_with_zero(addr: Ipv4Addr) -> bool {
    addr.octets()[0] == 0
}

fn is_unicast_link_local(addr: Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xffc0) == 0xfe80
}

fn is_unique_local(addr: Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xfe00) == 0xfc00
}

fn is_documentation_v6(addr: Ipv6Addr) -> bool {
    (addr.segments()[0] == 0x2001) && (addr.segments()[1] == 0xdb8)
}

#[cfg(test)]
mod tests {
    use std::{net::IpAddr, str::FromStr};

    use super::*;

    #[test]
    fn is_allowed_test() {
        // Copied shamelessly from the standard library `is_global` docs
        assert!(!is_allowed(IpAddr::from([10, 254, 0, 0])));
        assert!(!is_allowed(IpAddr::from([192, 168, 10, 65])));
        assert!(!is_allowed(IpAddr::from([172, 16, 10, 65])));
        assert!(!is_allowed(IpAddr::from([0, 1, 2, 3])));
        assert!(!is_allowed(IpAddr::from([0, 0, 0, 0])));
        assert!(!is_allowed(IpAddr::from([127, 0, 0, 1])));
        assert!(!is_allowed(IpAddr::from([169, 254, 45, 1])));
        assert!(!is_allowed(IpAddr::from([255, 255, 255, 255])));
        assert!(!is_allowed(IpAddr::from([192, 0, 2, 255])));
        assert!(!is_allowed(IpAddr::from([198, 51, 100, 65])));
        assert!(!is_allowed(IpAddr::from([203, 0, 113, 6])));
        assert!(!is_allowed(IpAddr::from([100, 100, 0, 0])));
        assert!(!is_allowed(IpAddr::from([192, 0, 0, 0])));
        assert!(!is_allowed(IpAddr::from([192, 0, 0, 255])));
        assert!(!is_allowed(IpAddr::from([250, 10, 20, 30])));
        assert!(!is_allowed(IpAddr::from([198, 18, 0, 0])));

        assert!(is_allowed(IpAddr::from([1, 1, 1, 1])));

        assert!(!is_allowed(IpAddr::from([0, 0, 0, 0, 0, 0, 0, 0x1])));

        assert!(is_allowed(IpAddr::from([0, 0, 0, 0xffff, 0, 0, 0, 0x1])));
    }

    #[test]
    fn test_builder() {
        match RequestBuilder::new().build() {
            Err(e) => assert_eq!("Build failed: uri missing; version missing", e.to_string()),
            Ok(_) => panic!(),
        }

        assert!(RequestBuilder::new()
            .version(Version::HTTP_11)
            .build()
            .is_err());

        assert!(RequestBuilder::new()
            .uri(url::Url::from_str("http://127.0.0.1/").unwrap())
            .version(Version::HTTP_11)
            .build()
            .is_ok());
    }

    #[test]
    fn test_header_casings() {
        let hdrs = CaseSensitiveHeaderMap::from([(
            "tEsT-header-1".to_owned(),
            HeaderValue::from_static("value"),
        )]);

        let req = RequestBuilder::new()
            .uri(url::Url::from_str("http://127.0.0.1/").unwrap())
            .version(Version::HTTP_11)
            .headers(hdrs)
            .build()
            .unwrap();

        assert_eq!(
            req.header_names
                .unwrap()
                .get("test-header-1".parse().unwrap())
                .unwrap(),
            "tEsT-header-1".as_bytes()
        );
        assert_eq!(
            req.headers.get("test-header-1").unwrap(),
            HeaderValue::from_static("value")
        );
    }

    #[test]
    fn test_url_basic_auth() {
        let req = RequestBuilder::new()
            .uri(url::Url::from_str("http://test:123@127.0.0.1/").unwrap())
            .version(Version::HTTP_11)
            .build()
            .unwrap();

        assert_eq!(
            req.headers.get("authorization").unwrap(),
            "Basic dGVzdDoxMjM=".as_bytes()
        );

        let req_user_only = RequestBuilder::new()
            .uri(url::Url::from_str("http://test:@127.0.0.1/").unwrap())
            .version(Version::HTTP_11)
            .build()
            .unwrap();

        assert_eq!(
            req_user_only.headers.get("authorization").unwrap(),
            "Basic dGVzdDo=".as_bytes()
        );

        let req_pass_only = RequestBuilder::new()
            .uri(url::Url::from_str("http://:123@127.0.0.1/").unwrap())
            .version(Version::HTTP_11)
            .build()
            .unwrap();

        assert_eq!(
            req_pass_only.headers.get("authorization").unwrap(),
            "Basic OjEyMw==".as_bytes()
        );

        let req_no_basic_auth = RequestBuilder::new()
            .uri(url::Url::from_str("http://127.0.0.1/").unwrap())
            .version(Version::HTTP_11)
            .build()
            .unwrap();

        assert!(req_no_basic_auth.headers.get("authorization").is_none());

        let req_special_chars = RequestBuilder::new()
            .uri(url::Url::from_str("http://test==:123==@127.0.0.1/").unwrap())
            .version(Version::HTTP_11)
            .build()
            .unwrap();

        assert_eq!(
            req_special_chars.headers.get("authorization").unwrap(),
            "Basic dGVzdD09OjEyMz09".as_bytes()
        );
    }

    #[test]
    fn test_host_header() {
        let req = RequestBuilder::new()
            .uri(url::Url::from_str("http://127.0.0.1/").unwrap())
            .version(Version::HTTP_11)
            .build()
            .unwrap();

        assert_eq!(req.headers.get("host").unwrap(), "127.0.0.1".as_bytes());

        let req_with_port = RequestBuilder::new()
            .uri(url::Url::from_str("http://127.0.0.1:8000/").unwrap())
            .version(Version::HTTP_11)
            .build()
            .unwrap();

        assert_eq!(
            req_with_port.headers.get("host").unwrap(),
            "127.0.0.1:8000".as_bytes()
        );
    }
}
