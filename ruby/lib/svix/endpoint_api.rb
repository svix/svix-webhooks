# frozen_string_literal: true

module Svix
    class EndpointAPI
        def initialize(api_client)
            @api = EndpointApi.new(api_client)
        end

        def list(str, options = EndpointListOptions.new)
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
    
    end
end
