# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class EventType
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/event-type",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"],
          "include_archived" => options["include_archived"],
          "with_content" => options["with_content"]
        }
      )
      ListResponseEventTypeOut.deserialize(res)
    end

    def create(event_type_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/event-type",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: event_type_in
      )
      EventTypeOut.deserialize(res)
    end

    def import_openapi(event_type_import_open_api_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/event-type/import/openapi",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: event_type_import_open_api_in
      )
      EventTypeImportOpenApiOut.deserialize(res)
    end

    def get(event_type_name)
      res = @client.execute_request(
        "GET",
        "/api/v1/event-type/#{event_type_name}"
      )
      EventTypeOut.deserialize(res)
    end

    def update(event_type_name, event_type_update)
      res = @client.execute_request(
        "PUT",
        "/api/v1/event-type/#{event_type_name}",
        body: event_type_update
      )
      EventTypeOut.deserialize(res)
    end

    def delete(event_type_name, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/api/v1/event-type/#{event_type_name}",
        query_params: {
          "expunge" => options["expunge"]
        }
      )
    end

    def patch(event_type_name, event_type_patch)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/event-type/#{event_type_name}",
        body: event_type_patch
      )
      EventTypeOut.deserialize(res)
    end

  end
end
