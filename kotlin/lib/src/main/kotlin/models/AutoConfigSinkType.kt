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

@Serializable(with = AutoConfigSinkTypeSerializer::class)
data class AutoConfigSinkType(val config: AutoConfigSinkTypeConfig)

@Serializable
sealed class AutoConfigSinkTypeConfig {
    val variantName: String
        get() = this::class.annotations.filterIsInstance<VariantName>().first().name

    abstract fun toJsonElement(): JsonElement

    @VariantName("poller")
    data class Poller(val poller: SinkInCommon) : AutoConfigSinkTypeConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SinkInCommon.serializer(), poller)
    }

    @VariantName("http")
    data class Http(val http: EndpointIn) : AutoConfigSinkTypeConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(EndpointIn.serializer(), http)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> AutoConfigSinkTypeConfig>(
                "poller" to
                    { config ->
                        Poller(Json.decodeFromJsonElement(SinkInCommon.serializer(), config))
                    },
                "http" to
                    { config ->
                        Http(Json.decodeFromJsonElement(EndpointIn.serializer(), config))
                    },
            )

        fun fromTypeAndConfig(type: String, config: JsonElement): AutoConfigSinkTypeConfig {
            return typeMap[type]?.invoke(config)
                ?: throw SerializationException("Unknown type: $type")
        }
    }
}

class AutoConfigSinkTypeSerializer : KSerializer<AutoConfigSinkType> {
    @Serializable
    private data class AutoConfigSinkTypeSurrogate(val type: String, val config: JsonElement)

    override val descriptor: SerialDescriptor = AutoConfigSinkTypeSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: AutoConfigSinkType) {
        val surrogate =
            AutoConfigSinkTypeSurrogate(
                type = value.config.variantName,
                config = value.config.toJsonElement(),
            )
        encoder.encodeSerializableValue(AutoConfigSinkTypeSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): AutoConfigSinkType {
        val surrogate = decoder.decodeSerializableValue(AutoConfigSinkTypeSurrogate.serializer())
        return AutoConfigSinkType(
            config = AutoConfigSinkTypeConfig.fromTypeAndConfig(surrogate.type, surrogate.config)
        )
    }
}
