# frozen_string_literal: true
# This file is @generated
module Svix
  class ArchiveStatus
    ACTIVE = "active".freeze
    ARCHIVED = "archived".freeze
    ALL = "all".freeze

    def self.all_vars
      @all_vars ||= [ACTIVE, ARCHIVED, ALL].freeze
    end

    def initialize(value)
      unless ArchiveStatus.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #ArchiveStatus"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if ArchiveStatus.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #ArchiveStatus"
    end

    def serialize
      @value
    end
  end
end
