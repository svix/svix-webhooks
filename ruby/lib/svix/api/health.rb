# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Health
    def initialize(client)
      @client = client
    end

    def get(options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "GET",
        "/api/v1/health",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

  end
end
