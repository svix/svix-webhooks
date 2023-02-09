using Svix.Model;

namespace Svix.Models
{
    public class ListOptions
    {
        public string? Iterator { get; set; }

        public int? Limit { get; set; }

        public Ordering? Order { get; set; }
    }
}