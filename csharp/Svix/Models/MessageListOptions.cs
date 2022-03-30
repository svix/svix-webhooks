using System;

namespace Svix.Models
{
    public sealed class MessageListOptions
    {
        public string? Iterator { get; set; }

        public int? Limit { get; set; }

        public string[] EventTypes { get; set; }

        public string? Channel { get; set; }

        public DateTime? Before { get; set; }
    }
}