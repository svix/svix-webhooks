import typing as t
from datetime import datetime
from standardwebhooks.webhooks import Webhook as StdWh
from standardwebhooks.exceptions import WebhookVerificationError


class Webhook:
    _inner: StdWh

    def __init__(self, whsecret: t.Union[str, bytes]):
        self._inner = StdWh(whsecret)

    def verify(self, data: t.Union[bytes, str], headers: t.Dict[str, str]) -> t.Any:
        headers = {k.lower(): v for k, v in headers.items()}

        headers["webhook-id"] = headers.get("svix-id", headers.get("webhook-id", ""))
        headers["webhook-signature"] = headers.get(
            "svix-signature", headers.get("webhook-signature", "")
        )
        headers["webhook-timestamp"] = headers.get(
            "svix-timestamp", headers.get("webhook-timestamp", "")
        )

        return self._inner.verify(data, headers)

    def sign(self, msg_id: str, timestamp: datetime, data: str) -> str:
        return self._inner.sign(msg_id, timestamp, data)


__all__ = ["Webhook", "WebhookVerificationError"]
