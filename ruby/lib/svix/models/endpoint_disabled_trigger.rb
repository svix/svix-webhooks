# frozen_string_literal: true
# This file is @generated
module Svix
  class EndpointDisabledTrigger
    MANUAL = "manual".freeze
    AUTOMATIC = "automatic".freeze

    def self.all_vars
      @all_vars ||= [MANUAL, AUTOMATIC].freeze
    end

    def initialize(value)
      unless EndpointDisabledTrigger.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #EndpointDisabledTrigger"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if EndpointDisabledTrigger.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #EndpointDisabledTrigger"
    end

    def serialize
      @value
    end
  end
end
