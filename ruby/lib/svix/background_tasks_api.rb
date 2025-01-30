# frozen_string_literal: true

module Svix
  class BackgroundTasksAPI
    def initialize(api_client)
      @api = BackgroundTasksApi.new(api_client)
    end

    def list(options = {})
      return @api.v1_background_task_list(options)
    end

    def get(task_id, options = {})
      return @api.v1_background_task_get(task_id, options)
    end
  end
end
