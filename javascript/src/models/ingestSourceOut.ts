// this file is @generated
import {
  type AdobeSignConfigOut,
  AdobeSignConfigOutSerializer,
} from "./adobeSignConfigOut";
import {
  type AirwallexConfigOut,
  AirwallexConfigOutSerializer,
} from "./airwallexConfigOut";
import {
  type CheckbookConfigOut,
  CheckbookConfigOutSerializer,
} from "./checkbookConfigOut";
import { type CronConfig, CronConfigSerializer } from "./cronConfig";
import { type DocusignConfigOut, DocusignConfigOutSerializer } from "./docusignConfigOut";
import { type EasypostConfigOut, EasypostConfigOutSerializer } from "./easypostConfigOut";
import { type GithubConfigOut, GithubConfigOutSerializer } from "./githubConfigOut";
import { type HubspotConfigOut, HubspotConfigOutSerializer } from "./hubspotConfigOut";
import { type OrumIoConfigOut, OrumIoConfigOutSerializer } from "./orumIoConfigOut";
import { type PandaDocConfigOut, PandaDocConfigOutSerializer } from "./pandaDocConfigOut";
import { type PortIoConfigOut, PortIoConfigOutSerializer } from "./portIoConfigOut";
import { type RutterConfigOut, RutterConfigOutSerializer } from "./rutterConfigOut";
import { type SegmentConfigOut, SegmentConfigOutSerializer } from "./segmentConfigOut";
import { type ShopifyConfigOut, ShopifyConfigOutSerializer } from "./shopifyConfigOut";
import { type SlackConfigOut, SlackConfigOutSerializer } from "./slackConfigOut";
import { type StripeConfigOut, StripeConfigOutSerializer } from "./stripeConfigOut";
import { type SvixConfigOut, SvixConfigOutSerializer } from "./svixConfigOut";
import { type TelnyxConfigOut, TelnyxConfigOutSerializer } from "./telnyxConfigOut";
import { type VapiConfigOut, VapiConfigOutSerializer } from "./vapiConfigOut";
import { type VeriffConfigOut, VeriffConfigOutSerializer } from "./veriffConfigOut";
import { type ZoomConfigOut, ZoomConfigOutSerializer } from "./zoomConfigOut";
interface _IngestSourceOutFields {
  createdAt: Date;
  /** The Source's ID. */
  id: string;
  ingestUrl?: string | null;
  metadata: { [key: string]: string };
  name: string;
  /** The Source's UID. */
  uid?: string | null;
  updatedAt: Date;
}

// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface IngestSourceOutGenericWebhookConfig {}

interface IngestSourceOutGenericWebhook {
  type: "generic-webhook";
  config?: IngestSourceOutGenericWebhookConfig;
}

interface IngestSourceOutCron {
  type: "cron";
  config: CronConfig;
}

interface IngestSourceOutAdobeSign {
  type: "adobe-sign";
  config: AdobeSignConfigOut;
}

interface IngestSourceOutBeehiiv {
  type: "beehiiv";
  config: SvixConfigOut;
}

interface IngestSourceOutBrex {
  type: "brex";
  config: SvixConfigOut;
}

interface IngestSourceOutCheckbook {
  type: "checkbook";
  config: CheckbookConfigOut;
}

interface IngestSourceOutClerk {
  type: "clerk";
  config: SvixConfigOut;
}

interface IngestSourceOutDocusign {
  type: "docusign";
  config: DocusignConfigOut;
}

interface IngestSourceOutEasypost {
  type: "easypost";
  config: EasypostConfigOut;
}

interface IngestSourceOutGithub {
  type: "github";
  config: GithubConfigOut;
}

interface IngestSourceOutGuesty {
  type: "guesty";
  config: SvixConfigOut;
}

interface IngestSourceOutHubspot {
  type: "hubspot";
  config: HubspotConfigOut;
}

interface IngestSourceOutIncidentIo {
  type: "incident-io";
  config: SvixConfigOut;
}

interface IngestSourceOutLithic {
  type: "lithic";
  config: SvixConfigOut;
}

interface IngestSourceOutNash {
  type: "nash";
  config: SvixConfigOut;
}

interface IngestSourceOutOrumIo {
  type: "orum-io";
  config: OrumIoConfigOut;
}

interface IngestSourceOutPandaDoc {
  type: "panda-doc";
  config: PandaDocConfigOut;
}

interface IngestSourceOutPortIo {
  type: "port-io";
  config: PortIoConfigOut;
}

