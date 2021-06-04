<?php

namespace Svix;

final class WebhookTest extends \PHPUnit\Framework\TestCase
{
    public const SECRET = 'MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw';
    public const PAYLOAD = '{"test": 2432232314}';
    public const HEADER = array(
        'svix-id'  => 'msg_p5jXN8AQM9LWM0D4loKWxJek',
        'svix-timestamp' => '1614265330',
        'svix-signature' => 'v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=',
    );

    public function testValidSignatureIsValidAndReturnsJson()
    {
        $wh = new Webhook(self::SECRET);
        $json = $wh->verify(self::PAYLOAD, self::HEADER);
        $this->assertEquals(
            $json['test'],
            2432232314,
            "did not return expected json"
        );
    }

    public function testInvalidSignatureThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("No matching signature found");

        $wh = new Webhook(self::SECRET);
        $wh->verify('{"test": 2432232315}', self::HEADER);
    }

    public function testMissingIdThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");

        $header = self::HEADER;
        unset($header['svix-id']);

        $wh = new Webhook(self::SECRET);
        $wh->verify(self::PAYLOAD, $header);
    }

    public function testMissingTimestampThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");

        $header = self::HEADER;
        unset($header['svix-timestamp']);

        $wh = new Webhook(self::SECRET);
        $wh->verify(self::PAYLOAD, $header);
    }

    public function testMissingSignatureThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");

        $header = self::HEADER;
        unset($header['svix-signature']);

        $wh = new Webhook(self::SECRET);
        $wh->verify(self::PAYLOAD, $header);
    }
}
