import base64
import json

import pytest

from svix import AutoConfigError
from svix.autoconfig import _AUTOCONFIG_TOKEN_PREFIX_V1, _decode_autoconfig_token_v1


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
    payload = {"aid": "a", "eid": "e", "surl": "https://x", "esec": "whsec_Zm9v", "tok": "t"}
    raw = json.dumps(payload).encode()
    token = f"wrong_{base64.b64encode(raw).decode('ascii')}"

    with pytest.raises(AutoConfigError):
        _decode_autoconfig_token_v1(token)


def test_decode_autoconfig_token_v1_rejects_invalid_json():
    token = f"{_AUTOCONFIG_TOKEN_PREFIX_V1}{base64.b64encode(b'not json').decode('ascii')}"

    with pytest.raises(AutoConfigError):
        _decode_autoconfig_token_v1(token)
