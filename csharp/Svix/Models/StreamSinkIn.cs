// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    [JsonConverter(typeof(StreamSinkInConverter))]
    public class StreamSinkIn
    {
        [JsonProperty("batchSize")]
        public ushort? BatchSize { get; set; } = null;

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("maxWaitSecs")]
        public ushort? MaxWaitSecs { get; set; } = null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("status")]
        public SinkStatusIn? Status { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonIgnore]
        public required StreamSinkInConfig Config { get; set; }

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

            sb.Append("class StreamSinkIn {\n");
            sb.Append("  BatchSize: ").Append(BatchSize).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  MaxWaitSecs: ").Append(MaxWaitSecs).Append('\n');
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  Config: ").Append(Config).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }

    public class StreamSinkInConfig
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

        private StreamSinkInConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static StreamSinkInConfig Poller() =>
            new(new Dictionary<string, string>(), ConfigType.Poller);

        public static StreamSinkInConfig AzureBlobStorage(
            AzureBlobStorageConfig azureBlobStorageConfig
        ) => new(azureBlobStorageConfig, ConfigType.AzureBlobStorage);

        public static StreamSinkInConfig OtelTracing(SinkOtelV1Config sinkOtelV1Config) =>
            new(sinkOtelV1Config, ConfigType.OtelTracing);

        public static StreamSinkInConfig Http(SinkHttpConfig sinkHttpConfig) =>
            new(sinkHttpConfig, ConfigType.Http);

        public static StreamSinkInConfig AmazonS3(S3Config s3Config) =>
            new(s3Config, ConfigType.AmazonS3);

        public static StreamSinkInConfig GoogleCloudStorage(
            GoogleCloudStorageConfig googleCloudStorageConfig
        ) => new(googleCloudStorageConfig, ConfigType.GoogleCloudStorage);

        private enum ConfigType
        {
            [EnumMember(Value = "poller")]
            Poller,

            [EnumMember(Value = "azureBlobStorage")]
            AzureBlobStorage,

            [EnumMember(Value = "otelTracing")]
            OtelTracing,

            [EnumMember(Value = "http")]
            Http,

            [EnumMember(Value = "amazonS3")]
            AmazonS3,

            [EnumMember(Value = "googleCloudStorage")]
            GoogleCloudStorage,
        }

        public TResult Match<TResult>(
            Func<TResult> onPoller,
            Func<AzureBlobStorageConfig, TResult> onAzureBlobStorage,
            Func<SinkOtelV1Config, TResult> onOtelTracing,
            Func<SinkHttpConfig, TResult> onHttp,
            Func<S3Config, TResult> onAmazonS3,
            Func<GoogleCloudStorageConfig, TResult> onGoogleCloudStorage
        )
        {
            return _type switch
            {
                ConfigType.Poller => onPoller(),
                ConfigType.AzureBlobStorage => onAzureBlobStorage((AzureBlobStorageConfig)_value),
                ConfigType.OtelTracing => onOtelTracing((SinkOtelV1Config)_value),
                ConfigType.Http => onHttp((SinkHttpConfig)_value),
                ConfigType.AmazonS3 => onAmazonS3((S3Config)_value),
                ConfigType.GoogleCloudStorage => onGoogleCloudStorage(
                    (GoogleCloudStorageConfig)_value
                ),
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(
            Action onPoller,
            Action<AzureBlobStorageConfig> onAzureBlobStorage,
            Action<SinkOtelV1Config> onOtelTracing,
            Action<SinkHttpConfig> onHttp,
            Action<S3Config> onAmazonS3,
            Action<GoogleCloudStorageConfig> onGoogleCloudStorage
        )
        {
            switch (_type)
            {
                case ConfigType.Poller:
                    onPoller();
                    break;
                case ConfigType.AzureBlobStorage:
                    onAzureBlobStorage((AzureBlobStorageConfig)_value);
                    break;
                case ConfigType.OtelTracing:
                    onOtelTracing((SinkOtelV1Config)_value);
                    break;
                case ConfigType.Http:
                    onHttp((SinkHttpConfig)_value);
                    break;
                case ConfigType.AmazonS3:
                    onAmazonS3((S3Config)_value);
                    break;
                case ConfigType.GoogleCloudStorage:
                    onGoogleCloudStorage((GoogleCloudStorageConfig)_value);
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class StreamSinkInSurrogate
    {
        [JsonProperty("batchSize")]
        public ushort? BatchSize { get; set; } = null;

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("maxWaitSecs")]
        public ushort? MaxWaitSecs { get; set; } = null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("status")]
        public SinkStatusIn? Status { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        [JsonProperty("config", Required = Required.Always)]
        public required JObject Config { get; set; }
    }

    public class StreamSinkInConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(StreamSinkIn);
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
                serializer.Deserialize<StreamSinkInSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to StreamSinkInSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), StreamSinkInConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for StreamSinkInConfig.config"
                );
            }

            StreamSinkInConfig config = func((surrogate.Config, surrogate.Type));

            return new StreamSinkIn
            {
                BatchSize = surrogate.BatchSize,
                EventTypes = surrogate.EventTypes,
                MaxWaitSecs = surrogate.MaxWaitSecs,
                Metadata = surrogate.Metadata,
                Status = surrogate.Status,
                Uid = surrogate.Uid,
                Config = config,
            };
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

        private readonly Dictionary<string, Func<(JObject, string), StreamSinkInConfig>> typeMap =
            new()
            {
                ["poller"] = c => StreamSinkInConfig.Poller(),
                ["azureBlobStorage"] = c =>
                    StreamSinkInConfig.AzureBlobStorage(ToObj<AzureBlobStorageConfig>(c)),
                ["otelTracing"] = c => StreamSinkInConfig.OtelTracing(ToObj<SinkOtelV1Config>(c)),
                ["http"] = c => StreamSinkInConfig.Http(ToObj<SinkHttpConfig>(c)),
                ["amazonS3"] = c => StreamSinkInConfig.AmazonS3(ToObj<S3Config>(c)),
                ["googleCloudStorage"] = c =>
                    StreamSinkInConfig.GoogleCloudStorage(ToObj<GoogleCloudStorageConfig>(c)),
            };
    }
}
