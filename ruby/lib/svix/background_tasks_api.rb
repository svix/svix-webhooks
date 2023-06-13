# frozen_string_literal: true

module Svix
  class BackgroundTasksAPI
    def initialize(api_client)
      @api = BackgroundTasksApi.new(api_client)
    end

    def list(options = {})
      return @api.list_background_tasks(options)
    end

    def get(task_id, options = {})
      return @api.get_background_task(task_id, options)
    end
  end
end
