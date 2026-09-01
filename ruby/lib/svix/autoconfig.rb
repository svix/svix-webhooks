# frozen_string_literal: true

require "json"
require "base64"
require "uri"

require "svix/models/subscribe_in"
require "svix/api_internal/endpoint_auto_config_deprecated"
require "svix/api_internal/endpoint_autoconfig"

module Svix
  class AutoConfig
    AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_"
    AUTOCONFIG_TOKEN_PREFIX_V2 = "auto_v2_"
    UNSUPPORTED_TOKEN_VERSION = "Unsupported token version. You might need to update the Svix SDK to use this token"

    class InvalidTokenError < StandardError; end

    attr_reader :app_id, :endpoint_id, :endpoint

    def initialize(token, endpoint_in)
      content = AutoConfig.decode_token!(token)
      @app_id = content.fetch("app_id")
      @endpoint_id = content["endpoint_id"]
      @autoconfig_id = content["autoconfig_id"]
      @endpoint = endpoint_in
      @webhook = Webhook.new(content.fetch("endpoint_secret"))
      @client = SvixHttpClient.new(
        content.fetch("token_plaintext"),
        URI(content.fetch("server_url"))
      )
    end

    def subscribe
      if @autoconfig_id
        EndpointAutoconfig.new(@client).subscribe(
          @app_id,
          @autoconfig_id,
          @endpoint
        )
      else
        EndpointAutoConfigDeprecated.new(@client).update(
          @app_id,
          @endpoint_id,
          SubscribeIn.new("endpoint" => @endpoint)
        )
      end
    end

    def verify(payload, headers)
      @webhook.verify(payload, headers)
    end

    class << self
      def decode_token!(token)
        unless token.is_a?(String)
          raise InvalidTokenError, UNSUPPORTED_TOKEN_VERSION
        end

        if token.start_with?(AUTOCONFIG_TOKEN_PREFIX_V1)
          data = parse_token_payload!(token, AUTOCONFIG_TOKEN_PREFIX_V1)
          {
            "version" => "v1",
            "app_id" => data.fetch("aid"),
            "endpoint_id" => data.fetch("eid"),
            "server_url" => data.fetch("surl"),
            "endpoint_secret" => data.fetch("esec"),
            "token_plaintext" => data.fetch("tok"),
          }
        elsif token.start_with?(AUTOCONFIG_TOKEN_PREFIX_V2)
          data = parse_token_payload!(token, AUTOCONFIG_TOKEN_PREFIX_V2)
          {
            "version" => "v2",
            "app_id" => data.fetch("aid"),
            "autoconfig_id" => data.fetch("sid"),
            "server_url" => data.fetch("surl"),
            "endpoint_secret" => data.fetch("esec"),
            "token_plaintext" => data.fetch("tok"),
          }
        else
          raise InvalidTokenError, UNSUPPORTED_TOKEN_VERSION
        end
      rescue ArgumentError, JSON::ParserError, KeyError, TypeError
        raise InvalidTokenError
      end

      private

      def parse_token_payload!(token, prefix)
        encoded = token.byteslice(prefix.length..-1)
        json = Base64.decode64(encoded)
        data = JSON.parse(json)
        unless data.is_a?(Hash)
          raise InvalidTokenError
        end

        data
      end
    end
  end
end
