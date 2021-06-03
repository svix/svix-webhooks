require 'svix'

describe Svix::Webhook do
    it "missing id raises error" do
        wh = Svix::Webhook.new("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw")
        payload = '{"test": 2432232314}'
        headers = {
            "svix-timestamp" => "1614265330",
            "svix-signature" => "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="
        }
        expect { wh.verify(payload, headers) }.to raise_error(Svix::WebhookVerificationError)
    end
    
    it "missing timestamp raises error" do
        wh = Svix::Webhook.new("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw")
        payload = '{"test": 2432232314}'
        headers = {
            "svix-id" => "msg_p5jXN8AQM9LWM0D4loKWxJek",
            "svix-signature" => "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="
        }
        expect { wh.verify(payload, headers) }.to raise_error(Svix::WebhookVerificationError)
    end

    it "missing signature raises error" do
        wh = Svix::Webhook.new("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw")
        payload = '{"test": 2432232314}'
        headers = {
            "svix-id" => "msg_p5jXN8AQM9LWM0D4loKWxJek",
            "svix-timestamp" => "1614265330",
        }
        expect { wh.verify(payload, headers) }.to raise_error(Svix::WebhookVerificationError)
    end

    it "invalid signature raises error" do
        wh = Svix::Webhook.new("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw")
        payload = '{"test": 2432232314}'
        headers = {
            "svix-id" => "msg_p5jXN8AQM9LWM0D4loKWxJek",
            "svix-timestamp" => "1614265330",
            "svix-signature" => "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLawdd"
        }
        expect { wh.verify(payload, headers) }.to raise_error(Svix::WebhookVerificationError)
    end

    it "valid signature is valid and returns valid json" do
        wh = Svix::Webhook.new("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw")
        payload = '{"test": 2432232314}'
        headers = {
            "svix-id" => "msg_p5jXN8AQM9LWM0D4loKWxJek",
            "svix-timestamp" => "1614265330",
            "svix-signature" => "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="
        }
        json = wh.verify(payload, headers)
        expect(json[:test]).to eq(2432232314)
    end
end
  