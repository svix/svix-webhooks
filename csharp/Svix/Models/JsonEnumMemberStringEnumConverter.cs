using System.Reflection;
using System.Runtime.Serialization;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Svix.Models
{
    // https://stackoverflow.com/a/59061296
    public class JsonEnumMemberStringEnumConverter : JsonConverterFactory
    {
        private readonly JsonNamingPolicy? namingPolicy;
        private readonly bool allowIntegerValues;
        private readonly JsonStringEnumConverter baseConverter;

        public JsonEnumMemberStringEnumConverter()
            : this(null, true) { }

        public JsonEnumMemberStringEnumConverter(
            JsonNamingPolicy? namingPolicy = null,
            bool allowIntegerValues = true
        )
        {
            this.namingPolicy = namingPolicy;
            this.allowIntegerValues = allowIntegerValues;
            this.baseConverter = new JsonStringEnumConverter(namingPolicy, allowIntegerValues);
        }

        public override bool CanConvert(Type typeToConvert) =>
            baseConverter.CanConvert(typeToConvert);

        public override JsonConverter CreateConverter(
            Type typeToConvert,
            JsonSerializerOptions options
        )
        {
            var query =
                from field in typeToConvert.GetFields(BindingFlags.Public | BindingFlags.Static)
                let attr = field.GetCustomAttribute<EnumMemberAttribute>()
                where attr != null && attr.Value != null
                select (field.Name, attr.Value);
            var dictionary = query.ToDictionary(p => p.Item1, p => p.Item2);
            if (dictionary.Count > 0)
                return new JsonStringEnumConverter(
                    new DictionaryLookupNamingPolicy(dictionary, namingPolicy),
                    allowIntegerValues
                ).CreateConverter(typeToConvert, options);
            else
                return baseConverter.CreateConverter(typeToConvert, options);
        }
    }

    public class JsonNamingPolicyDecorator : JsonNamingPolicy
    {
        readonly JsonNamingPolicy? underlyingNamingPolicy;

        public JsonNamingPolicyDecorator(JsonNamingPolicy? underlyingNamingPolicy) =>
            this.underlyingNamingPolicy = underlyingNamingPolicy;

        public override string ConvertName(string name) =>
            underlyingNamingPolicy?.ConvertName(name) ?? name;
    }

    internal class DictionaryLookupNamingPolicy : JsonNamingPolicyDecorator
    {
        readonly Dictionary<string, string> dictionary;

        public DictionaryLookupNamingPolicy(
            Dictionary<string, string> dictionary,
            JsonNamingPolicy? underlyingNamingPolicy
        )
            : base(underlyingNamingPolicy) =>
            this.dictionary = dictionary ?? throw new ArgumentNullException();

        public override string ConvertName(string name) =>
            dictionary.TryGetValue(name, out var value) ? value : base.ConvertName(name);
    }
}
