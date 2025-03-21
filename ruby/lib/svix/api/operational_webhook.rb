# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class OperationalWebhook
    attr_accessor :endpoint
    def initialize(client)
      @client = client
      @endpoint = OperationalWebhookEndpoint.new(client)
    end
  end
end
