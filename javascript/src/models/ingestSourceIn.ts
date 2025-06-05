// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { AdobeSignConfig, AdobeSignConfigSerializer } from "./adobeSignConfig";
import { CronConfig, CronConfigSerializer } from "./cronConfig";
import { DocusignConfig, DocusignConfigSerializer } from "./docusignConfig";
import { GithubConfig, GithubConfigSerializer } from "./githubConfig";
import { HubspotConfig, HubspotConfigSerializer } from "./hubspotConfig";
import { PandaDocConfig, PandaDocConfigSerializer } from "./pandaDocConfig";
import { SegmentConfig, SegmentConfigSerializer } from "./segmentConfig";
import { ShopifyConfig, ShopifyConfigSerializer } from "./shopifyConfig";
import { SlackConfig, SlackConfigSerializer } from "./slackConfig";
import { StripeConfig, StripeConfigSerializer } from "./stripeConfig";
import { SvixConfig, SvixConfigSerializer } from "./svixConfig";
import { ZoomConfig, ZoomConfigSerializer } from "./zoomConfig";
interface _IngestSourceInFields {
  name: string;
  /** The Source's UID. */
  uid?: string | null;
}

interface IngestSourceInGenericWebhook {
  type: "generic-webhook";
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

interface IngestSourceInClerk {
  type: "clerk";
  config: SvixConfig;
}

interface IngestSourceInDocusign {
  type: "docusign";
  config: DocusignConfig;
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

interface IngestSourceInPandaDoc {
  type: "panda-doc";
  config: PandaDocConfig;
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

export type IngestSourceIn = _IngestSourceInFields &
  (
    | IngestSourceInGenericWebhook
    | IngestSourceInCron
    | IngestSourceInAdobeSign
    | IngestSourceInBeehiiv
    | IngestSourceInBrex
    | IngestSourceInClerk
    | IngestSourceInDocusign
    | IngestSourceInGithub
    | IngestSourceInGuesty
    | IngestSourceInHubspot
    | IngestSourceInIncidentIo
    | IngestSourceInLithic
    | IngestSourceInNash
    | IngestSourceInPandaDoc
    | IngestSourceInPleo
    | IngestSourceInReplicate
    | IngestSourceInResend
    | IngestSourceInSafebase
    | IngestSourceInSardine
    | IngestSourceInSegment
    | IngestSourceInShopify
    | IngestSourceInSlack
    | IngestSourceInStripe
    | IngestSourceInStych
    | IngestSourceInSvix
    | IngestSourceInZoom
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
        case "clerk":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "docusign":
          return DocusignConfigSerializer._fromJsonObject(object["config"]);
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
        case "panda-doc":
          return PandaDocConfigSerializer._fromJsonObject(object["config"]);
        case "pleo":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "replicate":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
        case "resend":
          return SvixConfigSerializer._fromJsonObject(object["config"]);
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
        default:
          throw new Error(`Unexpected type: ${type}`);
      }
    }
    return {
      type,
      config: getConfig(type),
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
      case "clerk":
        config = SvixConfigSerializer._toJsonObject(self.config);
        break;
      case "docusign":
        config = DocusignConfigSerializer._toJsonObject(self.config);
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
      case "panda-doc":
        config = PandaDocConfigSerializer._toJsonObject(self.config);
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
    }

    return {
      type: self.type,
      config: config,
      name: self.name,
      uid: self.uid,
    };
  },
};
