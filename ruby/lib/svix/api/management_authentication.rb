# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class ManagementAuthentication
    def initialize(client)
      @client = client
    end

    def list_api_tokens(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/management/authentication/api-token",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseApiTokenCensoredOut.deserialize(res)
    end

    def create_api_token(api_token_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/management/authentication/api-token",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: api_token_in
      )
      ApiTokenOut.deserialize(res)
    end

    def expire_api_token(key_id, api_token_expire_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/api/v1/management/authentication/api-token/#{key_id}/expire",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: api_token_expire_in
      )
    end

  end
end
