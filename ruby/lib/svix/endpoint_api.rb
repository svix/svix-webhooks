# frozen_string_literal: true

module Svix
    class EndpointAPI
        def initialize(api_client)
            @api = EndpointApi.new(api_client)
        end

        def list(app_id, options = {})
            return @api.list_endpoints_api_v1_app_app_id_endpoint_get(app_id, options)
        end

        def create(app_id, endpoint_in)
            return @api.create_endpoint_api_v1_app_app_id_endpoint_post(app_id, endpoint_in)
        end

        def get(app_id, endpoint_id)
            return @api.get_endpoint_api_v1_app_app_id_endpoint_endpoint_id_get(endpoint_id, app_id)
        end

        def update(app_id, endpoint_id, endpoint_in)
            return @api.update_endpoint_api_v1_app_app_id_endpoint_endpoint_id_put(endpoint_id, app_id, endpoint_update)
        end

        def delete(app_id, endpoint_id)
            return @api.delete_endpoint_api_v1_app_app_id_endpoint_endpoint_id_delete(endpoint_id, app_id)
        end

        def get_secret(app_id, endpoint_id)
            return @api.get_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_get(endpoint_id, app_id)
        end

        def rotate_secret(app_id, endpoint_id, endpoint_secret_rotate_in)
            return @api.rotate_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_rotate_post(endpoint_id, app_id, endpoint_secret_rotate_in)
        end

        def recover(app_id, endpoint_id, recover_in)
            @api.resend_failed_webhooks_api_v1_app_app_id_endpoint_endpoint_id_recover_post(app_id, endpoint_id, recover_in)
            nil
        end

    end
end
