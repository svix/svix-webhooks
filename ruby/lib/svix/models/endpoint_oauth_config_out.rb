# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointOauthConfigOut
    attr_accessor :grant_type
    attr_accessor :auth_method
    attr_accessor :client_id
    attr_accessor :token_url
    attr_accessor :scopes
    attr_accessor :extra_params
    attr_accessor :extra_headers

    ALL_FIELD ||= ["grant_type", "auth_method", "client_id", "token_url", "scopes", "extra_params", "extra_headers"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::EndpointOauthConfigOut` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointOauthConfigOut")
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
      attrs["client_id"] = attributes["clientId"]
      attrs["token_url"] = attributes["tokenUrl"]
      attrs["scopes"] = attributes["scopes"]
      attrs["extra_params"] = attributes["extraParams"]
      attrs["extra_headers"] = attributes["extraHeaders"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["grantType"] = Svix::serialize_schema_ref(@grant_type) unless @grant_type.nil?
      out["authMethod"] = Svix::serialize_schema_ref(@auth_method) unless @auth_method.nil?
      out["clientId"] = Svix::serialize_primitive(@client_id) unless @client_id.nil?
      out["tokenUrl"] = Svix::serialize_primitive(@token_url) unless @token_url.nil?
      out["scopes"] = Svix::serialize_primitive(@scopes) unless @scopes.nil?
      out["extraParams"] = Svix::serialize_primitive(@extra_params) unless @extra_params.nil?
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
