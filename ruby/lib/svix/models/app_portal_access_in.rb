# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class AppPortalAccessIn
    # Optionally creates a new application while generating the access link.
    #
    # If the application id or uid that is used in the path already exists, this argument is ignored.
    attr_accessor :application
    # Custom capabilities attached to the token, You can combine as many capabilities as necessary.
    #
    # The `ViewBase` capability is always required
    #
    # - `ViewBase`: Basic read only permissions, does not allow the user to see the endpoint secret.
    #
    # - `ViewEndpointSecret`: Allows user to view the endpoint secret.
    #
    # - `ManageEndpointSecret`: Allows user to rotate and view the endpoint secret.
    #
    # - `ManageTransformations`: Allows user to modify the endpoint transformations.
    #
    # - `CreateAttempts`: Allows user to replay missing messages and send example messages.
    #
    # - `ManageEndpoint`: Allows user to read/modify any field or configuration of an endpoint (including secrets)
    #
    # By default, the token will get all capabilities if the capabilities are not explicitly specified.
    attr_accessor :capabilities
    # How long the token will be valid for, in seconds.
    #
    # Valid values are between 1 hour and 7 days. The default is 7 days.
    attr_accessor :expiry
    # The set of feature flags the created token will have access to.
    attr_accessor :feature_flags
    # Whether the app portal should be in read-only mode.
    attr_accessor :read_only
    # An optional session ID to attach to the token.
    #
    # When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID.
    attr_accessor :session_id

    ALL_FIELD ||= ["application", "capabilities", "expiry", "feature_flags", "read_only", "session_id"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::AppPortalAccessIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::AppPortalAccessIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["application"] = Svix::ApplicationIn.deserialize(attributes["application"]) if attributes["application"]
      if attributes["capabilities"]
        attrs["capabilities"] = attributes["capabilities"].map { |v| Svix::AppPortalCapability.deserialize(v) }
      end

      attrs["expiry"] = attributes["expiry"]
      attrs["feature_flags"] = attributes["featureFlags"]
      attrs["read_only"] = attributes["readOnly"]
      attrs["session_id"] = attributes["sessionId"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["application"] = Svix::serialize_schema_ref(@application) if @application
      out["capabilities"] = @capabilities.map { |v| v.serialize } if @capabilities
      out["expiry"] = Svix::serialize_primitive(@expiry) if @expiry
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @feature_flags
      out["readOnly"] = Svix::serialize_primitive(@read_only) if @read_only
      out["sessionId"] = Svix::serialize_primitive(@session_id) if @session_id
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
