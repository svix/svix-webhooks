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
        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("status", Required = Required.Always)]
        public required SinkStatus Status { get; set; }

        [JsonProperty("currentIterator", Required = Required.Always)]
        public required string CurrentIterator { get; set; }

        [JsonProperty("failureReason")]
        public string? FailureReason { get; set; } = null;

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        [JsonProperty("batchSize", Required = Required.Always)]
        public required int BatchSize { get; set; }

        [JsonProperty("maxWaitSecs", Required = Required.Always)]
        public required int MaxWaitSecs { get; set; }

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("nextRetryAt")]
        public DateTime? NextRetryAt { get; set; } = null;

        [JsonProperty("metadata", Required = Required.Always)]
        public required Dictionary<string, string> Metadata { get; set; }

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
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  CurrentIterator: ").Append(CurrentIterator).Append('\n');
            sb.Append("  FailureReason: ").Append(FailureReason).Append('\n');
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("  BatchSize: ").Append(BatchSize).Append('\n');
            sb.Append("  MaxWaitSecs: ").Append(MaxWaitSecs).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("  NextRetryAt: ").Append(NextRetryAt).Append('\n');
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
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
            AzureBlobStorageConfigOut azureBlobStorageConfigOut
        ) => new(azureBlobStorageConfigOut, ConfigType.AzureBlobStorage);

        public static StreamSinkOutConfig OtelTracing(OtelTracingConfigOut otelTracingConfigOut) =>
            new(otelTracingConfigOut, ConfigType.OtelTracing);

        public static StreamSinkOutConfig Http(SinkHttpConfigOut sinkHttpConfigOut) =>
            new(sinkHttpConfigOut, ConfigType.Http);

        public static StreamSinkOutConfig AmazonS3(S3ConfigOut s3ConfigOut) =>
            new(s3ConfigOut, ConfigType.AmazonS3);

        public static StreamSinkOutConfig Snowflake(SnowflakeConfigOut snowflakeConfigOut) =>
            new(snowflakeConfigOut, ConfigType.Snowflake);

        public static StreamSinkOutConfig GoogleCloudStorage(
            GoogleCloudStorageConfigOut googleCloudStorageConfigOut
        ) => new(googleCloudStorageConfigOut, ConfigType.GoogleCloudStorage);

        public static StreamSinkOutConfig GoogleCloudPubSub(
            GoogleCloudPubSubConfigOut googleCloudPubSubConfigOut
        ) => new(googleCloudPubSubConfigOut, ConfigType.GoogleCloudPubSub);

        public static StreamSinkOutConfig Redshift(RedshiftConfigOut redshiftConfigOut) =>
            new(redshiftConfigOut, ConfigType.Redshift);

        public static StreamSinkOutConfig BigQuery(BigQueryConfigOut bigQueryConfigOut) =>
            new(bigQueryConfigOut, ConfigType.BigQuery);

        public static StreamSinkOutConfig Clickhouse(ClickhouseConfigOut clickhouseConfigOut) =>
            new(clickhouseConfigOut, ConfigType.Clickhouse);

        public static StreamSinkOutConfig RabbitMq(RabbitMqConfigOut rabbitMqConfigOut) =>
            new(rabbitMqConfigOut, ConfigType.RabbitMq);

        public static StreamSinkOutConfig Sqs(SqsConfigOut sqsConfigOut) =>
            new(sqsConfigOut, ConfigType.Sqs);

        public static StreamSinkOutConfig EventBridge(EventBridgeConfigOut eventBridgeConfigOut) =>
            new(eventBridgeConfigOut, ConfigType.EventBridge);

        public static StreamSinkOutConfig Sns(SnsConfigOut snsConfigOut) =>
            new(snsConfigOut, ConfigType.Sns);

        public static StreamSinkOutConfig Postgres(PostgresConfigOut postgresConfigOut) =>
            new(postgresConfigOut, ConfigType.Postgres);

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

            [EnumMember(Value = "snowflake")]
            Snowflake,

            [EnumMember(Value = "googleCloudStorage")]
            GoogleCloudStorage,

            [EnumMember(Value = "googleCloudPubSub")]
            GoogleCloudPubSub,

            [EnumMember(Value = "redshift")]
            Redshift,

            [EnumMember(Value = "bigQuery")]
            BigQuery,

            [EnumMember(Value = "clickhouse")]
            Clickhouse,

            [EnumMember(Value = "rabbitMq")]
            RabbitMq,

            [EnumMember(Value = "sqs")]
            Sqs,

            [EnumMember(Value = "eventBridge")]
            EventBridge,

            [EnumMember(Value = "sns")]
            Sns,

            [EnumMember(Value = "postgres")]
            Postgres,
        }

        public TResult Match<TResult>(
            Func<TResult> onPoller,
            Func<AzureBlobStorageConfigOut, TResult> onAzureBlobStorage,
            Func<OtelTracingConfigOut, TResult> onOtelTracing,
            Func<SinkHttpConfigOut, TResult> onHttp,
            Func<S3ConfigOut, TResult> onAmazonS3,
            Func<SnowflakeConfigOut, TResult> onSnowflake,
            Func<GoogleCloudStorageConfigOut, TResult> onGoogleCloudStorage,
            Func<GoogleCloudPubSubConfigOut, TResult> onGoogleCloudPubSub,
            Func<RedshiftConfigOut, TResult> onRedshift,
            Func<BigQueryConfigOut, TResult> onBigQuery,
            Func<ClickhouseConfigOut, TResult> onClickhouse,
            Func<RabbitMqConfigOut, TResult> onRabbitMq,
            Func<SqsConfigOut, TResult> onSqs,
            Func<EventBridgeConfigOut, TResult> onEventBridge,
            Func<SnsConfigOut, TResult> onSns,
            Func<PostgresConfigOut, TResult> onPostgres
        )
        {
            return _type switch
            {
                ConfigType.Poller => onPoller(),
                ConfigType.AzureBlobStorage => onAzureBlobStorage(
                    (AzureBlobStorageConfigOut)_value
                ),
                ConfigType.OtelTracing => onOtelTracing((OtelTracingConfigOut)_value),
                ConfigType.Http => onHttp((SinkHttpConfigOut)_value),
                ConfigType.AmazonS3 => onAmazonS3((S3ConfigOut)_value),
                ConfigType.Snowflake => onSnowflake((SnowflakeConfigOut)_value),
                ConfigType.GoogleCloudStorage => onGoogleCloudStorage(
                    (GoogleCloudStorageConfigOut)_value
                ),
                ConfigType.GoogleCloudPubSub => onGoogleCloudPubSub(
                    (GoogleCloudPubSubConfigOut)_value
                ),
                ConfigType.Redshift => onRedshift((RedshiftConfigOut)_value),
                ConfigType.BigQuery => onBigQuery((BigQueryConfigOut)_value),
                ConfigType.Clickhouse => onClickhouse((ClickhouseConfigOut)_value),
                ConfigType.RabbitMq => onRabbitMq((RabbitMqConfigOut)_value),
                ConfigType.Sqs => onSqs((SqsConfigOut)_value),
                ConfigType.EventBridge => onEventBridge((EventBridgeConfigOut)_value),
                ConfigType.Sns => onSns((SnsConfigOut)_value),
                ConfigType.Postgres => onPostgres((PostgresConfigOut)_value),
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(
            Action onPoller,
            Action<AzureBlobStorageConfigOut> onAzureBlobStorage,
            Action<OtelTracingConfigOut> onOtelTracing,
            Action<SinkHttpConfigOut> onHttp,
            Action<S3ConfigOut> onAmazonS3,
            Action<SnowflakeConfigOut> onSnowflake,
            Action<GoogleCloudStorageConfigOut> onGoogleCloudStorage,
            Action<GoogleCloudPubSubConfigOut> onGoogleCloudPubSub,
            Action<RedshiftConfigOut> onRedshift,
            Action<BigQueryConfigOut> onBigQuery,
            Action<ClickhouseConfigOut> onClickhouse,
            Action<RabbitMqConfigOut> onRabbitMq,
            Action<SqsConfigOut> onSqs,
            Action<EventBridgeConfigOut> onEventBridge,
            Action<SnsConfigOut> onSns,
            Action<PostgresConfigOut> onPostgres
        )
        {
            switch (_type)
            {
                case ConfigType.Poller:
                    onPoller();
                    break;
                case ConfigType.AzureBlobStorage:
                    onAzureBlobStorage((AzureBlobStorageConfigOut)_value);
                    break;
                case ConfigType.OtelTracing:
                    onOtelTracing((OtelTracingConfigOut)_value);
                    break;
                case ConfigType.Http:
                    onHttp((SinkHttpConfigOut)_value);
                    break;
                case ConfigType.AmazonS3:
                    onAmazonS3((S3ConfigOut)_value);
                    break;
                case ConfigType.Snowflake:
                    onSnowflake((SnowflakeConfigOut)_value);
                    break;
                case ConfigType.GoogleCloudStorage:
                    onGoogleCloudStorage((GoogleCloudStorageConfigOut)_value);
                    break;
                case ConfigType.GoogleCloudPubSub:
                    onGoogleCloudPubSub((GoogleCloudPubSubConfigOut)_value);
                    break;
                case ConfigType.Redshift:
                    onRedshift((RedshiftConfigOut)_value);
                    break;
                case ConfigType.BigQuery:
                    onBigQuery((BigQueryConfigOut)_value);
                    break;
                case ConfigType.Clickhouse:
                    onClickhouse((ClickhouseConfigOut)_value);
                    break;
                case ConfigType.RabbitMq:
                    onRabbitMq((RabbitMqConfigOut)_value);
                    break;
                case ConfigType.Sqs:
                    onSqs((SqsConfigOut)_value);
                    break;
                case ConfigType.EventBridge:
                    onEventBridge((EventBridgeConfigOut)_value);
                    break;
                case ConfigType.Sns:
                    onSns((SnsConfigOut)_value);
                    break;
                case ConfigType.Postgres:
                    onPostgres((PostgresConfigOut)_value);
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class StreamSinkOutSurrogate
    {
        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("status", Required = Required.Always)]
        public required SinkStatus Status { get; set; }

        [JsonProperty("currentIterator", Required = Required.Always)]
        public required string CurrentIterator { get; set; }

        [JsonProperty("failureReason")]
        public string? FailureReason { get; set; } = null;

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        [JsonProperty("batchSize", Required = Required.Always)]
        public required int BatchSize { get; set; }

        [JsonProperty("maxWaitSecs", Required = Required.Always)]
        public required int MaxWaitSecs { get; set; }

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("nextRetryAt")]
        public DateTime? NextRetryAt { get; set; } = null;

        [JsonProperty("metadata", Required = Required.Always)]
        public required Dictionary<string, string> Metadata { get; set; }

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
                Id = surrogate.Id,
                Uid = surrogate.Uid,
                Status = surrogate.Status,
                CurrentIterator = surrogate.CurrentIterator,
                FailureReason = surrogate.FailureReason,
                CreatedAt = surrogate.CreatedAt,
                UpdatedAt = surrogate.UpdatedAt,
                BatchSize = surrogate.BatchSize,
                MaxWaitSecs = surrogate.MaxWaitSecs,
                EventTypes = surrogate.EventTypes,
                Channels = surrogate.Channels,
                NextRetryAt = surrogate.NextRetryAt,
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

        private readonly Dictionary<string, Func<(JObject, string), StreamSinkOutConfig>> typeMap =
            new()
            {
                ["poller"] = c => StreamSinkOutConfig.Poller(),
                ["azureBlobStorage"] = c =>
                    StreamSinkOutConfig.AzureBlobStorage(ToObj<AzureBlobStorageConfigOut>(c)),
                ["otelTracing"] = c =>
                    StreamSinkOutConfig.OtelTracing(ToObj<OtelTracingConfigOut>(c)),
                ["http"] = c => StreamSinkOutConfig.Http(ToObj<SinkHttpConfigOut>(c)),
                ["amazonS3"] = c => StreamSinkOutConfig.AmazonS3(ToObj<S3ConfigOut>(c)),
                ["snowflake"] = c => StreamSinkOutConfig.Snowflake(ToObj<SnowflakeConfigOut>(c)),
                ["googleCloudStorage"] = c =>
                    StreamSinkOutConfig.GoogleCloudStorage(ToObj<GoogleCloudStorageConfigOut>(c)),
                ["googleCloudPubSub"] = c =>
                    StreamSinkOutConfig.GoogleCloudPubSub(ToObj<GoogleCloudPubSubConfigOut>(c)),
                ["redshift"] = c => StreamSinkOutConfig.Redshift(ToObj<RedshiftConfigOut>(c)),
                ["bigQuery"] = c => StreamSinkOutConfig.BigQuery(ToObj<BigQueryConfigOut>(c)),
                ["clickhouse"] = c => StreamSinkOutConfig.Clickhouse(ToObj<ClickhouseConfigOut>(c)),
                ["rabbitMq"] = c => StreamSinkOutConfig.RabbitMq(ToObj<RabbitMqConfigOut>(c)),
                ["sqs"] = c => StreamSinkOutConfig.Sqs(ToObj<SqsConfigOut>(c)),
                ["eventBridge"] = c =>
                    StreamSinkOutConfig.EventBridge(ToObj<EventBridgeConfigOut>(c)),
                ["sns"] = c => StreamSinkOutConfig.Sns(ToObj<SnsConfigOut>(c)),
                ["postgres"] = c => StreamSinkOutConfig.Postgres(ToObj<PostgresConfigOut>(c)),
            };
    }
}
