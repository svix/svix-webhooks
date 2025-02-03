using System.Text.Json;
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public abstract class BaseModel
    {
        private static readonly JsonSerializerOptions _jsonOptions = new JsonSerializerOptions
        {
            DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingDefault,
            WriteIndented = true,
        };
    }
}
