# frozen_string_literal: true

module Svix
    class EnvironmentAPI
        def initialize(api_client)
            @api = EnvironmentApi.new(api_client)
        end

        def export
            return @api.export_environment_configuration_api_v1_environment_export_post({})
        end

        def import(environment_in)
            @api.import_environment_configuration_api_v1_environment_import_post(environment_in)
            nil
        end
    end
end
