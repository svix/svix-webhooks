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
    val batchSize: MaybeUnset<UShort> = MaybeUnset.Unset,
    val eventTypes: List<String>? = null,
    val maxWaitSecs: MaybeUnset<UShort> = MaybeUnset.Unset,
    val metadata: Map<String, String>? = null,
    val status: MaybeUnset<SinkStatusIn> = MaybeUnset.Unset,
    /** The StreamSink's UID. */
    val uid: MaybeUnset<String> = MaybeUnset.Unset,
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
    data class AzureBlobStorage(val azureBlobStorage: AzureBlobStoragePatchConfig) :
        StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AzureBlobStoragePatchConfig.serializer(), azureBlobStorage)
    }

    @VariantName("otelTracing")
    data class OtelTracing(val otelTracing: OtelTracingPatchConfig) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(OtelTracingPatchConfig.serializer(), otelTracing)
    }

    @VariantName("http")
    data class Http(val http: HttpPatchConfig) : StreamSinkPatchConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(HttpPatchConfig.serializer(), http)
    }

    @VariantName("amazonS3")
    data class AmazonS3(val amazonS3: AmazonS3PatchConfig) : StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AmazonS3PatchConfig.serializer(), amazonS3)
    }

    @VariantName("googleCloudStorage")
    data class GoogleCloudStorage(val googleCloudStorage: GoogleCloudStoragePatchConfig) :
        StreamSinkPatchConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GoogleCloudStoragePatchConfig.serializer(), googleCloudStorage)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> StreamSinkPatchConfig>(
                "poller" to { _ -> Poller },
                "azureBlobStorage" to
                    { config ->
                        AzureBlobStorage(
                            Json.decodeFromJsonElement(
                                AzureBlobStoragePatchConfig.serializer(),
                                config,
                            )
                        )
                    },
                "otelTracing" to
                    { config ->
                        OtelTracing(
                            Json.decodeFromJsonElement(OtelTracingPatchConfig.serializer(), config)
                        )
                    },
                "http" to
                    { config ->
                        Http(Json.decodeFromJsonElement(HttpPatchConfig.serializer(), config))
                    },
                "amazonS3" to
                    { config ->
                        AmazonS3(
                            Json.decodeFromJsonElement(AmazonS3PatchConfig.serializer(), config)
                        )
                    },
                "googleCloudStorage" to
                    { config ->
                        GoogleCloudStorage(
                            Json.decodeFromJsonElement(
                                GoogleCloudStoragePatchConfig.serializer(),
                                config,
                            )
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
        val batchSize: MaybeUnset<UShort> = MaybeUnset.Unset,
        val eventTypes: List<String>? = null,
        val maxWaitSecs: MaybeUnset<UShort> = MaybeUnset.Unset,
        val metadata: Map<String, String>? = null,
        val status: MaybeUnset<SinkStatusIn> = MaybeUnset.Unset,
        /** The StreamSink's UID. */
        val uid: MaybeUnset<String> = MaybeUnset.Unset,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = StreamSinkPatchSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: StreamSinkPatch) {
        val surrogate =
            StreamSinkPatchSurrogate(
                batchSize = value.batchSize,
                eventTypes = value.eventTypes,
                maxWaitSecs = value.maxWaitSecs,
                metadata = value.metadata,
                status = value.status,
                uid = value.uid,
                type = value.config.variantName,
                config = value.config.toJsonElement(),
            )
        encoder.encodeSerializableValue(StreamSinkPatchSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): StreamSinkPatch {
        val surrogate = decoder.decodeSerializableValue(StreamSinkPatchSurrogate.serializer())
        return StreamSinkPatch(
            batchSize = surrogate.batchSize,
            eventTypes = surrogate.eventTypes,
            maxWaitSecs = surrogate.maxWaitSecs,
            metadata = surrogate.metadata,
            status = surrogate.status,
            uid = surrogate.uid,
            config = StreamSinkPatchConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
