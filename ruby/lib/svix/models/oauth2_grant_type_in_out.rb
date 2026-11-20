# frozen_string_literal: true
# This file is @generated
module Svix
  class Oauth2GrantTypeInOut
    CLIENT_CREDENTIALS = "clientCredentials".freeze
    REFRESH_TOKEN = "refreshToken".freeze

    def self.all_vars
      @all_vars ||= [CLIENT_CREDENTIALS, REFRESH_TOKEN].freeze
    end

    def initialize(value)
      unless Oauth2GrantTypeInOut.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #Oauth2GrantTypeInOut"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if Oauth2GrantTypeInOut.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #Oauth2GrantTypeInOut"
    end

    def serialize
      @value
    end
  end
end
