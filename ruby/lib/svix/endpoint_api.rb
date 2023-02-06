# frozen_string_literal: true

module Svix
  class EndpointAPI
    def initialize(api_client)
      @api = EndpointApi.new(api_client)
    end

    def list(app_id, options = {})
      return @api.list_endpoints_api_v1_app_app_id_endpoint_get(app_id, options)
    end

    def create(app_id, endpoint_in, options = {})
      return @api.create_endpoint_api_v1_app_app_id_endpoint_post(app_id, endpoint_in, options)
    end

    def get(app_id, endpoint_id)
      return @api.get_endpoint_api_v1_app_app_id_endpoint_endpoint_id_get(endpoint_id, app_id)
    end

    def update(app_id, endpoint_id, endpoint_update)
      return @api.update_endpoint_api_v1_app_app_id_endpoint_endpoint_id_put(endpoint_id, app_id, endpoint_update)
    end

    def delete(app_id, endpoint_id)
      return @api.delete_endpoint_api_v1_app_app_id_endpoint_endpoint_id_delete(endpoint_id, app_id)
    end

    def get_secret(app_id, endpoint_id)
      return @api.get_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_get(endpoint_id, app_id)
    end

    def rotate_secret(app_id, endpoint_id, endpoint_secret_rotate_in, options = {})
      return @api.rotate_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_rotate_post(endpoint_id, app_id, endpoint_secret_rotate_in, options)
    end

    def recover(app_id, endpoint_id, recover_in, options = {})
      @api.recover_failed_webhooks_api_v1_app_app_id_endpoint_endpoint_id_recover_post(app_id, endpoint_id, recover_in, options)
      nil
    end

    def get_headers(app_id, endpoint_id)
      return @api.get_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_get(endpoint_id app_id)
    end

    def update_headers(app_id, endpoint_id, endpoint_headers_in)
      return @api.update_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_put(app_id, endpoint_id, endpoint_headers_in)
    end

    def patch_headers(app_id, endpoint_id, endpoint_headers_in)
      return @api.patch_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_patch(app_id, endpoint_id, endpoint_headers_in)
    end

    def get_stats(app_id, endpoint_id)
      return @api.get_endpoint_stats_api_v1_app_app_id_endpoint_endpoint_id_stats_get(endpoint_id app_id)
    end

    def replay_missing(app_id, endpoint_id, replay_in, options = {})
      @api.replay_missing_webhooks_api_v1_app_app_id_endpoint_endpoint_id_replay_missing_post(app_id, endpoint_id, replay_in, options)
      nil
    end

    def transformation_get(app_id, endpoint_id, options = {})
      return @api.get_endpoint_transformation_api_v1_app_app_id_endpoint_endpoint_id_transformation_get(endpoint_id, app_id, options)
    end

    def transformation_partial_update(app_id, endpoint_id, endpoint_transformation_in, options = {})
      @api.set_endpoint_transformation_api_v1_app_app_id_endpoint_endpoint_id_transformation_patch(app_id, endpoint_id, endpoint_transformation_in, options)
      nil
    end
  end
end
