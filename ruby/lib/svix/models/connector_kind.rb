# frozen_string_literal: true
# This file is @generated
module Svix
  class ConnectorKind
    CUSTOM = "Custom".freeze
    AGENTIC_COMMERCE_PROTOCOL = "AgenticCommerceProtocol".freeze
    CLOSE_CRM = "CloseCRM".freeze
    CUSTOMER_IO = "CustomerIO".freeze
    DISCORD = "Discord".freeze
    HUBSPOT = "Hubspot".freeze
    INNGEST = "Inngest".freeze
    LOOPS = "Loops".freeze
    RESEND = "Resend".freeze
    SALESFORCE = "Salesforce".freeze
    SEGMENT = "Segment".freeze
    SENDGRID = "Sendgrid".freeze
    SLACK = "Slack".freeze
    TEAMS = "Teams".freeze
    TRIGGER_DEV = "TriggerDev".freeze
    WINDMILL = "Windmill".freeze
    ZAPIER = "Zapier".freeze

    def self.all_vars
      @all_vars ||= [
        CUSTOM,
        AGENTIC_COMMERCE_PROTOCOL,
        CLOSE_CRM,
        CUSTOMER_IO,
        DISCORD,
        HUBSPOT,
        INNGEST,
        LOOPS,
        RESEND,
        SALESFORCE,
        SEGMENT,
        SENDGRID,
        SLACK,
        TEAMS,
        TRIGGER_DEV,
        WINDMILL,
        ZAPIER
      ].freeze
    end

    def initialize(value)
      unless ConnectorKind.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #ConnectorKind"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if ConnectorKind.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #ConnectorKind"
    end

    def serialize
      @value
    end
  end
end
