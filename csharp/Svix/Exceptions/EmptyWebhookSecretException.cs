namespace Svix.Exceptions
{
    public class EmptyWebhookSecretException : Exception
    {
        public EmptyWebhookSecretException()
            : base() { }

        public EmptyWebhookSecretException(string message)
            : base(message) { }

        public EmptyWebhookSecretException(string message, Exception inner)
            : base(message, inner) { }
    }
}
