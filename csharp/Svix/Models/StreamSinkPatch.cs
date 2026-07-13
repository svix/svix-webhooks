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
            AzureBlobStoragePatchConfig azureBlobStoragePatchConfig
        ) => new(azureBlobStoragePatchConfig, ConfigType.AzureBlobStorage);

        public static StreamSinkPatchConfig OtelTracing(
            OtelTracingPatchConfig otelTracingPatchConfig
        ) => new(otelTracingPatchConfig, ConfigType.OtelTracing);

        public static StreamSinkPatchConfig Http(HttpPatchConfig httpPatchConfig) =>
            new(httpPatchConfig, ConfigType.Http);

        public static StreamSinkPatchConfig AmazonS3(AmazonS3PatchConfig amazonS3PatchConfig) =>
            new(amazonS3PatchConfig, ConfigType.AmazonS3);

        public static StreamSinkPatchConfig GoogleCloudStorage(
            GoogleCloudStoragePatchConfig googleCloudStoragePatchConfig
        ) => new(googleCloudStoragePatchConfig, ConfigType.GoogleCloudStorage);

        public static StreamSinkPatchConfig GoogleCloudPubSub(
            GoogleCloudPubSubPatchConfig googleCloudPubSubPatchConfig
        ) => new(googleCloudPubSubPatchConfig, ConfigType.GoogleCloudPubSub);

        public static StreamSinkPatchConfig Sqs(SqsPatchConfig sqsPatchConfig) =>
            new(sqsPatchConfig, ConfigType.Sqs);

        public static StreamSinkPatchConfig Sns(SnsPatchConfig snsPatchConfig) =>
            new(snsPatchConfig, ConfigType.Sns);

        public static StreamSinkPatchConfig BigQuery(BigQueryPatchConfig bigQueryPatchConfig) =>
            new(bigQueryPatchConfig, ConfigType.BigQuery);

        public static StreamSinkPatchConfig Clickhouse(
            ClickhousePatchConfig clickhousePatchConfig
        ) => new(clickhousePatchConfig, ConfigType.Clickhouse);

        public static StreamSinkPatchConfig EventBridge(
            EventBridgePatchConfig eventBridgePatchConfig
        ) => new(eventBridgePatchConfig, ConfigType.EventBridge);

        public static StreamSinkPatchConfig Snowflake(SnowflakePatchConfig snowflakePatchConfig) =>
            new(snowflakePatchConfig, ConfigType.Snowflake);

        public static StreamSinkPatchConfig RabbitMq(RabbitMqPatchConfig rabbitMqPatchConfig) =>
            new(rabbitMqPatchConfig, ConfigType.RabbitMq);

        public static StreamSinkPatchConfig Redshift(RedshiftPatchConfig redshiftPatchConfig) =>
            new(redshiftPatchConfig, ConfigType.Redshift);

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
            Func<AzureBlobStoragePatchConfig, TResult> onAzureBlobStorage,
            Func<OtelTracingPatchConfig, TResult> onOtelTracing,
            Func<HttpPatchConfig, TResult> onHttp,
            Func<AmazonS3PatchConfig, TResult> onAmazonS3,
            Func<GoogleCloudStoragePatchConfig, TResult> onGoogleCloudStorage,
            Func<GoogleCloudPubSubPatchConfig, TResult> onGoogleCloudPubSub,
            Func<SqsPatchConfig, TResult> onSqs,
            Func<SnsPatchConfig, TResult> onSns,
            Func<BigQueryPatchConfig, TResult> onBigQuery,
            Func<ClickhousePatchConfig, TResult> onClickhouse,
            Func<EventBridgePatchConfig, TResult> onEventBridge,
            Func<SnowflakePatchConfig, TResult> onSnowflake,
            Func<RabbitMqPatchConfig, TResult> onRabbitMq,
            Func<RedshiftPatchConfig, TResult> onRedshift
        )
        {
            return _type switch
            {
                ConfigType.Poller => onPoller(),
                ConfigType.AzureBlobStorage => onAzureBlobStorage(
                    (AzureBlobStoragePatchConfig)_value
                ),
                ConfigType.OtelTracing => onOtelTracing((OtelTracingPatchConfig)_value),
                ConfigType.Http => onHttp((HttpPatchConfig)_value),
                ConfigType.AmazonS3 => onAmazonS3((AmazonS3PatchConfig)_value),
                ConfigType.GoogleCloudStorage => onGoogleCloudStorage(
                    (GoogleCloudStoragePatchConfig)_value
                ),
                ConfigType.GoogleCloudPubSub => onGoogleCloudPubSub(
                    (GoogleCloudPubSubPatchConfig)_value
                ),
                ConfigType.Sqs => onSqs((SqsPatchConfig)_value),
                ConfigType.Sns => onSns((SnsPatchConfig)_value),
                ConfigType.BigQuery => onBigQuery((BigQueryPatchConfig)_value),
                ConfigType.Clickhouse => onClickhouse((ClickhousePatchConfig)_value),
                ConfigType.EventBridge => onEventBridge((EventBridgePatchConfig)_value),
                ConfigType.Snowflake => onSnowflake((SnowflakePatchConfig)_value),
                ConfigType.RabbitMq => onRabbitMq((RabbitMqPatchConfig)_value),
                ConfigType.Redshift => onRedshift((RedshiftPatchConfig)_value),
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(
            Action onPoller,
            Action<AzureBlobStoragePatchConfig> onAzureBlobStorage,
            Action<OtelTracingPatchConfig> onOtelTracing,
            Action<HttpPatchConfig> onHttp,
            Action<AmazonS3PatchConfig> onAmazonS3,
            Action<GoogleCloudStoragePatchConfig> onGoogleCloudStorage,
            Action<GoogleCloudPubSubPatchConfig> onGoogleCloudPubSub,
            Action<SqsPatchConfig> onSqs,
            Action<SnsPatchConfig> onSns,
            Action<BigQueryPatchConfig> onBigQuery,
            Action<ClickhousePatchConfig> onClickhouse,
            Action<EventBridgePatchConfig> onEventBridge,
            Action<SnowflakePatchConfig> onSnowflake,
            Action<RabbitMqPatchConfig> onRabbitMq,
            Action<RedshiftPatchConfig> onRedshift
        )
        {
            switch (_type)
            {
                case ConfigType.Poller:
                    onPoller();
                    break;
                case ConfigType.AzureBlobStorage:
                    onAzureBlobStorage((AzureBlobStoragePatchConfig)_value);
                    break;
                case ConfigType.OtelTracing:
                    onOtelTracing((OtelTracingPatchConfig)_value);
                    break;
                case ConfigType.Http:
                    onHttp((HttpPatchConfig)_value);
                    break;
                case ConfigType.AmazonS3:
                    onAmazonS3((AmazonS3PatchConfig)_value);
                    break;
                case ConfigType.GoogleCloudStorage:
                    onGoogleCloudStorage((GoogleCloudStoragePatchConfig)_value);
                    break;
                case ConfigType.GoogleCloudPubSub:
                    onGoogleCloudPubSub((GoogleCloudPubSubPatchConfig)_value);
                    break;
                case ConfigType.Sqs:
                    onSqs((SqsPatchConfig)_value);
                    break;
                case ConfigType.Sns:
                    onSns((SnsPatchConfig)_value);
                    break;
                case ConfigType.BigQuery:
                    onBigQuery((BigQueryPatchConfig)_value);
                    break;
                case ConfigType.Clickhouse:
                    onClickhouse((ClickhousePatchConfig)_value);
                    break;
                case ConfigType.EventBridge:
                    onEventBridge((EventBridgePatchConfig)_value);
                    break;
                case ConfigType.Snowflake:
                    onSnowflake((SnowflakePatchConfig)_value);
                    break;
                case ConfigType.RabbitMq:
                    onRabbitMq((RabbitMqPatchConfig)_value);
                    break;
                case ConfigType.Redshift:
                    onRedshift((RedshiftPatchConfig)_value);
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
                StreamSinkPatchConfig.AzureBlobStorage(ToObj<AzureBlobStoragePatchConfig>(c)),
            ["otelTracing"] = c =>
                StreamSinkPatchConfig.OtelTracing(ToObj<OtelTracingPatchConfig>(c)),
            ["http"] = c => StreamSinkPatchConfig.Http(ToObj<HttpPatchConfig>(c)),
            ["amazonS3"] = c => StreamSinkPatchConfig.AmazonS3(ToObj<AmazonS3PatchConfig>(c)),
            ["googleCloudStorage"] = c =>
                StreamSinkPatchConfig.GoogleCloudStorage(ToObj<GoogleCloudStoragePatchConfig>(c)),
            ["googleCloudPubSub"] = c =>
                StreamSinkPatchConfig.GoogleCloudPubSub(ToObj<GoogleCloudPubSubPatchConfig>(c)),
            ["sqs"] = c => StreamSinkPatchConfig.Sqs(ToObj<SqsPatchConfig>(c)),
            ["sns"] = c => StreamSinkPatchConfig.Sns(ToObj<SnsPatchConfig>(c)),
            ["bigQuery"] = c => StreamSinkPatchConfig.BigQuery(ToObj<BigQueryPatchConfig>(c)),
            ["clickhouse"] = c => StreamSinkPatchConfig.Clickhouse(ToObj<ClickhousePatchConfig>(c)),
            ["eventBridge"] = c =>
                StreamSinkPatchConfig.EventBridge(ToObj<EventBridgePatchConfig>(c)),
            ["snowflake"] = c => StreamSinkPatchConfig.Snowflake(ToObj<SnowflakePatchConfig>(c)),
            ["rabbitMq"] = c => StreamSinkPatchConfig.RabbitMq(ToObj<RabbitMqPatchConfig>(c)),
            ["redshift"] = c => StreamSinkPatchConfig.Redshift(ToObj<RedshiftPatchConfig>(c)),
        };
    }
}
