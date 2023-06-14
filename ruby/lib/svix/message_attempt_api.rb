# frozen_string_literal: true

module Svix
  class MessageAttemptAPI
    def initialize(api_client)
      @api = MessageAttemptApi.new(api_client)
    end

    # <b>DEPRECATED:</b> Please use <tt>list_by_msg</tt> or <tt>list_by_endpoint</tt> instead.
    def list(app_id, msg_id, options = {})
      warn "[DEPRECATION] `list` is deprecated.  Please use `list_by_msg` or `list_by_endpoint` instead."
      return self.list_by_msg(app_id, msg_id, options)
    end

    def list_by_msg(app_id, msg_id, options = {})
      return @api.v1_message_attempt_list_by_msg(app_id, msg_id, options)
    end

    def list_by_endpoint(app_id, endpoint_id, options = {})
      return @api.v1_message_attempt_list_by_endpoint(app_id, endpoint_id, options)
    end

    def get(app_id, msg_id, attempt_id)
      return @api.v1_message_attempt_get(app_id, msg_id, attempt_id)
    end

    def resend(app_id, msg_id, endpoint_id, options = {})
      return @api.v1_message_attempt_resend(app_id, msg_id, endpoint_id, options)
    end

    def list_attempted_messages(app_id, endpoint_id, options = {})
      return @api.v1_message_attempt_list_attempted_messages(app_id, endpoint_id, options)
    end

    def list_attempted_destinations(app_id, msg_id, options = {})
      return @api.v1_message_attempt_list_attempted_destinations(app_id, msg_id, options)
    end

    def list_attempts_for_endpoint(app_id, endpoint_id, msg_id, options = {})
      return @api.v1_message_attempt_list_by_endpoint_deprecated(app_id, msg_id, endpoint_id, options)
    end

    def expunge_content(app_id, msg_id, attempt_id)
      return @api.v1_message_attempt_expunge_content(app_id, msg_id, attempt_id)
    end
  end
end
