// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    [JsonConverter(typeof(StreamSinkPatchConverter))]
    public class StreamSinkPatch
    {
        [JsonProperty("uid")]
        public MaybeUnset<string?> Uid { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUid() => !Uid.IsUnset;

        [JsonProperty("status")]
        public MaybeUnset<SinkStatusIn?> Status { get; set; } = MaybeUnset<SinkStatusIn?>.Unset();

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

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        public bool ShouldSerializeMetadata() => Metadata != null;

        [JsonIgnore]
        public required StreamSinkPatchConfig Config { get; set; }

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

            sb.Append("class StreamSinkPatch {\n");
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

    public class StreamSinkPatchConfig
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

        private StreamSinkPatchConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static StreamSinkPatchConfig Poller() =>
            new(new Dictionary<string, string>(), ConfigType.Poller);

        public static StreamSinkPatchConfig AzureBlobStorage(
            AzureBlobStorageConfigPatch azureBlobStorageConfigPatch
        ) => new(azureBlobStorageConfigPatch, ConfigType.AzureBlobStorage);

        public static StreamSinkPatchConfig OtelTracing(
            SinkOtelTracingConfigPatch sinkOtelTracingConfigPatch
        ) => new(sinkOtelTracingConfigPatch, ConfigType.OtelTracing);

        public static StreamSinkPatchConfig Http(SinkHttpConfigPatch sinkHttpConfigPatch) =>
            new(sinkHttpConfigPatch, ConfigType.Http);

        public static StreamSinkPatchConfig AmazonS3(S3ConfigPatch s3ConfigPatch) =>
            new(s3ConfigPatch, ConfigType.AmazonS3);

        public static StreamSinkPatchConfig GoogleCloudStorage(
            GoogleCloudStorageConfigPatch googleCloudStorageConfigPatch
        ) => new(googleCloudStorageConfigPatch, ConfigType.GoogleCloudStorage);

        public static StreamSinkPatchConfig GoogleCloudPubSub(
            GoogleCloudPubSubConfigPatch googleCloudPubSubConfigPatch
        ) => new(googleCloudPubSubConfigPatch, ConfigType.GoogleCloudPubSub);

        public static StreamSinkPatchConfig Sqs(SqsConfigPatch sqsConfigPatch) =>
            new(sqsConfigPatch, ConfigType.Sqs);

        public static StreamSinkPatchConfig Sns(SnsConfigPatch snsConfigPatch) =>
            new(snsConfigPatch, ConfigType.Sns);

        public static StreamSinkPatchConfig BigQuery(BigQueryConfigPatch bigQueryConfigPatch) =>
            new(bigQueryConfigPatch, ConfigType.BigQuery);

        public static StreamSinkPatchConfig Clickhouse(
            ClickhouseConfigPatch clickhouseConfigPatch
        ) => new(clickhouseConfigPatch, ConfigType.Clickhouse);

        public static StreamSinkPatchConfig EventBridge(
            EventBridgeConfigPatch eventBridgeConfigPatch
        ) => new(eventBridgeConfigPatch, ConfigType.EventBridge);

        public static StreamSinkPatchConfig Snowflake(SnowflakeConfigPatch snowflakeConfigPatch) =>
            new(snowflakeConfigPatch, ConfigType.Snowflake);

        public static StreamSinkPatchConfig RabbitMq(RabbitMqConfigPatch rabbitMqConfigPatch) =>
            new(rabbitMqConfigPatch, ConfigType.RabbitMq);

        public static StreamSinkPatchConfig Redshift(RedshiftConfigPatch redshiftConfigPatch) =>
            new(redshiftConfigPatch, ConfigType.Redshift);

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
            Func<AzureBlobStorageConfigPatch, TResult> onAzureBlobStorage,
            Func<SinkOtelTracingConfigPatch, TResult> onOtelTracing,
            Func<SinkHttpConfigPatch, TResult> onHttp,
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
            Func<RedshiftConfigPatch, TResult> onRedshift
        )
        {
            return _type switch
            {
                ConfigType.Poller => onPoller(),
                ConfigType.AzureBlobStorage => onAzureBlobStorage(
                    (AzureBlobStorageConfigPatch)_value
                ),
                ConfigType.OtelTracing => onOtelTracing((SinkOtelTracingConfigPatch)_value),
                ConfigType.Http => onHttp((SinkHttpConfigPatch)_value),
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
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(
            Action onPoller,
            Action<AzureBlobStorageConfigPatch> onAzureBlobStorage,
            Action<SinkOtelTracingConfigPatch> onOtelTracing,
            Action<SinkHttpConfigPatch> onHttp,
            Action<S3ConfigPatch> onAmazonS3,
            Action<GoogleCloudStorageConfigPatch> onGoogleCloudStorage,
            Action<GoogleCloudPubSubConfigPatch> onGoogleCloudPubSub,
            Action<SqsConfigPatch> onSqs,
            Action<SnsConfigPatch> onSns,
            Action<BigQueryConfigPatch> onBigQuery,
            Action<ClickhouseConfigPatch> onClickhouse,
            Action<EventBridgeConfigPatch> onEventBridge,
            Action<SnowflakeConfigPatch> onSnowflake,
            Action<RabbitMqConfigPatch> onRabbitMq,
            Action<RedshiftConfigPatch> onRedshift
        )
        {
            switch (_type)
            {
                case ConfigType.Poller:
                    onPoller();
                    break;
                case ConfigType.AzureBlobStorage:
                    onAzureBlobStorage((AzureBlobStorageConfigPatch)_value);
                    break;
                case ConfigType.OtelTracing:
                    onOtelTracing((SinkOtelTracingConfigPatch)_value);
                    break;
                case ConfigType.Http:
                    onHttp((SinkHttpConfigPatch)_value);
                    break;
                case ConfigType.AmazonS3:
                    onAmazonS3((S3ConfigPatch)_value);
                    break;
                case ConfigType.GoogleCloudStorage:
                    onGoogleCloudStorage((GoogleCloudStorageConfigPatch)_value);
                    break;
                case ConfigType.GoogleCloudPubSub:
                    onGoogleCloudPubSub((GoogleCloudPubSubConfigPatch)_value);
                    break;
                case ConfigType.Sqs:
                    onSqs((SqsConfigPatch)_value);
                    break;
                case ConfigType.Sns:
                    onSns((SnsConfigPatch)_value);
                    break;
                case ConfigType.BigQuery:
                    onBigQuery((BigQueryConfigPatch)_value);
                    break;
                case ConfigType.Clickhouse:
                    onClickhouse((ClickhouseConfigPatch)_value);
                    break;
                case ConfigType.EventBridge:
                    onEventBridge((EventBridgeConfigPatch)_value);
                    break;
                case ConfigType.Snowflake:
                    onSnowflake((SnowflakeConfigPatch)_value);
                    break;
                case ConfigType.RabbitMq:
                    onRabbitMq((RabbitMqConfigPatch)_value);
                    break;
                case ConfigType.Redshift:
                    onRedshift((RedshiftConfigPatch)_value);
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class StreamSinkPatchSurrogate
    {
        [JsonProperty("uid")]
        public MaybeUnset<string?> Uid { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUid() => !Uid.IsUnset;

        [JsonProperty("status")]
        public MaybeUnset<SinkStatusIn?> Status { get; set; } = MaybeUnset<SinkStatusIn?>.Unset();

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

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        public bool ShouldSerializeMetadata() => Metadata != null;

        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        [JsonProperty("config", Required = Required.Always)]
        public required JObject Config { get; set; }
    }

    public class StreamSinkPatchConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(StreamSinkPatch);
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
                serializer.Deserialize<StreamSinkPatchSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to StreamSinkPatchSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), StreamSinkPatchConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for StreamSinkPatchConfig.config"
                );
            }

            StreamSinkPatchConfig config = func((surrogate.Config, surrogate.Type));

            return new StreamSinkPatch
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

        private readonly Dictionary<
            string,
            Func<(JObject, string), StreamSinkPatchConfig>
        > typeMap = new()
        {
            ["poller"] = c => StreamSinkPatchConfig.Poller(),
            ["azureBlobStorage"] = c =>
                StreamSinkPatchConfig.AzureBlobStorage(ToObj<AzureBlobStorageConfigPatch>(c)),
            ["otelTracing"] = c =>
                StreamSinkPatchConfig.OtelTracing(ToObj<SinkOtelTracingConfigPatch>(c)),
            ["http"] = c => StreamSinkPatchConfig.Http(ToObj<SinkHttpConfigPatch>(c)),
            ["amazonS3"] = c => StreamSinkPatchConfig.AmazonS3(ToObj<S3ConfigPatch>(c)),
            ["googleCloudStorage"] = c =>
                StreamSinkPatchConfig.GoogleCloudStorage(ToObj<GoogleCloudStorageConfigPatch>(c)),
            ["googleCloudPubSub"] = c =>
                StreamSinkPatchConfig.GoogleCloudPubSub(ToObj<GoogleCloudPubSubConfigPatch>(c)),
            ["sqs"] = c => StreamSinkPatchConfig.Sqs(ToObj<SqsConfigPatch>(c)),
            ["sns"] = c => StreamSinkPatchConfig.Sns(ToObj<SnsConfigPatch>(c)),
            ["bigQuery"] = c => StreamSinkPatchConfig.BigQuery(ToObj<BigQueryConfigPatch>(c)),
            ["clickhouse"] = c => StreamSinkPatchConfig.Clickhouse(ToObj<ClickhouseConfigPatch>(c)),
            ["eventBridge"] = c =>
                StreamSinkPatchConfig.EventBridge(ToObj<EventBridgeConfigPatch>(c)),
            ["snowflake"] = c => StreamSinkPatchConfig.Snowflake(ToObj<SnowflakeConfigPatch>(c)),
            ["rabbitMq"] = c => StreamSinkPatchConfig.RabbitMq(ToObj<RabbitMqConfigPatch>(c)),
            ["redshift"] = c => StreamSinkPatchConfig.Redshift(ToObj<RedshiftConfigPatch>(c)),
        };
    }
}
