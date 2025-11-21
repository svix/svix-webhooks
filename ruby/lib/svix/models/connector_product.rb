# frozen_string_literal: true
# This file is @generated
module Svix
  class ConnectorProduct
    DISPATCH = "Dispatch".freeze
    STREAM = "Stream".freeze

    def self.all_vars
      @all_vars ||= [DISPATCH, STREAM].freeze
    end

    def initialize(value)
      unless ConnectorProduct.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #ConnectorProduct"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if ConnectorProduct.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #ConnectorProduct"
    end

    def serialize
      @value
    end
  end
end
