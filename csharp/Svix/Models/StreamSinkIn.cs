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
        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("status")]
        public SinkStatusIn? Status { get; set; } = null;

        [JsonProperty("batchSize")]
        public ushort? BatchSize { get; set; } = null;

        [JsonProperty("maxWaitSecs")]
        public ushort? MaxWaitSecs { get; set; } = null;

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

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
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  BatchSize: ").Append(BatchSize).Append('\n');
            sb.Append("  MaxWaitSecs: ").Append(MaxWaitSecs).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
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

        public static StreamSinkInConfig GoogleCloudPubSub(
            GoogleCloudPubSubConfig googleCloudPubSubConfig
        ) => new(googleCloudPubSubConfig, ConfigType.GoogleCloudPubSub);

        public static StreamSinkInConfig Sqs(SqsConfig sqsConfig) => new(sqsConfig, ConfigType.Sqs);

        public static StreamSinkInConfig Sns(SnsConfig snsConfig) => new(snsConfig, ConfigType.Sns);

        public static StreamSinkInConfig BigQuery(BigQueryConfig bigQueryConfig) =>
            new(bigQueryConfig, ConfigType.BigQuery);

        public static StreamSinkInConfig Clickhouse(ClickhouseConfig clickhouseConfig) =>
            new(clickhouseConfig, ConfigType.Clickhouse);

        public static StreamSinkInConfig EventBridge(EventBridgeConfig eventBridgeConfig) =>
            new(eventBridgeConfig, ConfigType.EventBridge);

        public static StreamSinkInConfig Snowflake(SnowflakeConfig snowflakeConfig) =>
            new(snowflakeConfig, ConfigType.Snowflake);

        public static StreamSinkInConfig RabbitMq(RabbitMqConfig rabbitMqConfig) =>
            new(rabbitMqConfig, ConfigType.RabbitMq);

        public static StreamSinkInConfig Redshift(RedshiftConfig redshiftConfig) =>
            new(redshiftConfig, ConfigType.Redshift);

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

            [EnumMember(Value = "googleCloudPubSub")]
            GoogleCloudPubSub,

            [EnumMember(Value = "sqs")]
            Sqs,

            [EnumMember(Value = "sns")]
            Sns,

            [EnumMember(Value = "bigQuery")]
            BigQuery,

            [EnumMember(Value = "clickhouse")]
            Clickhouse,

            [EnumMember(Value = "eventBridge")]
            EventBridge,

            [EnumMember(Value = "snowflake")]
            Snowflake,

            [EnumMember(Value = "rabbitMq")]
            RabbitMq,

            [EnumMember(Value = "redshift")]
            Redshift,
        }

        public TResult Match<TResult>(
            Func<TResult> onPoller,
            Func<AzureBlobStorageConfig, TResult> onAzureBlobStorage,
            Func<SinkOtelV1Config, TResult> onOtelTracing,
            Func<SinkHttpConfig, TResult> onHttp,
            Func<S3Config, TResult> onAmazonS3,
            Func<GoogleCloudStorageConfig, TResult> onGoogleCloudStorage,
            Func<GoogleCloudPubSubConfig, TResult> onGoogleCloudPubSub,
            Func<SqsConfig, TResult> onSqs,
            Func<SnsConfig, TResult> onSns,
            Func<BigQueryConfig, TResult> onBigQuery,
            Func<ClickhouseConfig, TResult> onClickhouse,
            Func<EventBridgeConfig, TResult> onEventBridge,
            Func<SnowflakeConfig, TResult> onSnowflake,
            Func<RabbitMqConfig, TResult> onRabbitMq,
            Func<RedshiftConfig, TResult> onRedshift
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
                ConfigType.GoogleCloudPubSub => onGoogleCloudPubSub(
                    (GoogleCloudPubSubConfig)_value
                ),
                ConfigType.Sqs => onSqs((SqsConfig)_value),
                ConfigType.Sns => onSns((SnsConfig)_value),
                ConfigType.BigQuery => onBigQuery((BigQueryConfig)_value),
                ConfigType.Clickhouse => onClickhouse((ClickhouseConfig)_value),
                ConfigType.EventBridge => onEventBridge((EventBridgeConfig)_value),
                ConfigType.Snowflake => onSnowflake((SnowflakeConfig)_value),
                ConfigType.RabbitMq => onRabbitMq((RabbitMqConfig)_value),
                ConfigType.Redshift => onRedshift((RedshiftConfig)_value),
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
            Action<GoogleCloudStorageConfig> onGoogleCloudStorage,
            Action<GoogleCloudPubSubConfig> onGoogleCloudPubSub,
            Action<SqsConfig> onSqs,
            Action<SnsConfig> onSns,
            Action<BigQueryConfig> onBigQuery,
            Action<ClickhouseConfig> onClickhouse,
            Action<EventBridgeConfig> onEventBridge,
            Action<SnowflakeConfig> onSnowflake,
            Action<RabbitMqConfig> onRabbitMq,
            Action<RedshiftConfig> onRedshift
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
                case ConfigType.GoogleCloudPubSub:
                    onGoogleCloudPubSub((GoogleCloudPubSubConfig)_value);
                    break;
                case ConfigType.Sqs:
                    onSqs((SqsConfig)_value);
                    break;
                case ConfigType.Sns:
                    onSns((SnsConfig)_value);
                    break;
                case ConfigType.BigQuery:
                    onBigQuery((BigQueryConfig)_value);
                    break;
                case ConfigType.Clickhouse:
                    onClickhouse((ClickhouseConfig)_value);
                    break;
                case ConfigType.EventBridge:
                    onEventBridge((EventBridgeConfig)_value);
                    break;
                case ConfigType.Snowflake:
                    onSnowflake((SnowflakeConfig)_value);
                    break;
                case ConfigType.RabbitMq:
                    onRabbitMq((RabbitMqConfig)_value);
                    break;
                case ConfigType.Redshift:
                    onRedshift((RedshiftConfig)_value);
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class StreamSinkInSurrogate
    {
        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("status")]
        public SinkStatusIn? Status { get; set; } = null;

        [JsonProperty("batchSize")]
        public ushort? BatchSize { get; set; } = null;

        [JsonProperty("maxWaitSecs")]
        public ushort? MaxWaitSecs { get; set; } = null;

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

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
                Uid = surrogate.Uid,
                Status = surrogate.Status,
                BatchSize = surrogate.BatchSize,
                MaxWaitSecs = surrogate.MaxWaitSecs,
                EventTypes = surrogate.EventTypes,
                Metadata = surrogate.Metadata,
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
                ["googleCloudPubSub"] = c =>
                    StreamSinkInConfig.GoogleCloudPubSub(ToObj<GoogleCloudPubSubConfig>(c)),
                ["sqs"] = c => StreamSinkInConfig.Sqs(ToObj<SqsConfig>(c)),
                ["sns"] = c => StreamSinkInConfig.Sns(ToObj<SnsConfig>(c)),
                ["bigQuery"] = c => StreamSinkInConfig.BigQuery(ToObj<BigQueryConfig>(c)),
                ["clickhouse"] = c => StreamSinkInConfig.Clickhouse(ToObj<ClickhouseConfig>(c)),
                ["eventBridge"] = c => StreamSinkInConfig.EventBridge(ToObj<EventBridgeConfig>(c)),
                ["snowflake"] = c => StreamSinkInConfig.Snowflake(ToObj<SnowflakeConfig>(c)),
                ["rabbitMq"] = c => StreamSinkInConfig.RabbitMq(ToObj<RabbitMqConfig>(c)),
                ["redshift"] = c => StreamSinkInConfig.Redshift(ToObj<RedshiftConfig>(c)),
            };
    }
}
