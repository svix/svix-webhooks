# frozen_string_literal: true

require_relative "../lib/svix/message_api.rb"

RSpec.describe Svix::MessageAPI do
  let(:app_id) { "app_123" }
  let(:msg_id) { "msg_123" }
  let(:options) { {with_content: true} }
  subject { described_class.new(param_mock) }

  let(:param_mock) { double("MessageApiParam") }
  let(:api_instance_mock) { double("MessageApi") }
  let(:api_class_mock) { double("MessageApiClass") }

  # Mock out the API calls
  before(:each) do
    stub_const("Svix::MessageApi", api_class_mock)
    expect(api_class_mock).to receive(:new).with(param_mock).and_return(api_instance_mock)
  end

  describe "#get" do
    it "passes it's parameters to the correct method" do
      # Assert that the correct method is called with the correct parameters
      expect(api_instance_mock).to receive(:v1_message_get).with(app_id, msg_id, options)

      subject.get(app_id, msg_id, options)
    end

    context "without options" do
      it "defaults to an empty hash" do
        # Assert that the correct method is called with the correct parameters
        expect(api_instance_mock).to receive(:v1_message_get).with(app_id, msg_id, {})

        subject.get(app_id, msg_id)
      end
    end
  end
end
