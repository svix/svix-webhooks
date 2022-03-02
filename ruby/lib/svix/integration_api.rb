# frozen_string_literal: true

module Svix
    class IntegrationAPI
        def initialize(api_client)
            @api = IntegrationApi.new(api_client)
        end

        def list(app_id, options = {})
            return @api.list_integrations_api_v1_app_app_id_integration_get(app_id, options)
        end

        def create(app_id, integ_in, options = {})
            return @api.create_integration_api_v1_app_app_id_integration_post(app_id, integ_in, options)
        end

        def get(app_id, integ_id)
            return @api.get_integration_api_v1_app_app_id_integration_integ_id_get(integ_id, app_id)
        end

        def update(app_id, integ_id, integ_update)
            return @api.update_integration_api_v1_app_app_id_integration_integ_id_put(integ_id, app_id, integ_update)
        end

        def delete(app_id, integ_id)
            return @api.delete_integration_api_v1_app_app_id_integration_integ_id_delete(integ_id, app_id)
        end

        def get_key(app_id, integ_id)
            return @api.get_integration_key_api_v1_app_app_id_integration_integ_id_key_get(integ_id, app_id)
        end

        def rotate_key(app_id, integ_id, options = {})
            return @api.rotate_integration_key_api_v1_app_app_id_integration_integ_id_key_rotate_post(integ_id, app_id, options)
        end

    end
end
