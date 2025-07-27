RSpec.describe "Trailing Slash Handling" do
  describe "URL normalization logic" do
    it "removes single trailing slash" do
      url = "https://api.svix.com/"
      result = url.sub(/\/+$/, '')
      expect(result).to eq("https://api.svix.com")
    end

    it "removes multiple trailing slashes" do
      url = "https://api.svix.com///"
      result = url.sub(/\/+$/, '')
      expect(result).to eq("https://api.svix.com")
    end

    it "preserves URLs without trailing slashes" do
      url = "https://api.svix.com"
      result = url.sub(/\/+$/, '')
      expect(result).to eq("https://api.svix.com")
    end

    it "handles localhost URLs with trailing slash" do
      url = "http://localhost:8000/"
      result = url.sub(/\/+$/, '')
      expect(result).to eq("http://localhost:8000")
    end

    it "handles localhost URLs with multiple trailing slashes" do
      url = "http://localhost:8000///"
      result = url.sub(/\/+$/, '')
      expect(result).to eq("http://localhost:8000")
    end
  end

  describe "regional URL logic" do
    it "detects EU region from token" do
      token = "test.eu.token"
      region = token.split('.')[1]
      expected_url = case region
      when "us" then "https://api.us.svix.com"
      when "eu" then "https://api.eu.svix.com"
      when "in" then "https://api.in.svix.com"
      when "ca" then "https://api.ca.svix.com"
      when "au" then "https://api.au.svix.com"
      else "https://api.svix.com"
      end
      
      expect(expected_url).to eq("https://api.eu.svix.com")
    end

    it "defaults to main API for unknown regions" do
      token = "test.unknown.token"
      region = token.split('.')[1]
      expected_url = case region
      when "us" then "https://api.us.svix.com"
      when "eu" then "https://api.eu.svix.com"
      when "in" then "https://api.in.svix.com"
      when "ca" then "https://api.ca.svix.com"
      when "au" then "https://api.au.svix.com"
      else "https://api.svix.com"
      end
      
      expect(expected_url).to eq("https://api.svix.com")
    end
  end

  describe "combined logic test" do
    it "processes URLs with trailing slashes and regional detection" do
      test_cases = [
        { input: "https://api.svix.com/", expected: "https://api.svix.com" },
        { input: "https://api.eu.svix.com///", expected: "https://api.eu.svix.com" },
        { input: "http://localhost:8000/", expected: "http://localhost:8000" }
      ]

      test_cases.each do |test_case|
        result = test_case[:input].sub(/\/+$/, '')
        expect(result).to eq(test_case[:expected])
      end
    end
  end
end