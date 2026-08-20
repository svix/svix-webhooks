# frozen_string_literal: true
# This file is @generated
module Svix
  class BulkExpungeStatus
    EXPUNGED = "expunged".freeze
    NOT_FOUND = "not-found".freeze

    def self.all_vars
      @all_vars ||= [EXPUNGED, NOT_FOUND].freeze
    end

    def initialize(value)
      unless BulkExpungeStatus.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #BulkExpungeStatus"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if BulkExpungeStatus.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #BulkExpungeStatus"
    end

    def serialize
      @value
    end
  end
end
