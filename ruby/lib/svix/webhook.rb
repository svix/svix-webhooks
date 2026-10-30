# frozen_string_literal: true

module Svix
  class Webhook

    # `secret` is the raw key as an array of bytes, used as the HMAC key as-is.
    def self.new_using_raw_bytes(secret, tolerance: DEFAULT_TOLERANCE)
      self.new(raw_secret: secret.pack("C*"), tolerance: tolerance)
    end

    # `secret` is the base64-encoded signing secret, with or without the
    # `whsec_` prefix. Pass `raw_secret:` instead to use a binary string as the
    # key as-is.
    #
    # `tolerance` is the maximum difference allowed, in seconds, between the
    # webhook's timestamp and the current time. Defaults to 5 minutes.
    def initialize(secret = nil, raw_secret: nil, tolerance: DEFAULT_TOLERANCE)
      if !secret.nil? && !raw_secret.nil?
        raise ArgumentError, "pass either secret or raw_secret, not both"
      elsif !secret.nil?
        @secret = Base64.decode64(secret.delete_prefix(SECRET_PREFIX))
      elsif !raw_secret.nil?
        @secret = raw_secret.b
      else
        raise ArgumentError, "must pass secret or raw_secret"
      end

      if @secret.empty?
        raise EmptyWebhookSecretError, "Webhook secret must not be blank"
      end

      if !tolerance.is_a?(Integer) || tolerance < 0
        raise ArgumentError, "tolerance must be a non-negative integer"
      end
      @tolerance = tolerance
    end

    def verify(payload, headers)
      # Svix sends the headers in lowercase, but some servers hand them back
      # with a different casing (Rack typically yields "Svix-Id"), so normalize
      # once up front the way the JavaScript library does.
      msgId, msgTimestamp, msgSignature = find_signed_headers(downcase_keys(headers))
      if !msgSignature || !msgId || !msgTimestamp
        raise WebhookVerificationError, "Missing required headers"
      end

      verify_timestamp(msgTimestamp)

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
    DEFAULT_TOLERANCE = 5 * 60

    # Returns the id, timestamp and signature values, or nil when neither the
    # branded nor the unbranded set of headers is present.
    def find_signed_headers(headers)
      ["svix", "webhook"].each do |prefix|
        msgId = headers["#{prefix}-id"]
        msgTimestamp = headers["#{prefix}-timestamp"]
        msgSignature = headers["#{prefix}-signature"]

        if msgId && msgTimestamp && msgSignature
          return [msgId, msgTimestamp, msgSignature]
        end
      end

      nil
    end

    def downcase_keys(headers)
      return headers unless headers.respond_to?(:each_pair)

      headers.each_pair.with_object({}) do |(name, value), downcased|
        downcased[name.to_s.downcase] = value
      end
    end

    def verify_timestamp(timestampHeader)
      begin
        now = Integer(Time.now)
        timestamp = Integer(timestampHeader)
      rescue
        raise WebhookVerificationError, "Invalid Signature Headers"
      end

      if timestamp < (now - @tolerance)
        raise WebhookVerificationError, "Message timestamp too old"
      end

      if timestamp > (now + @tolerance)
        raise WebhookVerificationError, "Message timestamp too new"
      end

      if timestamp <= 0
        # Like the Rust SDK, timestamps before 1970 are not honored even when
        # the tolerance window would allow them.
        raise WebhookVerificationError, "Invalid Signature Headers"
      end
    end
  end
end
