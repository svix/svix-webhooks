// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { AdobeSignConfigOut, AdobeSignConfigOutSerializer } from "./adobeSignConfigOut";
import { CronConfig, CronConfigSerializer } from "./cronConfig";
import { DocusignConfigOut, DocusignConfigOutSerializer } from "./docusignConfigOut";
import { GithubConfigOut, GithubConfigOutSerializer } from "./githubConfigOut";
import { HubspotConfigOut, HubspotConfigOutSerializer } from "./hubspotConfigOut";
import { SegmentConfigOut, SegmentConfigOutSerializer } from "./segmentConfigOut";
import { ShopifyConfigOut, ShopifyConfigOutSerializer } from "./shopifyConfigOut";
import { SlackConfigOut, SlackConfigOutSerializer } from "./slackConfigOut";
import { StripeConfigOut, StripeConfigOutSerializer } from "./stripeConfigOut";
import { SvixConfigOut, SvixConfigOutSerializer } from "./svixConfigOut";
import { ZoomConfigOut, ZoomConfigOutSerializer } from "./zoomConfigOut";
interface _IngestSourceOutFields {
  createdAt: Date;
  /** The Source's ID. */
  id: string;
  ingestUrl?: string | null;
  name: string;
  /** The Source's UID. */
  uid?: string | null;
  updatedAt: Date;
}

interface IngestSourceOutGenericWebhook {
  type: "generic-webhook";
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

interface IngestSourceOutClerk {
  type: "clerk";
  config: SvixConfigOut;
}

interface IngestSourceOutDocusign {
  type: "docusign";
  config: DocusignConfigOut;
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

export type IngestSourceOut = _IngestSourceOutFields &
  (
    | IngestSourceOutGenericWebhook
    | IngestSourceOutCron
    | IngestSourceOutAdobeSign
    | IngestSourceOutBeehiiv
    | IngestSourceOutBrex
    | IngestSourceOutClerk
    | IngestSourceOutDocusign
    | IngestSourceOutGithub
    | IngestSourceOutGuesty
    | IngestSourceOutHubspot
    | IngestSourceOutIncidentIo
    | IngestSourceOutLithic
    | IngestSourceOutNash
    | IngestSourceOutPleo
    | IngestSourceOutReplicate
    | IngestSourceOutResend
    | IngestSourceOutSafebase
    | IngestSourceOutSardine
    | IngestSourceOutSegment
    | IngestSourceOutShopify
    | IngestSourceOutSlack
    | IngestSourceOutStripe
    | IngestSourceOutStych
    | IngestSourceOutSvix
    | IngestSourceOutZoom
  );

export const IngestSourceOutSerializer = {
  _fromJsonObject(object: any): IngestSourceOut {
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
        config = AdobeSignConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "beehiiv":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "brex":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "clerk":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "docusign":
        config = DocusignConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "github":
        config = GithubConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "guesty":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "hubspot":
        config = HubspotConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "incident-io":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "lithic":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "nash":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "pleo":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "replicate":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "resend":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "safebase":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "sardine":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "segment":
        config = SegmentConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "shopify":
        config = ShopifyConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "slack":
        config = SlackConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "stripe":
        config = StripeConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "stych":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "svix":
        config = SvixConfigOutSerializer._fromJsonObject(object["config"]);
        break;
      case "zoom":
        config = ZoomConfigOutSerializer._fromJsonObject(object["config"]);
        break;
    }

    return {
      type,
      config,
      createdAt: new Date(object["createdAt"]),
      id: object["id"],
      ingestUrl: object["ingestUrl"],
      name: object["name"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: IngestSourceOut): any {
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

      case "clerk":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;

      case "docusign":
        config = DocusignConfigOutSerializer._toJsonObject(self.config);
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

      case "pleo":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;

      case "replicate":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
        break;

      case "resend":
        config = SvixConfigOutSerializer._toJsonObject(self.config);
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
    }

    return {
      type: self.type,
      config: config,
      createdAt: self.createdAt,
      id: self.id,
      ingestUrl: self.ingestUrl,
      name: self.name,
      uid: self.uid,
      updatedAt: self.updatedAt,
    };
  },
};
