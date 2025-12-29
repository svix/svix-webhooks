import typing as t
from datetime import datetime
from standardwebhooks.webhooks import Webhook as StdWh
from standardwebhooks.exceptions import WebhookVerificationError


class Webhook:
    _inner: StdWh

    def __init__(self, whsecret: t.Union[str, bytes]):
        self._inner = StdWh(whsecret)

    def verify(self, data: t.Union[bytes, str], headers: t.Dict[str, str]) -> t.Any:
        for h in ["id", "signature", "timestamp"]:
            if (
                headers.get(f"svix-{h}") is not None
                and headers.get(f"webhook-{h}") is None
            ):
                headers[f"webhook-{h}"] = headers[f"svix-{h}"]

        return self._inner.verify(data, headers)

    def sign(self, msg_id: str, timestamp: datetime, data: str) -> str:
        return self._inner.sign(msg_id, timestamp, data)


__all__ = ["Webhook", "WebhookVerificationError"]
