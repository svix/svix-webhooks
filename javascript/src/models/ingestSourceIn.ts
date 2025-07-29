// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { AdobeSignConfig, AdobeSignConfigSerializer } from "./adobeSignConfig";
import { AirwallexConfig, AirwallexConfigSerializer } from "./airwallexConfig";
import { CheckbookConfig, CheckbookConfigSerializer } from "./checkbookConfig";
import { CronConfig, CronConfigSerializer } from "./cronConfig";
import { DocusignConfig, DocusignConfigSerializer } from "./docusignConfig";
import { EasypostConfig, EasypostConfigSerializer } from "./easypostConfig";
import { GithubConfig, GithubConfigSerializer } from "./githubConfig";
import { HubspotConfig, HubspotConfigSerializer } from "./hubspotConfig";
import { OrumIoConfig, OrumIoConfigSerializer } from "./orumIoConfig";
import { PandaDocConfig, PandaDocConfigSerializer } from "./pandaDocConfig";
import { PortIoConfig, PortIoConfigSerializer } from "./portIoConfig";
import { RutterConfig, RutterConfigSerializer } from "./rutterConfig";
import { SegmentConfig, SegmentConfigSerializer } from "./segmentConfig";
import { ShopifyConfig, ShopifyConfigSerializer } from "./shopifyConfig";
import { SlackConfig, SlackConfigSerializer } from "./slackConfig";
import { StripeConfig, StripeConfigSerializer } from "./stripeConfig";
import { SvixConfig, SvixConfigSerializer } from "./svixConfig";
import { TelnyxConfig, TelnyxConfigSerializer } from "./telnyxConfig";
import { VapiConfig, VapiConfigSerializer } from "./vapiConfig";
import { VeriffConfig, VeriffConfigSerializer } from "./veriffConfig";
import { ZoomConfig, ZoomConfigSerializer } from "./zoomConfig";
interface _IngestSourceInFields {
  metadata?: { [key: string]: string };
  name: string;
  /** The Source's UID. */
  uid?: string | null;
}

/* eslint @typescript-eslint/no-empty-object-type: 0 */
interface IngestSourceInGenericWebhookConfig {}

interface IngestSourceInGenericWebhook {
  type: "generic-webhook";
  config?: IngestSourceInGenericWebhookConfig;
}

interface IngestSourceInCron {
  type: "cron";
  config: CronConfig;
}

interface IngestSourceInAdobeSign {
  type: "adobe-sign";
  config: AdobeSignConfig;
}

interface IngestSourceInBeehiiv {
  type: "beehiiv";
  config: SvixConfig;
}

interface IngestSourceInBrex {
  type: "brex";
  config: SvixConfig;
}

interface IngestSourceInCheckbook {
  type: "checkbook";
  config: CheckbookConfig;
}

interface IngestSourceInClerk {
  type: "clerk";
  config: SvixConfig;
}

interface IngestSourceInDocusign {
  type: "docusign";
  config: DocusignConfig;
}

interface IngestSourceInEasypost {
  type: "easypost";
  config: EasypostConfig;
}

interface IngestSourceInGithub {
  type: "github";
  config: GithubConfig;
}

interface IngestSourceInGuesty {
  type: "guesty";
  config: SvixConfig;
}

interface IngestSourceInHubspot {
  type: "hubspot";
  config: HubspotConfig;
}

interface IngestSourceInIncidentIo {
  type: "incident-io";
  config: SvixConfig;
}

interface IngestSourceInLithic {
  type: "lithic";
  config: SvixConfig;
}

interface IngestSourceInNash {
  type: "nash";
  config: SvixConfig;
}

interface IngestSourceInOrumIo {
  type: "orum-io";
  config: OrumIoConfig;
}

interface IngestSourceInPandaDoc {
  type: "panda-doc";
  config: PandaDocConfig;
}

interface IngestSourceInPortIo {
  type: "port-io";
  config: PortIoConfig;
}

interface IngestSourceInPleo {
  type: "pleo";
  config: SvixConfig;
}

