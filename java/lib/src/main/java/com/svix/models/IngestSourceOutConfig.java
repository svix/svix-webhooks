// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.svix.Utils;

import lombok.*;

import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
public abstract class IngestSourceOutConfig {
    @JsonIgnore
    public String getVariantName() {
        VariantName annotation = this.getClass().getAnnotation(VariantName.class);
        return annotation != null ? annotation.value() : null;
    }

    public abstract JsonNode toJsonNode();

    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("generic-webhook")
    public static class GenericWebhook extends IngestSourceOutConfig {
        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().createObjectNode();
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("cron")
    public static class Cron extends IngestSourceOutConfig {
        private final CronConfig cron;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(cron);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("adobe-sign")
    public static class AdobeSign extends IngestSourceOutConfig {
        private final AdobeSignConfigOut adobeSign;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(adobeSign);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("beehiiv")
    public static class Beehiiv extends IngestSourceOutConfig {
        private final SvixConfigOut beehiiv;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(beehiiv);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("brex")
    public static class Brex extends IngestSourceOutConfig {
        private final SvixConfigOut brex;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(brex);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("checkbook")
    public static class Checkbook extends IngestSourceOutConfig {
        private final CheckbookConfigOut checkbook;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(checkbook);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("clerk")
    public static class Clerk extends IngestSourceOutConfig {
        private final SvixConfigOut clerk;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(clerk);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("docusign")
    public static class Docusign extends IngestSourceOutConfig {
        private final DocusignConfigOut docusign;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(docusign);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("easypost")
    public static class Easypost extends IngestSourceOutConfig {
        private final EasypostConfigOut easypost;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(easypost);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("github")
    public static class Github extends IngestSourceOutConfig {
        private final GithubConfigOut github;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(github);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("guesty")
    public static class Guesty extends IngestSourceOutConfig {
        private final SvixConfigOut guesty;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(guesty);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("hubspot")
    public static class Hubspot extends IngestSourceOutConfig {
        private final HubspotConfigOut hubspot;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(hubspot);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("incident-io")
    public static class IncidentIo extends IngestSourceOutConfig {
        private final SvixConfigOut incidentIo;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(incidentIo);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("lithic")
    public static class Lithic extends IngestSourceOutConfig {
        private final SvixConfigOut lithic;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(lithic);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("nash")
    public static class Nash extends IngestSourceOutConfig {
        private final SvixConfigOut nash;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(nash);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("orum-io")
    public static class OrumIo extends IngestSourceOutConfig {
        private final OrumIoConfigOut orumIo;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(orumIo);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("panda-doc")
    public static class PandaDoc extends IngestSourceOutConfig {
        private final PandaDocConfigOut pandaDoc;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(pandaDoc);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("port-io")
    public static class PortIo extends IngestSourceOutConfig {
        private final PortIoConfigOut portIo;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(portIo);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("pleo")
    public static class Pleo extends IngestSourceOutConfig {
        private final SvixConfigOut pleo;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(pleo);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("replicate")
    public static class Replicate extends IngestSourceOutConfig {
        private final SvixConfigOut replicate;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(replicate);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("resend")
    public static class Resend extends IngestSourceOutConfig {
        private final SvixConfigOut resend;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(resend);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("rutter")
    public static class Rutter extends IngestSourceOutConfig {
        private final RutterConfigOut rutter;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(rutter);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("safebase")
    public static class Safebase extends IngestSourceOutConfig {
        private final SvixConfigOut safebase;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(safebase);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("sardine")
    public static class Sardine extends IngestSourceOutConfig {
        private final SvixConfigOut sardine;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(sardine);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("segment")
    public static class Segment extends IngestSourceOutConfig {
        private final SegmentConfigOut segment;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(segment);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("shopify")
    public static class Shopify extends IngestSourceOutConfig {
        private final ShopifyConfigOut shopify;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(shopify);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("slack")
    public static class Slack extends IngestSourceOutConfig {
        private final SlackConfigOut slack;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(slack);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("stripe")
    public static class Stripe extends IngestSourceOutConfig {
        private final StripeConfigOut stripe;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(stripe);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("stych")
    public static class Stych extends IngestSourceOutConfig {
        private final SvixConfigOut stych;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(stych);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("svix")
    public static class Svix extends IngestSourceOutConfig {
        private final SvixConfigOut svix;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(svix);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("zoom")
    public static class Zoom extends IngestSourceOutConfig {
        private final ZoomConfigOut zoom;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(zoom);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("telnyx")
    public static class Telnyx extends IngestSourceOutConfig {
        private final TelnyxConfigOut telnyx;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(telnyx);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("vapi")
    public static class Vapi extends IngestSourceOutConfig {
        private final VapiConfigOut vapi;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(vapi);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("open-ai")
    public static class OpenAi extends IngestSourceOutConfig {
        private final SvixConfigOut openAi;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(openAi);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("render")
    public static class Render extends IngestSourceOutConfig {
        private final SvixConfigOut render;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(render);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("veriff")
    public static class Veriff extends IngestSourceOutConfig {
        private final VeriffConfigOut veriff;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(veriff);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("airwallex")
    public static class Airwallex extends IngestSourceOutConfig {
        private final AirwallexConfigOut airwallex;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(airwallex);
        }
    }

