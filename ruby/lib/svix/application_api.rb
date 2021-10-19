# frozen_string_literal: true

module Svix
    class ApplicationAPI
        def initialize(api_client)
            @api = ApplicationApi.new(api_client)
        end

        def list(options = ApplicationListOptions.new)
            return @api.list_applications_api_v1_app_get(opts)
        end
    
        def create(application_in)
            return @api.create_application_api_v1_app_post(application_in)
        end

        def get(app_id) 
            return @api.get_application_api_v1_app_app_id_get(app_id)
        end

        def update(app_id, application_in)
            return @api.update_application_api_v1_app_app_id_put(app_id, application_in)
        end

        def delete(app_id)
            return @api.delete_application_api_v1_app_app_id_delete(app_id)
        end
    end
end
