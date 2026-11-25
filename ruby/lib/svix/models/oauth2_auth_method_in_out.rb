# frozen_string_literal: true
# This file is @generated
module Svix
  # The method used for authenticating to the OAuth authorization server.
  #
  # `clientSecretJwt` will construct a JWT used for authentication
  # with the oauth authorization server. This method is less
  # commonly used and may not be supported by all oauth providers.
  # `clientSecretBasic` will authenticate to the oauth authorization server
  # using an `Authorization` header with the client secret as the value.
  # This is the most common means of authentication.
  # `clientSecretPost` will authenticate to the oauth authorization server
  # by passing the client secret in a `client_secret` field in the
  # request body. This method may not be supported by all oauth providers,
  # and in general `clientSecretBasic` should be preferred.
  class Oauth2AuthMethodInOut
    CLIENT_SECRET_JWT = "clientSecretJwt".freeze
    CLIENT_SECRET_BASIC = "clientSecretBasic".freeze
    CLIENT_SECRET_POST = "clientSecretPost".freeze

    def self.all_vars
      @all_vars ||= [CLIENT_SECRET_JWT, CLIENT_SECRET_BASIC, CLIENT_SECRET_POST].freeze
    end

    def initialize(value)
      unless Oauth2AuthMethodInOut.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #Oauth2AuthMethodInOut"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if Oauth2AuthMethodInOut.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #Oauth2AuthMethodInOut"
    end

    def serialize
      @value
    end
  end
end
