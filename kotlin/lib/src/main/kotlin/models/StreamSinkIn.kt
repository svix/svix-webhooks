// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.buildJsonObject

@Serializable(with = StreamSinkInSerializer::class)
data class StreamSinkIn(
    /** An optional unique identifier for the sink. */
    val uid: String? = null,
    /**
     * Whether the sink will receive events.
     *
     * If the sink is `enabled`, any events posted to the stream will be dispatched to the Sink in
     * the same order that events were posted to the stream.
     *
     * If the sink is `disabled`, events will not be dispatched to the sink until the sink is
     * reenabled.
     */
    val status: SinkStatusIn? = null,
    /** How many events will be batched in a request to the Sink. */
    val batchSize: UShort? = null,
    /**
     * How long to wait before a batch of events is sent, if the `batchSize` is not reached.
     *
     * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request after
     * 10 seconds or 100 events, whichever comes first.
     *
     * Note that we will never send an empty batch of events to the Sink.
     */
    val maxWaitSecs: UShort? = null,
    /**
     * A list of event types that filter which events are dispatched to the Sink. An empty list (or
     * null) will not filter out any events.
     */
    val eventTypes: List<String>? = null,
    val metadata: Map<String, String>? = null,
    val config: StreamSinkInConfig,
)

@Serializable
sealed class StreamSinkInConfig {
    val variantName: String
        get() = this::class.annotations.filterIsInstance<VariantName>().first().name

    abstract fun toJsonElement(): JsonElement

    @VariantName("poller")
    data object Poller : StreamSinkInConfig() {
        override fun toJsonElement() = buildJsonObject {}
    }

    @VariantName("azureBlobStorage")
    data class AzureBlobStorage(val azureBlobStorage: AzureBlobStorageConfig) :
        StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AzureBlobStorageConfig.serializer(), azureBlobStorage)
    }

    @VariantName("otelTracing")
    data class OtelTracing(val otelTracing: SinkOtelV1Config) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SinkOtelV1Config.serializer(), otelTracing)
    }

    @VariantName("http")
    data class Http(val http: SinkHttpConfig) : StreamSinkInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SinkHttpConfig.serializer(), http)
    }

    @VariantName("amazonS3")
    data class AmazonS3(val amazonS3: S3Config) : StreamSinkInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(S3Config.serializer(), amazonS3)
    }

    @VariantName("googleCloudStorage")
    data class GoogleCloudStorage(val googleCloudStorage: GoogleCloudStorageConfig) :
        StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudStorageConfig.serializer(), googleCloudStorage)
    }

    @VariantName("googleCloudPubSub")
    data class GoogleCloudPubSub(val googleCloudPubSub: GoogleCloudPubSubConfig) :
        StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudPubSubConfig.serializer(), googleCloudPubSub)
    }

    @VariantName("sqs")
    data class Sqs(val sqs: SqsConfig) : StreamSinkInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SqsConfig.serializer(), sqs)
    }

    @VariantName("sns")
    data class Sns(val sns: SnsConfig) : StreamSinkInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SnsConfig.serializer(), sns)
    }

    @VariantName("bigQuery")
    data class BigQuery(val bigQuery: BigQueryConfig) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(BigQueryConfig.serializer(), bigQuery)
    }

    @VariantName("clickhouse")
    data class Clickhouse(val clickhouse: ClickhouseConfig) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(ClickhouseConfig.serializer(), clickhouse)
    }

    @VariantName("eventBridge")
    data class EventBridge(val eventBridge: EventBridgeConfig) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(EventBridgeConfig.serializer(), eventBridge)
    }

    @VariantName("snowflake")
    data class Snowflake(val snowflake: SnowflakeConfig) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SnowflakeConfig.serializer(), snowflake)
    }

    @VariantName("rabbitMq")
    data class RabbitMq(val rabbitMq: RabbitMqConfig) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RabbitMqConfig.serializer(), rabbitMq)
    }

    @VariantName("redshift")
    data class Redshift(val redshift: RedshiftConfig) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RedshiftConfig.serializer(), redshift)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> StreamSinkInConfig>(
                "poller" to { _ -> Poller },
                "azureBlobStorage" to
                    { config ->
                        AzureBlobStorage(
                            Json.decodeFromJsonElement(AzureBlobStorageConfig.serializer(), config)
                        )
                    },
                "otelTracing" to
                    { config ->
                        OtelTracing(
                            Json.decodeFromJsonElement(SinkOtelV1Config.serializer(), config)
                        )
                    },
                "http" to
                    { config ->
                        Http(Json.decodeFromJsonElement(SinkHttpConfig.serializer(), config))
                    },
                "amazonS3" to
                    { config ->
                        AmazonS3(Json.decodeFromJsonElement(S3Config.serializer(), config))
                    },
                "googleCloudStorage" to
                    { config ->
                        GoogleCloudStorage(
                            Json.decodeFromJsonElement(
                                GoogleCloudStorageConfig.serializer(),
                                config,
                            )
                        )
                    },
                "googleCloudPubSub" to
                    { config ->
                        GoogleCloudPubSub(
                            Json.decodeFromJsonElement(GoogleCloudPubSubConfig.serializer(), config)
                        )
                    },
                "sqs" to
                    { config ->
                        Sqs(Json.decodeFromJsonElement(SqsConfig.serializer(), config))
                    },
                "sns" to
                    { config ->
                        Sns(Json.decodeFromJsonElement(SnsConfig.serializer(), config))
                    },
                "bigQuery" to
                    { config ->
                        BigQuery(Json.decodeFromJsonElement(BigQueryConfig.serializer(), config))
                    },
                "clickhouse" to
                    { config ->
                        Clickhouse(
                            Json.decodeFromJsonElement(ClickhouseConfig.serializer(), config)
                        )
                    },
                "eventBridge" to
                    { config ->
                        EventBridge(
                            Json.decodeFromJsonElement(EventBridgeConfig.serializer(), config)
                        )
                    },
                "snowflake" to
                    { config ->
                        Snowflake(Json.decodeFromJsonElement(SnowflakeConfig.serializer(), config))
                    },
                "rabbitMq" to
                    { config ->
                        RabbitMq(Json.decodeFromJsonElement(RabbitMqConfig.serializer(), config))
                    },
                "redshift" to
                    { config ->
                        Redshift(Json.decodeFromJsonElement(RedshiftConfig.serializer(), config))
                    },
            )

        fun fromTypeAndConfig(type: String, config: JsonElement): StreamSinkInConfig {
            return typeMap[type]?.invoke(config)
                ?: throw SerializationException("Unknown type: $type")
        }
    }
}

