# frozen_string_literal: true
# This file is @generated
module Svix
  class SinkStatus
    ENABLED = "enabled".freeze
    PAUSED = "paused".freeze
    DISABLED = "disabled".freeze
    RETRYING = "retrying".freeze

    def self.all_vars
      @all_vars ||= [ENABLED, PAUSED, DISABLED, RETRYING].freeze
    end

    def initialize(value)
      unless SinkStatus.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #SinkStatus"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if SinkStatus.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #SinkStatus"
    end

    def serialize
      @value
    end
  end
end
