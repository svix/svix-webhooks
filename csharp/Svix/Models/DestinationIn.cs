// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    /// <summary>
    /// The destination's type and type-specific configuration.
    /// <summary>
    [JsonConverter(typeof(DestinationInConverter))]
    public class DestinationIn
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

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonIgnore]
        public required DestinationInConfig Config { get; set; }

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

            sb.Append("class DestinationIn {\n");
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  BatchSize: ").Append(BatchSize).Append('\n');
            sb.Append("  MaxWaitSecs: ").Append(MaxWaitSecs).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("  Config: ").Append(Config).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }

    public class DestinationInConfig
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

        private DestinationInConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static DestinationInConfig PollingEndpoint() =>
            new(new Dictionary<string, string>(), ConfigType.PollingEndpoint);

        public static DestinationInConfig AzureBlobStorage(
            AzureBlobStorageConfigIn azureBlobStorageConfigIn
        ) => new(azureBlobStorageConfigIn, ConfigType.AzureBlobStorage);

        public static DestinationInConfig OtelTracing(OtelTracingConfigIn otelTracingConfigIn) =>
            new(otelTracingConfigIn, ConfigType.OtelTracing);

        public static DestinationInConfig FifoEndpoint(FifoEndpointConfigIn fifoEndpointConfigIn) =>
            new(fifoEndpointConfigIn, ConfigType.FifoEndpoint);

        public static DestinationInConfig AmazonS3(S3ConfigIn s3ConfigIn) =>
            new(s3ConfigIn, ConfigType.AmazonS3);

        public static DestinationInConfig GoogleCloudStorage(
            GoogleCloudStorageConfigIn googleCloudStorageConfigIn
        ) => new(googleCloudStorageConfigIn, ConfigType.GoogleCloudStorage);

        public static DestinationInConfig GoogleCloudPubSub(
            GoogleCloudPubSubConfigIn googleCloudPubSubConfigIn
        ) => new(googleCloudPubSubConfigIn, ConfigType.GoogleCloudPubSub);

        public static DestinationInConfig Sqs(SqsConfigIn sqsConfigIn) =>
            new(sqsConfigIn, ConfigType.Sqs);

        public static DestinationInConfig Sns(SnsConfigIn snsConfigIn) =>
            new(snsConfigIn, ConfigType.Sns);

        public static DestinationInConfig BigQuery(BigQueryConfigIn bigQueryConfigIn) =>
            new(bigQueryConfigIn, ConfigType.BigQuery);

        public static DestinationInConfig Clickhouse(ClickhouseConfigIn clickhouseConfigIn) =>
            new(clickhouseConfigIn, ConfigType.Clickhouse);

        public static DestinationInConfig EventBridge(EventBridgeConfigIn eventBridgeConfigIn) =>
            new(eventBridgeConfigIn, ConfigType.EventBridge);

        public static DestinationInConfig Snowflake(SnowflakeConfigIn snowflakeConfigIn) =>
            new(snowflakeConfigIn, ConfigType.Snowflake);

        public static DestinationInConfig RabbitMq(RabbitMqConfigIn rabbitMqConfigIn) =>
            new(rabbitMqConfigIn, ConfigType.RabbitMq);

        public static DestinationInConfig Redshift(RedshiftConfigIn redshiftConfigIn) =>
            new(redshiftConfigIn, ConfigType.Redshift);

        public static DestinationInConfig Postgres(PostgresConfigIn postgresConfigIn) =>
            new(postgresConfigIn, ConfigType.Postgres);

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

            [EnumMember(Value = "postgres")]
            Postgres,
        }

        public TResult Match<TResult>(
            Func<TResult> onPollingEndpoint,
            Func<AzureBlobStorageConfigIn, TResult> onAzureBlobStorage,
            Func<OtelTracingConfigIn, TResult> onOtelTracing,
            Func<FifoEndpointConfigIn, TResult> onFifoEndpoint,
            Func<S3ConfigIn, TResult> onAmazonS3,
            Func<GoogleCloudStorageConfigIn, TResult> onGoogleCloudStorage,
            Func<GoogleCloudPubSubConfigIn, TResult> onGoogleCloudPubSub,
            Func<SqsConfigIn, TResult> onSqs,
            Func<SnsConfigIn, TResult> onSns,
            Func<BigQueryConfigIn, TResult> onBigQuery,
            Func<ClickhouseConfigIn, TResult> onClickhouse,
            Func<EventBridgeConfigIn, TResult> onEventBridge,
            Func<SnowflakeConfigIn, TResult> onSnowflake,
            Func<RabbitMqConfigIn, TResult> onRabbitMq,
            Func<RedshiftConfigIn, TResult> onRedshift,
            Func<PostgresConfigIn, TResult> onPostgres
        )
        {
            return _type switch
            {
                ConfigType.PollingEndpoint => onPollingEndpoint(),
                ConfigType.AzureBlobStorage => onAzureBlobStorage((AzureBlobStorageConfigIn)_value),
                ConfigType.OtelTracing => onOtelTracing((OtelTracingConfigIn)_value),
                ConfigType.FifoEndpoint => onFifoEndpoint((FifoEndpointConfigIn)_value),
                ConfigType.AmazonS3 => onAmazonS3((S3ConfigIn)_value),
                ConfigType.GoogleCloudStorage => onGoogleCloudStorage(
                    (GoogleCloudStorageConfigIn)_value
                ),
                ConfigType.GoogleCloudPubSub => onGoogleCloudPubSub(
                    (GoogleCloudPubSubConfigIn)_value
                ),
                ConfigType.Sqs => onSqs((SqsConfigIn)_value),
                ConfigType.Sns => onSns((SnsConfigIn)_value),
                ConfigType.BigQuery => onBigQuery((BigQueryConfigIn)_value),
                ConfigType.Clickhouse => onClickhouse((ClickhouseConfigIn)_value),
                ConfigType.EventBridge => onEventBridge((EventBridgeConfigIn)_value),
                ConfigType.Snowflake => onSnowflake((SnowflakeConfigIn)_value),
                ConfigType.RabbitMq => onRabbitMq((RabbitMqConfigIn)_value),
                ConfigType.Redshift => onRedshift((RedshiftConfigIn)_value),
                ConfigType.Postgres => onPostgres((PostgresConfigIn)_value),
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(
            Action onPollingEndpoint,
            Action<AzureBlobStorageConfigIn> onAzureBlobStorage,
            Action<OtelTracingConfigIn> onOtelTracing,
            Action<FifoEndpointConfigIn> onFifoEndpoint,
            Action<S3ConfigIn> onAmazonS3,
            Action<GoogleCloudStorageConfigIn> onGoogleCloudStorage,
            Action<GoogleCloudPubSubConfigIn> onGoogleCloudPubSub,
            Action<SqsConfigIn> onSqs,
            Action<SnsConfigIn> onSns,
            Action<BigQueryConfigIn> onBigQuery,
            Action<ClickhouseConfigIn> onClickhouse,
            Action<EventBridgeConfigIn> onEventBridge,
            Action<SnowflakeConfigIn> onSnowflake,
            Action<RabbitMqConfigIn> onRabbitMq,
            Action<RedshiftConfigIn> onRedshift,
            Action<PostgresConfigIn> onPostgres
        )
        {
            switch (_type)
            {
                case ConfigType.PollingEndpoint:
                    onPollingEndpoint();
                    break;
                case ConfigType.AzureBlobStorage:
                    onAzureBlobStorage((AzureBlobStorageConfigIn)_value);
                    break;
                case ConfigType.OtelTracing:
                    onOtelTracing((OtelTracingConfigIn)_value);
                    break;
                case ConfigType.FifoEndpoint:
                    onFifoEndpoint((FifoEndpointConfigIn)_value);
                    break;
                case ConfigType.AmazonS3:
                    onAmazonS3((S3ConfigIn)_value);
                    break;
                case ConfigType.GoogleCloudStorage:
                    onGoogleCloudStorage((GoogleCloudStorageConfigIn)_value);
                    break;
                case ConfigType.GoogleCloudPubSub:
                    onGoogleCloudPubSub((GoogleCloudPubSubConfigIn)_value);
                    break;
                case ConfigType.Sqs:
                    onSqs((SqsConfigIn)_value);
                    break;
                case ConfigType.Sns:
                    onSns((SnsConfigIn)_value);
                    break;
                case ConfigType.BigQuery:
                    onBigQuery((BigQueryConfigIn)_value);
                    break;
                case ConfigType.Clickhouse:
                    onClickhouse((ClickhouseConfigIn)_value);
                    break;
                case ConfigType.EventBridge:
                    onEventBridge((EventBridgeConfigIn)_value);
                    break;
                case ConfigType.Snowflake:
                    onSnowflake((SnowflakeConfigIn)_value);
                    break;
                case ConfigType.RabbitMq:
                    onRabbitMq((RabbitMqConfigIn)_value);
                    break;
                case ConfigType.Redshift:
                    onRedshift((RedshiftConfigIn)_value);
                    break;
                case ConfigType.Postgres:
                    onPostgres((PostgresConfigIn)_value);
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class DestinationInSurrogate
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

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        [JsonProperty("config", Required = Required.Always)]
        public required JObject Config { get; set; }
    }

    public class DestinationInConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(DestinationIn);
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
                serializer.Deserialize<DestinationInSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to DestinationInSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), DestinationInConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for DestinationInConfig.config"
                );
            }

            DestinationInConfig config = func((surrogate.Config, surrogate.Type));

            return new DestinationIn
            {
                Uid = surrogate.Uid,
                Status = surrogate.Status,
                BatchSize = surrogate.BatchSize,
                MaxWaitSecs = surrogate.MaxWaitSecs,
                EventTypes = surrogate.EventTypes,
                Channels = surrogate.Channels,
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

        private readonly Dictionary<string, Func<(JObject, string), DestinationInConfig>> typeMap =
            new()
            {
                ["pollingEndpoint"] = c => DestinationInConfig.PollingEndpoint(),
                ["azureBlobStorage"] = c =>
                    DestinationInConfig.AzureBlobStorage(ToObj<AzureBlobStorageConfigIn>(c)),
                ["otelTracing"] = c =>
                    DestinationInConfig.OtelTracing(ToObj<OtelTracingConfigIn>(c)),
                ["fifoEndpoint"] = c =>
                    DestinationInConfig.FifoEndpoint(ToObj<FifoEndpointConfigIn>(c)),
                ["amazonS3"] = c => DestinationInConfig.AmazonS3(ToObj<S3ConfigIn>(c)),
                ["googleCloudStorage"] = c =>
                    DestinationInConfig.GoogleCloudStorage(ToObj<GoogleCloudStorageConfigIn>(c)),
                ["googleCloudPubSub"] = c =>
                    DestinationInConfig.GoogleCloudPubSub(ToObj<GoogleCloudPubSubConfigIn>(c)),
                ["sqs"] = c => DestinationInConfig.Sqs(ToObj<SqsConfigIn>(c)),
                ["sns"] = c => DestinationInConfig.Sns(ToObj<SnsConfigIn>(c)),
                ["bigQuery"] = c => DestinationInConfig.BigQuery(ToObj<BigQueryConfigIn>(c)),
                ["clickhouse"] = c => DestinationInConfig.Clickhouse(ToObj<ClickhouseConfigIn>(c)),
                ["eventBridge"] = c =>
                    DestinationInConfig.EventBridge(ToObj<EventBridgeConfigIn>(c)),
                ["snowflake"] = c => DestinationInConfig.Snowflake(ToObj<SnowflakeConfigIn>(c)),
                ["rabbitMq"] = c => DestinationInConfig.RabbitMq(ToObj<RabbitMqConfigIn>(c)),
                ["redshift"] = c => DestinationInConfig.Redshift(ToObj<RedshiftConfigIn>(c)),
                ["postgres"] = c => DestinationInConfig.Postgres(ToObj<PostgresConfigIn>(c)),
            };
    }
}