    @FunctionalInterface
    private interface TypeFactory {
        IngestSourceOutConfig create(JsonNode config);
    }

    private static final Map<String, TypeFactory> TY_M = new HashMap<>();
    private static final ObjectMapper m = Utils.getObjectMapper();

    static {
        TY_M.put("generic-webhook", c -> new GenericWebhook());
        TY_M.put("cron", c -> new Cron(m.convertValue(c, CronConfig.class)));
        TY_M.put("adobe-sign", c -> new AdobeSign(m.convertValue(c, AdobeSignConfigOut.class)));
        TY_M.put("beehiiv", c -> new Beehiiv(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("brex", c -> new Brex(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("checkbook", c -> new Checkbook(m.convertValue(c, CheckbookConfigOut.class)));
        TY_M.put("clerk", c -> new Clerk(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("docusign", c -> new Docusign(m.convertValue(c, DocusignConfigOut.class)));
        TY_M.put("easypost", c -> new Easypost(m.convertValue(c, EasypostConfigOut.class)));
        TY_M.put("github", c -> new Github(m.convertValue(c, GithubConfigOut.class)));
        TY_M.put("guesty", c -> new Guesty(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("hubspot", c -> new Hubspot(m.convertValue(c, HubspotConfigOut.class)));
        TY_M.put("incident-io", c -> new IncidentIo(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("lithic", c -> new Lithic(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("nash", c -> new Nash(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("orum-io", c -> new OrumIo(m.convertValue(c, OrumIoConfigOut.class)));
        TY_M.put("panda-doc", c -> new PandaDoc(m.convertValue(c, PandaDocConfigOut.class)));
        TY_M.put("port-io", c -> new PortIo(m.convertValue(c, PortIoConfigOut.class)));
        TY_M.put("pleo", c -> new Pleo(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("replicate", c -> new Replicate(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("resend", c -> new Resend(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("rutter", c -> new Rutter(m.convertValue(c, RutterConfigOut.class)));
        TY_M.put("safebase", c -> new Safebase(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("sardine", c -> new Sardine(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("segment", c -> new Segment(m.convertValue(c, SegmentConfigOut.class)));
        TY_M.put("shopify", c -> new Shopify(m.convertValue(c, ShopifyConfigOut.class)));
        TY_M.put("slack", c -> new Slack(m.convertValue(c, SlackConfigOut.class)));
        TY_M.put("stripe", c -> new Stripe(m.convertValue(c, StripeConfigOut.class)));
        TY_M.put("stych", c -> new Stych(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("svix", c -> new Svix(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("zoom", c -> new Zoom(m.convertValue(c, ZoomConfigOut.class)));
        TY_M.put("telnyx", c -> new Telnyx(m.convertValue(c, TelnyxConfigOut.class)));
        TY_M.put("vapi", c -> new Vapi(m.convertValue(c, VapiConfigOut.class)));
        TY_M.put("open-ai", c -> new OpenAi(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("render", c -> new Render(m.convertValue(c, SvixConfigOut.class)));
        TY_M.put("veriff", c -> new Veriff(m.convertValue(c, VeriffConfigOut.class)));
        TY_M.put("airwallex", c -> new Airwallex(m.convertValue(c, AirwallexConfigOut.class)));
    }

    public static IngestSourceOutConfig fromTypeAndConfig(String type, JsonNode config) {
        TypeFactory factory = TY_M.get(type);
        if (factory == null) {
            throw new IllegalArgumentException("Unknown type: " + type);
        }
        return factory.create(config);
    }
}
