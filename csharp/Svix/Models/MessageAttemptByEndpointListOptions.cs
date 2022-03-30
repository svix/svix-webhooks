namespace Svix.Models
{
    public sealed class MessageAttemptByEndpointListOptions : MessageAttemptListOptions
    {
        public int? StatusCodeClass { get; set; }
    }
}