import pytest
import base64
import typing as t
from math import floor
from datetime import datetime, timedelta, timezone

from svix.webhooks import hmac_data, Webhook, WebhookVerificationError

defaultMsgID = "msg_p5jXN8AQM9LWM0D4loKWxJek"
defaultPayload = '{"test": 2432232314}'
defaultSecret = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw"

tolerance = timedelta(minutes=5)


class PayloadForTesting:
    id: str
    timestamp: int
    payload: str
    secret: str
    signature: str
    header: t.Dict[str, str]

    def __init__(self, timestamp: datetime = datetime.now(tz=timezone.utc)):
        ts = str(floor(timestamp.timestamp()))
        to_sign = f"{defaultMsgID}.{ts}.{defaultPayload}".encode()
        signature = base64.b64encode(
            hmac_data(base64.b64decode(defaultSecret), to_sign)
        ).decode("utf-8")

        self.id = defaultMsgID
        self.timestamp = ts
        self.payload = defaultPayload
        self.secret = defaultSecret
        self.signature = signature
        self.header = {
            "svix-id": defaultMsgID,
            "svix-signature": "v1," + signature,
            "svix-timestamp": self.timestamp,
        }


def test_missing_id_raises_error():
    testPayload = PayloadForTesting()
    del testPayload.header["svix-id"]

    wh = Webhook(testPayload.secret)

    with pytest.raises(WebhookVerificationError):
        wh.verify(testPayload.payload, testPayload.header)


def test_timestamp_raises_error():
    testPayload = PayloadForTesting()
    del testPayload.header["svix-timestamp"]

    wh = Webhook(testPayload.secret)

    with pytest.raises(WebhookVerificationError):
        wh.verify(testPayload.payload, testPayload.header)


def test_invalid_timestamp_raises_error():
    testPayload = PayloadForTesting()
    testPayload.header["svix-timestamp"] = "hello"

    wh = Webhook(testPayload.secret)

    with pytest.raises(WebhookVerificationError):
        wh.verify(testPayload.payload, testPayload.header)


def test_missing_signature_raises_error():
    testPayload = PayloadForTesting()
    del testPayload.header["svix-signature"]

    wh = Webhook(testPayload.secret)

    with pytest.raises(WebhookVerificationError):
        wh.verify(testPayload.payload, testPayload.header)


def test_invalid_signature_raises_error():
    testPayload = PayloadForTesting()
    testPayload.header["svix-signature"] = (
        "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OA="
    )

    wh = Webhook(testPayload.secret)

    with pytest.raises(WebhookVerificationError):
        wh.verify(testPayload.payload, testPayload.header)


def test_valid_signature_is_valid_and_returns_json():
    testPayload = PayloadForTesting()

    wh = Webhook(testPayload.secret)

    json = wh.verify(testPayload.payload, testPayload.header)
    assert json["test"] == 2432232314


def test_valid_unbranded_signature_is_valid_and_returns_json():
    testPayload = PayloadForTesting()
    unbrandedHeaders = {
        "webhook-id": testPayload.header.get("svix-id"),
        "webhook-signature": testPayload.header.get("svix-signature"),
        "webhook-timestamp": testPayload.header.get("svix-timestamp"),
    }
    testPayload.header = unbrandedHeaders

    wh = Webhook(testPayload.secret)

    json = wh.verify(testPayload.payload, testPayload.header)
    assert json["test"] == 2432232314


def test_old_timestamp_fails():
    testPayload = PayloadForTesting(
        datetime.now(tz=timezone.utc) - tolerance - timedelta(seconds=1)
    )

    wh = Webhook(testPayload.secret)

    with pytest.raises(WebhookVerificationError):
        wh.verify(testPayload.payload, testPayload.header)


def test_new_timestamp_fails():
    testPayload = PayloadForTesting(
        datetime.now(tz=timezone.utc) + tolerance + timedelta(seconds=1)
    )

    wh = Webhook(testPayload.secret)

    with pytest.raises(WebhookVerificationError):
        wh.verify(testPayload.payload, testPayload.header)


def test_multi_sig_payload_is_valid():
    testPayload = PayloadForTesting()
    sigs = [
        "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
        "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
        testPayload.header["svix-signature"],  # valid signature
        "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
    ]
    testPayload.header["svix-signature"] = " ".join(sigs)

    wh = Webhook(testPayload.secret)

    json = wh.verify(testPayload.payload, testPayload.header)
    assert json["test"] == 2432232314


def test_signature_verification_with_and_without_prefix():
    testPayload = PayloadForTesting()

    wh = Webhook(testPayload.secret)
    json = wh.verify(testPayload.payload, testPayload.header)
    assert json["test"] == 2432232314

    wh = Webhook("whsec_" + testPayload.secret)

    json = wh.verify(testPayload.payload, testPayload.header)
    assert json["test"] == 2432232314


def test_sign_function():
    key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw"
    msg_id = "msg_p5jXN8AQM9LWM0D4loKWxJek"
    timestamp = datetime.utcfromtimestamp(1614265330)
    payload = '{"test": 2432232314}'
    expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="

    wh = Webhook(key)
    signature = wh.sign(msg_id=msg_id, timestamp=timestamp, data=payload)
    assert signature == expected
