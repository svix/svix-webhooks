# frozen_string_literal: true

module Svix
  class StatisticsAPI
    def initialize(api_client)
      @api = StatisticsApi.new(api_client)
    end

    def calculate_aggregate_app_stats(options = {})
      return @api.calculate_aggregate_app_stats(options)
    end

    def aggregate_event_types(options = {})
      return @api.aggregate_event_types(options)
    end
  end
end
