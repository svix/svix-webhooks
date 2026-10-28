# frozen_string_literal: true
# This file is @generated
module Svix
  # Operation for bulk indexing.
  #
  # `index` will do an upsert for the given document ID, and `create` will insert it
  # (and do nothing on conflict). Only `create` is supported for Data Stream targets.
  class ElasticSearchOperation
    INDEX = "index".freeze
    CREATE = "create".freeze

    def self.all_vars
      @all_vars ||= [INDEX, CREATE].freeze
    end

    def initialize(value)
      unless ElasticSearchOperation.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #ElasticSearchOperation"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if ElasticSearchOperation.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #ElasticSearchOperation"
    end

    def serialize
      @value
    end
  end
end