interface IngestSourceInReplicate {
  type: "replicate";
  config: SvixConfig;
}

interface IngestSourceInResend {
  type: "resend";
  config: SvixConfig;
}

interface IngestSourceInRutter {
  type: "rutter";
  config: RutterConfig;
}

interface IngestSourceInSafebase {
  type: "safebase";
  config: SvixConfig;
}

interface IngestSourceInSardine {
  type: "sardine";
  config: SvixConfig;
}

interface IngestSourceInSegment {
  type: "segment";
  config: SegmentConfig;
}

interface IngestSourceInShopify {
  type: "shopify";
  config: ShopifyConfig;
}

interface IngestSourceInSlack {
  type: "slack";
  config: SlackConfig;
}

interface IngestSourceInStripe {
  type: "stripe";
  config: StripeConfig;
}

interface IngestSourceInStych {
  type: "stych";
  config: SvixConfig;
}

interface IngestSourceInSvix {
  type: "svix";
  config: SvixConfig;
}

interface IngestSourceInZoom {
  type: "zoom";
  config: ZoomConfig;
}

interface IngestSourceInTelnyx {
  type: "telnyx";
  config: TelnyxConfig;
}

interface IngestSourceInVapi {
  type: "vapi";
  config: VapiConfig;
}

interface IngestSourceInOpenAi {
  type: "open-ai";
  config: SvixConfig;
}

interface IngestSourceInRender {
  type: "render";
  config: SvixConfig;
}

interface IngestSourceInVeriff {
  type: "veriff";
  config: VeriffConfig;
}

interface IngestSourceInAirwallex {
  type: "airwallex";
  config: AirwallexConfig;
}

export type IngestSourceIn = _IngestSourceInFields &
  (
    | IngestSourceInGenericWebhook
    | IngestSourceInCron
    | IngestSourceInAdobeSign
    | IngestSourceInBeehiiv
    | IngestSourceInBrex
    | IngestSourceInCheckbook
    | IngestSourceInClerk
    | IngestSourceInDocusign
    | IngestSourceInEasypost
    | IngestSourceInGithub
    | IngestSourceInGuesty
    | IngestSourceInHubspot
    | IngestSourceInIncidentIo
    | IngestSourceInLithic
    | IngestSourceInNash
    | IngestSourceInOrumIo
    | IngestSourceInPandaDoc
    | IngestSourceInPortIo
    | IngestSourceInPleo
    | IngestSourceInReplicate
    | IngestSourceInResend
    | IngestSourceInRutter
    | IngestSourceInSafebase
    | IngestSourceInSardine
    | IngestSourceInSegment
    | IngestSourceInShopify
    | IngestSourceInSlack
    | IngestSourceInStripe
    | IngestSourceInStych
    | IngestSourceInSvix
    | IngestSourceInZoom
    | IngestSourceInTelnyx
    | IngestSourceInVapi
    | IngestSourceInOpenAi
    | IngestSourceInRender
    | IngestSourceInVeriff
    | IngestSourceInAirwallex
  );

