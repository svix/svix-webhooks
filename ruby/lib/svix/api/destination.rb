# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Destination
    attr_accessor :transformation
    def initialize(client)
      @client = client
      @transformation = DestinationTransformation.new(client)
    end

    def list(app_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/destination",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseDestinationOut.deserialize(res)
    end

    def create(app_id, destination_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/destination",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: destination_in
      )
      DestinationOut.deserialize(res)
    end

    def get(app_id, destination_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/destination/#{destination_id}"
      )
      DestinationOut.deserialize(res)
    end

    def upsert(app_id, destination_id, destination_in)
      res = @client.execute_request(
        "PUT",
        "/api/v1/app/#{app_id}/destination/#{destination_id}",
        body: destination_in
      )
      DestinationOut.deserialize(res)
    end

    def delete(app_id, destination_id)
      @client.execute_request(
        "DELETE",
        "/api/v1/app/#{app_id}/destination/#{destination_id}"
      )
    end

    def patch(app_id, destination_id, destination_patch)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}/destination/#{destination_id}",
        body: destination_patch
      )
      DestinationOut.deserialize(res)
    end

  end
end
