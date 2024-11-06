# frozen_string_literal: true

module Svix
  class OperationalWebhookEndpointAPI
    def initialize(api_client)
      @api = OperationalWebhookEndpointApi.new(api_client)
    end

    def list(options = {})
      return @api.v1_endpoint_list(options)
    end

    def create(endpoint_in, options = {})
      return @api.v1_endpoint_create(endpoint_in, options)
    end

    def get(endpoint_id)
      return @api.v1_endpoint_get(endpoint_id)
    end

    def update(endpoint_id, endpoint_update)
      return @api.v1_endpoint_update(endpoint_id, endpoint_update)
    end

    def delete(endpoint_id)
      return @api.v1_endpoint_delete(endpoint_id)
    end

    def get_secret(endpoint_id)
      return @api.v1_endpoint_get_secret(endpoint_id)
    end

    def rotate_secret(endpoint_id, endpoint_secret_rotate_in, options = {})
      return @api.v1_endpoint_rotate_secret(endpoint_id, endpoint_secret_rotate_in, options)
    end
  end
end
