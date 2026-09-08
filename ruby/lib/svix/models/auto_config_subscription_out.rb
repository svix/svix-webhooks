# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class AutoConfigSubscriptionOut
    attr_accessor :created_at
    attr_accessor :token_censored
    # The AutoConfigSubscription's ID.
    attr_accessor :id
    # The Endpoint's ID.
    attr_accessor :endp_id
    # The StreamSink's ID.
    attr_accessor :dest_id
    attr_accessor :status

    ALL_FIELD ||= ["created_at", "token_censored", "id", "endp_id", "dest_id", "status"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::AutoConfigSubscriptionOut` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::AutoConfigSubscriptionOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["token_censored"] = attributes["tokenCensored"]
      attrs["id"] = attributes["id"]
      attrs["endp_id"] = attributes["endpId"]
      attrs["dest_id"] = attributes["destId"]
      attrs["status"] = Svix::Status.deserialize(attributes["status"])
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["createdAt"] = Svix::serialize_primitive(@created_at) unless @created_at.nil?
      out["tokenCensored"] = Svix::serialize_primitive(@token_censored) unless @token_censored.nil?
      out["id"] = Svix::serialize_primitive(@id) unless @id.nil?
      out["endpId"] = Svix::serialize_primitive(@endp_id) unless @endp_id.nil?
      out["destId"] = Svix::serialize_primitive(@dest_id) unless @dest_id.nil?
      out["status"] = Svix::serialize_schema_ref(@status) unless @status.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
