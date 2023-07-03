# frozen_string_literal: true

module Svix
  class EndpointAPI
    def initialize(api_client)
      @api = EndpointApi.new(api_client)
    end

    def list(app_id, options = {})
      return @api.v1_endpoint_list(app_id, options)
    end

    def create(app_id, endpoint_in, options = {})
      return @api.v1_endpoint_create(app_id, endpoint_in, options)
    end

    def get(app_id, endpoint_id)
      return @api.v1_endpoint_get(app_id, endpoint_id)
    end

    def update(app_id, endpoint_id, endpoint_update)
      return @api.v1_endpoint_update(app_id, endpoint_id, endpoint_update)
    end

    def patch(app_id, endpoint_id, endpoint_patch)
      return @api.v1_endpoint_patch(app_id, endpoint_id, endpoint_patch)
    end

    def delete(app_id, endpoint_id)
      return @api.v1_endpoint_delete(app_id, endpoint_id)
    end

    def get_secret(app_id, endpoint_id)
      return @api.v1_endpoint_get_secret(app_id, endpoint_id)
    end

    def rotate_secret(app_id, endpoint_id, endpoint_secret_rotate_in, options = {})
      return @api.v1_endpoint_rotate_secret(app_id, endpoint_id, endpoint_secret_rotate_in, options)
    end

    def recover(app_id, endpoint_id, recover_in, options = {})
      @api.v1_endpoint_recover(app_id, endpoint_id, recover_in, options)
      nil
    end

    def get_headers(app_id, endpoint_id)
      return @api.v1_endpoint_get_headers(app_id, endpoint_id)
    end

    def update_headers(app_id, endpoint_id, endpoint_headers_in)
      return @api.v1_endpoint_update_headers(app_id, endpoint_id, endpoint_headers_in)
    end

    def patch_headers(app_id, endpoint_id, endpoint_headers_in)
      return @api.v1_endpoint_patch_headers(app_id, endpoint_id, endpoint_headers_in)
    end

    def get_stats(app_id, endpoint_id, options = {})
      return @api.v1_endpoint_get_stats(app_id, endpoint_id, options)
    end

    def replay_missing(app_id, endpoint_id, replay_in, options = {})
      @api.v1_endpoint_replay(app_id, endpoint_id, replay_in, options)
      nil
    end

    def transformation_get(app_id, endpoint_id, options = {})
      return @api.v1_endpoint_transformation_get(app_id, endpoint_id, options)
    end

    def transformation_partial_update(app_id, endpoint_id, endpoint_transformation_in, options = {})
      @api.v1_endpoint_transformation_partial_update(app_id, endpoint_id, endpoint_transformation_in, options)
      nil
    end

    def send_example(app_id, endpoint_id, event_example_in, options = {})
      @api.v1_endpoint_send_example(app_id, endpoint_id, event_example_in, options)
    end
  end
end
