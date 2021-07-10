using System;
using System.Text;
using System.Net;
using System.Security.Cryptography;

using Svix.Exceptions;
namespace Svix {
    public class Webhook {

        internal static readonly UTF8Encoding SafeUTF8Encoding = new UTF8Encoding(false, true);

        private static readonly int TOLERANCE_IN_SECONDS = 60 * 5;
        private static string prefix = "whsec_";
        private string key;
        public Webhook(string key) {
            if (key.StartsWith(prefix)) {
                key = key.Substring(prefix.Length);
            }

            byte[] keyBytes = Convert.FromBase64String(key);
            string decodedKey = Encoding.UTF8.GetString(keyBytes);

            this.key = decodedKey;
        }

        public void Verify(string payload, WebHeaderCollection headers) {            
            string msgId = headers.Get("svix-id");
            string msgSignature = headers.Get("svix-signature");
            string msgTimestamp = headers.Get("svix-timestamp");

            if (String.IsNullOrEmpty(msgId) || String.IsNullOrEmpty(msgSignature) ||  String.IsNullOrEmpty(msgTimestamp)) {
                throw new WebhookVerificationException("Missing Required Headers");
            }

            Webhook.VerifyTimestamp(msgTimestamp);

            var toSign = $"{msgId}.{msgTimestamp}.{payload}";
            var signature = Webhook.Sign(this.key, toSign);

            var passedSignatures = msgSignature.Split(' ');
            foreach (string versionedSignature in passedSignatures) {
                var parts = versionedSignature.Split(",");
                if (parts.Length < 2) {
                    throw new WebhookVerificationException("Invalid Signature Headers");
                }
                var version = parts[0];
                var expectedSignature = parts[1];

                if (version != "v1") {
                    continue;
                }
                if (Utils.SecureCompare(signature, expectedSignature)) {
                    return;
                }

            }
            throw new WebhookVerificationException("No matching signature found");
        }

        private static void VerifyTimestamp(string timestampHeader) {
            DateTimeOffset timestamp;
            var now = DateTimeOffset.UtcNow;
            try {
                var timestampInt = long.Parse(timestampHeader);
                timestamp = DateTimeOffset.FromUnixTimeSeconds(timestampInt);
            } catch {
                throw new WebhookVerificationException("Invalid Signature Headers");
            }

            if (timestamp < (now.AddSeconds( -1 * TOLERANCE_IN_SECONDS))) {
                throw new WebhookVerificationException("Message timestamp too old");
            }
            if (timestamp > (now.AddSeconds(TOLERANCE_IN_SECONDS))) {
                throw new WebhookVerificationException("Message timestamp too new");
            }

        }

        private static string Sign(string secret, string toSign) {
            var secretBytes = SafeUTF8Encoding.GetBytes(secret);
            var toSignBytes = SafeUTF8Encoding.GetBytes(toSign);

            using (var hmac = new HMACSHA256(secretBytes))
            {
                var hash = hmac.ComputeHash(toSignBytes);
                return Convert.ToBase64String(hash);
            }
        }
    }
}