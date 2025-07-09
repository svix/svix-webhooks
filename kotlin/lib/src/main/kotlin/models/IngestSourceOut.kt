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

@Serializable(with = IngestSourceOutSerializer::class)
data class IngestSourceOut(
    val createdAt: Instant,
    /** The Source's ID. */
    val id: String,
    val ingestUrl: String? = null,
    val metadata: Map<String, String>,
    val name: String,
    /** The Source's UID. */
    val uid: String? = null,
    val updatedAt: Instant,
    val config: IngestSourceOutConfig,
)

@Serializable
sealed class IngestSourceOutConfig {
    val variantName: String
        get() = this::class.annotations.filterIsInstance<VariantName>().first().name

    abstract fun toJsonElement(): JsonElement

    @VariantName("generic-webhook")
    data object GenericWebhook : IngestSourceOutConfig() {
        override fun toJsonElement() = buildJsonObject {}
    }

    @VariantName("cron")
    data class Cron(val cron: CronConfig) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(CronConfig.serializer(), cron)
    }

    @VariantName("adobe-sign")
    data class AdobeSign(val adobeSign: AdobeSignConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AdobeSignConfigOut.serializer(), adobeSign)
    }

    @VariantName("beehiiv")
    data class Beehiiv(val beehiiv: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), beehiiv)
    }

    @VariantName("brex")
    data class Brex(val brex: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), brex)
    }

    @VariantName("checkbook")
    data class Checkbook(val checkbook: CheckbookConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(CheckbookConfigOut.serializer(), checkbook)
    }

    @VariantName("clerk")
    data class Clerk(val clerk: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), clerk)
    }

    @VariantName("docusign")
    data class Docusign(val docusign: DocusignConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(DocusignConfigOut.serializer(), docusign)
    }

    @VariantName("github")
    data class Github(val github: GithubConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(GithubConfigOut.serializer(), github)
    }

    @VariantName("guesty")
    data class Guesty(val guesty: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), guesty)
    }

    @VariantName("hubspot")
    data class Hubspot(val hubspot: HubspotConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(HubspotConfigOut.serializer(), hubspot)
    }

    @VariantName("incident-io")
    data class IncidentIo(val incidentIo: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SvixConfigOut.serializer(), incidentIo)
    }

    @VariantName("lithic")
    data class Lithic(val lithic: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), lithic)
    }

    @VariantName("nash")
    data class Nash(val nash: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), nash)
    }

    @VariantName("orum-io")
    data class OrumIo(val orumIo: OrumIoConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(OrumIoConfigOut.serializer(), orumIo)
    }

    @VariantName("panda-doc")
    data class PandaDoc(val pandaDoc: PandaDocConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(PandaDocConfigOut.serializer(), pandaDoc)
    }

    @VariantName("pleo")
    data class Pleo(val pleo: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), pleo)
    }

    @VariantName("replicate")
    data class Replicate(val replicate: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SvixConfigOut.serializer(), replicate)
    }

    @VariantName("resend")
    data class Resend(val resend: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), resend)
    }

    @VariantName("rutter")
    data class Rutter(val rutter: RutterConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(RutterConfigOut.serializer(), rutter)
    }

    @VariantName("safebase")
    data class Safebase(val safebase: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SvixConfigOut.serializer(), safebase)
    }

    @VariantName("sardine")
    data class Sardine(val sardine: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), sardine)
    }

    @VariantName("segment")
    data class Segment(val segment: SegmentConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(SegmentConfigOut.serializer(), segment)
    }

    @VariantName("shopify")
    data class Shopify(val shopify: ShopifyConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(ShopifyConfigOut.serializer(), shopify)
    }

    @VariantName("slack")
    data class Slack(val slack: SlackConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SlackConfigOut.serializer(), slack)
    }

    @VariantName("stripe")
    data class Stripe(val stripe: StripeConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(StripeConfigOut.serializer(), stripe)
    }

    @VariantName("stych")
    data class Stych(val stych: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), stych)
    }

    @VariantName("svix")
    data class Svix(val svix: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), svix)
    }

    @VariantName("zoom")
    data class Zoom(val zoom: ZoomConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(ZoomConfigOut.serializer(), zoom)
    }

    @VariantName("telnyx")
    data class Telnyx(val telnyx: TelnyxConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(TelnyxConfigOut.serializer(), telnyx)
    }

    @VariantName("open-ai")
    data class OpenAi(val openAi: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), openAi)
    }

    @VariantName("render")
    data class Render(val render: SvixConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() = Json.encodeToJsonElement(SvixConfigOut.serializer(), render)
    }

    @VariantName("veriff")
    data class Veriff(val veriff: VeriffConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(VeriffConfigOut.serializer(), veriff)
    }

    @VariantName("airwallex")
    data class Airwallex(val airwallex: AirwallexConfigOut) : IngestSourceOutConfig() {
        override fun toJsonElement() =
            Json.encodeToJsonElement(AirwallexConfigOut.serializer(), airwallex)
    }

    companion object {
        private val typeMap =
            mapOf<String, (JsonElement) -> IngestSourceOutConfig>(
                "generic-webhook" to { _ -> GenericWebhook },
                "cron" to
                    { config ->
                        Cron(Json.decodeFromJsonElement(CronConfig.serializer(), config))
                    },
                "adobe-sign" to
                    { config ->
                        AdobeSign(
                            Json.decodeFromJsonElement(AdobeSignConfigOut.serializer(), config)
                        )
                    },
                "beehiiv" to
                    { config ->
                        Beehiiv(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "brex" to
                    { config ->
                        Brex(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "checkbook" to
                    { config ->
                        Checkbook(
                            Json.decodeFromJsonElement(CheckbookConfigOut.serializer(), config)
                        )
                    },
                "clerk" to
                    { config ->
                        Clerk(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "docusign" to
                    { config ->
                        Docusign(Json.decodeFromJsonElement(DocusignConfigOut.serializer(), config))
                    },
                "github" to
                    { config ->
                        Github(Json.decodeFromJsonElement(GithubConfigOut.serializer(), config))
                    },
                "guesty" to
                    { config ->
                        Guesty(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "hubspot" to
                    { config ->
                        Hubspot(Json.decodeFromJsonElement(HubspotConfigOut.serializer(), config))
                    },
                "incident-io" to
                    { config ->
                        IncidentIo(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "lithic" to
                    { config ->
                        Lithic(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "nash" to
                    { config ->
                        Nash(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "orum-io" to
                    { config ->
                        OrumIo(Json.decodeFromJsonElement(OrumIoConfigOut.serializer(), config))
                    },
                "panda-doc" to
                    { config ->
                        PandaDoc(Json.decodeFromJsonElement(PandaDocConfigOut.serializer(), config))
                    },
                "pleo" to
                    { config ->
                        Pleo(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "replicate" to
                    { config ->
                        Replicate(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "resend" to
                    { config ->
                        Resend(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "rutter" to
                    { config ->
                        Rutter(Json.decodeFromJsonElement(RutterConfigOut.serializer(), config))
                    },
                "safebase" to
                    { config ->
                        Safebase(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "sardine" to
                    { config ->
                        Sardine(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "segment" to
                    { config ->
                        Segment(Json.decodeFromJsonElement(SegmentConfigOut.serializer(), config))
                    },
                "shopify" to
                    { config ->
                        Shopify(Json.decodeFromJsonElement(ShopifyConfigOut.serializer(), config))
                    },
                "slack" to
                    { config ->
                        Slack(Json.decodeFromJsonElement(SlackConfigOut.serializer(), config))
                    },
                "stripe" to
                    { config ->
                        Stripe(Json.decodeFromJsonElement(StripeConfigOut.serializer(), config))
                    },
                "stych" to
                    { config ->
                        Stych(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "svix" to
                    { config ->
                        Svix(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "zoom" to
                    { config ->
                        Zoom(Json.decodeFromJsonElement(ZoomConfigOut.serializer(), config))
                    },
                "telnyx" to
                    { config ->
                        Telnyx(Json.decodeFromJsonElement(TelnyxConfigOut.serializer(), config))
                    },
                "open-ai" to
                    { config ->
                        OpenAi(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "render" to
                    { config ->
                        Render(Json.decodeFromJsonElement(SvixConfigOut.serializer(), config))
                    },
                "veriff" to
                    { config ->
                        Veriff(Json.decodeFromJsonElement(VeriffConfigOut.serializer(), config))
                    },
                "airwallex" to
                    { config ->
                        Airwallex(
                            Json.decodeFromJsonElement(AirwallexConfigOut.serializer(), config)
                        )
                    },
            )

        fun fromTypeAndConfig(type: String, config: JsonElement): IngestSourceOutConfig {
            return typeMap[type]?.invoke(config)
                ?: throw SerializationException("Unknown type: $type")
        }
    }
}

class IngestSourceOutSerializer : KSerializer<IngestSourceOut> {
    @Serializable
    private data class IngestSourceOutSurrogate(
        val createdAt: Instant,
        /** The Source's ID. */
        val id: String,
        val ingestUrl: String? = null,
        val metadata: Map<String, String>,
        val name: String,
        /** The Source's UID. */
        val uid: String? = null,
        val updatedAt: Instant,
        val type: String,
        val config: JsonElement,
    )

    override val descriptor: SerialDescriptor = IngestSourceOutSurrogate.serializer().descriptor

    override fun serialize(encoder: Encoder, value: IngestSourceOut) {
        val surrogate =
            IngestSourceOutSurrogate(
                createdAt = value.createdAt,
                id = value.id,
                ingestUrl = value.ingestUrl,
                metadata = value.metadata,
                name = value.name,
                uid = value.uid,
                updatedAt = value.updatedAt,
                type = value.config.variantName,
                config = value.config.toJsonElement(),
            )
        encoder.encodeSerializableValue(IngestSourceOutSurrogate.serializer(), surrogate)
    }

    override fun deserialize(decoder: Decoder): IngestSourceOut {
        val surrogate = decoder.decodeSerializableValue(IngestSourceOutSurrogate.serializer())
        return IngestSourceOut(
            createdAt = surrogate.createdAt,
            id = surrogate.id,
            ingestUrl = surrogate.ingestUrl,
            metadata = surrogate.metadata,
            name = surrogate.name,
            uid = surrogate.uid,
            updatedAt = surrogate.updatedAt,
            config = IngestSourceOutConfig.fromTypeAndConfig(surrogate.type, surrogate.config),
        )
    }
}
