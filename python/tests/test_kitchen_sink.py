import os
import typing as t

import pytest
import requests

from svix import AutoConfig, SvixOptions
from svix.api import (
    ApplicationIn,
    EndpointIn,
    EventTypeIn,
    Svix,
)
from svix.exceptions import HttpError
from svix.models import EndpointPatch

TOKEN = os.getenv("SVIX_TOKEN")
SERVER_URL = os.getenv("SVIX_SERVER_URL")


@pytest.fixture
def client() -> t.Optional[Svix]:
    if TOKEN is not None and SERVER_URL is not None:
        return Svix(TOKEN, SvixOptions(server_url=SERVER_URL))
    else:
        return None


def test_endpoint_crud(client) -> None:
    if client is None:
        # the version of pytest that works with python < 3.10 has a bug in its type
        # annotations that reports pytest.skip() as taking 0 arguments
        pytest.skip("$SVIX_TOKEN and $SVIX_SERVER_URL must be set to run this test")  # ty: ignore[too-many-positional-arguments]
    app = client.application.create(ApplicationIn(name="app"))
    try:
        client.event_type.create(
            EventTypeIn(name="event.started", description="Something started")
        )
    except HttpError as e:
        assert e.status_code == 409, "conflicts are expected but other statuses are not"
    try:
        client.event_type.create(
            EventTypeIn(name="event.ended", description="Something ended")
        )
    except HttpError as e:
        assert e.status_code == 409, "conflicts are expected but other statuses are not"

    ep = client.endpoint.create(
        app.id, EndpointIn(url="https://example.svix.com/", channels=["ch0", "ch1"])
    )
    assert {s for s in ep.channels} == {"ch0", "ch1"}
    ep_patched = client.endpoint.patch(
        app.id, ep.id, EndpointPatch(filter_types=["event.started", "event.ended"])
    )
    assert {s for s in ep_patched.channels} == {"ch0", "ch1"}
    assert {s for s in ep_patched.filter_types} == {"event.started", "event.ended"}

    # Should succeed without error if the deserialization handles empty response bodies
    # correctly
    client.endpoint.delete(app.id, ep.id)


def test_autoconfig(client) -> None:
    if client is None:
        # the version of pytest that works with python3.8 has a bug in its type
        # annotations that reports pytest.skip() as taking 0 arguments
        pytest.skip("$SVIX_TOKEN and $SVIX_SERVER_URL must be set to run this test")  # ty: ignore[too-many-positional-arguments]

    app = client.application.create(ApplicationIn(name="app"))

    response = requests.post(
        f"{SERVER_URL}/api/v1/app/{app.id}/auto-config",
        headers={"Authorization": f"Bearer {TOKEN}"},
        timeout=30,
    )
    response.raise_for_status()
    assert response.status_code == 201
    autoconfig_out = response.json()
    endpoint_id = autoconfig_out["endpId"]
    autoconfig_token = autoconfig_out["token"]

    endpoint_url = "https://example.svix.com/autoconfig"
    configured = AutoConfig(
        autoconfig_token,
        EndpointIn(url=endpoint_url),
    ).subscribe()
    assert configured.id == endpoint_id
    assert configured.url == endpoint_url

    ep = client.endpoint.get(app.id, endpoint_id)
    assert ep.id == endpoint_id
    assert ep.url == endpoint_url

    client.endpoint.delete(app.id, endpoint_id)
