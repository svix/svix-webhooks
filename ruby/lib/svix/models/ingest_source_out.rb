# frozen_string_literal: true
# This file is @generated
require "json"

require_relative "./adobe_sign_config_out"
require_relative "./airwallex_config_out"
require_relative "./checkbook_config_out"
require_relative "./cron_config"
require_relative "./docusign_config_out"
require_relative "./easypost_config_out"
require_relative "./github_config_out"
require_relative "./hubspot_config_out"
require_relative "./orum_io_config_out"
require_relative "./panda_doc_config_out"
require_relative "./port_io_config_out"
require_relative "./rutter_config_out"
require_relative "./segment_config_out"
require_relative "./shopify_config_out"
require_relative "./slack_config_out"
require_relative "./stripe_config_out"
require_relative "./svix_config_out"
require_relative "./telnyx_config_out"
require_relative "./vapi_config_out"
require_relative "./veriff_config_out"
require_relative "./zoom_config_out"

module Svix
  class IngestSourceOutConfig
    class GenericWebhook

      def serialize
        Hash.new
      end

      def self.deserialize(attributes = {})
        new
      end
      # Serializes the object to a json string
      # @return String
      def to_json
        JSON.dump(serialize)
      end
    end

    class Cron < CronConfig
    end

    class AdobeSign < AdobeSignConfigOut
    end

    class Beehiiv < SvixConfigOut
    end

    class Brex < SvixConfigOut
    end

    class Checkbook < CheckbookConfigOut
    end

    class Clerk < SvixConfigOut
    end

    class Docusign < DocusignConfigOut
    end

    class Easypost < EasypostConfigOut
    end

    class Github < GithubConfigOut
    end

    class Guesty < SvixConfigOut
    end

    class Hubspot < HubspotConfigOut
    end

    class IncidentIo < SvixConfigOut
    end

    class Lithic < SvixConfigOut
    end

    class Nash < SvixConfigOut
    end

    class OrumIo < OrumIoConfigOut
    end

    class PandaDoc < PandaDocConfigOut
    end

    class PortIo < PortIoConfigOut
    end

    class PsiFi < SvixConfigOut
    end

    class Pleo < SvixConfigOut
    end

    class Replicate < SvixConfigOut
    end

    class Resend < SvixConfigOut
    end

    class Rutter < RutterConfigOut
    end

    class Safebase < SvixConfigOut
    end

    class Sardine < SvixConfigOut
    end

    class Segment < SegmentConfigOut
    end

    class Shopify < ShopifyConfigOut
    end

    class Slack < SlackConfigOut
    end

    class Stripe < StripeConfigOut
    end

    class Stych < SvixConfigOut
    end

    class Svix < SvixConfigOut
    end

    class Zoom < ZoomConfigOut
    end

    class Telnyx < TelnyxConfigOut
    end

    class Vapi < VapiConfigOut
    end

    class OpenAi < SvixConfigOut
    end

    class Render < SvixConfigOut
    end

    class Veriff < VeriffConfigOut
    end

    class Airwallex < AirwallexConfigOut
    end
  end

  class IngestSourceOut
    attr_accessor :created_at
    # The Source's ID.
    attr_accessor :id
    attr_accessor :ingest_url
    attr_accessor :metadata
    attr_accessor :name
    # The Source's UID.
    attr_accessor :uid
    attr_accessor :updated_at
    attr_accessor :config

    ALL_FIELD ||= ["created_at", "id", "ingest_url", "metadata", "name", "uid", "updated_at", "config"].freeze
    private_constant :ALL_FIELD
    TYPE_TO_NAME = {
      IngestSourceOutConfig::GenericWebhook => "generic-webhook",
      IngestSourceOutConfig::Cron => "cron",
      IngestSourceOutConfig::AdobeSign => "adobe-sign",
      IngestSourceOutConfig::Beehiiv => "beehiiv",
      IngestSourceOutConfig::Brex => "brex",
      IngestSourceOutConfig::Checkbook => "checkbook",
      IngestSourceOutConfig::Clerk => "clerk",
      IngestSourceOutConfig::Docusign => "docusign",
      IngestSourceOutConfig::Easypost => "easypost",
      IngestSourceOutConfig::Github => "github",
      IngestSourceOutConfig::Guesty => "guesty",
      IngestSourceOutConfig::Hubspot => "hubspot",
      IngestSourceOutConfig::IncidentIo => "incident-io",
      IngestSourceOutConfig::Lithic => "lithic",
      IngestSourceOutConfig::Nash => "nash",
      IngestSourceOutConfig::OrumIo => "orum-io",
      IngestSourceOutConfig::PandaDoc => "panda-doc",
      IngestSourceOutConfig::PortIo => "port-io",
      IngestSourceOutConfig::PsiFi => "psi-fi",
      IngestSourceOutConfig::Pleo => "pleo",
      IngestSourceOutConfig::Replicate => "replicate",
      IngestSourceOutConfig::Resend => "resend",
      IngestSourceOutConfig::Rutter => "rutter",
      IngestSourceOutConfig::Safebase => "safebase",
      IngestSourceOutConfig::Sardine => "sardine",
      IngestSourceOutConfig::Segment => "segment",
      IngestSourceOutConfig::Shopify => "shopify",
      IngestSourceOutConfig::Slack => "slack",
      IngestSourceOutConfig::Stripe => "stripe",
      IngestSourceOutConfig::Stych => "stych",
      IngestSourceOutConfig::Svix => "svix",
      IngestSourceOutConfig::Zoom => "zoom",
      IngestSourceOutConfig::Telnyx => "telnyx",
      IngestSourceOutConfig::Vapi => "vapi",
      IngestSourceOutConfig::OpenAi => "open-ai",
      IngestSourceOutConfig::Render => "render",
      IngestSourceOutConfig::Veriff => "veriff",
      IngestSourceOutConfig::Airwallex => "airwallex"
    }
    private_constant :TYPE_TO_NAME
    NAME_TO_TYPE = TYPE_TO_NAME.invert
    private_constant :NAME_TO_TYPE

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::IngestSourceOut` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::IngestSourceOut")
        end

        if k == "config"
          unless TYPE_TO_NAME.key?(v.class)
            fail(ArgumentError, "The field #{k} can't be a `#{v.class}` expected one of #{TYPE_TO_NAME.keys}")
          end

          instance_variable_set("@__enum_discriminator", TYPE_TO_NAME[v.class])
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end

      if @__enum_discriminator.nil?
        fail(ArgumentError, "Required config field was not set")
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["id"] = attributes["id"]
      attrs["ingest_url"] = attributes["ingestUrl"]
      attrs["metadata"] = attributes["metadata"]
      attrs["name"] = attributes["name"]
      attrs["uid"] = attributes["uid"]
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      unless NAME_TO_TYPE.key?(attributes["type"])
        fail(ArgumentError, "Invalid type `#{attributes["type"]}` expected on of #{NAME_TO_TYPE.keys}")
      end

      unless attributes.key?("config")
        fail(ArgumentError, "Missing required field config")
      end

      attrs["config"] = NAME_TO_TYPE[attributes["type"]].deserialize(attributes["config"])
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["ingestUrl"] = Svix::serialize_primitive(@ingest_url) if @ingest_url
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) if @updated_at
      out["type"] = @__enum_discriminator
      out["config"] = @config.serialize
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end

  end
end
