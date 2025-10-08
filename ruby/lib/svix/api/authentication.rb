# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Authentication
    def initialize(client)
      @client = client
    end

    def app_portal_access(app_id, app_portal_access_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/auth/app-portal-access/#{app_id}",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: app_portal_access_in
      )
      AppPortalAccessOut.deserialize(res)
    end

    def expire_all(app_id, application_token_expire_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/api/v1/auth/app/#{app_id}/expire-all",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: application_token_expire_in
      )
    end

    def dashboard_access(app_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/auth/dashboard-access/#{app_id}",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        }
      )
      DashboardAccessOut.deserialize(res)
    end

    def logout(options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/api/v1/auth/logout",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        }
      )
    end

    def stream_portal_access(stream_id, stream_portal_access_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/auth/stream-portal-access/#{stream_id}",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: stream_portal_access_in
      )
      AppPortalAccessOut.deserialize(res)
    end

    def get_stream_poller_token(stream_id, sink_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/auth/stream/#{stream_id}/sink/#{sink_id}/poller/token"
      )
      ApiTokenOut.deserialize(res)
    end

    def rotate_stream_poller_token(stream_id, sink_id, rotate_poller_token_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/auth/stream/#{stream_id}/sink/#{sink_id}/poller/token/rotate",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: rotate_poller_token_in
      )
      ApiTokenOut.deserialize(res)
    end

  end
end
