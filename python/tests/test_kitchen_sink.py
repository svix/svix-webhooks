import os

import pytest

from svix import SvixOptions
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

CLIENT = (
    Svix(TOKEN, SvixOptions(server_url=SERVER_URL))
    if TOKEN is not None and SERVER_URL is not None
    else None
)


@pytest.mark.skipif(
    CLIENT is None,
    reason="SVIX_TOKEN and SVIX_SERVER_URL are required to run this test",
)
def test_endpoint_crud() -> None:
    app = CLIENT.application.create(ApplicationIn(name="app"))
    try:
        CLIENT.event_type.create(
            EventTypeIn(name="event.started", description="Something started")
        )
    except HttpError as e:
        assert e.status_code == 409, "conflicts are expected but other statuses are not"
    try:
        CLIENT.event_type.create(
            EventTypeIn(name="event.ended", description="Something ended")
        )
    except HttpError as e:
        assert e.status_code == 409, "conflicts are expected but other statuses are not"

    ep = CLIENT.endpoint.create(
        app.id, EndpointIn(url="https://example.svix.com/", channels=["ch0", "ch1"])
    )
    assert {s for s in ep.channels} == {"ch0", "ch1"}
    ep_patched = CLIENT.endpoint.patch(
        app.id, ep.id, EndpointPatch(filter_types=["event.started", "event.ended"])
    )
    assert {s for s in ep_patched.channels} == {"ch0", "ch1"}
    assert {s for s in ep_patched.filter_types} == {"event.started", "event.ended"}

    # Should succeed without error if the deserialization handles empty response bodies correctly
    CLIENT.endpoint.delete(app.id, ep.id)
