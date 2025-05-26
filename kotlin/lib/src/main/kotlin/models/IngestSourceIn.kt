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

@Serializable(with = IngestSourceInSerializer::class)
data class IngestSourceIn(
    val name: String,
    /** The Source's UID. */
    val uid: String? = null,
    val config: IngestSourceInConfig,
)

@Serializable
sealed class IngestSourceInConfig {
    val variantName: String
        get() = this::class.annotations.filterIsInstance<VariantName>().first().name

    abstract fun toJsonElement(): JsonElement

    @VariantName("generic-webhook")
    data object GenericWebhook : IngestSourceInConfig() {
        override fun toJsonElement() = buildJsonObject {}
    }

    @VariantName("cron")
    data class Cron(val cron: CronConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(CronConfig.serializer(), cron)
    }

    @VariantName("adobe-sign")
    data class AdobeSign(val adobeSign: AdobeSignConfig) : IngestSourceInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AdobeSignConfig.serializer(), adobeSign)
    }

    @VariantName("beehiiv")
    data class Beehiiv(val beehiiv: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), beehiiv)
    }

    @VariantName("brex")
    data class Brex(val brex: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), brex)
    }

    @VariantName("clerk")
    data class Clerk(val clerk: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), clerk)
    }

    @VariantName("docusign")
    data class Docusign(val docusign: DocusignConfig) : IngestSourceInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(DocusignConfig.serializer(), docusign)
    }

    @VariantName("github")
    data class Github(val github: GithubConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(GithubConfig.serializer(), github)
    }

    @VariantName("guesty")
    data class Guesty(val guesty: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), guesty)
    }

    @VariantName("hubspot")
    data class Hubspot(val hubspot: HubspotConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(HubspotConfig.serializer(), hubspot)
    }

    @VariantName("incident-io")
    data class IncidentIo(val incidentIo: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), incidentIo)
    }

    @VariantName("lithic")
    data class Lithic(val lithic: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), lithic)
    }

    @VariantName("nash")
    data class Nash(val nash: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), nash)
    }

    @VariantName("panda-doc")
    data class PandaDoc(val pandaDoc: PandaDocConfig) : IngestSourceInConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(PandaDocConfig.serializer(), pandaDoc)
    }

    @VariantName("pleo")
    data class Pleo(val pleo: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), pleo)
    }

    @VariantName("replicate")
    data class Replicate(val replicate: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), replicate)
    }

    @VariantName("resend")
    data class Resend(val resend: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), resend)
    }

    @VariantName("safebase")
    data class Safebase(val safebase: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), safebase)
    }

    @VariantName("sardine")
    data class Sardine(val sardine: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), sardine)
    }

    @VariantName("segment")
    data class Segment(val segment: SegmentConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SegmentConfig.serializer(), segment)
    }

    @VariantName("shopify")
    data class Shopify(val shopify: ShopifyConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(ShopifyConfig.serializer(), shopify)
    }

    @VariantName("slack")
    data class Slack(val slack: SlackConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SlackConfig.serializer(), slack)
    }

    @VariantName("stripe")
    data class Stripe(val stripe: StripeConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(StripeConfig.serializer(), stripe)
    }

    @VariantName("stych")
    data class Stych(val stych: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), stych)
    }

    @VariantName("svix")
    data class Svix(val svix: SvixConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfig.serializer(), svix)
    }

    @VariantName("zoom")
    data class Zoom(val zoom: ZoomConfig) : IngestSourceInConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(ZoomConfig.serializer(), zoom)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> IngestSourceInConfig>(
                "generic-webhook" to { _ -> GenericWebhook },
                "cron" to
                    { config ->
                        Cron(Json.decodeFromJsonElement(CronConfig.serializer(), config))
                    },
                "adobe-sign" to
                    { config ->
                        AdobeSign(Json.decodeFromJsonElement(AdobeSignConfig.serializer(), config))
                    },
                "beehiiv" to
                    { config ->
                        Beehiiv(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "brex" to
                    { config ->
                        Brex(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "clerk" to
                    { config ->
                        Clerk(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "docusign" to
                    { config ->
                        Docusign(Json.decodeFromJsonElement(DocusignConfig.serializer(), config))
                    },
                "github" to
                    { config ->
                        Github(Json.decodeFromJsonElement(GithubConfig.serializer(), config))
                    },
                "guesty" to
                    { config ->
                        Guesty(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "hubspot" to
                    { config ->
                        Hubspot(Json.decodeFromJsonElement(HubspotConfig.serializer(), config))
                    },
                "incident-io" to
                    { config ->
                        IncidentIo(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "lithic" to
                    { config ->
                        Lithic(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "nash" to
                    { config ->
                        Nash(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "panda-doc" to
                    { config ->
                        PandaDoc(Json.decodeFromJsonElement(PandaDocConfig.serializer(), config))
                    },
                "pleo" to
                    { config ->
                        Pleo(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "replicate" to
                    { config ->
                        Replicate(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "resend" to
                    { config ->
                        Resend(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "safebase" to
                    { config ->
                        Safebase(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "sardine" to
                    { config ->
                        Sardine(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "segment" to
                    { config ->
                        Segment(Json.decodeFromJsonElement(SegmentConfig.serializer(), config))
                    },
                "shopify" to
                    { config ->
                        Shopify(Json.decodeFromJsonElement(ShopifyConfig.serializer(), config))
                    },
                "slack" to
                    { config ->
                        Slack(Json.decodeFromJsonElement(SlackConfig.serializer(), config))
                    },
                "stripe" to
                    { config ->
                        Stripe(Json.decodeFromJsonElement(StripeConfig.serializer(), config))
                    },
                "stych" to
                    { config ->
                        Stych(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "svix" to
                    { config ->
                        Svix(Json.decodeFromJsonElement(SvixConfig.serializer(), config))
                    },
                "zoom" to
                    { config ->
                        Zoom(Json.decodeFromJsonElement(ZoomConfig.serializer(), config))
                    },
            )

        fun fromTypeAndConfig(type: String, config: JsonElement): IngestSourceInConfig {
            return typeMap[type]?.invoke(config)
                ?: throw SerializationException("Unknown type: $type")
        }
    }
}

class IngestSourceInSerializer : KSerializer<IngestSourceIn> {
    @Serializable
    private data class IngestSourceInSurrogate(
        val name: String,
        /** The Source's UID. */
        val uid: String? = null,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = IngestSourceInSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: IngestSourceIn) {
        val surrogate =
            IngestSourceInSurrogate(
                name = value.name,
                uid = value.uid,
                type = value.config.variantName,
                config = value.config.toJsonElement(),
            )
        encoder.encodeSerializableValue(IngestSourceInSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): IngestSourceIn {
        val surrogate = decoder.decodeSerializableValue(IngestSourceInSurrogate.serializer())
        return IngestSourceIn(
            name = surrogate.name,
            uid = surrogate.uid,
            config = IngestSourceInConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
