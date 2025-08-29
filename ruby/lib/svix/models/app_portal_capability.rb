# frozen_string_literal: true
# This file is @generated
module Svix
  class AppPortalCapability
    VIEW_BASE = "ViewBase".freeze
    VIEW_ENDPOINT_SECRET = "ViewEndpointSecret".freeze
    MANAGE_ENDPOINT_SECRET = "ManageEndpointSecret".freeze
    MANAGE_TRANSFORMATIONS = "ManageTransformations".freeze
    CREATE_ATTEMPTS = "CreateAttempts".freeze
    MANAGE_ENDPOINT = "ManageEndpoint".freeze

    def self.all_vars
      @all_vars ||= [
        VIEW_BASE,
        VIEW_ENDPOINT_SECRET,
        MANAGE_ENDPOINT_SECRET,
        MANAGE_TRANSFORMATIONS,
        CREATE_ATTEMPTS,
        MANAGE_ENDPOINT
      ].freeze
    end

    def initialize(value)
      unless AppPortalCapability.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #AppPortalCapability"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if AppPortalCapability.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #AppPortalCapability"
    end

    def serialize
      @value
    end
  end
end
