using Newtonsoft.Json;

namespace Svix.Models
{
    public abstract class BaseModel
    {
        private static readonly JsonSerializerSettings _jsonSettings = new()
        {
            Formatting = Formatting.Indented,
        };
    }
}