export const IngestSourceInSerializer = {
  _fromJsonObject(object: any): IngestSourceIn {
    const type = object["type"];

    function getConfig(type: string): any {
      switch (type) {
        case "generic-webhook":
          return {};
        case "cron":
          return CronConfigSerializer._fromJsonObject(object["config"]);
        case "adobe-sign":
          return AdobeSignConfigSerializer._fromJsonObject(object["config"]);
        case "beehiiv":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "brex":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "checkbook":
          return CheckbookConfigSerializer._fromJsonObject(object["config"]);
        case "clerk":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "docusign":
          return DocusignConfigSerializer._fromJsonObject(object["config"]);
        case "easypost":
          return EasypostConfigSerializer._fromJsonObject(object["config"]);
        case "github":
          return GithubConfigSerializer._fromJsonObject(object["config"]);
        case "guesty":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "hubspot":
          return HubspotConfigSerializer._fromJsonObject(object["config"]);
        case "incident-io":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "lithic":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "nash":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "orum-io":
          return OrumIoConfigSerializer._fromJsonObject(object["config"]);
        case "panda-doc":
          return PandaDocConfigSerializer._fromJsonObject(object["config"]);
        case "port-io":
          return PortIoConfigSerializer._fromJsonObject(object["config"]);
        case "pleo":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "replicate":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "resend":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "rutter":
          return RutterConfigSerializer._fromJsonObject(object["config"]);
        case "safebase":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "sardine":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "segment":
          return SegmentConfigSerializer._fromJsonObject(object["config"]);
        case "shopify":
          return ShopifyConfigSerializer._fromJsonObject(object["config"]);
        case "slack":
          return SlackConfigSerializer._fromJsonObject(object["config"]);
        case "stripe":
          return StripeConfigSerializer._fromJsonObject(object["config"]);
        case "stych":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "svix":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "zoom":
          return ZoomConfigSerializer._fromJsonObject(object["config"]);
        case "telnyx":
          return TelnyxConfigSerializer._fromJsonObject(object["config"]);
        case "vapi":
          return VapiConfigSerializer._fromJsonObject(object["config"]);
        case "open-ai":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "render":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "veriff":
          return VeriffConfigSerializer._fromJsonObject(object["config"]);
        case "airwallex":
          return AirwallexConfigSerializer._fromJsonObject(object["config"]);
        default:
          throw new Error(`Unexpected type: ${type}`);
      }
    }
    return {
      type,
      config: getConfig(type),
      metadata: object["metadata"],
      name: object["name"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: IngestSourceIn): any {
    let config;
    switch (self.type) {
      case "generic-webhook":
        config = {};
        break;
      case "cron":
        config = CronConfigSerializer._toJsonObject(self.config);
        break;
      case "adobe-sign":
        config = AdobeSignConfigSerializer._toJsonObject(self.config);
        break;
      case "beehiiv":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "brex":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "checkbook":
        config = CheckbookConfigSerializer._toJsonObject(self.config);
        break;
      case "clerk":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "docusign":
        config = DocusignConfigSerializer._toJsonObject(self.config);
        break;
      case "easypost":
        config = EasypostConfigSerializer._toJsonObject(self.config);
        break;
      case "github":
        config = GithubConfigSerializer._toJsonObject(self.config);
        break;
      case "guesty":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "hubspot":
        config = HubspotConfigSerializer._toJsonObject(self.config);
        break;
      case "incident-io":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "lithic":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "nash":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "orum-io":
        config = OrumIoConfigSerializer._toJsonObject(self.config);
        break;
      case "panda-doc":
        config = PandaDocConfigSerializer._toJsonObject(self.config);
        break;
      case "port-io":
        config = PortIoConfigSerializer._toJsonObject(self.config);
        break;
      case "pleo":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "replicate":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "resend":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "rutter":
        config = RutterConfigSerializer._toJsonObject(self.config);
        break;
      case "safebase":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "sardine":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "segment":
        config = SegmentConfigSerializer._toJsonObject(self.config);
        break;
      case "shopify":
        config = ShopifyConfigSerializer._toJsonObject(self.config);
        break;
      case "slack":
        config = SlackConfigSerializer._toJsonObject(self.config);
        break;
      case "stripe":
        config = StripeConfigSerializer._toJsonObject(self.config);
        break;
      case "stych":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "svix":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "zoom":
        config = ZoomConfigSerializer._toJsonObject(self.config);
        break;
      case "telnyx":
        config = TelnyxConfigSerializer._toJsonObject(self.config);
        break;
      case "vapi":
        config = VapiConfigSerializer._toJsonObject(self.config);
        break;
      case "open-ai":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "render":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "veriff":
        config = VeriffConfigSerializer._toJsonObject(self.config);
        break;
      case "airwallex":
        config = AirwallexConfigSerializer._toJsonObject(self.config);
        break;
    }

    return {
      type: self.type,
      config: config,
      metadata: self.metadata,
      name: self.name,
      uid: self.uid,
    };
  },
};
