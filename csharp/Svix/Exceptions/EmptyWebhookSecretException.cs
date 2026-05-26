namespace Svix.Exceptions
{
    public class EmptyWebhookSecretException : WebhookVerificationException
    {
        public EmptyWebhookSecretException()
            : base() { }

        public EmptyWebhookSecretException(string message)
            : base(message) { }

        public EmptyWebhookSecretException(string message, Exception inner)
            : base(message, inner) { }
    }
}
