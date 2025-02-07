using System;
using System.Collections.Generic;

namespace Svix.Models
{
    public sealed class AttemptsByEndpointListOptions : ListOptions
    {
        public int? Status { get; set; }

        public int? Code { get; set; }

        public List<string> EventTypes { get; set; }

        public string? Channel { get; set; }

        public DateTime? Before { get; set; }

        public DateTime? After { get; set; }

        public bool? WithContent { get; set; }

        public bool? WithMsg { get; set; }

        public string? Tag { get; set; }
    }
}