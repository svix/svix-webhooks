# frozen_string_literal: true

module Svix
    class EventTypeAPI
        def initialize(api_client)
            @api = EventTypeApi.new(api_client)
        end

        def list(options = FetchOptions.new)
            return @api.list_event_types_api_v1_event_type_get({})
        end
    
        def create(event_type_in_out)
            return @api.create_event_type_api_v1_event_type_post(event_type_in_out)
        end
    
        def update(event_type_name, event_type_update)
            return @api.update_event_type_api_v1_event_type_event_type_name_put(event_type_name, event_type_update)
        end
    
        def delete(event_type_name)
            return @api.delete_event_type_api_v1_event_type_event_type_name_delete(event_type_name)
        end
    end
end