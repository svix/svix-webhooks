// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.buildJsonObject

@Serializable(with = DestinationPatchSerializer::class)
data class DestinationPatch(
    /** The Destination's UID. */
    val uid: MaybeUnset<String> = MaybeUnset.Unset,
    val status: MaybeUnset<DestinationStatusIn> = MaybeUnset.Unset,
    val batchSize: MaybeUnset<UShort> = MaybeUnset.Unset,
    val maxWaitSecs: MaybeUnset<UShort> = MaybeUnset.Unset,
    val eventTypes: List<String>? = null,
    val channels: List<String>? = null,
    val metadata: Map<String, String>? = null,
    val config: DestinationPatchConfig,
)

@Serializable
sealed class DestinationPatchConfig {
    val variantName: String
        get() = this::class.annotations.filterIsInstance<VariantName>().first().name

    abstract fun toJsonElement(): JsonElement

    @VariantName("pollingEndpoint")
    data object PollingEndpoint : DestinationPatchConfig() {
        override fun toJsonElement() = buildJsonObject {}
    }

    @VariantName("azureBlobStorage")
    data class AzureBlobStorage(val azureBlobStorage: AzureBlobStorageConfigPatch) :
        DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AzureBlobStorageConfigPatch.serializer(), azureBlobStorage)
    }

    @VariantName("otelTracing")
    data class OtelTracing(val otelTracing: OtelTracingConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(OtelTracingConfigPatch.serializer(), otelTracing)
    }

    @VariantName("fifoEndpoint")
    data class FifoEndpoint(val fifoEndpoint: SinkHttpConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SinkHttpConfigPatch.serializer(), fifoEndpoint)
    }

    @VariantName("amazonS3")
    data class AmazonS3(val amazonS3: S3ConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(S3ConfigPatch.serializer(), amazonS3)
    }

    @VariantName("googleCloudStorage")
    data class GoogleCloudStorage(val googleCloudStorage: GoogleCloudStorageConfigPatch) :
        DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudStorageConfigPatch.serializer(), googleCloudStorage)
    }

    @VariantName("googleCloudPubSub")
    data class GoogleCloudPubSub(val googleCloudPubSub: GoogleCloudPubSubConfigPatch) :
        DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudPubSubConfigPatch.serializer(), googleCloudPubSub)
    }

    @VariantName("sqs")
    data class Sqs(val sqs: SqsConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SqsConfigPatch.serializer(), sqs)
    }

    @VariantName("sns")
    data class Sns(val sns: SnsConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SnsConfigPatch.serializer(), sns)
    }

    @VariantName("bigQuery")
    data class BigQuery(val bigQuery: BigQueryConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(BigQueryConfigPatch.serializer(), bigQuery)
    }

    @VariantName("clickhouse")
    data class Clickhouse(val clickhouse: ClickhouseConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(ClickhouseConfigPatch.serializer(), clickhouse)
    }

    @VariantName("eventBridge")
    data class EventBridge(val eventBridge: EventBridgeConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(EventBridgeConfigPatch.serializer(), eventBridge)
    }

    @VariantName("snowflake")
    data class Snowflake(val snowflake: SnowflakeConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SnowflakeConfigPatch.serializer(), snowflake)
    }

    @VariantName("rabbitMq")
    data class RabbitMq(val rabbitMq: RabbitMqConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RabbitMqConfigPatch.serializer(), rabbitMq)
    }

    @VariantName("redshift")
    data class Redshift(val redshift: RedshiftConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RedshiftConfigPatch.serializer(), redshift)
    }

    @VariantName("postgres")
    data class Postgres(val postgres: PostgresConfigPatch) : DestinationPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(PostgresConfigPatch.serializer(), postgres)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> DestinationPatchConfig>(
                "pollingEndpoint" to { _ -> PollingEndpoint },
                "azureBlobStorage" to
                    { config ->
                        AzureBlobStorage(
                            Json.decodeFromJsonElement(
                                AzureBlobStorageConfigPatch.serializer(),
                                config,
                            )
                        )
                    },
                "otelTracing" to
                    { config ->
                        OtelTracing(
                            Json.decodeFromJsonElement(OtelTracingConfigPatch.serializer(), config)
                        )
                    },
                "fifoEndpoint" to
                    { config ->
                        FifoEndpoint(
                            Json.decodeFromJsonElement(SinkHttpConfigPatch.serializer(), config)
                        )
                    },
                "amazonS3" to
                    { config ->
                        AmazonS3(Json.decodeFromJsonElement(S3ConfigPatch.serializer(), config))
                    },
                "googleCloudStorage" to
                    { config ->
                        GoogleCloudStorage(
                            Json.decodeFromJsonElement(
                                GoogleCloudStorageConfigPatch.serializer(),
                                config,
                            )
                        )
                    },
                "googleCloudPubSub" to
                    { config ->
                        GoogleCloudPubSub(
                            Json.decodeFromJsonElement(
                                GoogleCloudPubSubConfigPatch.serializer(),
                                config,
                            )
                        )
                    },
                "sqs" to
                    { config ->
                        Sqs(Json.decodeFromJsonElement(SqsConfigPatch.serializer(), config))
                    },
                "sns" to
                    { config ->
                        Sns(Json.decodeFromJsonElement(SnsConfigPatch.serializer(), config))
                    },
                "bigQuery" to
                    { config ->
                        BigQuery(
                            Json.decodeFromJsonElement(BigQueryConfigPatch.serializer(), config)
                        )
                    },
                "clickhouse" to
                    { config ->
                        Clickhouse(
                            Json.decodeFromJsonElement(ClickhouseConfigPatch.serializer(), config)
                        )
                    },
                "eventBridge" to
                    { config ->
                        EventBridge(
                            Json.decodeFromJsonElement(EventBridgeConfigPatch.serializer(), config)
                        )
                    },
                "snowflake" to
                    { config ->
                        Snowflake(
                            Json.decodeFromJsonElement(SnowflakeConfigPatch.serializer(), config)
                        )
                    },
                "rabbitMq" to
                    { config ->
                        RabbitMq(
                            Json.decodeFromJsonElement(RabbitMqConfigPatch.serializer(), config)
                        )
                    },
                "redshift" to
                    { config ->
                        Redshift(
                            Json.decodeFromJsonElement(RedshiftConfigPatch.serializer(), config)
                        )
                    },
                "postgres" to
                    { config ->
                        Postgres(
                            Json.decodeFromJsonElement(PostgresConfigPatch.serializer(), config)
                        )
                    },
            )

        fun fromTypeAndConfig(type: String, config: JsonElement): DestinationPatchConfig {
            return typeMap[type]?.invoke(config)
                ?: throw SerializationException("Unknown type: $type")
        }
    }
}

