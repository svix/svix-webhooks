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

@Serializable(with = DestinationInSerializer::class)
data class DestinationIn(
    /** An optional unique identifier for the destination. */
    val uid: String? = null,
    /**
     * Whether the destination will receive events.
     *
     * If the destination is `enabled`, events sent to the application will be dispatched to the
     * destination in order.
     *
     * If the destination is `disabled`, events will not be dispatched until the destination is
     * reenabled.
     */
    val status: SinkStatusIn? = null,
    /** How many events will be batched in a request to the destination. */
    val batchSize: UShort? = null,
    /**
     * How long to wait before a batch of events is sent, if the `batchSize` is not reached.
     *
     * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, a request is sent after 10
     * seconds or 100 events, whichever comes first.
     *
     * Note that an empty batch is never sent to the destination.
     */
    val maxWaitSecs: UShort? = null,
    /**
     * A list of event types that filter which events are dispatched to the destination. An empty
     * list (or null) will not filter out any events.
     */
    val eventTypes: List<String>? = null,
    /**
     * A list of channels that filter which events are dispatched to the destination. An empty list
     * (or null) will not filter out any events.
     */
    val channels: List<String>? = null,
    val metadata: Map<String, String>? = null,
    val config: DestinationInConfig,
)

@Serializable
sealed class DestinationInConfig {
    val variantName: String
        get() = this::class.annotations.filterIsInstance<VariantName>().first().name

    abstract fun toJsonElement(): JsonElement

    @VariantName("pollingEndpoint")
    data object PollingEndpoint : DestinationInConfig() {
        override fun toJsonElement() = buildJsonObject {}
    }

    @VariantName("azureBlobStorage")
    data class AzureBlobStorage(val azureBlobStorage: AzureBlobStorageConfigIn) :
        DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AzureBlobStorageConfigIn.serializer(), azureBlobStorage)
    }

    @VariantName("otelTracing")
    data class OtelTracing(val otelTracing: OtelTracingConfigIn) : DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(OtelTracingConfigIn.serializer(), otelTracing)
    }

    @VariantName("fifoEndpoint")
    data class FifoEndpoint(val fifoEndpoint: FifoEndpointConfigIn) : DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(FifoEndpointConfigIn.serializer(), fifoEndpoint)
    }

    @VariantName("amazonS3")
    data class AmazonS3(val amazonS3: S3ConfigIn) : DestinationInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(S3ConfigIn.serializer(), amazonS3)
    }

    @VariantName("googleCloudStorage")
    data class GoogleCloudStorage(val googleCloudStorage: GoogleCloudStorageConfigIn) :
        DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudStorageConfigIn.serializer(), googleCloudStorage)
    }

    @VariantName("googleCloudPubSub")
    data class GoogleCloudPubSub(val googleCloudPubSub: GoogleCloudPubSubConfigIn) :
        DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudPubSubConfigIn.serializer(), googleCloudPubSub)
    }

    @VariantName("sqs")
    data class Sqs(val sqs: SqsConfigIn) : DestinationInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SqsConfigIn.serializer(), sqs)
    }

    @VariantName("sns")
    data class Sns(val sns: SnsConfigIn) : DestinationInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SnsConfigIn.serializer(), sns)
    }

    @VariantName("bigQuery")
    data class BigQuery(val bigQuery: BigQueryConfigIn) : DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(BigQueryConfigIn.serializer(), bigQuery)
    }

    @VariantName("clickhouse")
    data class Clickhouse(val clickhouse: ClickhouseConfigIn) : DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(ClickhouseConfigIn.serializer(), clickhouse)
    }

    @VariantName("eventBridge")
    data class EventBridge(val eventBridge: EventBridgeConfigIn) : DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(EventBridgeConfigIn.serializer(), eventBridge)
    }

    @VariantName("snowflake")
    data class Snowflake(val snowflake: SnowflakeConfigIn) : DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SnowflakeConfigIn.serializer(), snowflake)
    }

    @VariantName("rabbitMq")
    data class RabbitMq(val rabbitMq: RabbitMqConfigIn) : DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RabbitMqConfigIn.serializer(), rabbitMq)
    }

    @VariantName("redshift")
    data class Redshift(val redshift: RedshiftConfigIn) : DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RedshiftConfigIn.serializer(), redshift)
    }

    @VariantName("postgres")
    data class Postgres(val postgres: PostgresConfigIn) : DestinationInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(PostgresConfigIn.serializer(), postgres)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> DestinationInConfig>(
                "pollingEndpoint" to { _ -> PollingEndpoint },
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
                "fifoEndpoint" to
                    { config ->
                        FifoEndpoint(
                            Json.decodeFromJsonElement(FifoEndpointConfigIn.serializer(), config)
                        )
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
                "postgres" to
                    { config ->
                        Postgres(Json.decodeFromJsonElement(PostgresConfigIn.serializer(), config))
                    },
            )

        fun fromTypeAndConfig(type: String, config: JsonElement): DestinationInConfig {
            return typeMap[type]?.invoke(config)
                ?: throw SerializationException("Unknown type: $type")
        }
    }
}

class DestinationInSerializer : KSerializer<DestinationIn> {
    @Serializable
    private data class DestinationInSurrogate(
        /** An optional unique identifier for the destination. */
        val uid: String? = null,
        /**
         * Whether the destination will receive events.
         *
         * If the destination is `enabled`, events sent to the application will be dispatched to the
         * destination in order.
         *
         * If the destination is `disabled`, events will not be dispatched until the destination is
         * reenabled.
         */
        val status: SinkStatusIn? = null,
        /** How many events will be batched in a request to the destination. */
        val batchSize: UShort? = null,
        /**
         * How long to wait before a batch of events is sent, if the `batchSize` is not reached.
         *
         * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, a request is sent after
         * 10 seconds or 100 events, whichever comes first.
         *
         * Note that an empty batch is never sent to the destination.
         */
        val maxWaitSecs: UShort? = null,
        /**
         * A list of event types that filter which events are dispatched to the destination. An
         * empty list (or null) will not filter out any events.
         */
        val eventTypes: List<String>? = null,
        /**
         * A list of channels that filter which events are dispatched to the destination. An empty
         * list (or null) will not filter out any events.
         */
        val channels: List<String>? = null,
        val metadata: Map<String, String>? = null,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = DestinationInSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: DestinationIn) {
        val surrogate =
            DestinationInSurrogate(
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
        encoder.encodeSerializableValue(DestinationInSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): DestinationIn {
        val surrogate = decoder.decodeSerializableValue(DestinationInSurrogate.serializer())
        return DestinationIn(
            uid = surrogate.uid,
            status = surrogate.status,
            batchSize = surrogate.batchSize,
            maxWaitSecs = surrogate.maxWaitSecs,
            eventTypes = surrogate.eventTypes,
            channels = surrogate.channels,
            metadata = surrogate.metadata,
            config = DestinationInConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
