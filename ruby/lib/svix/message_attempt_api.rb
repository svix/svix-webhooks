# frozen_string_literal: true

module Svix
    class MessageAttemptAPI
        def initialize(api_client)
            @api = MessageAttemptApi.new(api_client)
        end

        
        def list(app_id, msg_id, options = FetchOptionsMessageAttempt.new)
            return @api.list_attempts_api_v1_app_app_id_msg_msg_id_attempt_get(app_id, msg_id, {})
        end
    
        def get(app_id, msg_id, attempt_id)
            return @api.get_attempt_api_v1_app_app_id_msg_msg_id_attempt_attempt_id_get(app_id, msg_id, attempt_id)
        end
    
        def resend(app_id: str, msg_id: str, endpoint_id: str)
            return @api.resend_webhook_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_resend_post(app_id, msg_id, endpoint_id)
        end
    end
end