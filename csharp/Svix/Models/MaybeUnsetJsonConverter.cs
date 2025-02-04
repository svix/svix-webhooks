using Newtonsoft.Json;

namespace Svix.Models
{
    public class MaybeUnsetJsonConverterFactory : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType.IsGenericType
                && objectType.GetGenericTypeDefinition() == typeof(MaybeUnset<>);
        }

        public override object? ReadJson(
            JsonReader reader,
            Type objectType,
            object? existingValue,
            JsonSerializer serializer
        )
        {
            var valueType = objectType.GetGenericArguments()[0];
            if (reader.TokenType == JsonToken.Null)
                return objectType.GetMethod("Null")!.Invoke(null, Array.Empty<object>());

            var value = serializer.Deserialize(reader, valueType);
            return value == null
                ? objectType.GetMethod("Null")!.Invoke(null, Array.Empty<object>())
                : objectType.GetMethod("Set")!.Invoke(null, new[] { value });
        }

        public override void WriteJson(JsonWriter writer, object? value, JsonSerializer serializer)
        {
            var maybeUnset = value!;
            var isUnset = (bool)maybeUnset.GetType().GetProperty("IsUnset")!.GetValue(maybeUnset)!;
            var isNull = (bool)maybeUnset.GetType().GetProperty("IsNull")!.GetValue(maybeUnset)!;

            if (isUnset)
                return;
            if (isNull)
            {
                writer.WriteNull();
                return;
            }

            var actualValue = maybeUnset.GetType().GetProperty("Value")!.GetValue(maybeUnset);
            serializer.Serialize(writer, actualValue);
        }
    }
}
