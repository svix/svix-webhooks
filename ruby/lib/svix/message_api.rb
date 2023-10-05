# frozen_string_literal: true

module Svix
  class MessageAPI
    def initialize(api_client)
      @api = MessageApi.new(api_client)
    end

    def list(app_id, options = {})
      return @api.v1_message_list(app_id, options)
    end

    def create(app_id, message_in, options = {})
      return @api.v1_message_create(app_id, message_in, options)
    end

    def get(app_id, msg_id, options = {})
      return @api.v1_message_get(app_id, msg_id, options)
    end

    def expunge_content(app_id, msg_id)
      return @api.v1_message_expunge_content(app_id, msg_id)
    end
  end
end
