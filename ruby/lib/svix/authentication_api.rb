# frozen_string_literal: true

module Svix
  class AuthenticationAPI
    def initialize(api_client)
      @api = AuthenticationApi.new(api_client)
    end

    def app_portal_access(app_id, app_portal_access_in, options = {})
      return @api.v1_authentication_app_portal_access(app_id, app_portal_access_in, options)
    end

    def dashboard_access(app_id, options = {})
      return @api.v1_authentication_dashboard_access(app_id, options)
    end

    def logout(options = {})
      return @api.v1_authentication_logout(options)
    end
  end
end
