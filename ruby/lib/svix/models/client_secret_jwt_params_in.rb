# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ClientSecretJwtParamsIn
    # The base64-encoded secret used for signing the JWT.
    attr_accessor :secret_base64
    # The algorithm used to sign the JWT.
    attr_accessor :signing_algorithm
    # Optional secret identifier.
    #
    # If supplied, this will be populated in the JWT header in the `kid` field.
    attr_accessor :secret_id
    # Optional number of seconds after which the JWT should expire.
    #
    # Defaults to 300 seconds.
    attr_accessor :token_expiry_secs

    ALL_FIELD ||= ["secret_base64", "signing_algorithm", "secret_id", "token_expiry_secs"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::ClientSecretJwtParamsIn` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ClientSecretJwtParamsIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["secret_base64"] = attributes["secretBase64"]
      attrs["signing_algorithm"] = Svix::OauthJwsSigningAlgorithm.deserialize(attributes["signingAlgorithm"])
      attrs["secret_id"] = attributes["secretId"]
      attrs["token_expiry_secs"] = attributes["tokenExpirySecs"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["secretBase64"] = Svix::serialize_primitive(@secret_base64) unless @secret_base64.nil?
      out["signingAlgorithm"] = Svix::serialize_schema_ref(@signing_algorithm) unless @signing_algorithm.nil?
      out["secretId"] = Svix::serialize_primitive(@secret_id) unless @secret_id.nil?
      out["tokenExpirySecs"] = Svix::serialize_primitive(@token_expiry_secs) unless @token_expiry_secs.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
