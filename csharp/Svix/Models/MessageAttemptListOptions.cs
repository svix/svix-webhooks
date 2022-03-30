using System;

namespace Svix.Models
{
    public class MessageAttemptListOptions : ListOptions
    {
        public string? Channel { get; set; }

        public int? Status { get; set; }

        public DateTime? Before { get; set; }
    }
}