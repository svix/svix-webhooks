using System;
using System.Text;

namespace Svix {
    public class Webhook {

        private static string prefix = "whsec_";
        private string key;
        public Webhook(string key) {
            if (key.StartsWith(prefix)) {
                key.Substring(prefix.Length);
            }
            byte[] keyBytes = Convert.FromBase64String(key);
            string decodedKey = Encoding.UTF8.GetString(keyBytes);

            this.key = decodedKey;
        }
    }
}