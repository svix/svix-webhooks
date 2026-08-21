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

@Serializable(with = StreamSinkPatchSerializer::class)
data class StreamSinkPatch(
    /** The StreamSink's UID. */
    val uid: MaybeUnset<String> = MaybeUnset.Unset,
    val status: MaybeUnset<SinkStatusIn> = MaybeUnset.Unset,
    val batchSize: MaybeUnset<UShort> = MaybeUnset.Unset,
    val maxWaitSecs: MaybeUnset<UShort> = MaybeUnset.Unset,
    val eventTypes: List<String>? = null,
    val channels: List<String>? = null,
    val metadata: Map<String, String>? = null,
    val config: StreamSinkPatchConfig,
)

@Serializable
sealed class StreamSinkPatchConfig {
    val variantName: String
        get() = this::class.annotations.filterIsInstance<VariantName>().first().name

    abstract fun toJsonElement(): JsonElement

    @VariantName("poller")
    data object Poller : StreamSinkPatchConfig() {
        override fun toJsonElement() = buildJsonObject {}
    }

    @VariantName("azureBlobStorage")
    data class AzureBlobStorage(val azureBlobStorage: AzureBlobStorageConfigPatch) :
        StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AzureBlobStorageConfigPatch.serializer(), azureBlobStorage)
    }

    @VariantName("otelTracing")
    data class OtelTracing(val otelTracing: OtelTracingConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(OtelTracingConfigPatch.serializer(), otelTracing)
    }

    @VariantName("http")
    data class Http(val http: SinkHttpConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SinkHttpConfigPatch.serializer(), http)
    }

    @VariantName("amazonS3")
    data class AmazonS3(val amazonS3: S3ConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(S3ConfigPatch.serializer(), amazonS3)
    }

    @VariantName("googleCloudStorage")
    data class GoogleCloudStorage(val googleCloudStorage: GoogleCloudStorageConfigPatch) :
        StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudStorageConfigPatch.serializer(), googleCloudStorage)
    }

    @VariantName("googleCloudPubSub")
    data class GoogleCloudPubSub(val googleCloudPubSub: GoogleCloudPubSubConfigPatch) :
        StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudPubSubConfigPatch.serializer(), googleCloudPubSub)
    }

    @VariantName("sqs")
    data class Sqs(val sqs: SqsConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SqsConfigPatch.serializer(), sqs)
    }

    @VariantName("sns")
    data class Sns(val sns: SnsConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SnsConfigPatch.serializer(), sns)
    }

    @VariantName("bigQuery")
    data class BigQuery(val bigQuery: BigQueryConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(BigQueryConfigPatch.serializer(), bigQuery)
    }

    @VariantName("clickhouse")
    data class Clickhouse(val clickhouse: ClickhouseConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(ClickhouseConfigPatch.serializer(), clickhouse)
    }

    @VariantName("eventBridge")
    data class EventBridge(val eventBridge: EventBridgeConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(EventBridgeConfigPatch.serializer(), eventBridge)
    }

    @VariantName("snowflake")
    data class Snowflake(val snowflake: SnowflakeConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SnowflakeConfigPatch.serializer(), snowflake)
    }

    @VariantName("rabbitMq")
    data class RabbitMq(val rabbitMq: RabbitMqConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RabbitMqConfigPatch.serializer(), rabbitMq)
    }

    @VariantName("redshift")
    data class Redshift(val redshift: RedshiftConfigPatch) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RedshiftConfigPatch.serializer(), redshift)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> StreamSinkPatchConfig>(
                "poller" to { _ -> Poller },
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
                "http" to
                    { config ->
                        Http(Json.decodeFromJsonElement(SinkHttpConfigPatch.serializer(), config))
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
            )

        fun fromTypeAndConfig(type: String, config: JsonElement): StreamSinkPatchConfig {
            return typeMap[type]?.invoke(config)
                ?: throw SerializationException("Unknown type: $type")
        }
    }
}

class StreamSinkPatchSerializer : KSerializer<StreamSinkPatch> {
    @Serializable
    private data class StreamSinkPatchSurrogate(
        /** The StreamSink's UID. */
        val uid: MaybeUnset<String> = MaybeUnset.Unset,
        val status: MaybeUnset<SinkStatusIn> = MaybeUnset.Unset,
        val batchSize: MaybeUnset<UShort> = MaybeUnset.Unset,
        val maxWaitSecs: MaybeUnset<UShort> = MaybeUnset.Unset,
        val eventTypes: List<String>? = null,
        val channels: List<String>? = null,
        val metadata: Map<String, String>? = null,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = StreamSinkPatchSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: StreamSinkPatch) {
        val surrogate =
            StreamSinkPatchSurrogate(
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
        encoder.encodeSerializableValue(StreamSinkPatchSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): StreamSinkPatch {
        val surrogate = decoder.decodeSerializableValue(StreamSinkPatchSurrogate.serializer())
        return StreamSinkPatch(
            uid = surrogate.uid,
            status = surrogate.status,
            batchSize = surrogate.batchSize,
            maxWaitSecs = surrogate.maxWaitSecs,
            eventTypes = surrogate.eventTypes,
            channels = surrogate.channels,
            metadata = surrogate.metadata,
            config = StreamSinkPatchConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
