# frozen_string_literal: true
# This file is @generated
module Svix
  class MessageStatusText
    SUCCESS = "success".freeze
    PENDING = "pending".freeze
    FAIL = "fail".freeze
    SENDING = "sending".freeze
    CANCELED = "canceled".freeze

    def self.all_vars
      @all_vars ||= [SUCCESS, PENDING, FAIL, SENDING, CANCELED].freeze
    end

    def initialize(value)
      unless MessageStatusText.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #MessageStatusText"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if MessageStatusText.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #MessageStatusText"
    end

    def serialize
      @value
    end
  end
end
