# frozen_string_literal: true

module Svix
    class AuthenticationAPI
        def initialize(api_client)
            @api = AuthenticationApi.new(api_client)
        end

        def dashboard_access(app_id)
            return @api.get_dashboard_access_api_v1_auth_dashboard_access_app_id_post(app_id)
        end
    
        def logout
            return @api.logout_api_v1_auth_logout_post()
        end
    end
end