from .internal.http_client import AuthenticatedHttpClient, DEFAULT_SERVER_URL
from .options import CoyoteOptions


class ClientBase:
    _client: AuthenticatedHttpClient

    def __init__(
        self, auth_token: str, options: CoyoteOptions = CoyoteOptions()
    ) -> None:
        if len(options.retry_schedule) > 5:
            raise ValueError("number of retries must not exceed 5")

        host = options.server_url or DEFAULT_SERVER_URL
        client = AuthenticatedHttpClient(
            base_url=host,
            token=auth_token,
            headers={"user-agent": "svix-libs/0.1.0/python"},
            verify_ssl=True,
            retry_schedule=options.retry_schedule,
            timeout=options.timeout,
            follow_redirects=False,
            raise_on_unexpected_status=True,
            proxy=options.proxy,
        )
        self._client = client
