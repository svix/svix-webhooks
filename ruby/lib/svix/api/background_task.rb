# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class BackgroundTask
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/background-task",
        query_params: {
          "status" => options["status"],
          "task" => options["task"],
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseBackgroundTaskOut.deserialize(res)
    end

    def get(task_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/background-task/#{task_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      BackgroundTaskOut.deserialize(res)
    end

  end
end
