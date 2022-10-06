using Svix.Exceptions;
using System;
using System.Net;
using System.Security.Cryptography;
using System.Text;

namespace Svix
{
    public sealed class Webhook
    {

        internal static readonly UTF8Encoding SafeUTF8Encoding = new UTF8Encoding(false, true);
        internal const string SVIX_ID_HEADER_KEY = "svix-id";
        internal const string SVIX_SIGNATURE_HEADER_KEY = "svix-signature";
        internal const string SVIX_TIMESTAMP_HEADER_KEY = "svix-timestamp";
        internal const string UNBRANDED_ID_HEADER_KEY = "webhook-id";
        internal const string UNBRANDED_SIGNATURE_HEADER_KEY = "webhook-signature";
        internal const string UNBRANDED_TIMESTAMP_HEADER_KEY = "webhook-timestamp";

        private const int TOLERANCE_IN_SECONDS = 60 * 5;
        private static string prefix = "whsec_";
        private byte[] key;
        public Webhook(string key)
        {
            if (key.StartsWith(prefix))
            {
                key = key.Substring(prefix.Length);
            }

            this.key = Convert.FromBase64String(key);
        }

        public Webhook(byte[] key)
        {
            this.key = key;
        }

        public void Verify(string payload, WebHeaderCollection headers)
        {
            string msgId = headers.Get(SVIX_ID_HEADER_KEY);
            string msgSignature = headers.Get(SVIX_SIGNATURE_HEADER_KEY);
            string msgTimestamp = headers.Get(SVIX_TIMESTAMP_HEADER_KEY);

            if (String.IsNullOrEmpty(msgId) || String.IsNullOrEmpty(msgSignature) || String.IsNullOrEmpty(msgTimestamp))
            {
                msgId = headers.Get(UNBRANDED_ID_HEADER_KEY);
                msgSignature = headers.Get(UNBRANDED_SIGNATURE_HEADER_KEY);
                msgTimestamp = headers.Get(UNBRANDED_TIMESTAMP_HEADER_KEY);
                if (String.IsNullOrEmpty(msgId) || String.IsNullOrEmpty(msgSignature) || String.IsNullOrEmpty(msgTimestamp))
                {
                    throw new WebhookVerificationException("Missing Required Headers");
                }
            }

            var timestamp = Webhook.VerifyTimestamp(msgTimestamp);

            var signature = this.Sign(msgId, timestamp, payload);
            var expectedSignature = signature.Split(',')[1];

            var passedSignatures = msgSignature.Split(' ');
            foreach (string versionedSignature in passedSignatures)
            {
                var parts = versionedSignature.Split(',');
                if (parts.Length < 2)
                {
                    throw new WebhookVerificationException("Invalid Signature Headers");
                }
                var version = parts[0];
                var passedSignature = parts[1];

                if (version != "v1")
                {
                    continue;
                }
                if (Utils.SecureCompare(expectedSignature, passedSignature))
                {
                    return;
                }

            }
            throw new WebhookVerificationException("No matching signature found");
        }

        private static DateTimeOffset VerifyTimestamp(string timestampHeader)
        {
            DateTimeOffset timestamp;
            var now = DateTimeOffset.UtcNow;
            try
            {
                var timestampInt = long.Parse(timestampHeader);
                timestamp = DateTimeOffset.FromUnixTimeSeconds(timestampInt);
            }
            catch
            {
                throw new WebhookVerificationException("Invalid Signature Headers");
            }

            if (timestamp < (now.AddSeconds(-1 * TOLERANCE_IN_SECONDS)))
            {
                throw new WebhookVerificationException("Message timestamp too old");
            }
            if (timestamp > (now.AddSeconds(TOLERANCE_IN_SECONDS)))
            {
                throw new WebhookVerificationException("Message timestamp too new");
            }
            return timestamp;
        }

        public string Sign(string msgId, DateTimeOffset timestamp, string payload)
        {
            var toSign = $"{msgId}.{timestamp.ToUnixTimeSeconds().ToString()}.{payload}";
            var toSignBytes = SafeUTF8Encoding.GetBytes(toSign);
            using (var hmac = new HMACSHA256(this.key))
            {
                var hash = hmac.ComputeHash(toSignBytes);
                var signature = Convert.ToBase64String(hash);
                return $"v1,{signature}";
            }
        }
    }
}
