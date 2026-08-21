use redis::{
    AsyncConnectionConfig, Client, ErrorKind, IntoConnectionInfo, RedisError, ServerErrorKind,
    aio::MultiplexedConnection, io::tcp::TcpSettings,
};

// Needed to override the new default response timeout
// which breaks blocking requests
#[derive(Clone, Debug)]
pub struct RedisConnectionManager {
    client: Client,
}

impl RedisConnectionManager {
    pub fn new<T: IntoConnectionInfo>(info: T) -> Result<Self, RedisError> {
        let conn_info = info
            .into_connection_info()?
            .set_tcp_settings(TcpSettings::default().set_nodelay(true));
        Ok(Self {
            client: Client::open(conn_info)?,
        })
    }
}

impl bb8::ManageConnection for RedisConnectionManager {
    type Connection = MultiplexedConnection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client
            .get_multiplexed_async_connection_with_config(
                &AsyncConnectionConfig::new().set_response_timeout(None),
            )
            .await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => {
                let kind = ErrorKind::Server(ServerErrorKind::ResponseError);
                Err((kind, "ping request").into())
            }
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
