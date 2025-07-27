require "svix"

RSpec.describe "Trailing Slash Handling" do
  it "removes trailing slash from server URL" do
    client = Svix::Client.new("key", Svix::SvixOptions.new(false, "http://localhost:8000/"))
    uri = client.instance_variable_get(:@application)
                .instance_variable_get(:@api_client)
                .instance_variable_get(:@uri)
    
    expect(uri.to_s).to eq("http://localhost:8000/api/v1")
  end

  it "handles URL without trailing slash" do
    client = Svix::Client.new("key", Svix::SvixOptions.new(false, "http://localhost:8000"))
    uri = client.instance_variable_get(:@application)
                .instance_variable_get(:@api_client)
                .instance_variable_get(:@uri)
    
    expect(uri.to_s).to eq("http://localhost:8000/api/v1")
  end

  it "handles multiple trailing slashes" do
    client = Svix::Client.new("key", Svix::SvixOptions.new(false, "http://localhost:8000///"))
    uri = client.instance_variable_get(:@application)
                .instance_variable_get(:@api_client)
                .instance_variable_get(:@uri)
    
    expect(uri.to_s).to eq("http://localhost:8000/api/v1")
  end

  it "handles regional URLs correctly" do
    # Test with EU region (should use regional URL)
    client_eu = Svix::Client.new("test.eu.token")
    uri_eu = client_eu.instance_variable_get(:@application)
                     .instance_variable_get(:@api_client)
                     .instance_variable_get(:@uri)
    
    expect(uri_eu.to_s).to eq("https://api.eu.svix.com/api/v1")
  end
end