# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class IngestAuthentication
    def initialize(client)
      @client = client
    end

    def consumer_portal_access(source_id, ingest_source_consumer_portal_access_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/ingest/api/v1/source/#{source_id}/dashboard",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: ingest_source_consumer_portal_access_in
      )
      AppPortalAccessOut.deserialize(res)
    end

  end
end