interface IngestSourceOutPleo {
  type: "pleo";
  config: SvixConfigOut;
}

interface IngestSourceOutReplicate {
  type: "replicate";
  config: SvixConfigOut;
}

interface IngestSourceOutResend {
  type: "resend";
  config: SvixConfigOut;
}

interface IngestSourceOutRutter {
  type: "rutter";
  config: RutterConfigOut;
}

interface IngestSourceOutSafebase {
  type: "safebase";
  config: SvixConfigOut;
}

interface IngestSourceOutSardine {
  type: "sardine";
  config: SvixConfigOut;
}

interface IngestSourceOutSegment {
  type: "segment";
  config: SegmentConfigOut;
}

interface IngestSourceOutShopify {
  type: "shopify";
  config: ShopifyConfigOut;
}

interface IngestSourceOutSlack {
  type: "slack";
  config: SlackConfigOut;
}

interface IngestSourceOutStripe {
  type: "stripe";
  config: StripeConfigOut;
}

interface IngestSourceOutStych {
  type: "stych";
  config: SvixConfigOut;
}

interface IngestSourceOutSvix {
  type: "svix";
  config: SvixConfigOut;
}

interface IngestSourceOutZoom {
  type: "zoom";
  config: ZoomConfigOut;
}

interface IngestSourceOutTelnyx {
  type: "telnyx";
  config: TelnyxConfigOut;
}

interface IngestSourceOutVapi {
  type: "vapi";
  config: VapiConfigOut;
}

interface IngestSourceOutOpenAi {
  type: "open-ai";
  config: SvixConfigOut;
}

interface IngestSourceOutRender {
  type: "render";
  config: SvixConfigOut;
}

interface IngestSourceOutVeriff {
  type: "veriff";
  config: VeriffConfigOut;
}

interface IngestSourceOutAirwallex {
  type: "airwallex";
  config: AirwallexConfigOut;
}

export type IngestSourceOut = _IngestSourceOutFields &
  (
    | IngestSourceOutGenericWebhook
    | IngestSourceOutCron
    | IngestSourceOutAdobeSign
    | IngestSourceOutBeehiiv
    | IngestSourceOutBrex
    | IngestSourceOutCheckbook
    | IngestSourceOutClerk
    | IngestSourceOutDocusign
    | IngestSourceOutEasypost
    | IngestSourceOutGithub
    | IngestSourceOutGuesty
    | IngestSourceOutHubspot
    | IngestSourceOutIncidentIo
    | IngestSourceOutLithic
    | IngestSourceOutNash
    | IngestSourceOutOrumIo
    | IngestSourceOutPandaDoc
    | IngestSourceOutPortIo
    | IngestSourceOutPleo
    | IngestSourceOutReplicate
    | IngestSourceOutResend
    | IngestSourceOutRutter
    | IngestSourceOutSafebase
    | IngestSourceOutSardine
    | IngestSourceOutSegment
    | IngestSourceOutShopify
    | IngestSourceOutSlack
    | IngestSourceOutStripe
    | IngestSourceOutStych
    | IngestSourceOutSvix
    | IngestSourceOutZoom
    | IngestSourceOutTelnyx
    | IngestSourceOutVapi
    | IngestSourceOutOpenAi
    | IngestSourceOutRender
    | IngestSourceOutVeriff
    | IngestSourceOutAirwallex
  );

