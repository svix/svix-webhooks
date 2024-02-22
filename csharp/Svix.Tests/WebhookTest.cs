using Xunit;

using System;
using System.Net;

using Svix.Exceptions;

namespace Svix.Tests
{

    class TestPayload
    {
        internal const string SVIX_ID_HEADER_KEY = "svix-id";
        internal const string SVIX_SIGNATURE_HEADER_KEY = "svix-signature";
        internal const string SVIX_TIMESTAMP_HEADER_KEY = "svix-timestamp";
        private const string DEFAULT_MSG_ID = "msg_p5jXN8AQM9LWM0D4loKWxJek";
        private const string DEFAULT_PAYLOAD = "{\"test\": 2432232314}";
        private const string DEFAULT_SECRET = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";

        public string id;
        public DateTimeOffset timestamp;
        public WebHeaderCollection headers;

        public string secret;
        public string payload;
        public string signature;
        public TestPayload(DateTimeOffset timestamp)
        {
            id = DEFAULT_MSG_ID;
            this.timestamp = timestamp;

            payload = DEFAULT_PAYLOAD;
            secret = DEFAULT_SECRET;

            Webhook wh = new Webhook(secret);
            var signature = wh.Sign(id, this.timestamp, payload);

            headers = new WebHeaderCollection();
            headers.Set(SVIX_ID_HEADER_KEY, id);
            headers.Set(SVIX_SIGNATURE_HEADER_KEY, signature);
            headers.Set(SVIX_TIMESTAMP_HEADER_KEY, timestamp.ToUnixTimeSeconds().ToString());
        }
    }

    public class WebhookTests
    {
        public const int TOLERANCE_IN_SECONDS = 5 * 60;

        [Fact]
        public void TestMissingIdRaisesException()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            testPayload.headers.Remove(TestPayload.SVIX_ID_HEADER_KEY);

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestMissingTimestampThrowsException()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            testPayload.headers.Remove(TestPayload.SVIX_TIMESTAMP_HEADER_KEY);

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestMissingSignatureThrowsException()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            testPayload.headers.Remove(TestPayload.SVIX_SIGNATURE_HEADER_KEY);

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestInvalidSignatureThrowsException()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            testPayload.headers.Set(TestPayload.SVIX_SIGNATURE_HEADER_KEY, "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLawdd");

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestValidSignatureIsValid()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            var wh = new Webhook(testPayload.secret);

            wh.Verify(testPayload.payload, testPayload.headers);
        }

        [Fact]
        public void TestUnbrandedSignatureIsValid()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            WebHeaderCollection unbrandedHeaders = new WebHeaderCollection();
            unbrandedHeaders.Set("webhook-id", testPayload.headers.Get(TestPayload.SVIX_ID_HEADER_KEY));
            unbrandedHeaders.Set("webhook-signature", testPayload.headers.Get(TestPayload.SVIX_SIGNATURE_HEADER_KEY));
            unbrandedHeaders.Set("webhook-timestamp", testPayload.headers.Get(TestPayload.SVIX_TIMESTAMP_HEADER_KEY));
            testPayload.headers = unbrandedHeaders;

            var wh = new Webhook(testPayload.secret);

            wh.Verify(testPayload.payload, testPayload.headers);
        }

        [Fact]
        public void TestOldTimestampThrowsException()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow.AddSeconds(-1 * (TOLERANCE_IN_SECONDS + 1)));

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestNewTimestampThrowsException()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow.AddSeconds(TOLERANCE_IN_SECONDS + 1));

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestMultiSigPayloadIsValid()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);

            string[] sigs = new string[] {
                "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
                "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
                testPayload.headers.Get(TestPayload.SVIX_SIGNATURE_HEADER_KEY), // valid signature
                "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
            };
            testPayload.headers.Set(TestPayload.SVIX_SIGNATURE_HEADER_KEY, String.Join(" ", sigs));

            var wh = new Webhook(testPayload.secret);

            wh.Verify(testPayload.payload, testPayload.headers);
        }

        [Fact]
        public void TestSivnatureVerificationWorksWithoutPrefix()
        {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);

            var wh = new Webhook(testPayload.secret);
            wh.Verify(testPayload.payload, testPayload.headers);

            wh = new Webhook("whsec_" + testPayload.secret);
            wh.Verify(testPayload.payload, testPayload.headers);
        }

        [Fact]
        public void verifyWebhookSignWorks()
        {
            var key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";
            var msgId = "msg_p5jXN8AQM9LWM0D4loKWxJek";
            var timestamp = DateTimeOffset.FromUnixTimeSeconds(1614265330);
            var payload = "{\"test\": 2432232314}";
            var expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=";

            var wh = new Webhook(key);
            var signature = wh.Sign(msgId, timestamp, payload);
            Assert.Equal(signature, expected);
        }
    }
}