class StreamSinkInSerializer : KSerializer<StreamSinkIn> {
    @Serializable
    private data class StreamSinkInSurrogate(
        /** An optional unique identifier for the sink. */
        val uid: String? = null,
        /**
         * Whether the sink will receive events.
         *
         * If the sink is `enabled`, any events posted to the stream will be dispatched to the Sink
         * in the same order that events were posted to the stream.
         *
         * If the sink is `disabled`, events will not be dispatched to the sink until the sink is
         * reenabled.
         */
        val status: SinkStatusIn? = null,
        /** How many events will be batched in a request to the Sink. */
        val batchSize: UShort? = null,
        /**
         * How long to wait before a batch of events is sent, if the `batchSize` is not reached.
         *
         * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request
         * after 10 seconds or 100 events, whichever comes first.
         *
         * Note that we will never send an empty batch of events to the Sink.
         */
        val maxWaitSecs: UShort? = null,
        /**
         * A list of event types that filter which events are dispatched to the Sink. An empty list
         * (or null) will not filter out any events.
         */
        val eventTypes: List<String>? = null,
        val metadata: Map<String, String>? = null,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = StreamSinkInSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: StreamSinkIn) {
        val surrogate =
            StreamSinkInSurrogate(
                uid = value.uid,
                status = value.status,
                batchSize = value.batchSize,
                maxWaitSecs = value.maxWaitSecs,
                eventTypes = value.eventTypes,
                metadata = value.metadata,
                type = value.config.variantName,
                config = value.config.toJsonElement(),
            )
        encoder.encodeSerializableValue(StreamSinkInSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): StreamSinkIn {
        val surrogate = decoder.decodeSerializableValue(StreamSinkInSurrogate.serializer())
        return StreamSinkIn(
            uid = surrogate.uid,
            status = surrogate.status,
            batchSize = surrogate.batchSize,
            maxWaitSecs = surrogate.maxWaitSecs,
            eventTypes = surrogate.eventTypes,
            metadata = surrogate.metadata,
            config = StreamSinkInConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
