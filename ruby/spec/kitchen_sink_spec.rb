# frozen_string_literal: true

require_relative "../lib/svix"
require 'subprocess'

SVIX_ORG_ID = "org_svix_ruby_tests"
DOCKER_COMPOSE_DIR = File.join(File.dirname(__FILE__), '..', '..', 'client-integration-tests')

def docker_compose(*args)
  args = ['docker', 'compose', *args]
  process = Subprocess::Process.new(args, {cwd: DOCKER_COMPOSE_DIR, stdout: Subprocess::PIPE, stderr: Subprocess::PIPE})
  stdout, stderr = process.communicate
  status = process.wait
  if !status.success?
    $stderr.puts "#{args} failed with status #{status}"
    $stderr.puts stderr
    raise "Called process failed"
  end
  stdout
end

def docker_generate_token
  raw_output = docker_compose('exec', '-T', 'backend', 'svix-server', 'jwt', 'generate', SVIX_ORG_ID)
  if (md = /Token \(Bearer\): (.*)/.match(raw_output))
    md[1]
  else
    raise "Failed to generate JWT: #{raw_output}"
  end
end

def docker_compose_port_for(service_name, container_port)
  docker_compose('ps', '--format=json').split("\n").filter_map do |line|
    json = JSON.parse(line)
    if json["Service"] == service_name
      if port_row = json["Publishers"].find { |row| row["TargetPort"] == container_port }
        port_row["PublishedPort"]
      end
    end
  end.first
end


RSpec.describe Svix::Client do
  before :all do
    if ENV["SVIX_TOKEN"].nil?
      docker_compose "up", "-d", "--quiet-pull"
      $token = docker_generate_token
      $server_url = "http://localhost:#{docker_compose_port_for("backend", 8071)}"
    else
      $token = ENV["SVIX_TOKEN"]
      $server_url = ENV["SVIX_SERVER_URL"]
    end
  end

  after :all do
    if ENV["SVIX_TOKEN"].nil?
      docker_compose("down", "-v")
    end
  end

  before :each do
    if ENV["SVIX_TOKEN"].nil?
      docker_compose('exec', '-T', 'backend', 'svix-server', 'wipe', '--yes-i-know-what-im-doing', SVIX_ORG_ID)
    end

    opts = Svix::SvixOptions.new(false, $server_url)
    @test_client = Svix::Client.new($token, opts)
  end

  describe "Endpoint CRUD" do
    it "seems to work okay" do
      app = @test_client.application.create(Svix::ApplicationIn.new(name: "App"))

      begin
        @test_client.event_type.create(Svix::EventTypeIn.new(name: "event.started", description: "Something started"))
      rescue Svix::ApiError => err
        # Conflicts are expected from test run to test run, but other statuses are not.
        expect(err.code).to(eq(409))
      end

      begin
        @test_client.event_type.create(Svix::EventTypeIn.new(name: "event.ended", description: "Something ended"))
      rescue Svix::ApiError => err
        # Conflicts are expected from test run to test run, but other statuses are not.
        expect(err.code).to(eq(409))
      end

      ep = @test_client.endpoint.create(
        app.id,
        Svix::EndpointIn.new(url: "https://example.svix.com/", channels: %w[ch0 ch1])
      )

      expect(ep.channels.to_set).to(eq(%w[ch0 ch1].to_set))
      expect(ep.event_types).to(be_nil)

      ep_patched = @test_client.endpoint.patch(
        app.id,
        ep.id,
        Svix::EndpointPatch.new(event_types: %w[event.started event.ended])
      )

      expect(ep_patched.channels.to_set).to(eq(%w[ch0 ch1].to_set))
      expect(ep_patched.event_types.to_set).to(eq(%w[event.started event.ended].to_set))

      # If the serialization is handling empty response bodies, this should not throw an exception
      @test_client.endpoint.delete(app.id, ep.id)
    end
  end
end
