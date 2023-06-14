# frozen_string_literal: true

module Svix
  class IntegrationAPI
    def initialize(api_client)
      @api = IntegrationApi.new(api_client)
    end

    def list(app_id, options = {})
      return @api.v1_integration_list(app_id, options)
    end

    def create(app_id, integ_in, options = {})
      return @api.v1_integration_create(app_id, integ_in, options)
    end

    def get(app_id, integ_id)
      return @api.v1_integration_get(app_id, integ_id)
    end

    def update(app_id, integ_id, integ_update)
      return @api.v1_integration_update(app_id, integ_id, integ_update)
    end

    def delete(app_id, integ_id)
      return @api.v1_integration_delete(app_id, integ_id)
    end

    def get_key(app_id, integ_id)
      return @api.v1_integration_get_key(app_id, integ_id)
    end

    def rotate_key(app_id, integ_id, options = {})
      return @api.v1_integration_rotate_key(app_id, integ_id, options)
    end

  end
end
