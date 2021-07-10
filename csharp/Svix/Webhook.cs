using System;
using System.Text;
using System.Net;
using System.Collections.Generic;
using System.Security.Cryptography;

using Svix.Exceptions;
namespace Svix {
    public class Webhook {
        internal static readonly UTF8Encoding SafeUTF8Encoding = new UTF8Encoding(false, true);

        private static string prefix = "whsec_";
        private string key;
        public Webhook(string key) {
            if (key.StartsWith(prefix)) {
                key.Substring(prefix.Length);
            }

            byte[] keyBytes = Convert.FromBase64String(key);
            string decodedKey = Encoding.UTF8.GetString(keyBytes);

            this.key = decodedKey;
            Console.WriteLine(this.key);
        }

        public void Verify(string payload, WebHeaderCollection headers) {            
            string msgId = headers.Get("svix-id");
            string msgSignature = headers.Get("svix-signature");
            string msgTimestamp = headers.Get("svix-timestamp");

            if (msgId == "" || msgSignature == "" ||  msgTimestamp == "") {
                throw new WebhookVerificationException("Missing Required Headers");
            }
            throw new WebhookVerificationException("not implemented");
        }

        public static string Sign(string secret, string toSign) {
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