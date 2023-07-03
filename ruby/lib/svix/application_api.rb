# frozen_string_literal: true

module Svix
  class ApplicationAPI
    def initialize(api_client)
      @api = ApplicationApi.new(api_client)
    end

    def list(options = {})
      return @api.v1_application_list(options)
    end

    def create(application_in, options = {})
      return @api.v1_application_create(application_in, options)
    end
    def get_or_create(application_in, options = {})
      return @api.v1_application_create(application_in, {**options, get_if_exists: true})
    end
    def get(app_id)
      return @api.v1_application_get(app_id)
    end

    def update(app_id, application_in)
      return @api.v1_application_update(app_id, application_in)
    end

    def patch(app_id, application_patch)
      return @api.v1_application_patch(app_id, application_patch)
    end

    def delete(app_id)
      return @api.v1_application_delete(app_id)
    end
  end
end
