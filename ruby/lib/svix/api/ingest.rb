# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Ingest
    attr_accessor :authentication
    attr_accessor :endpoint
    attr_accessor :source
    def initialize(client)
      @client = client
      @authentication = IngestAuthentication.new(client)
      @endpoint = IngestEndpoint.new(client)
      @source = IngestSource.new(client)
    end
  end
end
