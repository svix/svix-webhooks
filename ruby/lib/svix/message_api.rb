# frozen_string_literal: true

module Svix
  class MessageAPI
    def initialize(api_client)
      @api = MessageApi.new(api_client)
    end

    def list(app_id, options = {})
      return @api.list_messages_api_v1_app_app_id_msg_get(app_id, options)
    end

    def create(app_id, message_in, options = {})
      return @api.create_message_api_v1_app_app_id_msg_post(app_id, message_in, options)
    end

    def get(app_id, msg_id)
      return @api.get_message_api_v1_app_app_id_msg_msg_id_get(msg_id, app_id)
    end

    def expunge_content(app_id, msg_id)
      return @api.expunge_message_payload_api_v1_app_app_id_msg_msg_id_content_delete(msg_id, app_id)
    end
  end
end
