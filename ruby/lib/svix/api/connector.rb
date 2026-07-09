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
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseConnectorOut.deserialize(res)
    end

    def create(connector_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/connector",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: connector_in
      )
      ConnectorOut.deserialize(res)
    end

    def get(connector_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/connector/#{connector_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ConnectorOut.deserialize(res)
    end

    def update(connector_id, connector_update, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/api/v1/connector/#{connector_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: connector_update
      )
      ConnectorOut.deserialize(res)
    end

    def delete(connector_id, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/api/v1/connector/#{connector_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

    def patch(connector_id, connector_patch, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/connector/#{connector_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: connector_patch
      )
      ConnectorOut.deserialize(res)
    end

  end
end
