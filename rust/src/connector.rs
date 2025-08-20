use std::{future::Future, pin::Pin};

use http1::Uri;
use hyper_util::{
    client::legacy::connect::{
        proxy::{SocksV5, Tunnel},
        HttpConnector,
    },
    rt::TokioIo,
};
use tokio::net::TcpStream;
use tower_service::Service;

// If no TLS backend is enabled, use plain http connector.
#[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
type HttpsIfAvailable<T> = T;
#[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
type MaybeHttpsStream<T> = T;

// If only native TLS is enabled, use that.
#[cfg(all(feature = "native-tls", not(feature = "rustls-tls")))]
use hyper_tls::{HttpsConnector as HttpsIfAvailable, MaybeHttpsStream};

// If rustls is enabled, use that.
#[cfg(feature = "rustls-tls")]
use hyper_rustls::{HttpsConnector as HttpsIfAvailable, MaybeHttpsStream};

fn wrap_connector<H>(conn: H) -> HttpsIfAvailable<H> {
    #[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
    return conn;

    #[cfg(feature = "rustls-tls")]
    {
        let builder = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http();

        #[cfg(feature = "http1")]
        let builder = builder.enable_http1();

        #[cfg(feature = "http2")]
        let builder = builder.enable_http2();

        builder.wrap_connector(conn)
    }

    #[cfg(all(feature = "native-tls", not(feature = "rustls-tls")))]
    return hyper_tls::HttpsConnector::new_with_connector(conn);
}

#[derive(Clone, Debug)]
pub(crate) enum Connector {
    Regular(HttpsIfAvailable<HttpConnector>),
    Socks5Proxy(HttpsIfAvailable<SocksV5<HttpConnector>>),
    HttpProxy(HttpsIfAvailable<Tunnel<HttpConnector>>),
}

// FIXME: If we ever do a breaking release, change this
// to be fallible and bubble the error up from `Svix::new`.
pub(crate) fn make_connector(proxy_addr: Option<String>) -> Connector {
    let mut http = hyper_util::client::legacy::connect::HttpConnector::new();
    if cfg!(any(feature = "native-tls", feature = "rustls-tls")) {
        http.enforce_http(false);
    }

    let Some(proxy_addr) = proxy_addr else {
        return Connector::Regular(wrap_connector(http));
    };
    let proxy_addr = match proxy_addr.parse::<Uri>() {
        Ok(proxy_addr) => proxy_addr,
        Err(e) => {
            tracing::error!(
                error = &e as &dyn std::error::Error,
                "invalid proxy address"
            );
            return Connector::Regular(wrap_connector(http));
        }
    };

    match proxy_addr.scheme_str() {
        Some("http" | "https") => {
            let tunnel = Tunnel::new(proxy_addr, http);
            Connector::HttpProxy(wrap_connector(tunnel))
        }
        Some("socks5") => {
            let socks = SocksV5::new(proxy_addr, http).local_dns(true);
            Connector::Socks5Proxy(wrap_connector(socks))
        }
        Some("socks5h") => {
            let socks = SocksV5::new(proxy_addr, http);
            Connector::Socks5Proxy(wrap_connector(socks))
        }
        scheme => {
            tracing::error!(
                scheme,
                "invalid proxy address: scheme must be one of http, https, socks5, socks5h"
            );
            Connector::Regular(wrap_connector(http))
        }
    }
}

impl Service<Uri> for Connector {
    type Response = MaybeHttpsStream<TokioIo<TcpStream>>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match self {
            Self::Regular(inner) => inner.poll_ready(cx),
            Self::Socks5Proxy(inner) => inner.poll_ready(cx),
            Self::HttpProxy(inner) => inner.poll_ready(cx),
        }
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        match self {
            Self::Regular(inner) => Box::pin(inner.call(req)),
            Self::Socks5Proxy(inner) => Box::pin(inner.call(req)),
            Self::HttpProxy(inner) => Box::pin(inner.call(req)),
        }
    }
}
