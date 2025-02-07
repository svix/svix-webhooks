using System;
using System.Collections.Generic;

namespace Svix.Models
{
    public sealed class MessageListOptions : ListOptions
    {
        public List<string> EventTypes { get; set; }

        public string? Channel { get; set; }

        public DateTime? Before { get; set; }

        public DateTime? After { get; set; }

        public bool? WithContent { get; set; }

        public string? Tag { get; set; }
    }
}