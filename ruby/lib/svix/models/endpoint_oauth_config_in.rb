# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointOauthConfigIn
    # The OAuth grant type.
    attr_accessor :grant_type
    attr_accessor :auth_method
    # Optional JWT parameters. Only required for `clientSecretJwt`
    attr_accessor :jwt_params
    # The client ID. Required for all authentication types.
    attr_accessor :client_id
    # Optional client secret. This is only used for `clientSecretBasic` and `clientSecretPost`.
    #
    # For `clientSecretBasic`, the secret will be appended to the `Authorization` header.
    # For `clientSecretPost`, this will be added to the body in a `client_secret` parameter.
    attr_accessor :client_secret
    # The URL of the authorization server.
    attr_accessor :token_url
    # Optional OAuth scopes added to the request body.
    attr_accessor :scopes
    # Extra parameters added to the request body as key-value pairs.
    attr_accessor :extra_params
    # For `refreshToken` grant type.
    attr_accessor :refresh_token
    # Optional headers added to the token request.
    attr_accessor :extra_headers

    ALL_FIELD ||= [
      "grant_type",
      "auth_method",
      "jwt_params",
      "client_id",
      "client_secret",
      "token_url",
      "scopes",
      "extra_params",
      "refresh_token",
      "extra_headers"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::EndpointOauthConfigIn` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointOauthConfigIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["grant_type"] = Svix::Oauth2GrantTypeInOut.deserialize(attributes["grantType"])
      attrs["auth_method"] = Svix::Oauth2AuthMethodInOut.deserialize(attributes["authMethod"])
      if attributes["jwtParams"]
        attrs["jwt_params"] = Svix::ClientSecretJwtParamsIn.deserialize(attributes["jwtParams"])
      end

      attrs["client_id"] = attributes["clientId"]
      attrs["client_secret"] = attributes["clientSecret"]
      attrs["token_url"] = attributes["tokenUrl"]
      attrs["scopes"] = attributes["scopes"]
      attrs["extra_params"] = attributes["extraParams"]
      attrs["refresh_token"] = attributes["refreshToken"]
      attrs["extra_headers"] = attributes["extraHeaders"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["grantType"] = Svix::serialize_schema_ref(@grant_type) unless @grant_type.nil?
      out["authMethod"] = Svix::serialize_schema_ref(@auth_method) unless @auth_method.nil?
      out["jwtParams"] = Svix::serialize_schema_ref(@jwt_params) unless @jwt_params.nil?
      out["clientId"] = Svix::serialize_primitive(@client_id) unless @client_id.nil?
      out["clientSecret"] = Svix::serialize_primitive(@client_secret) unless @client_secret.nil?
      out["tokenUrl"] = Svix::serialize_primitive(@token_url) unless @token_url.nil?
      out["scopes"] = Svix::serialize_primitive(@scopes) unless @scopes.nil?
      out["extraParams"] = Svix::serialize_primitive(@extra_params) unless @extra_params.nil?
      out["refreshToken"] = Svix::serialize_primitive(@refresh_token) unless @refresh_token.nil?
      out["extraHeaders"] = Svix::serialize_primitive(@extra_headers) unless @extra_headers.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
