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

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

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
            sb.Append("  Channels: ").Append(Channels).Append('\n');
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
            AzureBlobStorageConfigIn azureBlobStorageConfigIn
        ) => new(azureBlobStorageConfigIn, ConfigType.AzureBlobStorage);

        public static StreamSinkInConfig OtelTracing(OtelTracingConfigIn otelTracingConfigIn) =>
            new(otelTracingConfigIn, ConfigType.OtelTracing);

        public static StreamSinkInConfig Http(SinkHttpConfigIn sinkHttpConfigIn) =>
            new(sinkHttpConfigIn, ConfigType.Http);

        public static StreamSinkInConfig AmazonS3(S3ConfigIn s3ConfigIn) =>
            new(s3ConfigIn, ConfigType.AmazonS3);

        public static StreamSinkInConfig GoogleCloudStorage(
            GoogleCloudStorageConfigIn googleCloudStorageConfigIn
        ) => new(googleCloudStorageConfigIn, ConfigType.GoogleCloudStorage);

        public static StreamSinkInConfig GoogleCloudPubSub(
            GoogleCloudPubSubConfigIn googleCloudPubSubConfigIn
        ) => new(googleCloudPubSubConfigIn, ConfigType.GoogleCloudPubSub);

        public static StreamSinkInConfig Sqs(SqsConfigIn sqsConfigIn) =>
            new(sqsConfigIn, ConfigType.Sqs);

        public static StreamSinkInConfig Sns(SnsConfigIn snsConfigIn) =>
            new(snsConfigIn, ConfigType.Sns);

        public static StreamSinkInConfig BigQuery(BigQueryConfigIn bigQueryConfigIn) =>
            new(bigQueryConfigIn, ConfigType.BigQuery);

        public static StreamSinkInConfig Clickhouse(ClickhouseConfigIn clickhouseConfigIn) =>
            new(clickhouseConfigIn, ConfigType.Clickhouse);

        public static StreamSinkInConfig EventBridge(EventBridgeConfigIn eventBridgeConfigIn) =>
            new(eventBridgeConfigIn, ConfigType.EventBridge);

        public static StreamSinkInConfig Snowflake(SnowflakeConfigIn snowflakeConfigIn) =>
            new(snowflakeConfigIn, ConfigType.Snowflake);

        public static StreamSinkInConfig RabbitMq(RabbitMqConfigIn rabbitMqConfigIn) =>
            new(rabbitMqConfigIn, ConfigType.RabbitMq);

        public static StreamSinkInConfig Redshift(RedshiftConfigIn redshiftConfigIn) =>
            new(redshiftConfigIn, ConfigType.Redshift);

        public static StreamSinkInConfig Postgres(PostgresConfigIn postgresConfigIn) =>
            new(postgresConfigIn, ConfigType.Postgres);

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

            [EnumMember(Value = "postgres")]
            Postgres,
        }

        public TResult Match<TResult>(
            Func<TResult> onPoller,
            Func<AzureBlobStorageConfigIn, TResult> onAzureBlobStorage,
            Func<OtelTracingConfigIn, TResult> onOtelTracing,
            Func<SinkHttpConfigIn, TResult> onHttp,
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
                ConfigType.Poller => onPoller(),
                ConfigType.AzureBlobStorage => onAzureBlobStorage((AzureBlobStorageConfigIn)_value),
                ConfigType.OtelTracing => onOtelTracing((OtelTracingConfigIn)_value),
                ConfigType.Http => onHttp((SinkHttpConfigIn)_value),
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
            Action onPoller,
            Action<AzureBlobStorageConfigIn> onAzureBlobStorage,
            Action<OtelTracingConfigIn> onOtelTracing,
            Action<SinkHttpConfigIn> onHttp,
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
                case ConfigType.Poller:
                    onPoller();
                    break;
                case ConfigType.AzureBlobStorage:
                    onAzureBlobStorage((AzureBlobStorageConfigIn)_value);
                    break;
                case ConfigType.OtelTracing:
                    onOtelTracing((OtelTracingConfigIn)_value);
                    break;
                case ConfigType.Http:
                    onHttp((SinkHttpConfigIn)_value);
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

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

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

        private readonly Dictionary<string, Func<(JObject, string), StreamSinkInConfig>> typeMap =
            new()
            {
                ["poller"] = c => StreamSinkInConfig.Poller(),
                ["azureBlobStorage"] = c =>
                    StreamSinkInConfig.AzureBlobStorage(ToObj<AzureBlobStorageConfigIn>(c)),
                ["otelTracing"] = c =>
                    StreamSinkInConfig.OtelTracing(ToObj<OtelTracingConfigIn>(c)),
                ["http"] = c => StreamSinkInConfig.Http(ToObj<SinkHttpConfigIn>(c)),
                ["amazonS3"] = c => StreamSinkInConfig.AmazonS3(ToObj<S3ConfigIn>(c)),
                ["googleCloudStorage"] = c =>
                    StreamSinkInConfig.GoogleCloudStorage(ToObj<GoogleCloudStorageConfigIn>(c)),
                ["googleCloudPubSub"] = c =>
                    StreamSinkInConfig.GoogleCloudPubSub(ToObj<GoogleCloudPubSubConfigIn>(c)),
                ["sqs"] = c => StreamSinkInConfig.Sqs(ToObj<SqsConfigIn>(c)),
                ["sns"] = c => StreamSinkInConfig.Sns(ToObj<SnsConfigIn>(c)),
                ["bigQuery"] = c => StreamSinkInConfig.BigQuery(ToObj<BigQueryConfigIn>(c)),
                ["clickhouse"] = c => StreamSinkInConfig.Clickhouse(ToObj<ClickhouseConfigIn>(c)),
                ["eventBridge"] = c =>
                    StreamSinkInConfig.EventBridge(ToObj<EventBridgeConfigIn>(c)),
                ["snowflake"] = c => StreamSinkInConfig.Snowflake(ToObj<SnowflakeConfigIn>(c)),
                ["rabbitMq"] = c => StreamSinkInConfig.RabbitMq(ToObj<RabbitMqConfigIn>(c)),
                ["redshift"] = c => StreamSinkInConfig.Redshift(ToObj<RedshiftConfigIn>(c)),
                ["postgres"] = c => StreamSinkInConfig.Postgres(ToObj<PostgresConfigIn>(c)),
            };
    }
}
