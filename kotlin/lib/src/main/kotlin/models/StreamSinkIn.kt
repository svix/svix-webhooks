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
    val channels: List<String>? = null,
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
    data class AzureBlobStorage(val azureBlobStorage: AzureBlobStorageConfigIn) :
        StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AzureBlobStorageConfigIn.serializer(), azureBlobStorage)
    }

    @VariantName("otelTracing")
    data class OtelTracing(val otelTracing: OtelTracingConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(OtelTracingConfigIn.serializer(), otelTracing)
    }

    @VariantName("http")
    data class Http(val http: SinkHttpConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SinkHttpConfigIn.serializer(), http)
    }

    @VariantName("amazonS3")
    data class AmazonS3(val amazonS3: S3ConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(S3ConfigIn.serializer(), amazonS3)
    }

    @VariantName("googleCloudStorage")
    data class GoogleCloudStorage(val googleCloudStorage: GoogleCloudStorageConfigIn) :
        StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudStorageConfigIn.serializer(), googleCloudStorage)
    }

    @VariantName("googleCloudPubSub")
    data class GoogleCloudPubSub(val googleCloudPubSub: GoogleCloudPubSubConfigIn) :
        StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudPubSubConfigIn.serializer(), googleCloudPubSub)
    }

    @VariantName("sqs")
    data class Sqs(val sqs: SqsConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SqsConfigIn.serializer(), sqs)
    }

    @VariantName("sns")
    data class Sns(val sns: SnsConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SnsConfigIn.serializer(), sns)
    }

    @VariantName("bigQuery")
    data class BigQuery(val bigQuery: BigQueryConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(BigQueryConfigIn.serializer(), bigQuery)
    }

    @VariantName("clickhouse")
    data class Clickhouse(val clickhouse: ClickhouseConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(ClickhouseConfigIn.serializer(), clickhouse)
    }

    @VariantName("eventBridge")
    data class EventBridge(val eventBridge: EventBridgeConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(EventBridgeConfigIn.serializer(), eventBridge)
    }

    @VariantName("snowflake")
    data class Snowflake(val snowflake: SnowflakeConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SnowflakeConfigIn.serializer(), snowflake)
    }

    @VariantName("rabbitMq")
    data class RabbitMq(val rabbitMq: RabbitMqConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RabbitMqConfigIn.serializer(), rabbitMq)
    }

    @VariantName("redshift")
    data class Redshift(val redshift: RedshiftConfigIn) : StreamSinkInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RedshiftConfigIn.serializer(), redshift)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> StreamSinkInConfig>(
                "poller" to { _ -> Poller },
                "azureBlobStorage" to
                    { config ->
                        AzureBlobStorage(
                            Json.decodeFromJsonElement(
                                AzureBlobStorageConfigIn.serializer(),
                                config,
                            )
                        )
                    },
                "otelTracing" to
                    { config ->
                        OtelTracing(
                            Json.decodeFromJsonElement(OtelTracingConfigIn.serializer(), config)
                        )
                    },
                "http" to
                    { config ->
                        Http(Json.decodeFromJsonElement(SinkHttpConfigIn.serializer(), config))
                    },
                "amazonS3" to
                    { config ->
                        AmazonS3(Json.decodeFromJsonElement(S3ConfigIn.serializer(), config))
                    },
                "googleCloudStorage" to
                    { config ->
                        GoogleCloudStorage(
                            Json.decodeFromJsonElement(
                                GoogleCloudStorageConfigIn.serializer(),
                                config,
                            )
                        )
                    },
                "googleCloudPubSub" to
                    { config ->
                        GoogleCloudPubSub(
                            Json.decodeFromJsonElement(
                                GoogleCloudPubSubConfigIn.serializer(),
                                config,
                            )
                        )
                    },
                "sqs" to
                    { config ->
                        Sqs(Json.decodeFromJsonElement(SqsConfigIn.serializer(), config))
                    },
                "sns" to
                    { config ->
                        Sns(Json.decodeFromJsonElement(SnsConfigIn.serializer(), config))
                    },
                "bigQuery" to
                    { config ->
                        BigQuery(Json.decodeFromJsonElement(BigQueryConfigIn.serializer(), config))
                    },
                "clickhouse" to
                    { config ->
                        Clickhouse(
                            Json.decodeFromJsonElement(ClickhouseConfigIn.serializer(), config)
                        )
                    },
                "eventBridge" to
                    { config ->
                        EventBridge(
                            Json.decodeFromJsonElement(EventBridgeConfigIn.serializer(), config)
                        )
                    },
                "snowflake" to
                    { config ->
                        Snowflake(
                            Json.decodeFromJsonElement(SnowflakeConfigIn.serializer(), config)
                        )
                    },
                "rabbitMq" to
                    { config ->
                        RabbitMq(Json.decodeFromJsonElement(RabbitMqConfigIn.serializer(), config))
                    },
                "redshift" to
                    { config ->
                        Redshift(Json.decodeFromJsonElement(RedshiftConfigIn.serializer(), config))
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
        val channels: List<String>? = null,
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
                channels = value.channels,
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
            channels = surrogate.channels,
            metadata = surrogate.metadata,
            config = StreamSinkInConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
