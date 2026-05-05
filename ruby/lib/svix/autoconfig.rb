# frozen_string_literal: true

require "json"
require "base64"
require "uri"

require "svix/models/subscribe_in"
require "svix/api_internal/endpoint_auto_config"

module Svix
  class AutoConfig
    AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_"

    class InvalidTokenError < StandardError; end

    attr_reader :app_id, :endpoint_id, :endpoint

    def initialize(token, endpoint_in)
      content = AutoConfig.decode_token!(token)
      @app_id = content.fetch("app_id")
      @endpoint_id = content.fetch("endpoint_id")
      @endpoint = endpoint_in
      @webhook = Webhook.new(content.fetch("endpoint_secret"))
      @client = SvixHttpClient.new(
        content.fetch("token_plaintext"),
        URI(content.fetch("server_url"))
      )
    end

    def subscribe
      EndpointAutoConfig.new(@client).update(
        @app_id,
        @endpoint_id,
        SubscribeIn.new("endpoint" => @endpoint)
      )
    end

    def verify(payload, headers)
      @webhook.verify(payload, headers)
    end

    class << self
      def decode_token!(token)
        unless token.is_a?(String) && token.start_with?(AUTOCONFIG_TOKEN_PREFIX_V1)
          raise InvalidTokenError
        end

        encoded = token.byteslice(AUTOCONFIG_TOKEN_PREFIX_V1.length..-1)
        json = Base64.decode64(encoded)
        data = JSON.parse(json)
        unless data.is_a?(Hash)
          raise InvalidTokenError
        end

        {
          "app_id" => data.fetch("aid"),
          "endpoint_id" => data.fetch("eid"),
          "server_url" => data.fetch("surl"),
          "endpoint_secret" => data.fetch("esec"),
          "token_plaintext" => data.fetch("tok"),
        }
      rescue ArgumentError, JSON::ParserError, KeyError, TypeError
        raise InvalidTokenError
      end
    end
  end
end
