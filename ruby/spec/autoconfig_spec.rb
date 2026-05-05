# frozen_string_literal: true

require "spec_helper"
require_relative "../lib/svix"

RSpec.describe Svix::AutoConfig do
  describe ".decode_token!" do
    it "parses a v1 payload" do
      json = '{"aid":"app_1","eid":"ep_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}'
      token = "#{described_class::AUTOCONFIG_TOKEN_PREFIX_V1}#{Base64.strict_encode64(json)}"
      content = described_class.decode_token!(token)

      expect(content["app_id"]).to eq("app_1")
      expect(content["endpoint_id"]).to eq("ep_2")
      expect(content["server_url"]).to eq("https://api.example.test")
      expect(content["endpoint_secret"]).to eq("whsec_Zm9v")
      expect(content["token_plaintext"]).to eq("sk_test_xyz")
    end

    it "rejects a bad prefix" do
      json = '{"aid":"a","eid":"e","surl":"https://x","esec":"whsec_Zm9v","tok":"t"}'
      token = "wrong_#{Base64.strict_encode64(json)}"
      expect { described_class.decode_token!(token) }.to raise_error(Svix::AutoConfig::InvalidTokenError)
    end

    it "rejects invalid json" do
      token = "#{described_class::AUTOCONFIG_TOKEN_PREFIX_V1}#{Base64.strict_encode64('not json')}"
      expect { described_class.decode_token!(token) }.to raise_error(Svix::AutoConfig::InvalidTokenError)
    end
  end
end
