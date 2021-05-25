import base64
import hashlib
import hmac
import json
import typing as t


def hmac_data(key: bytes, data: bytes) -> bytes:
    return hmac.new(key, data, hashlib.sha256).digest()


class WebhookVerificationError(Exception):
    pass


class Webhook:
    _whsecret: bytes
    _enc_key: t.Optional[bytes]

    def __init__(self, whsecret: str, *, enc_key: t.Optional[str] = None):
        self._whsecret = base64.b64decode(whsecret)
        self._enc_key = base64.b64decode(enc_key) if enc_key is not None else None

    def verify(self, data: t.Union[bytes, str], headers: t.Dict[str, str]) -> t.Dict[str, t.Any]:
        data = data if isinstance(data, str) else data.decode()
        headers = {k.lower(): v for (k, v) in headers.items()}
        msg_id = headers.get("svix-id")
        msg_signature = headers.get("svix-signature")
        msg_timestamp = headers.get("svix-timestamp")
        if not (msg_id and msg_timestamp and msg_signature):
            raise WebhookVerificationError("Missing required headers")

        to_sign = f"{msg_id}.{msg_timestamp}.{data}".encode()
        expected_sig = hmac_data(self._whsecret, to_sign)
        passed_sigs = msg_signature.split(" ")
        for versioned_sig in passed_sigs:
            (version, signature) = versioned_sig.split(",")
            if version != "v1":
                continue

            sig_bytes = base64.b64decode(signature)
            if hmac.compare_digest(expected_sig, sig_bytes):
                return json.loads(data)

        raise WebhookVerificationError("No matching signature found")
