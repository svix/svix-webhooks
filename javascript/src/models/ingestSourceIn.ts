// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { AdobeSignConfig, AdobeSignConfigSerializer } from "./adobeSignConfig";
import { CronConfig, CronConfigSerializer } from "./cronConfig";
import { DocusignConfig, DocusignConfigSerializer } from "./docusignConfig";
import { GithubConfig, GithubConfigSerializer } from "./githubConfig";
import { HubspotConfig, HubspotConfigSerializer } from "./hubspotConfig";
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
    let config;
    switch (type) {
      case "generic-webhook":
        config = {};
        break;

      case "cron":
        config = CronConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "adobe-sign":
        config = AdobeSignConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "beehiiv":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "brex":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "clerk":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "docusign":
        config = DocusignConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "github":
        config = GithubConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "guesty":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "hubspot":
        config = HubspotConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "incident-io":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "lithic":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "nash":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "pleo":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "replicate":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "resend":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "safebase":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "sardine":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "segment":
        config = SegmentConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "shopify":
        config = ShopifyConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "slack":
        config = SlackConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "stripe":
        config = StripeConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "stych":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "svix":
        config = SvixConfigSerializer._fromJsonObject(object["config"]);
        break;
      case "zoom":
        config = ZoomConfigSerializer._fromJsonObject(object["config"]);
        break;
    }

    return {
      type,
      config,
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
