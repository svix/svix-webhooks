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
    /** How many events will be batched in a request to the Sink. */
    val batchSize: UShort? = null,
    /**
     * A list of event types that filter which events are dispatched to the Sink. An empty list (or
     * null) will not filter out any events.
     */
    val eventTypes: List<String>? = null,
    /**
     * How long to wait before a batch of events is sent, if the `batchSize` is not reached.
     *
     * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request after
     * 10 seconds or 100 events, whichever comes first.
     *
     * Note that we will never send an empty batch of events to the Sink.
     */
    val maxWaitSecs: UShort? = null,
    val metadata: Map<String, String>? = null,
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
    /** An optional unique identifier for the sink. */
    val uid: String? = null,
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
        /** How many events will be batched in a request to the Sink. */
        val batchSize: UShort? = null,
        /**
         * A list of event types that filter which events are dispatched to the Sink. An empty list
         * (or null) will not filter out any events.
         */
        val eventTypes: List<String>? = null,
        /**
         * How long to wait before a batch of events is sent, if the `batchSize` is not reached.
         *
         * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request
         * after 10 seconds or 100 events, whichever comes first.
         *
         * Note that we will never send an empty batch of events to the Sink.
         */
        val maxWaitSecs: UShort? = null,
        val metadata: Map<String, String>? = null,
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
        /** An optional unique identifier for the sink. */
        val uid: String? = null,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = StreamSinkInSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: StreamSinkIn) {
        val surrogate =
            StreamSinkInSurrogate(
                batchSize = value.batchSize,
                eventTypes = value.eventTypes,
                maxWaitSecs = value.maxWaitSecs,
                metadata = value.metadata,
                status = value.status,
                uid = value.uid,
                type = value.config.variantName,
                config = value.config.toJsonElement(),
            )
        encoder.encodeSerializableValue(StreamSinkInSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): StreamSinkIn {
        val surrogate = decoder.decodeSerializableValue(StreamSinkInSurrogate.serializer())
        return StreamSinkIn(
            batchSize = surrogate.batchSize,
            eventTypes = surrogate.eventTypes,
            maxWaitSecs = surrogate.maxWaitSecs,
            metadata = surrogate.metadata,
            status = surrogate.status,
            uid = surrogate.uid,
            config = StreamSinkInConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
