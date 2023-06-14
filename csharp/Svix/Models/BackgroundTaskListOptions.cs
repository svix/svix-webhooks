using System;
using System.Collections.Generic;
using Svix.Model;

namespace Svix.Models
{
    public sealed class BackgroundTaskListOptions : ListOptions
    {
        public BackgroundTaskStatus? Status { get; set; }

        public BackgroundTaskType? Task { get; set; }
    }
}