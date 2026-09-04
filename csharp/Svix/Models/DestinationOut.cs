// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    /// <summary>
    /// Equivalent to [`SinkConfigOut`], with `fifoEndpoint` instead of `http`.
    /// <summary>
    [JsonConverter(typeof(DestinationOutConverter))]
    public class DestinationOut
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
        public required DestinationOutConfig Config { get; set; }

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

            sb.Append("class DestinationOut {\n");
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

    public class DestinationOutConfig
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

        private DestinationOutConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static DestinationOutConfig PollingEndpoint() =>
            new(new Dictionary<string, string>(), ConfigType.PollingEndpoint);

        public static DestinationOutConfig AzureBlobStorage(
            AzureBlobStorageConfigOut azureBlobStorageConfigOut
        ) => new(azureBlobStorageConfigOut, ConfigType.AzureBlobStorage);

        public static DestinationOutConfig OtelTracing(OtelTracingConfigOut otelTracingConfigOut) =>
            new(otelTracingConfigOut, ConfigType.OtelTracing);

        public static DestinationOutConfig FifoEndpoint(SinkHttpConfigOut sinkHttpConfigOut) =>
            new(sinkHttpConfigOut, ConfigType.FifoEndpoint);

        public static DestinationOutConfig AmazonS3(S3ConfigOut s3ConfigOut) =>
            new(s3ConfigOut, ConfigType.AmazonS3);

        public static DestinationOutConfig Snowflake(SnowflakeConfigOut snowflakeConfigOut) =>
            new(snowflakeConfigOut, ConfigType.Snowflake);

        public static DestinationOutConfig GoogleCloudStorage(
            GoogleCloudStorageConfigOut googleCloudStorageConfigOut
        ) => new(googleCloudStorageConfigOut, ConfigType.GoogleCloudStorage);

        public static DestinationOutConfig GoogleCloudPubSub(
            GoogleCloudPubSubConfigOut googleCloudPubSubConfigOut
        ) => new(googleCloudPubSubConfigOut, ConfigType.GoogleCloudPubSub);

        public static DestinationOutConfig Redshift(RedshiftConfigOut redshiftConfigOut) =>
            new(redshiftConfigOut, ConfigType.Redshift);

        public static DestinationOutConfig BigQuery(BigQueryConfigOut bigQueryConfigOut) =>
            new(bigQueryConfigOut, ConfigType.BigQuery);

        public static DestinationOutConfig Clickhouse(ClickhouseConfigOut clickhouseConfigOut) =>
            new(clickhouseConfigOut, ConfigType.Clickhouse);

        public static DestinationOutConfig RabbitMq(RabbitMqConfigOut rabbitMqConfigOut) =>
            new(rabbitMqConfigOut, ConfigType.RabbitMq);

        public static DestinationOutConfig Sqs(SqsConfigOut sqsConfigOut) =>
            new(sqsConfigOut, ConfigType.Sqs);

        public static DestinationOutConfig EventBridge(EventBridgeConfigOut eventBridgeConfigOut) =>
            new(eventBridgeConfigOut, ConfigType.EventBridge);

        public static DestinationOutConfig Sns(SnsConfigOut snsConfigOut) =>
            new(snsConfigOut, ConfigType.Sns);

        public static DestinationOutConfig Postgres(PostgresConfigOut postgresConfigOut) =>
            new(postgresConfigOut, ConfigType.Postgres);

        private enum ConfigType
        {
            [EnumMember(Value = "pollingEndpoint")]
            PollingEndpoint,

            [EnumMember(Value = "azureBlobStorage")]
            AzureBlobStorage,

            [EnumMember(Value = "otelTracing")]
            OtelTracing,

            [EnumMember(Value = "fifoEndpoint")]
            FifoEndpoint,

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
            Func<TResult> onPollingEndpoint,
            Func<AzureBlobStorageConfigOut, TResult> onAzureBlobStorage,
            Func<OtelTracingConfigOut, TResult> onOtelTracing,
            Func<SinkHttpConfigOut, TResult> onFifoEndpoint,
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
                ConfigType.PollingEndpoint => onPollingEndpoint(),
                ConfigType.AzureBlobStorage => onAzureBlobStorage(
                    (AzureBlobStorageConfigOut)_value
                ),
                ConfigType.OtelTracing => onOtelTracing((OtelTracingConfigOut)_value),
                ConfigType.FifoEndpoint => onFifoEndpoint((SinkHttpConfigOut)_value),
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
            Action onPollingEndpoint,
            Action<AzureBlobStorageConfigOut> onAzureBlobStorage,
            Action<OtelTracingConfigOut> onOtelTracing,
            Action<SinkHttpConfigOut> onFifoEndpoint,
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
                case ConfigType.PollingEndpoint:
                    onPollingEndpoint();
                    break;
                case ConfigType.AzureBlobStorage:
                    onAzureBlobStorage((AzureBlobStorageConfigOut)_value);
                    break;
                case ConfigType.OtelTracing:
                    onOtelTracing((OtelTracingConfigOut)_value);
                    break;
                case ConfigType.FifoEndpoint:
                    onFifoEndpoint((SinkHttpConfigOut)_value);
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

    internal class DestinationOutSurrogate
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

    public class DestinationOutConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(DestinationOut);
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
                serializer.Deserialize<DestinationOutSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to DestinationOutSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), DestinationOutConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for DestinationOutConfig.config"
                );
            }

            DestinationOutConfig config = func((surrogate.Config, surrogate.Type));

            return new DestinationOut
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

        private readonly Dictionary<string, Func<(JObject, string), DestinationOutConfig>> typeMap =
            new()
            {
                ["pollingEndpoint"] = c => DestinationOutConfig.PollingEndpoint(),
                ["azureBlobStorage"] = c =>
                    DestinationOutConfig.AzureBlobStorage(ToObj<AzureBlobStorageConfigOut>(c)),
                ["otelTracing"] = c =>
                    DestinationOutConfig.OtelTracing(ToObj<OtelTracingConfigOut>(c)),
                ["fifoEndpoint"] = c =>
                    DestinationOutConfig.FifoEndpoint(ToObj<SinkHttpConfigOut>(c)),
                ["amazonS3"] = c => DestinationOutConfig.AmazonS3(ToObj<S3ConfigOut>(c)),
                ["snowflake"] = c => DestinationOutConfig.Snowflake(ToObj<SnowflakeConfigOut>(c)),
                ["googleCloudStorage"] = c =>
                    DestinationOutConfig.GoogleCloudStorage(ToObj<GoogleCloudStorageConfigOut>(c)),
                ["googleCloudPubSub"] = c =>
                    DestinationOutConfig.GoogleCloudPubSub(ToObj<GoogleCloudPubSubConfigOut>(c)),
                ["redshift"] = c => DestinationOutConfig.Redshift(ToObj<RedshiftConfigOut>(c)),
                ["bigQuery"] = c => DestinationOutConfig.BigQuery(ToObj<BigQueryConfigOut>(c)),
                ["clickhouse"] = c =>
                    DestinationOutConfig.Clickhouse(ToObj<ClickhouseConfigOut>(c)),
                ["rabbitMq"] = c => DestinationOutConfig.RabbitMq(ToObj<RabbitMqConfigOut>(c)),
                ["sqs"] = c => DestinationOutConfig.Sqs(ToObj<SqsConfigOut>(c)),
                ["eventBridge"] = c =>
                    DestinationOutConfig.EventBridge(ToObj<EventBridgeConfigOut>(c)),
                ["sns"] = c => DestinationOutConfig.Sns(ToObj<SnsConfigOut>(c)),
                ["postgres"] = c => DestinationOutConfig.Postgres(ToObj<PostgresConfigOut>(c)),
            };
    }
}
