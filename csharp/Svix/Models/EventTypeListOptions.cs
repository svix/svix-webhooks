namespace Svix.Models
{
    public sealed class EventTypeListOptions : ListOptions
    {
        public bool? WithContent { get; set; }

        public bool? IncludeArchived { get; set; }
    }
}