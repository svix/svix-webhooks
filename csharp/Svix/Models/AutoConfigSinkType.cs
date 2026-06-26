// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    [JsonConverter(typeof(AutoConfigSinkTypeConverter))]
    public class AutoConfigSinkType
    {
        [JsonIgnore]
        public required AutoConfigSinkTypeConfig Config { get; set; }

        [JsonProperty("type")]
        private string Type => Config.GetDiscriminator();

        [JsonProperty("config")]
        private object realConfig
        {
            get => Config.GetContent();
            set => Config.SetContent(value);
        }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AutoConfigSinkType {\n");
            sb.Append("  Config: ").Append(Config).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }

    public class AutoConfigSinkTypeConfig
    {
        private object _value;
        private readonly ConfigType _type;

        internal string GetDiscriminator()
        {
            var memberInfo = typeof(ConfigType).GetMember(_type.ToString());
            var enumMember = memberInfo[0]
                .GetCustomAttributes(false)
                .OfType<EnumMemberAttribute>()
                .FirstOrDefault();
            return enumMember?.Value ?? _type.ToString().ToLower();
        }

        internal void SetContent(object value)
        {
            _value = value;
        }

        public object GetContent()
        {
            return _value;
        }

        private AutoConfigSinkTypeConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static AutoConfigSinkTypeConfig Poller(SinkInCommon sinkInCommon) =>
            new(sinkInCommon, ConfigType.Poller);

        public static AutoConfigSinkTypeConfig Http(EndpointIn endpointIn) =>
            new(endpointIn, ConfigType.Http);

        private enum ConfigType
        {
            [EnumMember(Value = "poller")]
            Poller,

            [EnumMember(Value = "http")]
            Http,
        }

        public TResult Match<TResult>(
            Func<SinkInCommon, TResult> onPoller,
            Func<EndpointIn, TResult> onHttp
        )
        {
            return _type switch
            {
                ConfigType.Poller => onPoller((SinkInCommon)_value),
                ConfigType.Http => onHttp((EndpointIn)_value),
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(Action<SinkInCommon> onPoller, Action<EndpointIn> onHttp)
        {
            switch (_type)
            {
                case ConfigType.Poller:
                    onPoller((SinkInCommon)_value);
                    break;
                case ConfigType.Http:
                    onHttp((EndpointIn)_value);
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class AutoConfigSinkTypeSurrogate
    {
        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        [JsonProperty("config", Required = Required.Always)]
        public required JObject Config { get; set; }
    }

    public class AutoConfigSinkTypeConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(AutoConfigSinkType);
        }

        public override bool CanWrite
        {
            get { return false; }
        }

        public override void WriteJson(JsonWriter writer, object? value, JsonSerializer serializer)
        {
            // unreachable: CanWrite tells Newtonsoft not to call this method
            throw new NotImplementedException();
        }

        public override object ReadJson(
            JsonReader reader,
            Type objectType,
            object? existingValue,
            JsonSerializer serializer
        )
        {
            var surrogate =
                serializer.Deserialize<AutoConfigSinkTypeSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to AutoConfigSinkTypeSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), AutoConfigSinkTypeConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for AutoConfigSinkTypeConfig.config"
                );
            }

            AutoConfigSinkTypeConfig config = func((surrogate.Config, surrogate.Type));

            return new AutoConfigSinkType { Config = config };
        }

        private static T ToObj<T>((JObject, string) args)
        {
            var loadedObj =
                args.Item1.ToObject<T>()
                ?? throw new JsonSerializationException(
                    $"Failed to deserialize {args.Item2} config"
                );
            return loadedObj;
        }

        private readonly Dictionary<
            string,
            Func<(JObject, string), AutoConfigSinkTypeConfig>
        > typeMap = new()
        {
            ["poller"] = c => AutoConfigSinkTypeConfig.Poller(ToObj<SinkInCommon>(c)),
            ["http"] = c => AutoConfigSinkTypeConfig.Http(ToObj<EndpointIn>(c)),
        };
    }
}
