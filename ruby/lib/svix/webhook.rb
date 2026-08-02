# frozen_string_literal: true

module Svix
  class Webhook

    def self.new_using_raw_bytes(secret)
      self.new(secret.pack("C*").force_encoding("UTF-8"))
    end

    def initialize(secret)
      if secret.start_with?(SECRET_PREFIX)
        secret = secret[SECRET_PREFIX.length..-1]
      end

      @secret = Base64.decode64(secret)

      if @secret.empty?
        raise EmptyWebhookSecretError, "Webhook secret must not be blank"
      end
    end

    # Validates the payload against the svix signature headers using the
    # webhook's signing secret.
    #
    # Raises a WebhookVerificationError if the headers are missing/unreadable
    # or if the signature doesn't match.
    def verify(payload, headers)
      verify_internal(payload, headers, true)
    end

    # Validates the payload against the svix signature headers using the
    # webhook's signing secret.
    #
    # Raises a WebhookVerificationError if the headers are missing/unreadable
    # or if the signature doesn't match.
    #
    # WARNING: This method does not check the signature's timestamp.
    # We recommend using the `verify` method instead.
    def verify_ignoring_timestamp(payload, headers)
      verify_internal(payload, headers, false)
    end

    def sign(msgId, timestamp, payload)
      begin
        now = Integer(timestamp)
      rescue
        raise WebhookSigningError, "Invalid timestamp"
      end

      toSign = "#{msgId}.#{timestamp}.#{payload}"
      signature = Base64.encode64(OpenSSL::HMAC.digest(OpenSSL::Digest.new("sha256"), @secret, toSign)).strip
      return "v1,#{signature}"
    end

    private
    SECRET_PREFIX = "whsec_"
    TOLERANCE = 5 * 60

    def verify_internal(payload, headers, enforce_tolerance)
      msgId = headers["svix-id"]
      msgSignature = headers["svix-signature"]
      msgTimestamp = headers["svix-timestamp"]
      if !msgSignature || !msgId || !msgTimestamp
        msgId = headers["webhook-id"]
        msgSignature = headers["webhook-signature"]
        msgTimestamp = headers["webhook-timestamp"]
        if !msgSignature || !msgId || !msgTimestamp
          raise WebhookVerificationError, "Missing required headers"
        end
      end

      timestamp = parse_timestamp_header(msgTimestamp)
      verify_timestamp(timestamp) if enforce_tolerance

      _, signature = sign(msgId, msgTimestamp, payload).split(",", 2)

      passedSignatures = msgSignature.split(" ")
      passedSignatures.each do |versionedSignature|
        version, expectedSignature = versionedSignature.split(",", 2)
        if version != "v1"
          next
        end

        if ::Svix::secure_compare(signature, expectedSignature)
          return nil
        end
      end

      raise WebhookVerificationError, "No matching signature found"
    end

    def parse_timestamp_header(timestampHeader)
      begin
        Integer(timestampHeader)
      rescue
        raise WebhookVerificationError, "Invalid Signature Headers"
      end
    end

    def verify_timestamp(timestamp)
      now = Time.now.to_i

      if timestamp < (now - TOLERANCE)
        raise WebhookVerificationError, "Message timestamp too old"
      end

      if timestamp > (now + TOLERANCE)
        raise WebhookVerificationError, "Message timestamp too new"
      end
    end
  end
end
