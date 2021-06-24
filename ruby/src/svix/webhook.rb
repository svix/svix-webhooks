# frozen_string_literal: true

module Svix
    class Webhook

        def initialize(secret)
            if secret.start_with?(SECRET_PREFIX)
                secret = secret[SECRET_PREFIX.length..-1]
            end

            @secret = Base64.decode64(secret)
        end

        def verify(payload, headers)
            msgId = headers["svix-id"]
            msgSignature = headers["svix-signature"]
            msgTimestamp = headers["svix-timestamp"]
            if !msgSignature || !msgId || !msgTimestamp
                raise WebhookVerificationError, "Missing required headers"
            end

            verify_timestamp(msgTimestamp)

            toSign = "#{msgId}.#{msgTimestamp}.#{payload}"
            signature = Base64.encode64(OpenSSL::HMAC.digest(OpenSSL::Digest.new("sha256"), @secret, toSign)).strip

            passedSignatures = msgSignature.split(" ")
            passedSignatures.each do |versionedSignature|
                version, expectedSignature = versionedSignature.split(",", 2)
                if version != "v1"
                    next
                end
                if Svix.secure_compare(signature, expectedSignature)
                    return JSON.parse(payload, symbolize_names: true)
                end
            end
            raise WebhookVerificationError, "No matching signature found"
        end

        private
        SECRET_PREFIX = "whsec_"
        TOLERANCE = 5 * 60

        def verify_timestamp(timestampHeader)
            begin
                now = Integer(Time.now)
                timestamp = Integer(timestampHeader)
            rescue
                raise WebhookVerificationError, "Invalid Signature Headers"
            end

            if timestamp < (now - TOLERANCE)
                raise WebhookVerificationError, "Message timestamp too old"
            end
            if timestamp > (now + TOLERANCE)
                raise WebhookVerificationError, "Message timestamp too new"
            end
        end
    end
end
