# frozen_string_literal: true

require "svix"

describe Svix::Client do
    it "works" do
        svix = Svix::Client.new("AUTH_TOKEN", Svix::SvixOptions.new(true, "http://localhost:8040"))

        puts svix.application.list()
    end
end
