<?php

namespace Svix\Test;

const TOLERANCE = 5 * 60;
const DEFAULT_MSG_ID = "msg_p5jXN8AQM9LWM0D4loKWxJek";
const DEFAULT_PAYLOAD = '{"test": 2432232315}';
const DEFAULT_SECRET = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";

final class TestPayload
{
    public $id;
    public $timestamp;
    public $payload;
    public $secret;
    public $header;

    public function __construct(int $timestamp)
    {
        $this->id = DEFAULT_MSG_ID;
        $this->timestamp = strval($timestamp);

        $this->payload = DEFAULT_PAYLOAD;
        $this->secret = DEFAULT_SECRET;

        $toSign = "{$this->id}.{$this->timestamp}.{$this->payload}";
        $signature =  base64_encode(pack('H*', hash_hmac('sha256', $toSign, base64_decode($this->secret))));

        $this->header = array(
            "svix-id" => $this->id,
            "svix-signature" => "v1,{$signature}",
            "svix-timestamp" => $this->timestamp,
        );
    }
}

// phpcs:ignore
final class WebhookTest extends \PHPUnit\Framework\TestCase
{

    public function testValidSignatureIsValidAndReturnsJson()
    {
        $testPayload = new TestPayload(time());

        $wh = new \Svix\Webhook($testPayload->secret);
        $json = $wh->verify($testPayload->payload, $testPayload->header);

        $this->assertEquals(
            $json['test'],
            2432232315,
            "did not return expected json"
        );
    }

    public function testInvalidSignatureThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("No matching signature found");

        $testPayload = new TestPayload(time());
        $testPayload->header['svix-signature'] = 'v1,dawfeoifkpqwoekfpqoekf';

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testMissingIdThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");

        $testPayload = new TestPayload(time());
        unset($testPayload->header['svix-id']);

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testMissingTimestampThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");

        $testPayload = new TestPayload(time());
        unset($testPayload->header['svix-timestamp']);

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testMissingSignatureThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");

        $testPayload = new TestPayload(time());
        unset($testPayload->header['svix-signature']);

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testOldTimestampThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Message timestamp too old");

        $testPayload = new TestPayload(time() - TOLERANCE - 1);

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testNewTimestampThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Message timestamp too new");

        $testPayload = new TestPayload(time() + TOLERANCE + 1);

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }
}
