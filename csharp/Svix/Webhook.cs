using Svix.Exceptions;
using System;
using System.Buffers;
using System.Buffers.Text;
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

        private const int SIGNATURE_LENGTH_BYTES = 32;
        private const int SIGNATURE_LENGTH_BASE64 = 48;
        private const int SIGNATURE_LENGTH_STRING = 56;
        private const int TOLERANCE_IN_SECONDS = 60 * 5;
        private const string PREFIX = "whsec_";
        
        private readonly byte[] key;
        public Webhook(string key)
        {
            if (key.StartsWith(PREFIX))
            {
                key = key.Substring(PREFIX.Length);
            }

            this.key = Convert.FromBase64String(key);
        }

        public Webhook(byte[] key)
        {
            this.key = key;
        }

        public void Verify(ReadOnlySpan<char> payload, WebHeaderCollection headers)
        {
            ArgumentNullException.ThrowIfNull(headers);
            
            Verify(payload, headers.Get);
        }

        public void Verify(string payload, Func<string, string> headersProvider)
        {
            ArgumentNullException.ThrowIfNull(payload);
            ArgumentNullException.ThrowIfNull(headersProvider);
            
            ReadOnlySpan<char> msgId = headersProvider(SVIX_ID_HEADER_KEY);
            ReadOnlySpan<char> msgSignature = headersProvider(SVIX_SIGNATURE_HEADER_KEY);
            ReadOnlySpan<char> msgTimestamp = headersProvider(SVIX_TIMESTAMP_HEADER_KEY);

            if (msgId.IsEmpty || msgSignature.IsEmpty || msgTimestamp.IsEmpty)
            {
                msgId = headersProvider(UNBRANDED_ID_HEADER_KEY);
                msgSignature = headersProvider(UNBRANDED_SIGNATURE_HEADER_KEY);
                msgTimestamp = headersProvider(UNBRANDED_TIMESTAMP_HEADER_KEY);
                if (msgId.IsEmpty || msgSignature.IsEmpty || msgTimestamp.IsEmpty)
                {
                    throw new WebhookVerificationException("Missing Required Headers");
                }
            }

            var timestamp = Webhook.VerifyTimestamp(msgTimestamp);

            Span<char> expectedSignature = stackalloc char[SIGNATURE_LENGTH_STRING];
            CalculateSignature(msgId, timestamp, payload, expectedSignature, out var charsWritten);
            expectedSignature = expectedSignature.Slice(0, charsWritten);

            var signaturePtr = msgSignature;
            var spaceIndex = signaturePtr.IndexOf(' ');
            do
            {
                var versionedSignature = spaceIndex < 0 
                    ? msgSignature : signaturePtr.Slice(0, spaceIndex);
                
                signaturePtr = signaturePtr.Slice(spaceIndex + 1);
                spaceIndex = signaturePtr.IndexOf(' ');
                
                var commaIndex = versionedSignature.IndexOf(',');
                if (commaIndex < 0)
                {
                    throw new WebhookVerificationException("Invalid Signature Headers");
                }
                var version = versionedSignature.Slice(0, commaIndex);
                if (!version.Equals("v1", StringComparison.InvariantCulture))
                {
                    continue;
                }
                var passedSignature = versionedSignature.Slice(commaIndex + 1);
                if (Utils.SecureCompare(expectedSignature, passedSignature))
                {
                    return;
                }
            }
            while(spaceIndex >= 0);
            
            throw new WebhookVerificationException("No matching signature found");
        }

        private static DateTimeOffset VerifyTimestamp(ReadOnlySpan<char> timestampHeader)
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

            if (timestamp < now.AddSeconds(-1 * TOLERANCE_IN_SECONDS))
            {
                throw new WebhookVerificationException("Message timestamp too old");
            }
            if (timestamp > now.AddSeconds(TOLERANCE_IN_SECONDS))
            {
                throw new WebhookVerificationException("Message timestamp too new");
            }
            return timestamp;
        }
        
        public string Sign(ReadOnlySpan<char> msgId, DateTimeOffset timestamp, ReadOnlySpan<char> payload)
        {
            Span<char> signature = stackalloc char[SIGNATURE_LENGTH_STRING];
            signature[0] = 'v';
            signature[1] = '1';
            signature[2] = ',';
            CalculateSignature(msgId, timestamp, payload, signature.Slice(3), out var charsWritten);
            return signature.Slice(0, charsWritten + 3).ToString();
        }

        private void CalculateSignature(
            ReadOnlySpan<char> msgId, 
            DateTimeOffset timestamp, 
            ReadOnlySpan<char> payload, 
            Span<char> signature,
            out int charsWritten)
        {
            var timestampUnix = timestamp.ToUnixTimeSeconds().ToString();
            
            // Estimate buffer size and use stackalloc for smaller allocations
            int msgIdLength = SafeUTF8Encoding.GetByteCount(msgId);
            int payloadLength = SafeUTF8Encoding.GetByteCount(payload);
            int timestampUnixLength = SafeUTF8Encoding.GetByteCount(timestampUnix);
            
            Span<byte> toSignBytes = stackalloc byte[msgIdLength + 1 + timestampUnixLength + 1 + payloadLength];
            
            SafeUTF8Encoding.GetBytes(msgId, toSignBytes.Slice(0, msgIdLength));
            toSignBytes[msgIdLength] = (byte)'.';
            SafeUTF8Encoding.GetBytes(timestampUnix, toSignBytes.Slice(msgIdLength + 1, timestampUnixLength));
            toSignBytes[msgIdLength + 1 + timestampUnixLength] = (byte)'.';
            SafeUTF8Encoding.GetBytes(payload, toSignBytes.Slice(msgIdLength + 1 + timestampUnixLength + 1));

            Span<byte> signatureBin = stackalloc byte[SIGNATURE_LENGTH_BYTES];
            CalculateSignature(toSignBytes, signatureBin);
            
            Span<byte> signatureB64 = stackalloc byte[SIGNATURE_LENGTH_BASE64];
            var result = Base64.EncodeToUtf8(signatureBin, signatureB64, out _, out var bytesWritten);
            if (result != OperationStatus.Done)
                throw new WebhookVerificationException("Failed to encode signature to base64");
            
            if (!SafeUTF8Encoding.TryGetChars(signatureB64.Slice(0, bytesWritten), signature, out charsWritten))
                throw new WebhookVerificationException("Failed to convert signature to utf8");
        }
        
        private void CalculateSignature(ReadOnlySpan<byte> input, Span<byte> output)
        {
            using (var hmac = new HMACSHA256(this.key))
            {
                if (!hmac.TryComputeHash(input, output, out _))
                    throw new WebhookVerificationException("Output buffer too small");
            }
        }
    }
}
