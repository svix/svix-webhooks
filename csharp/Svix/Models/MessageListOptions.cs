﻿using System;

namespace Svix.Models
{
    public sealed class MessageListOptions : ListOptions
    {
        public string[] EventTypes { get; set; }

        public string? Channel { get; set; }

        public DateTime? Before { get; set; }
    }
}