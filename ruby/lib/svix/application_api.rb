# frozen_string_literal: true

module Svix
    class ApplicationAPI
        def initialize(api_client)
            @api = ApplicationApi.new(api_client)
        end

        def list(options = {})
            return @api.list_applications_api_v1_app_get(options)
        end

        def create(application_in, options = {})
            return @api.create_application_api_v1_app_post(application_in, options)
        end
        def get_or_create(application_in, options = {})
            return @api.create_application_api_v1_app_post(application_in, {**options, get_if_exists: true})
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