class DestinationPatchSerializer : KSerializer<DestinationPatch> {
    @Serializable
    private data class DestinationPatchSurrogate(
        /** The Destination's UID. */
        val uid: MaybeUnset<String> = MaybeUnset.Unset,
        val status: MaybeUnset<DestinationStatusIn> = MaybeUnset.Unset,
        val batchSize: MaybeUnset<UShort> = MaybeUnset.Unset,
        val maxWaitSecs: MaybeUnset<UShort> = MaybeUnset.Unset,
        val eventTypes: List<String>? = null,
        val channels: List<String>? = null,
        val metadata: Map<String, String>? = null,
        val type: String,
        val config: JsonElement = buildJsonObject {},
    )

    override val descriptor: SerialDescriptor = DestinationPatchSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: DestinationPatch) {
        val surrogate =
            DestinationPatchSurrogate(
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
        encoder.encodeSerializableValue(DestinationPatchSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): DestinationPatch {
        val surrogate = decoder.decodeSerializableValue(DestinationPatchSurrogate.serializer())
        return DestinationPatch(
            uid = surrogate.uid,
            status = surrogate.status,
            batchSize = surrogate.batchSize,
            maxWaitSecs = surrogate.maxWaitSecs,
            eventTypes = surrogate.eventTypes,
            channels = surrogate.channels,
            metadata = surrogate.metadata,
            config = DestinationPatchConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
