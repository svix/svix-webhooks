# frozen_string_literal: true
# This file is @generated
module Svix
  class SinkStatusIn
    ENABLED = "enabled".freeze
    DISABLED = "disabled".freeze

    def self.all_vars
      @all_vars ||= [ENABLED, DISABLED].freeze
    end

    def initialize(value)
      unless SinkStatusIn.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #SinkStatusIn"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if SinkStatusIn.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #SinkStatusIn"
    end

    def serialize
      @value
    end
  end
end
