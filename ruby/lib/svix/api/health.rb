# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Health
    def initialize(client)
      @client = client
    end

    def get
      @client.execute_request(
        "GET",
        "/api/v1/health"
      )
    end

  end
end
