import base64
import json

import pytest

from svix import AutoConfigConsumer, AutoConfigError
from svix.autoconfig import _AUTOCONFIG_TOKEN_PREFIX_V1, _decode_autoconfig_token_v1
from svix.models import SinkInCommon
from svix.models.auto_config_sink_type import AutoConfigSinkType
from svix.models.subscribe_in import SubscribeIn


def _make_token(**overrides) -> str:
    payload = {
        "aid": "app_1",
        "eid": "ep_2",
        "surl": "https://api.example.test",
        "esec": "whsec_Zm9v",
        "tok": "sk_test_xyz",
    }
    payload.update(overrides)
    raw = json.dumps(payload).encode("utf-8")
    # Strip padding so the token emulates the server's unpadded output.
    b64 = base64.b64encode(raw).decode("ascii").replace("=", "")
    return f"{_AUTOCONFIG_TOKEN_PREFIX_V1}{b64}"


def test_decode_autoconfig_token_v1_parses_payload():
    payload = {
        "aid": "app_1",
        "eid": "ep_2",
        "surl": "https://api.example.test",
        "esec": "whsec_Zm9v",
        "tok": "sk_test_xyz",
    }
    raw = json.dumps(payload).encode()
    token = f"{_AUTOCONFIG_TOKEN_PREFIX_V1}{base64.b64encode(raw).decode('ascii')}"
    content = _decode_autoconfig_token_v1(token)

    assert content.app_id == "app_1"
    assert content.endpoint_id == "ep_2"
    assert content.server_url == "https://api.example.test"
    assert content.endpoint_secret == "whsec_Zm9v"
    assert content.token_plaintext == "sk_test_xyz"


def test_decode_autoconfig_token_v1_rejects_bad_prefix():
    payload = {
        "aid": "a",
        "eid": "e",
        "surl": "https://x",
        "esec": "whsec_Zm9v",
        "tok": "t",
    }
    raw = json.dumps(payload).encode()
    token = f"wrong_{base64.b64encode(raw).decode('ascii')}"

    with pytest.raises(AutoConfigError):
        _decode_autoconfig_token_v1(token)


def test_decode_autoconfig_token_v1_rejects_invalid_json():
    token = (
        f"{_AUTOCONFIG_TOKEN_PREFIX_V1}{base64.b64encode(b'not json').decode('ascii')}"
    )

    with pytest.raises(AutoConfigError):
        _decode_autoconfig_token_v1(token)


def test_auto_config_sink_type_revalidates_when_nested():
    # Regression: nesting an already-built AutoConfigSinkType inside another
    # model re-runs its wrap validator with a model instance (not a dict),
    # which used to raise TypeError on `data["config"] = {}`.
    sink = AutoConfigSinkType(type="poller", config=SinkInCommon(filter_types=["a"]))
    subscribe_in = SubscribeIn(sink=sink)

    assert subscribe_in.sink is not None
    assert isinstance(subscribe_in.sink.config, SinkInCommon)
    assert subscribe_in.sink.config.filter_types == ["a"]


def test_auto_config_consumer_subscribe_in_payload():
    consumer = AutoConfigConsumer(
        _make_token(), SinkInCommon(filter_types=["issue.opened"])
    )
    subscribe_in = consumer._subscribe_in()

    assert subscribe_in.sink is not None
    assert subscribe_in.sink.type == "poller"
    assert isinstance(subscribe_in.sink.config, SinkInCommon)
    assert (
        subscribe_in.model_dump_json(exclude_unset=True, by_alias=True)
        == '{"sink":{"type":"poller","config":{"filterTypes":["issue.opened"]}}}'
    )
