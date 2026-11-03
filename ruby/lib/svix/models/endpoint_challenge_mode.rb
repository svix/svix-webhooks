# frozen_string_literal: true
# This file is @generated
module Svix
  # How an endpoint must respond to a challenge.
  #
  # - `challenge-response`: the Standard Webhooks challenge-response check.
  #   The endpoint must return `webhook-challenge-response`, calculated from
  #   the challenge and its secret.
  # - `echo`: the endpoint must echo the challenge from the request body back
  #   as `{"challenge": "..."}`. This doesn't prove that it holds the secret.
  class EndpointChallengeMode
    CHALLENGE_RESPONSE = "challenge-response".freeze
    ECHO = "echo".freeze

    def self.all_vars
      @all_vars ||= [CHALLENGE_RESPONSE, ECHO].freeze
    end

    def initialize(value)
      unless EndpointChallengeMode.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #EndpointChallengeMode"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if EndpointChallengeMode.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #EndpointChallengeMode"
    end

    def serialize
      @value
    end
  end
end
