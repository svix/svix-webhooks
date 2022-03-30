using System;

namespace Svix.Models
{
    public sealed class AttemptsByEndpointListOptions : ListOptions
    {
        public int? Status { get; set; }

        public string[] EventTypes { get; set; }

        public string? Channel { get; set; }

        public DateTime? Before { get; set; }
    }
}