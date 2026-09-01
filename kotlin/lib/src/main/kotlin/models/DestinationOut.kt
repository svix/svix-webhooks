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

@Serializable(with = DestinationOutSerializer::class)
data class DestinationOut(
    /** The destination's ID. */
    val id: String,
    /** The destination's UID. */
    val uid: String? = null,
    val status: SinkStatus,
    val currentIterator: String,
    val failureReason: String? = null,
    val createdAt: Instant,
    val updatedAt: Instant,
    val batchSize: Int,
    val maxWaitSecs: Int,
    val eventTypes: List<String>? = null,
    val channels: List<String>? = null,
    val nextRetryAt: Instant? = null,
    val metadata: Map<String, String>,
    val config: DestinationOutConfig,
)

@Serializable
sealed class DestinationOutConfig {
    val variantName: String
        get() = this::class.annotations.filterIsInstance<VariantName>().first().name

    abstract fun toJsonElement(): JsonElement

    @VariantName("pollingEndpoint")
    data object PollingEndpoint : DestinationOutConfig() {
        override fun toJsonElement() = buildJsonObject {}
    }

    @VariantName("azureBlobStorage")
    data class AzureBlobStorage(val azureBlobStorage: AzureBlobStorageConfigOut) :
        DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AzureBlobStorageConfigOut.serializer(), azureBlobStorage)
    }

    @VariantName("otelTracing")
    data class OtelTracing(val otelTracing: OtelTracingConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(OtelTracingConfigOut.serializer(), otelTracing)
    }

    @VariantName("fifoEndpoint")
    data class FifoEndpoint(val fifoEndpoint: SinkHttpConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SinkHttpConfigOut.serializer(), fifoEndpoint)
    }

    @VariantName("amazonS3")
    data class AmazonS3(val amazonS3: S3ConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(S3ConfigOut.serializer(), amazonS3)
    }

    @VariantName("snowflake")
    data class Snowflake(val snowflake: SnowflakeConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SnowflakeConfigOut.serializer(), snowflake)
    }

    @VariantName("googleCloudStorage")
    data class GoogleCloudStorage(val googleCloudStorage: GoogleCloudStorageConfigOut) :
        DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudStorageConfigOut.serializer(), googleCloudStorage)
    }

    @VariantName("googleCloudPubSub")
    data class GoogleCloudPubSub(val googleCloudPubSub: GoogleCloudPubSubConfigOut) :
        DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudPubSubConfigOut.serializer(), googleCloudPubSub)
    }

    @VariantName("redshift")
    data class Redshift(val redshift: RedshiftConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RedshiftConfigOut.serializer(), redshift)
    }

    @VariantName("bigQuery")
    data class BigQuery(val bigQuery: BigQueryConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(BigQueryConfigOut.serializer(), bigQuery)
    }

    @VariantName("clickhouse")
    data class Clickhouse(val clickhouse: ClickhouseConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(ClickhouseConfigOut.serializer(), clickhouse)
    }

    @VariantName("rabbitMq")
    data class RabbitMq(val rabbitMq: RabbitMqConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RabbitMqConfigOut.serializer(), rabbitMq)
    }

    @VariantName("sqs")
    data class Sqs(val sqs: SqsConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SqsConfigOut.serializer(), sqs)
    }

    @VariantName("eventBridge")
    data class EventBridge(val eventBridge: EventBridgeConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(EventBridgeConfigOut.serializer(), eventBridge)
    }

    @VariantName("sns")
    data class Sns(val sns: SnsConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SnsConfigOut.serializer(), sns)
    }

    @VariantName("postgres")
    data class Postgres(val postgres: PostgresConfigOut) : DestinationOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(PostgresConfigOut.serializer(), postgres)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> DestinationOutConfig>(
                "pollingEndpoint" to { _ -> PollingEndpoint },
                "azureBlobStorage" to
                    { config ->
                        AzureBlobStorage(
                            Json.decodeFromJsonElement(
                                AzureBlobStorageConfigOut.serializer(),
                                config,
                            )
                        )
                    },
                "otelTracing" to
                    { config ->
                        OtelTracing(
                            Json.decodeFromJsonElement(OtelTracingConfigOut.serializer(), config)
                        )
                    },
                "fifoEndpoint" to
                    { config ->
                        FifoEndpoint(
                            Json.decodeFromJsonElement(SinkHttpConfigOut.serializer(), config)
                        )
                    },
                "amazonS3" to
                    { config ->
                        AmazonS3(Json.decodeFromJsonElement(S3ConfigOut.serializer(), config))
                    },
                "snowflake" to
                    { config ->
                        Snowflake(
                            Json.decodeFromJsonElement(SnowflakeConfigOut.serializer(), config)
                        )
                    },
                "googleCloudStorage" to
                    { config ->
                        GoogleCloudStorage(
                            Json.decodeFromJsonElement(
                                GoogleCloudStorageConfigOut.serializer(),
                                config,
                            )
                        )
                    },
                "googleCloudPubSub" to
                    { config ->
                        GoogleCloudPubSub(
                            Json.decodeFromJsonElement(
                                GoogleCloudPubSubConfigOut.serializer(),
                                config,
                            )
                        )
                    },
                "redshift" to
                    { config ->
                        Redshift(Json.decodeFromJsonElement(RedshiftConfigOut.serializer(), config))
                    },
                "bigQuery" to
                    { config ->
                        BigQuery(Json.decodeFromJsonElement(BigQueryConfigOut.serializer(), config))
                    },
                "clickhouse" to
                    { config ->
                        Clickhouse(
                            Json.decodeFromJsonElement(ClickhouseConfigOut.serializer(), config)
                        )
                    },
                "rabbitMq" to
                    { config ->
                        RabbitMq(Json.decodeFromJsonElement(RabbitMqConfigOut.serializer(), config))
                    },
                "sqs" to
                    { config ->
                        Sqs(Json.decodeFromJsonElement(SqsConfigOut.serializer(), config))
                    },
                "eventBridge" to
                    { config ->
                        EventBridge(
                            Json.decodeFromJsonElement(EventBridgeConfigOut.serializer(), config)
                        )
                    },
                "sns" to
                    { config ->
                        Sns(Json.decodeFromJsonElement(SnsConfigOut.serializer(), config))
                    },
                "postgres" to
                    { config ->
                        Postgres(Json.decodeFromJsonElement(PostgresConfigOut.serializer(), config))
                    },
            )

        fun fromTypeAndConfig(type: String, config: JsonElement): DestinationOutConfig {
            return typeMap[type]?.invoke(config)
                ?: throw SerializationException("Unknown type: $type")
        }
    }
}

class DestinationOutSerializer : KSerializer<DestinationOut> {
    @Serializable
    private data class DestinationOutSurrogate(
        /** The destination's ID. */
        val id: String,
        /** The destination's UID. */
        val uid: String? = null,
        val status: SinkStatus,
        val currentIterator: String,
        val failureReason: String? = null,
        val createdAt: Instant,
        val updatedAt: Instant,
        val batchSize: Int,
        val maxWaitSecs: Int,
        val eventTypes: List<String>? = null,
        val channels: List<String>? = null,
        val nextRetryAt: Instant? = null,
        val metadata: Map<String, String>,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = DestinationOutSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: DestinationOut) {
        val surrogate =
            DestinationOutSurrogate(
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
                channels = value.channels,
                nextRetryAt = value.nextRetryAt,
                metadata = value.metadata,
                type = value.config.variantName,
                config = value.config.toJsonElement(),
            )
        encoder.encodeSerializableValue(DestinationOutSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): DestinationOut {
        val surrogate = decoder.decodeSerializableValue(DestinationOutSurrogate.serializer())
        return DestinationOut(
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
            channels = surrogate.channels,
            nextRetryAt = surrogate.nextRetryAt,
            metadata = surrogate.metadata,
            config = DestinationOutConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