export const IngestSourceOutSerializer = {
  _fromJsonObject(object: any): IngestSourceOut {
    const type = object["type"];

    function getConfig(type: string): any {
      switch (type) {
        case "generic-webhook":
          return {};
        case "cron":
          return CronConfigSerializer._fromJsonObject(object["config"]);
        case "adobe-sign":
          return AdobeSignConfigOutSerializer._fromJsonObject(object["config"]);
        case "beehiiv":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "brex":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "checkbook":
          return CheckbookConfigOutSerializer._fromJsonObject(object["config"]);
        case "clerk":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "docusign":
          return DocusignConfigOutSerializer._fromJsonObject(object["config"]);
        case "easypost":
          return EasypostConfigOutSerializer._fromJsonObject(object["config"]);
        case "github":
          return GithubConfigOutSerializer._fromJsonObject(object["config"]);
        case "guesty":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "hubspot":
          return HubspotConfigOutSerializer._fromJsonObject(object["config"]);
        case "incident-io":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "lithic":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "nash":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "orum-io":
          return OrumIoConfigOutSerializer._fromJsonObject(object["config"]);
        case "panda-doc":
          return PandaDocConfigOutSerializer._fromJsonObject(object["config"]);
        case "port-io":
          return PortIoConfigOutSerializer._fromJsonObject(object["config"]);
        case "pleo":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "replicate":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "resend":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "rutter":
          return RutterConfigOutSerializer._fromJsonObject(object["config"]);
        case "safebase":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "sardine":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "segment":
          return SegmentConfigOutSerializer._fromJsonObject(object["config"]);
        case "shopify":
          return ShopifyConfigOutSerializer._fromJsonObject(object["config"]);
        case "slack":
          return SlackConfigOutSerializer._fromJsonObject(object["config"]);
        case "stripe":
          return StripeConfigOutSerializer._fromJsonObject(object["config"]);
        case "stych":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "svix":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "zoom":
          return ZoomConfigOutSerializer._fromJsonObject(object["config"]);
        case "telnyx":
          return TelnyxConfigOutSerializer._fromJsonObject(object["config"]);
        case "vapi":
          return VapiConfigOutSerializer._fromJsonObject(object["config"]);
        case "open-ai":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "render":
          return SvixConfigOutSerializer._fromJsonObject(object["config"]);
        case "veriff":
          return VeriffConfigOutSerializer._fromJsonObject(object["config"]);
        case "airwallex":
          return AirwallexConfigOutSerializer._fromJsonObject(object["config"]);
        default:
          throw new Error(`Unexpected type: ${type}`);
      }
    }
    return {
      type,
      config: getConfig(type),
      createdAt: new Date(object["createdAt"]),
      id: object["id"],
      ingestUrl: object["ingestUrl"],
      metadata: object["metadata"],
      name: object["name"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: IngestSourceOut): any {
    // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
    let config;
    switch (self.type) {
      case "generic-webhook":
        config = {};
        break;
      case "cron":
        config = CronConfigSerializer._toJsonObject(self.config);
        break;
      case "adobe-sign":
        config = AdobeSignConfigOutSerializer._toJsonObject(self.config);
        break;
      case "beehiiv":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "brex":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "checkbook":
        config = CheckbookConfigOutSerializer._toJsonObject(self.config);
        break;
      case "clerk":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "docusign":
        config = DocusignConfigOutSerializer._toJsonObject(self.config);
        break;
      case "easypost":
        config = EasypostConfigOutSerializer._toJsonObject(self.config);
        break;
      case "github":
        config = GithubConfigOutSerializer._toJsonObject(self.config);
        break;
      case "guesty":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "hubspot":
        config = HubspotConfigOutSerializer._toJsonObject(self.config);
        break;
      case "incident-io":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "lithic":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "nash":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "orum-io":
        config = OrumIoConfigOutSerializer._toJsonObject(self.config);
        break;
      case "panda-doc":
        config = PandaDocConfigOutSerializer._toJsonObject(self.config);
        break;
      case "port-io":
        config = PortIoConfigOutSerializer._toJsonObject(self.config);
        break;
      case "pleo":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "replicate":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "resend":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "rutter":
        config = RutterConfigOutSerializer._toJsonObject(self.config);
        break;
      case "safebase":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "sardine":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "segment":
        config = SegmentConfigOutSerializer._toJsonObject(self.config);
        break;
      case "shopify":
        config = ShopifyConfigOutSerializer._toJsonObject(self.config);
        break;
      case "slack":
        config = SlackConfigOutSerializer._toJsonObject(self.config);
        break;
      case "stripe":
        config = StripeConfigOutSerializer._toJsonObject(self.config);
        break;
      case "stych":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "svix":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "zoom":
        config = ZoomConfigOutSerializer._toJsonObject(self.config);
        break;
      case "telnyx":
        config = TelnyxConfigOutSerializer._toJsonObject(self.config);
        break;
      case "vapi":
        config = VapiConfigOutSerializer._toJsonObject(self.config);
        break;
      case "open-ai":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "render":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;
      case "veriff":
        config = VeriffConfigOutSerializer._toJsonObject(self.config);
        break;
      case "airwallex":
        config = AirwallexConfigOutSerializer._toJsonObject(self.config);
        break;
    }

    return {
      type: self.type,
      config: config,
      createdAt: self.createdAt,
      id: self.id,
      ingestUrl: self.ingestUrl,
      metadata: self.metadata,
      name: self.name,
      uid: self.uid,
      updatedAt: self.updatedAt,
    };
  },
};
