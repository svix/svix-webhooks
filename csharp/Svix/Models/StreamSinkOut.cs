// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    [JsonConverter(typeof(StreamSinkOutConverter))]
    public class StreamSinkOut
    {
        [JsonProperty("batchSize", Required = Required.Always)]
        public required int BatchSize { get; set; }

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("currentIterator", Required = Required.Always)]
        public required string CurrentIterator { get; set; }

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("failureReason")]
        public string? FailureReason { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("maxWaitSecs", Required = Required.Always)]
        public required int MaxWaitSecs { get; set; }

        [JsonProperty("metadata", Required = Required.Always)]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonProperty("nextRetryAt")]
        public DateTime? NextRetryAt { get; set; } = null;

        [JsonProperty("status", Required = Required.Always)]
        public required SinkStatus Status { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        [JsonIgnore]
        public required StreamSinkOutConfig Config { get; set; }

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

            sb.Append("class StreamSinkOut {\n");
            sb.Append("  BatchSize: ").Append(BatchSize).Append('\n');
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  CurrentIterator: ").Append(CurrentIterator).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  FailureReason: ").Append(FailureReason).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  MaxWaitSecs: ").Append(MaxWaitSecs).Append('\n');
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("  NextRetryAt: ").Append(NextRetryAt).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("  Config: ").Append(Config).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }

    public class StreamSinkOutConfig
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

        private StreamSinkOutConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static StreamSinkOutConfig Poller() =>
            new(new Dictionary<string, string>(), ConfigType.Poller);

        public static StreamSinkOutConfig AzureBlobStorage(
            AzureBlobStorageConfig azureBlobStorageConfig
        ) => new(azureBlobStorageConfig, ConfigType.AzureBlobStorage);

        public static StreamSinkOutConfig OtelTracing(SinkOtelV1Config sinkOtelV1Config) =>
            new(sinkOtelV1Config, ConfigType.OtelTracing);

        public static StreamSinkOutConfig Http(SinkHttpConfig sinkHttpConfig) =>
            new(sinkHttpConfig, ConfigType.Http);

        public static StreamSinkOutConfig AmazonS3(S3Config s3Config) =>
            new(s3Config, ConfigType.AmazonS3);

        public static StreamSinkOutConfig GoogleCloudStorage(
            GoogleCloudStorageConfig googleCloudStorageConfig
        ) => new(googleCloudStorageConfig, ConfigType.GoogleCloudStorage);

        public static StreamSinkOutConfig GoogleCloudPubSub(
            GoogleCloudPubSubConfig googleCloudPubSubConfig
        ) => new(googleCloudPubSubConfig, ConfigType.GoogleCloudPubSub);

        public static StreamSinkOutConfig Sqs(SqsConfig sqsConfig) =>
            new(sqsConfig, ConfigType.Sqs);

        public static StreamSinkOutConfig Sns(SnsConfig snsConfig) =>
            new(snsConfig, ConfigType.Sns);

        public static StreamSinkOutConfig BigQuery(BigQueryConfig bigQueryConfig) =>
            new(bigQueryConfig, ConfigType.BigQuery);

        public static StreamSinkOutConfig Clickhouse(ClickhouseConfig clickhouseConfig) =>
            new(clickhouseConfig, ConfigType.Clickhouse);

        public static StreamSinkOutConfig EventBridge(EventBridgeConfig eventBridgeConfig) =>
            new(eventBridgeConfig, ConfigType.EventBridge);

        public static StreamSinkOutConfig Snowflake(SnowflakeConfig snowflakeConfig) =>
            new(snowflakeConfig, ConfigType.Snowflake);

        public static StreamSinkOutConfig RabbitMq(RabbitMqConfig rabbitMqConfig) =>
            new(rabbitMqConfig, ConfigType.RabbitMq);

        public static StreamSinkOutConfig Redshift(RedshiftConfig redshiftConfig) =>
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

    internal class StreamSinkOutSurrogate
    {
        [JsonProperty("batchSize", Required = Required.Always)]
        public required int BatchSize { get; set; }

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("currentIterator", Required = Required.Always)]
        public required string CurrentIterator { get; set; }

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("failureReason")]
        public string? FailureReason { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("maxWaitSecs", Required = Required.Always)]
        public required int MaxWaitSecs { get; set; }

        [JsonProperty("metadata", Required = Required.Always)]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonProperty("nextRetryAt")]
        public DateTime? NextRetryAt { get; set; } = null;

        [JsonProperty("status", Required = Required.Always)]
        public required SinkStatus Status { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        [JsonProperty("config", Required = Required.Always)]
        public required JObject Config { get; set; }
    }

    public class StreamSinkOutConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(StreamSinkOut);
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
                serializer.Deserialize<StreamSinkOutSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to StreamSinkOutSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), StreamSinkOutConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for StreamSinkOutConfig.config"
                );
            }

            StreamSinkOutConfig config = func((surrogate.Config, surrogate.Type));

            return new StreamSinkOut
            {
                BatchSize = surrogate.BatchSize,
                CreatedAt = surrogate.CreatedAt,
                CurrentIterator = surrogate.CurrentIterator,
                EventTypes = surrogate.EventTypes,
                FailureReason = surrogate.FailureReason,
                Id = surrogate.Id,
                MaxWaitSecs = surrogate.MaxWaitSecs,
                Metadata = surrogate.Metadata,
                NextRetryAt = surrogate.NextRetryAt,
                Status = surrogate.Status,
                Uid = surrogate.Uid,
                UpdatedAt = surrogate.UpdatedAt,
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

        private readonly Dictionary<string, Func<(JObject, string), StreamSinkOutConfig>> typeMap =
            new()
            {
                ["poller"] = c => StreamSinkOutConfig.Poller(),
                ["azureBlobStorage"] = c =>
                    StreamSinkOutConfig.AzureBlobStorage(ToObj<AzureBlobStorageConfig>(c)),
                ["otelTracing"] = c => StreamSinkOutConfig.OtelTracing(ToObj<SinkOtelV1Config>(c)),
                ["http"] = c => StreamSinkOutConfig.Http(ToObj<SinkHttpConfig>(c)),
                ["amazonS3"] = c => StreamSinkOutConfig.AmazonS3(ToObj<S3Config>(c)),
                ["googleCloudStorage"] = c =>
                    StreamSinkOutConfig.GoogleCloudStorage(ToObj<GoogleCloudStorageConfig>(c)),
                ["googleCloudPubSub"] = c =>
                    StreamSinkOutConfig.GoogleCloudPubSub(ToObj<GoogleCloudPubSubConfig>(c)),
                ["sqs"] = c => StreamSinkOutConfig.Sqs(ToObj<SqsConfig>(c)),
                ["sns"] = c => StreamSinkOutConfig.Sns(ToObj<SnsConfig>(c)),
                ["bigQuery"] = c => StreamSinkOutConfig.BigQuery(ToObj<BigQueryConfig>(c)),
                ["clickhouse"] = c => StreamSinkOutConfig.Clickhouse(ToObj<ClickhouseConfig>(c)),
                ["eventBridge"] = c => StreamSinkOutConfig.EventBridge(ToObj<EventBridgeConfig>(c)),
                ["snowflake"] = c => StreamSinkOutConfig.Snowflake(ToObj<SnowflakeConfig>(c)),
                ["rabbitMq"] = c => StreamSinkOutConfig.RabbitMq(ToObj<RabbitMqConfig>(c)),
                ["redshift"] = c => StreamSinkOutConfig.Redshift(ToObj<RedshiftConfig>(c)),
            };
    }
}
