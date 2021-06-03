require 'svix'

describe Svix::Webhook do
    it "valid signature is valid" do
        wh = Svix::Webhook.new("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw")
        payload = '{"test": 2432232314}'
        headers = {
            "svix-id": "msg_p5jXN8AQM9LWM0D4loKWxJek",
            "svix-timestamp": "1614265330",
            "svix-signature": "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="
        }
        wh.verify(payload, headers)
    end
end
  