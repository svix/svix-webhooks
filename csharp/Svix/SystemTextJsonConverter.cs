using System.Text.Json;
using System.Text.Json.Nodes;
using Newtonsoft.Json;

namespace Svix
{
    /// <summary>
    /// Writes System.Text.Json values (<see cref="JsonElement"/>, <see cref="JsonDocument"/> and
    /// <see cref="JsonNode"/>) as the JSON they hold. Without it, Newtonsoft.Json serializes
    /// their .NET properties instead, which gives e.g. <c>{"ValueKind":1}</c> for a JsonElement
    /// and a self-referencing loop error for a JsonNode.
    /// </summary>
    internal sealed class SystemTextJsonConverter : Newtonsoft.Json.JsonConverter
    {
        public override bool CanRead => false;

        public override bool CanConvert(Type objectType) =>
            objectType == typeof(JsonElement)
            || objectType == typeof(JsonElement?)
            || typeof(JsonDocument).IsAssignableFrom(objectType)
            || typeof(JsonNode).IsAssignableFrom(objectType);

        public override void WriteJson(
            JsonWriter writer,
            object? value,
            Newtonsoft.Json.JsonSerializer serializer
        )
        {
            switch (value)
            {
                case JsonElement element:
                    writer.WriteRawValue(System.Text.Json.JsonSerializer.Serialize(element));
                    break;
                case JsonDocument document:
                    writer.WriteRawValue(
                        System.Text.Json.JsonSerializer.Serialize(document.RootElement)
                    );
                    break;
                case JsonNode node:
                    writer.WriteRawValue(node.ToJsonString());
                    break;
            }
        }

        public override object? ReadJson(
            JsonReader reader,
            Type objectType,
            object? existingValue,
            Newtonsoft.Json.JsonSerializer serializer
        ) => throw new NotSupportedException();
    }
}
