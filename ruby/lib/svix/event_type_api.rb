# frozen_string_literal: true

module Svix
  class EventTypeAPI
    def initialize(api_client)
      @api = EventTypeApi.new(api_client)
    end

    def list(options = {})
      return @api.v1_event_type_list(options)
    end

    def create(event_type_in, options = {})
      return @api.v1_event_type_create(event_type_in, options)
    end

    def get(event_type_name)
      return @api.v1_event_type_get(event_type_name)
    end

    def update(event_type_name, event_type_update)
      return @api.v1_event_type_update(event_type_name, event_type_update)
    end

    def delete(event_type_name)
      return @api.v1_event_type_delete(event_type_name)
    end
  end
end
