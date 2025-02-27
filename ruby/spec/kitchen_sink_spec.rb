# frozen_string_literal: true

require_relative "../lib/svix"

token = ENV["SVIX_TOKEN"]
server_url = ENV["SVIX_SERVER_URL"]
test_client = nil

if token.nil? || server_url.nil?
  warn("Unable to instantiate test client without both `SVIX_TOKEN` and `SVIX_SERVER_URL`")
else
  opts = Svix::SvixOptions.new(false, server_url)
  test_client = Svix::Client.new(token, opts)
end

RSpec.describe Svix::Client do
  describe "Endpoint CRUD", {if: !test_client.nil?} do
    it "seems to work okay" do
      app = test_client.application.create(Svix::ApplicationIn.new(name: "App"))

      begin
        test_client.event_type.create(Svix::EventTypeIn.new(name: "event.started", description: "Something started"))
      rescue Svix::ApiError => err
        # Conflicts are expected from test run to test run, but other statuses are not.
        expect(err.code).to(eq(409))
      end

      begin
        test_client.event_type.create(Svix::EventTypeIn.new(name: "event.ended", description: "Something ended"))
      rescue Svix::ApiError => err
        # Conflicts are expected from test run to test run, but other statuses are not.
        expect(err.code).to(eq(409))
      end

      ep = test_client.endpoint.create(
        app.id,
        Svix::EndpointIn.new(url: "https://example.svix.com/", channels: %w[ch0 ch1])
      )

      expect(ep.channels.to_set).to(eq(%w[ch0 ch1].to_set))
      expect(ep.filter_types).to(be_nil)

      ep_patched = test_client.endpoint.patch(
        app.id,
        ep.id,
        Svix::EndpointPatch.new(filter_types: %w[event.started event.ended])
      )

      expect(ep_patched.channels.to_set).to(eq(%w[ch0 ch1].to_set))
      expect(ep_patched.filter_types.to_set).to(eq(%w[event.started event.ended].to_set))

      # If the serialization is handling empty response bodies, this should not throw an exception
      test_client.endpoint.delete(app.id, ep.id)
    end
  end
end
