using System;

namespace Svix.Models
{
    public class MessageAttemptListOptions : ListOptions
    {
        public int? Status { get; set; }

        public DateTime? Before { get; set; }
    }
}