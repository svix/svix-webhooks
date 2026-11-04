# frozen_string_literal: true
# This file is @generated
module Svix
  class Status
    PENDING = "pending".freeze
    ACTIVE = "active".freeze

    def self.all_vars
      @all_vars ||= [PENDING, ACTIVE].freeze
    end

    def initialize(value)
      unless Status.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #Status"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if Status.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #Status"
    end

    def serialize
      @value
    end
  end
end
