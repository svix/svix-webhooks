// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
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
    val batchSize: Int,
    val createdAt: Instant,
    val currentIterator: String,
    val eventTypes: List<String>? = null,
    val failureReason: String? = null,
    /** The sink's ID. */
    val id: String,
    val maxWaitSecs: Int,
    val metadata: Map<String, String>,
    val nextRetryAt: Instant? = null,
    val status: SinkStatus,
    /** The sink's UID. */
    val uid: String? = null,
    val updatedAt: Instant,
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
        val batchSize: Int,
        val createdAt: Instant,
        val currentIterator: String,
        val eventTypes: List<String>? = null,
        val failureReason: String? = null,
        /** The sink's ID. */
        val id: String,
        val maxWaitSecs: Int,
        val metadata: Map<String, String>,
        val nextRetryAt: Instant? = null,
        val status: SinkStatus,
        /** The sink's UID. */
        val uid: String? = null,
        val updatedAt: Instant,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = StreamSinkOutSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: StreamSinkOut) {
        val surrogate =
            StreamSinkOutSurrogate(
                batchSize = value.batchSize,
                createdAt = value.createdAt,
                currentIterator = value.currentIterator,
                eventTypes = value.eventTypes,
                failureReason = value.failureReason,
                id = value.id,
                maxWaitSecs = value.maxWaitSecs,
                metadata = value.metadata,
                nextRetryAt = value.nextRetryAt,
                status = value.status,
                uid = value.uid,
                updatedAt = value.updatedAt,
                type = value.config.variantName,
                config = value.config.toJsonElement(),
            )
        encoder.encodeSerializableValue(StreamSinkOutSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): StreamSinkOut {
        val surrogate = decoder.decodeSerializableValue(StreamSinkOutSurrogate.serializer())
        return StreamSinkOut(
            batchSize = surrogate.batchSize,
            createdAt = surrogate.createdAt,
            currentIterator = surrogate.currentIterator,
            eventTypes = surrogate.eventTypes,
            failureReason = surrogate.failureReason,
            id = surrogate.id,
            maxWaitSecs = surrogate.maxWaitSecs,
            metadata = surrogate.metadata,
            nextRetryAt = surrogate.nextRetryAt,
            status = surrogate.status,
            uid = surrogate.uid,
            updatedAt = surrogate.updatedAt,
            config = StreamSinkOutConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
