import base64
import hashlib
import hmac
import json
import typing as t
from datetime import datetime, timedelta, timezone
from math import floor


def hmac_data(key: bytes, data: bytes) -> bytes:
    return hmac.new(key, data, hashlib.sha256).digest()


class WebhookVerificationError(Exception):
    pass


class Webhook:
    _SECRET_PREFIX: str = "whsec_"
    _whsecret: bytes
    _enc_key: t.Optional[bytes]

    def __init__(self, whsecret: str, *, enc_key: t.Optional[str] = None):
        if whsecret.startswith(self._SECRET_PREFIX):
            whsecret = whsecret[len(self._SECRET_PREFIX) :]
        self._whsecret = base64.b64decode(whsecret)
        self._enc_key = base64.b64decode(enc_key) if enc_key is not None else None

    def verify(self, data: t.Union[bytes, str], headers: t.Dict[str, str]) -> t.Dict[str, t.Any]:
        data = data if isinstance(data, str) else data.decode()
        headers = {k.lower(): v for (k, v) in headers.items()}
        msg_id = headers.get("svix-id")
        msg_signature = headers.get("svix-signature")
        msg_timestamp = headers.get("svix-timestamp")
        if not (msg_id and msg_timestamp and msg_signature):
            msg_id = headers.get("webhook-id")
            msg_signature = headers.get("webhook-signature")
            msg_timestamp = headers.get("webhook-timestamp")
            if not (msg_id and msg_timestamp and msg_signature):
                raise WebhookVerificationError("Missing required headers")

        timestamp = self.__verify_timestamp(msg_timestamp)

        expected_sig = base64.b64decode(self.sign(msg_id=msg_id, timestamp=timestamp, data=data).split(",")[1])
        passed_sigs = msg_signature.split(" ")
        for versioned_sig in passed_sigs:
            (version, signature) = versioned_sig.split(",")
            if version != "v1":
                continue
            sig_bytes = base64.b64decode(signature)
            if hmac.compare_digest(expected_sig, sig_bytes):
                return json.loads(data)

        raise WebhookVerificationError("No matching signature found")

    def sign(self, msg_id: str, timestamp: datetime, data: str) -> str:
        timestamp_str = str(floor(timestamp.replace(tzinfo=timezone.utc).timestamp()))
        to_sign = f"{msg_id}.{timestamp_str}.{data}".encode()
        signature = hmac_data(self._whsecret, to_sign)
        return f"v1,{base64.b64encode(signature).decode('utf-8')}"

    def __verify_timestamp(self, timestamp_header: str) -> datetime:
        webhook_tolerance = timedelta(minutes=5)
        now = datetime.now(tz=timezone.utc)
        try:
            timestamp = datetime.fromtimestamp(float(timestamp_header), tz=timezone.utc)
        except Exception:
            raise WebhookVerificationError("Invalid Signature Headers")

        if timestamp < (now - webhook_tolerance):
            raise WebhookVerificationError("Message timestamp too old")
        if timestamp > (now + webhook_tolerance):
            raise WebhookVerificationError("Message timestamp too new")
        return timestamp
