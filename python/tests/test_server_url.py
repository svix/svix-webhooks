from svix.api import Svix, SvixAsync, SvixOptions


def base_url(server_url: str) -> str:
    return Svix("token", SvixOptions(server_url=server_url))._client.base_url


def test_server_url_trailing_slashes_are_stripped() -> None:
    assert base_url("https://api.example.com/") == "https://api.example.com"
    assert base_url("https://api.example.com///") == "https://api.example.com"
    assert (
        base_url("https://api.example.com/prefix/") == "https://api.example.com/prefix"
    )


def test_server_url_without_trailing_slash_is_unchanged() -> None:
    assert base_url("https://api.example.com") == "https://api.example.com"


def test_default_server_url() -> None:
    assert Svix("token")._client.base_url == "https://api.svix.com"


def test_regional_server_url() -> None:
    assert Svix("testsk.eu")._client.base_url == "https://api.eu.svix.com"


def test_async_client_normalizes_server_url() -> None:
    svx = SvixAsync("token", SvixOptions(server_url="https://api.example.com/"))
    assert svx._client.base_url == "https://api.example.com"


def request_url(httpx_mock, server_url: str) -> str:
    """Send a request through a client built with `server_url`, return the URL hit."""
    svx = Svix("token", SvixOptions(server_url=server_url))
    httpx_mock.add_response(
        json={
            "data": [],
            "done": True,
            "iterator": None,
            "prevIterator": None,
        },
    )
    svx.application.list()
    return str(httpx_mock.get_request().url)


def test_request_url_has_no_double_slash(httpx_mock) -> None:
    assert (
        request_url(httpx_mock, "https://api.example.test/")
        == "https://api.example.test/api/v1/app"
    )


def test_request_url_keeps_path_prefix(httpx_mock) -> None:
    assert (
        request_url(httpx_mock, "https://api.example.test/prefix/")
        == "https://api.example.test/prefix/api/v1/app"
    )
