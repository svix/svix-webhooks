# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Connector
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/connector",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"],
          "product_type" => options["product_type"]
        }
      )
      ListResponseConnectorOut.deserialize(res)
    end

    def create(connector_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/connector",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: connector_in
      )
      ConnectorOut.deserialize(res)
    end

    def get(connector_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/connector/#{connector_id}"
      )
      ConnectorOut.deserialize(res)
    end

    def update(connector_id, connector_update)
      res = @client.execute_request(
        "PUT",
        "/api/v1/connector/#{connector_id}",
        body: connector_update
      )
      ConnectorOut.deserialize(res)
    end

    def delete(connector_id)
      @client.execute_request(
        "DELETE",
        "/api/v1/connector/#{connector_id}"
      )
    end

    def patch(connector_id, connector_patch)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/connector/#{connector_id}",
        body: connector_patch
      )
      ConnectorOut.deserialize(res)
    end

  end
end
