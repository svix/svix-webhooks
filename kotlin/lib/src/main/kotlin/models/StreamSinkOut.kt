// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.buildJsonObject

@Serializable(with = StreamSinkOutSerializer::class)
data class StreamSinkOut(
    /** The sink's ID. */
    val id: String,
    /** The sink's UID. */
    val uid: String? = null,
    val status: SinkStatus,
    val currentIterator: String,
    val failureReason: String? = null,
    val createdAt: Instant,
    val updatedAt: Instant,
    val batchSize: Int,
    val maxWaitSecs: Int,
    val eventTypes: List<String>? = null,
    val nextRetryAt: Instant? = null,
    val metadata: Map<String, String>,
    val config: StreamSinkOutConfig,
)

@Serializable
sealed class StreamSinkOutConfig {
    val variantName: String
        get() = this::class.annotations.filterIsInstance<VariantName>().first().name

    abstract fun toJsonElement(): JsonElement

    @VariantName("poller")
    data object Poller : StreamSinkOutConfig() {
        override fun toJsonElement() = buildJsonObject {}
    }

    @VariantName("azureBlobStorage")
    data class AzureBlobStorage(val azureBlobStorage: AzureBlobStorageConfig) :
        StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AzureBlobStorageConfig.serializer(), azureBlobStorage)
    }

    @VariantName("otelTracing")
    data class OtelTracing(val otelTracing: SinkOtelV1Config) : StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SinkOtelV1Config.serializer(), otelTracing)
    }

    @VariantName("http")
    data class Http(val http: SinkHttpConfig) : StreamSinkOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SinkHttpConfig.serializer(), http)
    }

    @VariantName("amazonS3")
    data class AmazonS3(val amazonS3: S3Config) : StreamSinkOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(S3Config.serializer(), amazonS3)
    }

    @VariantName("googleCloudStorage")
    data class GoogleCloudStorage(val googleCloudStorage: GoogleCloudStorageConfig) :
        StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudStorageConfig.serializer(), googleCloudStorage)
    }

    @VariantName("googleCloudPubSub")
    data class GoogleCloudPubSub(val googleCloudPubSub: GoogleCloudPubSubConfig) :
        StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudPubSubConfig.serializer(), googleCloudPubSub)
    }

    @VariantName("sqs")
    data class Sqs(val sqs: SqsConfig) : StreamSinkOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SqsConfig.serializer(), sqs)
    }

    @VariantName("sns")
    data class Sns(val sns: SnsConfig) : StreamSinkOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SnsConfig.serializer(), sns)
    }

    @VariantName("bigQuery")
    data class BigQuery(val bigQuery: BigQueryConfig) : StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(BigQueryConfig.serializer(), bigQuery)
    }

    @VariantName("clickhouse")
    data class Clickhouse(val clickhouse: ClickhouseConfig) : StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(ClickhouseConfig.serializer(), clickhouse)
    }

    @VariantName("eventBridge")
    data class EventBridge(val eventBridge: EventBridgeConfig) : StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(EventBridgeConfig.serializer(), eventBridge)
    }

    @VariantName("snowflake")
    data class Snowflake(val snowflake: SnowflakeConfig) : StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SnowflakeConfig.serializer(), snowflake)
    }

    @VariantName("rabbitMq")
    data class RabbitMq(val rabbitMq: RabbitMqConfig) : StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RabbitMqConfig.serializer(), rabbitMq)
    }

    @VariantName("redshift")
    data class Redshift(val redshift: RedshiftConfig) : StreamSinkOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RedshiftConfig.serializer(), redshift)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> StreamSinkOutConfig>(
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

        fun fromTypeAndConfig(type: String, config: JsonElement): StreamSinkOutConfig {
            return typeMap[type]?.invoke(config)
                ?: throw SerializationException("Unknown type: $type")
        }
    }
}

class StreamSinkOutSerializer : KSerializer<StreamSinkOut> {
    @Serializable
    private data class StreamSinkOutSurrogate(
        /** The sink's ID. */
        val id: String,
        /** The sink's UID. */
        val uid: String? = null,
        val status: SinkStatus,
        val currentIterator: String,
        val failureReason: String? = null,
        val createdAt: Instant,
        val updatedAt: Instant,
        val batchSize: Int,
        val maxWaitSecs: Int,
        val eventTypes: List<String>? = null,
        val nextRetryAt: Instant? = null,
        val metadata: Map<String, String>,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = StreamSinkOutSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: StreamSinkOut) {
        val surrogate =
            StreamSinkOutSurrogate(
                id = value.id,
                uid = value.uid,
                status = value.status,
                currentIterator = value.currentIterator,
                failureReason = value.failureReason,
                createdAt = value.createdAt,
                updatedAt = value.updatedAt,
                batchSize = value.batchSize,
                maxWaitSecs = value.maxWaitSecs,
                eventTypes = value.eventTypes,
                nextRetryAt = value.nextRetryAt,
                metadata = value.metadata,
                type = value.config.variantName,
                config = value.config.toJsonElement(),
            )
        encoder.encodeSerializableValue(StreamSinkOutSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): StreamSinkOut {
        val surrogate = decoder.decodeSerializableValue(StreamSinkOutSurrogate.serializer())
        return StreamSinkOut(
            id = surrogate.id,
            uid = surrogate.uid,
            status = surrogate.status,
            currentIterator = surrogate.currentIterator,
            failureReason = surrogate.failureReason,
            createdAt = surrogate.createdAt,
            updatedAt = surrogate.updatedAt,
            batchSize = surrogate.batchSize,
            maxWaitSecs = surrogate.maxWaitSecs,
            eventTypes = surrogate.eventTypes,
            nextRetryAt = surrogate.nextRetryAt,
            metadata = surrogate.metadata,
            config = StreamSinkOutConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
