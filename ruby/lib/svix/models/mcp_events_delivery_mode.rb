# frozen_string_literal: true
# This file is @generated
module Svix
  class McpEventsDeliveryMode
    WEBHOOK = "webhook".freeze

    def self.all_vars
      @all_vars ||= [WEBHOOK].freeze
    end

    def initialize(value)
      unless McpEventsDeliveryMode.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #McpEventsDeliveryMode"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if McpEventsDeliveryMode.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #McpEventsDeliveryMode"
    end

    def serialize
      @value
    end
  end
end
