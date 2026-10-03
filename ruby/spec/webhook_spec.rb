# frozen_string_literal: true

require "svix"

DEFAULT_MSG_ID = "msg_p5jXN8AQM9LWM0D4loKWxJek"
DEFAULT_PAYLOAD = "{\"test\": 2432232314}"
DEFAULT_SECRET = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw"
TOLERANCE = 5 * 60

class TestPayload

  def initialize(id: DEFAULT_MSG_ID, timestamp: Time.now.to_i, payload: DEFAULT_PAYLOAD, secret: DEFAULT_SECRET)
    @id = id
    @timestamp = timestamp

    @payload = payload
    @secret = secret

    toSign = "#{@id}.#{@timestamp}.#{@payload}"
    @signature = Base64
      .encode64(OpenSSL::HMAC.digest(OpenSSL::Digest.new("sha256"), Base64.decode64(@secret), toSign))
      .strip

    @headers = {
      "svix-id" => @id,
      "svix-signature" => "v1,#{@signature}",
      "svix-timestamp" => @timestamp
    }
  end

  attr_accessor :secret
  attr_accessor :id
  attr_accessor :timestamp
  attr_accessor :payload
  attr_accessor :signature
  attr_accessor :headers
end

describe Svix::Webhook do
  describe ".new" do
    it "rejects empty webhook secrets" do
      expect { Svix::Webhook.new("") }.to raise_error(Svix::EmptyWebhookSecretError)
      expect { Svix::Webhook.new("whsec_") }.to raise_error(Svix::EmptyWebhookSecretError)
    end
  end

  describe ".new_using_raw_bytes" do
    it "rejects empty webhook secrets" do
      expect { Svix::Webhook.new("") }.to raise_error(Svix::EmptyWebhookSecretError)
    end
  end

  it "missing id raises error" do
    testPayload = TestPayload.new
    testPayload.headers.delete("svix-id")

    wh = Svix::Webhook.new(testPayload.secret)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "missing timestamp raises error" do
    testPayload = TestPayload.new
    testPayload.headers.delete("svix-timestamp")

    wh = Svix::Webhook.new(testPayload.secret)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "missing signature raises error" do
    testPayload = TestPayload.new
    testPayload.headers.delete("svix-signature")

    wh = Svix::Webhook.new(testPayload.secret)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "invalid signature raises error" do
    testPayload = TestPayload.new
    testPayload.headers["svix-signature"] = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLawdd"

    wh = Svix::Webhook.new(testPayload.secret)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "valid signature is valid" do
    testPayload = TestPayload.new
    wh = Svix::Webhook.new(testPayload.secret)

    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "valid unbranded signature is valid" do
    testPayload = TestPayload.new
    unbrandedHeaders = {
      "webhook-id" => testPayload.headers["svix-id"],
      "webhook-signature" => testPayload.headers["svix-signature"],
      "webhook-timestamp" => testPayload.headers["svix-timestamp"]
    }
    testPayload.headers = unbrandedHeaders

    wh = Svix::Webhook.new(testPayload.secret)

    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "title cased signature is valid" do
    testPayload = TestPayload.new
    testPayload.headers = {
      "Svix-Id" => testPayload.headers["svix-id"],
      "Svix-Signature" => testPayload.headers["svix-signature"],
      "Svix-Timestamp" => testPayload.headers["svix-timestamp"]
    }

    wh = Svix::Webhook.new(testPayload.secret)

    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "mixed case signature is valid" do
    testPayload = TestPayload.new
    testPayload.headers = {
      "SVIX-ID" => testPayload.headers["svix-id"],
      "sViX-SiGnAtUrE" => testPayload.headers["svix-signature"],
      "Svix-TimeStamp" => testPayload.headers["svix-timestamp"]
    }

    wh = Svix::Webhook.new(testPayload.secret)

    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "title cased unbranded signature is valid" do
    testPayload = TestPayload.new
    testPayload.headers = {
      "Webhook-Id" => testPayload.headers["svix-id"],
      "Webhook-Signature" => testPayload.headers["svix-signature"],
      "Webhook-Timestamp" => testPayload.headers["svix-timestamp"]
    }

    wh = Svix::Webhook.new(testPayload.secret)

    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "missing title cased id raises error" do
    testPayload = TestPayload.new
    testPayload.headers = {
      "Svix-Signature" => testPayload.headers["svix-signature"],
      "Svix-Timestamp" => testPayload.headers["svix-timestamp"]
    }

    wh = Svix::Webhook.new(testPayload.secret)

    expect { wh.verify(testPayload.payload, testPayload.headers) }
      .to(raise_error(Svix::WebhookVerificationError, /Missing required headers/))
  end

  it "old timestamp raises error" do
    testPayload = TestPayload.new(timestamp: Time.now.to_i - TOLERANCE - 1)

    wh = Svix::Webhook.new(testPayload.secret)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "new timestamp raises error" do
    testPayload = TestPayload.new(timestamp: Time.now.to_i + TOLERANCE + 1)

    wh = Svix::Webhook.new(testPayload.secret)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "invalid timestamp raises error" do
    testPayload = TestPayload.new(timestamp: "teadwd")

    wh = Svix::Webhook.new(testPayload.secret)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "multi sig payload is valid" do
    testPayload = TestPayload.new
    sigs = [
      "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
      "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
      # valid signature
      testPayload.headers["svix-signature"],
      "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc="
    ]
    testPayload.headers["svix-signature"] = sigs.join(" ")

    wh = Svix::Webhook.new(testPayload.secret)

    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "signature verification works with and without prefix" do
    testPayload = TestPayload.new

    wh = Svix::Webhook.new(testPayload.secret)
    wh.verify(testPayload.payload, testPayload.headers)

    wh = Svix::Webhook.new("whsec_" + testPayload.secret)
    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "sign function works" do
    key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw"
    msg_id = "msg_p5jXN8AQM9LWM0D4loKWxJek"
    timestamp = 1614265330
    payload = "{\"test\": 2432232314}"
    expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="

    wh = Svix::Webhook.new(key)
    signature = wh.sign(msg_id, timestamp, payload)
    expect(signature).to(eq(expected))
  end

  it "custom tolerance accepts an old timestamp the default would reject" do
    testPayload = TestPayload.new(timestamp: Time.now.to_i - (2 * TOLERANCE))

    wh = Svix::Webhook.new(testPayload.secret, tolerance: 3 * TOLERANCE)

    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "custom tolerance rejects an old timestamp the default would accept" do
    testPayload = TestPayload.new(timestamp: Time.now.to_i - 120)

    wh = Svix::Webhook.new(testPayload.secret, tolerance: 60)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "custom tolerance accepts a future timestamp the default would reject" do
    testPayload = TestPayload.new(timestamp: Time.now.to_i + (2 * TOLERANCE))

    wh = Svix::Webhook.new(testPayload.secret, tolerance: 3 * TOLERANCE)

    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "custom tolerance rejects a future timestamp the default would accept" do
    testPayload = TestPayload.new(timestamp: Time.now.to_i + 120)

    wh = Svix::Webhook.new(testPayload.secret, tolerance: 60)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "invalid signature still raises with a custom tolerance" do
    testPayload = TestPayload.new
    testPayload.headers["svix-signature"] = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLawdd"

    wh = Svix::Webhook.new(testPayload.secret, tolerance: 3 * TOLERANCE)

    expect { wh.verify(testPayload.payload, testPayload.headers) }.to(raise_error(Svix::WebhookVerificationError))
  end

  it "tampered timestamp still raises with a custom tolerance" do
    # The signature covers the timestamp, so a value changed after signing
    # must fail even when it is inside the tolerance window.
    testPayload = TestPayload.new
    testPayload.headers["svix-timestamp"] = testPayload.timestamp - 600

    wh = Svix::Webhook.new(testPayload.secret, tolerance: 3 * TOLERANCE)

    expect { wh.verify(testPayload.payload, testPayload.headers) }
      .to(raise_error(Svix::WebhookVerificationError, /No matching signature found/))
  end

  it "a tolerance spanning the epoch still rejects a non-positive timestamp" do
    # Like the Rust SDK, timestamps before 1970 are not honored even when the
    # tolerance window would allow them.
    testPayload = TestPayload.new(timestamp: -1234)

    wh = Svix::Webhook.new(testPayload.secret, tolerance: 10**10)

    expect { wh.verify(testPayload.payload, testPayload.headers) }
      .to(raise_error(Svix::WebhookVerificationError, /Invalid Signature Headers/))
  end

  it "negative tolerance raises error" do
    expect { Svix::Webhook.new(DEFAULT_SECRET, tolerance: -1) }.to(raise_error(ArgumentError))
  end

  it "non-integer tolerance raises error" do
    expect { Svix::Webhook.new(DEFAULT_SECRET, tolerance: Float::NAN) }.to(raise_error(ArgumentError))
    expect { Svix::Webhook.new(DEFAULT_SECRET, tolerance: nil) }.to(raise_error(ArgumentError))
  end

  it "new_using_raw_bytes accepts a custom tolerance" do
    testPayload = TestPayload.new(timestamp: Time.now.to_i - (2 * TOLERANCE))

    wh = Svix::Webhook.new_using_raw_bytes(testPayload.secret.bytes, tolerance: 3 * TOLERANCE)

    wh.verify(testPayload.payload, testPayload.headers)
  end

  it "can validate an empty payload" do
    testPayload = TestPayload.new(payload: '')

    wh = Svix::Webhook.new(testPayload.secret)

    wh.verify(testPayload.payload, testPayload.headers)
  end
end
