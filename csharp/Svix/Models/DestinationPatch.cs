// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    [JsonConverter(typeof(DestinationPatchConverter))]
    public class DestinationPatch
    {
        [JsonProperty("uid")]
        public MaybeUnset<string?> Uid { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUid() => !Uid.IsUnset;

        [JsonProperty("status")]
        public MaybeUnset<DestinationStatusIn?> Status { get; set; } =
            MaybeUnset<DestinationStatusIn?>.Unset();

        public bool ShouldSerializeStatus() => !Status.IsUnset;

        [JsonProperty("batchSize")]
        public MaybeUnset<ushort?> BatchSize { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeBatchSize() => !BatchSize.IsUnset;

        [JsonProperty("maxWaitSecs")]
        public MaybeUnset<ushort?> MaxWaitSecs { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeMaxWaitSecs() => !MaxWaitSecs.IsUnset;

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        public bool ShouldSerializeEventTypes() => EventTypes != null;

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        public bool ShouldSerializeChannels() => Channels != null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        public bool ShouldSerializeMetadata() => Metadata != null;

        [JsonIgnore]
        public required DestinationPatchConfig Config { get; set; }

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

            sb.Append("class DestinationPatch {\n");
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

    public class DestinationPatchConfig
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

        private DestinationPatchConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static DestinationPatchConfig PollingEndpoint() =>
            new(new Dictionary<string, string>(), ConfigType.PollingEndpoint);

        public static DestinationPatchConfig AzureBlobStorage(
            AzureBlobStorageConfigPatch azureBlobStorageConfigPatch
        ) => new(azureBlobStorageConfigPatch, ConfigType.AzureBlobStorage);

        public static DestinationPatchConfig OtelTracing(
            OtelTracingConfigPatch otelTracingConfigPatch
        ) => new(otelTracingConfigPatch, ConfigType.OtelTracing);

        public static DestinationPatchConfig FifoEndpoint(
            SinkHttpConfigPatch sinkHttpConfigPatch
        ) => new(sinkHttpConfigPatch, ConfigType.FifoEndpoint);

        public static DestinationPatchConfig AmazonS3(S3ConfigPatch s3ConfigPatch) =>
            new(s3ConfigPatch, ConfigType.AmazonS3);

        public static DestinationPatchConfig GoogleCloudStorage(
            GoogleCloudStorageConfigPatch googleCloudStorageConfigPatch
        ) => new(googleCloudStorageConfigPatch, ConfigType.GoogleCloudStorage);

        public static DestinationPatchConfig GoogleCloudPubSub(
            GoogleCloudPubSubConfigPatch googleCloudPubSubConfigPatch
        ) => new(googleCloudPubSubConfigPatch, ConfigType.GoogleCloudPubSub);

        public static DestinationPatchConfig Sqs(SqsConfigPatch sqsConfigPatch) =>
            new(sqsConfigPatch, ConfigType.Sqs);

        public static DestinationPatchConfig Sns(SnsConfigPatch snsConfigPatch) =>
            new(snsConfigPatch, ConfigType.Sns);

        public static DestinationPatchConfig BigQuery(BigQueryConfigPatch bigQueryConfigPatch) =>
            new(bigQueryConfigPatch, ConfigType.BigQuery);

        public static DestinationPatchConfig Clickhouse(
            ClickhouseConfigPatch clickhouseConfigPatch
        ) => new(clickhouseConfigPatch, ConfigType.Clickhouse);

        public static DestinationPatchConfig EventBridge(
            EventBridgeConfigPatch eventBridgeConfigPatch
        ) => new(eventBridgeConfigPatch, ConfigType.EventBridge);

        public static DestinationPatchConfig Snowflake(SnowflakeConfigPatch snowflakeConfigPatch) =>
            new(snowflakeConfigPatch, ConfigType.Snowflake);

        public static DestinationPatchConfig RabbitMq(RabbitMqConfigPatch rabbitMqConfigPatch) =>
            new(rabbitMqConfigPatch, ConfigType.RabbitMq);

        public static DestinationPatchConfig Redshift(RedshiftConfigPatch redshiftConfigPatch) =>
            new(redshiftConfigPatch, ConfigType.Redshift);

        public static DestinationPatchConfig Postgres(PostgresConfigPatch postgresConfigPatch) =>
            new(postgresConfigPatch, ConfigType.Postgres);

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
            Func<AzureBlobStorageConfigPatch, TResult> onAzureBlobStorage,
            Func<OtelTracingConfigPatch, TResult> onOtelTracing,
            Func<SinkHttpConfigPatch, TResult> onFifoEndpoint,
            Func<S3ConfigPatch, TResult> onAmazonS3,
            Func<GoogleCloudStorageConfigPatch, TResult> onGoogleCloudStorage,
            Func<GoogleCloudPubSubConfigPatch, TResult> onGoogleCloudPubSub,
            Func<SqsConfigPatch, TResult> onSqs,
            Func<SnsConfigPatch, TResult> onSns,
            Func<BigQueryConfigPatch, TResult> onBigQuery,
            Func<ClickhouseConfigPatch, TResult> onClickhouse,
            Func<EventBridgeConfigPatch, TResult> onEventBridge,
            Func<SnowflakeConfigPatch, TResult> onSnowflake,
            Func<RabbitMqConfigPatch, TResult> onRabbitMq,
            Func<RedshiftConfigPatch, TResult> onRedshift,
            Func<PostgresConfigPatch, TResult> onPostgres
        )
        {
            return _type switch
            {
                ConfigType.PollingEndpoint => onPollingEndpoint(),
                ConfigType.AzureBlobStorage => onAzureBlobStorage(
                    (AzureBlobStorageConfigPatch)_value
                ),
                ConfigType.OtelTracing => onOtelTracing((OtelTracingConfigPatch)_value),
                ConfigType.FifoEndpoint => onFifoEndpoint((SinkHttpConfigPatch)_value),
                ConfigType.AmazonS3 => onAmazonS3((S3ConfigPatch)_value),
                ConfigType.GoogleCloudStorage => onGoogleCloudStorage(
                    (GoogleCloudStorageConfigPatch)_value
                ),
                ConfigType.GoogleCloudPubSub => onGoogleCloudPubSub(
                    (GoogleCloudPubSubConfigPatch)_value
                ),
                ConfigType.Sqs => onSqs((SqsConfigPatch)_value),
                ConfigType.Sns => onSns((SnsConfigPatch)_value),
                ConfigType.BigQuery => onBigQuery((BigQueryConfigPatch)_value),
                ConfigType.Clickhouse => onClickhouse((ClickhouseConfigPatch)_value),
                ConfigType.EventBridge => onEventBridge((EventBridgeConfigPatch)_value),
                ConfigType.Snowflake => onSnowflake((SnowflakeConfigPatch)_value),
                ConfigType.RabbitMq => onRabbitMq((RabbitMqConfigPatch)_value),
                ConfigType.Redshift => onRedshift((RedshiftConfigPatch)_value),
                ConfigType.Postgres => onPostgres((PostgresConfigPatch)_value),
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(
            Action? onPollingEndpoint = null,
            Action<AzureBlobStorageConfigPatch>? onAzureBlobStorage = null,
            Action<OtelTracingConfigPatch>? onOtelTracing = null,
            Action<SinkHttpConfigPatch>? onFifoEndpoint = null,
            Action<S3ConfigPatch>? onAmazonS3 = null,
            Action<GoogleCloudStorageConfigPatch>? onGoogleCloudStorage = null,
            Action<GoogleCloudPubSubConfigPatch>? onGoogleCloudPubSub = null,
            Action<SqsConfigPatch>? onSqs = null,
            Action<SnsConfigPatch>? onSns = null,
            Action<BigQueryConfigPatch>? onBigQuery = null,
            Action<ClickhouseConfigPatch>? onClickhouse = null,
            Action<EventBridgeConfigPatch>? onEventBridge = null,
            Action<SnowflakeConfigPatch>? onSnowflake = null,
            Action<RabbitMqConfigPatch>? onRabbitMq = null,
            Action<RedshiftConfigPatch>? onRedshift = null,
            Action<PostgresConfigPatch>? onPostgres = null
        )
        {
            switch (_type)
            {
                case ConfigType.PollingEndpoint:
                    if (onPollingEndpoint != null)
                    {
                        onPollingEndpoint();
                    }
                    break;
                case ConfigType.AzureBlobStorage:
                    if (onAzureBlobStorage != null)
                    {
                        onAzureBlobStorage((AzureBlobStorageConfigPatch)_value);
                    }
                    break;
                case ConfigType.OtelTracing:
                    if (onOtelTracing != null)
                    {
                        onOtelTracing((OtelTracingConfigPatch)_value);
                    }
                    break;
                case ConfigType.FifoEndpoint:
                    if (onFifoEndpoint != null)
                    {
                        onFifoEndpoint((SinkHttpConfigPatch)_value);
                    }
                    break;
                case ConfigType.AmazonS3:
                    if (onAmazonS3 != null)
                    {
                        onAmazonS3((S3ConfigPatch)_value);
                    }
                    break;
                case ConfigType.GoogleCloudStorage:
                    if (onGoogleCloudStorage != null)
                    {
                        onGoogleCloudStorage((GoogleCloudStorageConfigPatch)_value);
                    }
                    break;
                case ConfigType.GoogleCloudPubSub:
                    if (onGoogleCloudPubSub != null)
                    {
                        onGoogleCloudPubSub((GoogleCloudPubSubConfigPatch)_value);
                    }
                    break;
                case ConfigType.Sqs:
                    if (onSqs != null)
                    {
                        onSqs((SqsConfigPatch)_value);
                    }
                    break;
                case ConfigType.Sns:
                    if (onSns != null)
                    {
                        onSns((SnsConfigPatch)_value);
                    }
                    break;
                case ConfigType.BigQuery:
                    if (onBigQuery != null)
                    {
                        onBigQuery((BigQueryConfigPatch)_value);
                    }
                    break;
                case ConfigType.Clickhouse:
                    if (onClickhouse != null)
                    {
                        onClickhouse((ClickhouseConfigPatch)_value);
                    }
                    break;
                case ConfigType.EventBridge:
                    if (onEventBridge != null)
                    {
                        onEventBridge((EventBridgeConfigPatch)_value);
                    }
                    break;
                case ConfigType.Snowflake:
                    if (onSnowflake != null)
                    {
                        onSnowflake((SnowflakeConfigPatch)_value);
                    }
                    break;
                case ConfigType.RabbitMq:
                    if (onRabbitMq != null)
                    {
                        onRabbitMq((RabbitMqConfigPatch)_value);
                    }
                    break;
                case ConfigType.Redshift:
                    if (onRedshift != null)
                    {
                        onRedshift((RedshiftConfigPatch)_value);
                    }
                    break;
                case ConfigType.Postgres:
                    if (onPostgres != null)
                    {
                        onPostgres((PostgresConfigPatch)_value);
                    }
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class DestinationPatchSurrogate
    {
        [JsonProperty("uid")]
        public MaybeUnset<string?> Uid { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUid() => !Uid.IsUnset;

        [JsonProperty("status")]
        public MaybeUnset<DestinationStatusIn?> Status { get; set; } =
            MaybeUnset<DestinationStatusIn?>.Unset();

        public bool ShouldSerializeStatus() => !Status.IsUnset;

        [JsonProperty("batchSize")]
        public MaybeUnset<ushort?> BatchSize { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeBatchSize() => !BatchSize.IsUnset;

        [JsonProperty("maxWaitSecs")]
        public MaybeUnset<ushort?> MaxWaitSecs { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeMaxWaitSecs() => !MaxWaitSecs.IsUnset;

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        public bool ShouldSerializeEventTypes() => EventTypes != null;

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        public bool ShouldSerializeChannels() => Channels != null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        public bool ShouldSerializeMetadata() => Metadata != null;

        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        [JsonProperty("config")]
        public JObject? Config { get; set; }
    }

    public class DestinationPatchConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(DestinationPatch);
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
                serializer.Deserialize<DestinationPatchSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to DestinationPatchSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), DestinationPatchConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for DestinationPatchConfig.config"
                );
            }

            DestinationPatchConfig config = func((surrogate.Config, surrogate.Type));

            return new DestinationPatch
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

        private readonly Dictionary<
            string,
            Func<(JObject, string), DestinationPatchConfig>
        > typeMap = new()
        {
            ["pollingEndpoint"] = c => DestinationPatchConfig.PollingEndpoint(),
            ["azureBlobStorage"] = c =>
                DestinationPatchConfig.AzureBlobStorage(ToObj<AzureBlobStorageConfigPatch>(c)),
            ["otelTracing"] = c =>
                DestinationPatchConfig.OtelTracing(ToObj<OtelTracingConfigPatch>(c)),
            ["fifoEndpoint"] = c =>
                DestinationPatchConfig.FifoEndpoint(ToObj<SinkHttpConfigPatch>(c)),
            ["amazonS3"] = c => DestinationPatchConfig.AmazonS3(ToObj<S3ConfigPatch>(c)),
            ["googleCloudStorage"] = c =>
                DestinationPatchConfig.GoogleCloudStorage(ToObj<GoogleCloudStorageConfigPatch>(c)),
            ["googleCloudPubSub"] = c =>
                DestinationPatchConfig.GoogleCloudPubSub(ToObj<GoogleCloudPubSubConfigPatch>(c)),
            ["sqs"] = c => DestinationPatchConfig.Sqs(ToObj<SqsConfigPatch>(c)),
            ["sns"] = c => DestinationPatchConfig.Sns(ToObj<SnsConfigPatch>(c)),
            ["bigQuery"] = c => DestinationPatchConfig.BigQuery(ToObj<BigQueryConfigPatch>(c)),
            ["clickhouse"] = c =>
                DestinationPatchConfig.Clickhouse(ToObj<ClickhouseConfigPatch>(c)),
            ["eventBridge"] = c =>
                DestinationPatchConfig.EventBridge(ToObj<EventBridgeConfigPatch>(c)),
            ["snowflake"] = c => DestinationPatchConfig.Snowflake(ToObj<SnowflakeConfigPatch>(c)),
            ["rabbitMq"] = c => DestinationPatchConfig.RabbitMq(ToObj<RabbitMqConfigPatch>(c)),
            ["redshift"] = c => DestinationPatchConfig.Redshift(ToObj<RedshiftConfigPatch>(c)),
            ["postgres"] = c => DestinationPatchConfig.Postgres(ToObj<PostgresConfigPatch>(c)),
        };
    }
}
