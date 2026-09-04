# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Destination
    attr_accessor :autoconfig
    def initialize(client)
      @client = client
      @autoconfig = DestinationAutoconfig.new(client)
    end
  end
end
