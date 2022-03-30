using System;

namespace Svix.Models
{
    public class MessageAttemptListOptions
    {
        public string? Iterator { get; set; }

        public int? Limit { get; set; }

        public string? Channel { get; set; }

        public int? Status { get; set; }

        public DateTime? Before { get; set; }
    }
}