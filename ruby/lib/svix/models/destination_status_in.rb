# frozen_string_literal: true
# This file is @generated
module Svix
  class DestinationStatusIn
    ENABLED = "enabled".freeze
    DISABLED = "disabled".freeze

    def self.all_vars
      @all_vars ||= [ENABLED, DISABLED].freeze
    end

    def initialize(value)
      unless DestinationStatusIn.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #DestinationStatusIn"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if DestinationStatusIn.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #DestinationStatusIn"
    end

    def serialize
      @value
    end
  end
end
