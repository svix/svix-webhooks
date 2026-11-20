# frozen_string_literal: true
# This file is @generated
module Svix
  class OauthJwsSigningAlgorithm
    RS256 = "RS256".freeze

    def self.all_vars
      @all_vars ||= [RS256].freeze
    end

    def initialize(value)
      unless OauthJwsSigningAlgorithm.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #OauthJwsSigningAlgorithm"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if OauthJwsSigningAlgorithm.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #OauthJwsSigningAlgorithm"
    end

    def serialize
      @value
    end
  end
end
