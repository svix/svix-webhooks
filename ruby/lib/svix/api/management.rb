# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Management
    attr_accessor :authentication
    def initialize(client)
      @client = client
      @authentication = ManagementAuthentication.new(client)
    end
  end
end
