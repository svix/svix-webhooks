import base64
import hashlib
import hmac
import json
import typing as t
from dataclasses import dataclass
from datetime import datetime

import dateutil.parser


def hmac_data(key: bytes, data: bytes) -> bytes:
    return hmac.new(key, data, hashlib.sha256).digest()


class InvalidSignature(Exception):
    pass


@dataclass
class Message:
    id: str
    event_type: str
    event_id: str
    event_timestamp: datetime
    content: str
    timestamp: datetime

    def content_json(self):
        return json.loads(self.content)


class Webhook:
    _whsecret: bytes
    _enc_key: t.Optional[bytes]

    def __init__(self, whsecret: str, *, enc_key: t.Optional[str] = None):
        self._whsecret = base64.b64decode(whsecret)
        self._enc_key = base64.b64decode(enc_key) if enc_key is not None else None

    def parse(self, msg: t.Union[bytes, str], sig: str) -> Message:
        msg = msg if isinstance(msg, bytes) else msg.encode()
        sig_bytes = base64.b64decode(sig)

        calc_sig = hmac_data(self._whsecret, msg)
        if not hmac.compare_digest(calc_sig, sig_bytes):
            raise InvalidSignature("Got invalid signature for event.")

        msg_dict = json.loads(msg)
        msg_dict["event_timestamp"] = dateutil.parser.parse(msg_dict["event_timestamp"])
        msg_dict["timestamp"] = dateutil.parser.parse(msg_dict["timestamp"])
        return Message(**msg_dict)
