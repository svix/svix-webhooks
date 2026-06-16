# frozen_string_literal: true
# This file is @generated
module Svix
  class StartingPosition
    EARLIEST = "earliest".freeze
    LATEST = "latest".freeze

    def self.all_vars
      @all_vars ||= [EARLIEST, LATEST].freeze
    end

    def initialize(value)
      unless StartingPosition.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #StartingPosition"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if StartingPosition.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #StartingPosition"
    end

    def serialize
      @value
    end
  end
end
