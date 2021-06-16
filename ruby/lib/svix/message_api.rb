# frozen_string_literal: true

module Svix
    class MessageAPI
        def initialize(api_client)
            @api = MessageApi.new(api_client)
        end

        def list(app_id, options = FetchOptions.new)
            return @api.list_messages_api_v1_app_app_id_msg_get(app_id, {})
        end

        def create(app_id, message_in)
            return @api.create_message_api_v1_app_app_id_msg_post(app_id, message_in)
        end

        def get(app_id, msg_id)
            return @api.get_message_api_v1_app_app_id_msg_msg_id_get(app_id, msg_id)
        end
    end
end