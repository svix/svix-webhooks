using System;
using System.Collections.Generic;

namespace Svix.Models
{
    public sealed class EndpointStatsOptions
    {
        public DateTime? Since { get; set; }

        public DateTime? Until { get; set; }
    }
}