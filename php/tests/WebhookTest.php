<?php

namespace Svix\Tests;

final class WebhookTest extends \PHPUnit\Framework\TestCase
{
    private const TOLERANCE = 5 * 60;

    public function testValidSignatureIsValid()
    {
        $testPayload = new TestPayload(time());

        $wh = new \Svix\Webhook($testPayload->secret);
        $json = $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testValidBrandlessSignatureIsValid()
    {
        $testPayload = new TestPayload(time());
        $unbrandedHeaders = array(
            "webhook-id" => $testPayload->header['svix-id'],
            "webhook-signature" => $testPayload->header['svix-signature'],
            "webhook-timestamp" => $testPayload->header['svix-timestamp'],
        );
        $testPayload->header = $unbrandedHeaders;

        $wh = new \Svix\Webhook($testPayload->secret);
        $json = $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testTitleCasedSignatureIsValidAndReturnsJson()
    {
        $testPayload = new TestPayload(time());
        $testPayload->header = array(
            "Svix-Id" => $testPayload->header['svix-id'],
            "Svix-Signature" => $testPayload->header['svix-signature'],
            "Svix-Timestamp" => $testPayload->header['svix-timestamp'],
        );

        $wh = new \Svix\Webhook($testPayload->secret);
        $json = $wh->verify($testPayload->payload, $testPayload->header);

        $this->assertEquals(
            $json['test'],
            2432232315,
            "did not return expected json"
        );
    }

    public function testMixedCaseSignatureIsValidAndReturnsJson()
    {
        $testPayload = new TestPayload(time());
        $testPayload->header = array(
            "SVIX-ID" => $testPayload->header['svix-id'],
            "sViX-SiGnAtUrE" => $testPayload->header['svix-signature'],
            "Svix-TimeStamp" => $testPayload->header['svix-timestamp'],
        );

        $wh = new \Svix\Webhook($testPayload->secret);
        $json = $wh->verify($testPayload->payload, $testPayload->header);

        $this->assertEquals(
            $json['test'],
            2432232315,
            "did not return expected json"
        );
    }

    public function testTitleCasedBrandlessSignatureIsValidAndReturnsJson()
    {
        $testPayload = new TestPayload(time());
        $testPayload->header = array(
            "Webhook-Id" => $testPayload->header['svix-id'],
            "Webhook-Signature" => $testPayload->header['svix-signature'],
            "Webhook-Timestamp" => $testPayload->header['svix-timestamp'],
        );

        $wh = new \Svix\Webhook($testPayload->secret);
        $json = $wh->verify($testPayload->payload, $testPayload->header);

        $this->assertEquals(
            $json['test'],
            2432232315,
            "did not return expected json"
        );
    }

    public function testMissingTitleCasedIdThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");

        $testPayload = new TestPayload(time());
        $testPayload->header = array(
            "Svix-Signature" => $testPayload->header['svix-signature'],
            "Svix-Timestamp" => $testPayload->header['svix-timestamp'],
        );

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
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

    public function testBadlyFormattedSignatureThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("No matching signature found");

        $testPayload = new TestPayload(time());
        $testPayload->header['svix-signature'] = 'BAD_SIG_NATURE';

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

        $testPayload = new TestPayload(time() - self::TOLERANCE - 1);

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testNewTimestampThrowsException()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Message timestamp too new");

        $testPayload = new TestPayload(time() + self::TOLERANCE + 1);

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testCustomToleranceAcceptsOldTimestamp()
    {
        $testPayload = new TestPayload(time() - (2 * self::TOLERANCE));

        $wh = new \Svix\Webhook($testPayload->secret, 3 * self::TOLERANCE);
        $json = $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testCustomToleranceRejectsOldTimestampInsideDefault()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Message timestamp too old");

        $testPayload = new TestPayload(time() - 120);

        $wh = new \Svix\Webhook($testPayload->secret, 60);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testCustomToleranceAcceptsNewTimestamp()
    {
        $testPayload = new TestPayload(time() + (2 * self::TOLERANCE));

        $wh = new \Svix\Webhook($testPayload->secret, 3 * self::TOLERANCE);
        $json = $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testCustomToleranceRejectsNewTimestampInsideDefault()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Message timestamp too new");

        $testPayload = new TestPayload(time() + 120);

        $wh = new \Svix\Webhook($testPayload->secret, 60);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testInvalidSignatureThrowsWithCustomTolerance()
    {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("No matching signature found");

        $testPayload = new TestPayload(time());
        $testPayload->header['svix-signature'] = 'v1,dawfeoifkpqwoekfpqoekf';

        $wh = new \Svix\Webhook($testPayload->secret, 3 * self::TOLERANCE);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testTamperedTimestampThrowsWithCustomTolerance()
    {
        // Changing the timestamp value after signing must fail the signature
        // check even when the altered value is inside the tolerance window.
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("No matching signature found");

        $testPayload = new TestPayload(time());
        $testPayload->header['svix-timestamp'] = strval(intval($testPayload->timestamp) - 600);

        $wh = new \Svix\Webhook($testPayload->secret, 3 * self::TOLERANCE);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testEpochSpanningToleranceRejectsNonPositiveTimestamp()
    {
        // sign() only accepts positive timestamps, so verify() reports a
        // non-positive value as a verification failure, not a signing one.
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Invalid Signature Headers");

        $testPayload = new TestPayload(-1234);

        $wh = new \Svix\Webhook($testPayload->secret, PHP_INT_MAX);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testNegativeToleranceThrows()
    {
        $this->expectException(\InvalidArgumentException::class);

        new \Svix\Webhook("whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw", -1);
    }

    public function testNonIntegerToleranceThrows()
    {
        $this->expectException(\InvalidArgumentException::class);

        new \Svix\Webhook("whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw", NAN);
    }

    public function testNullToleranceThrows()
    {
        $this->expectException(\InvalidArgumentException::class);

        new \Svix\Webhook("whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw", null);
    }

    public function testMultiSigPayloadIsValid()
    {
        // We're checking that `verify()` doesn't throw an exception.
        // It doesn't return anything we can assert about.
        $this->expectNotToPerformAssertions();

        $testPayload = new TestPayload(time());
        $sigs = [
            "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
            "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
            $testPayload->header['svix-signature'], // valid signature
            "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
        ];
        $testPayload->header['svix-signature'] = implode(" ", $sigs);

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testSignatureVerificationWithAndWithoutPrefix()
    {
        // We're checking that `verify()` doesn't throw an exception.
        // It doesn't return anything we can assert about.
        $this->expectNotToPerformAssertions();

        $testPayload = new TestPayload(time());

        $wh = new \Svix\Webhook($testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);


        $wh = new \Svix\Webhook("whsec_" . $testPayload->secret);
        $wh->verify($testPayload->payload, $testPayload->header);
    }

    public function testSignFunctionWorks()
    {
        $key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";
        $msgId = "msg_p5jXN8AQM9LWM0D4loKWxJek";
        $timestamp = 1614265330;
        $payload = '{"test": 2432232314}';
        $expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=";

        $wh = new \Svix\Webhook($key);

        $signature = $wh->sign($msgId, $timestamp, $payload);
        $this->assertEquals(
            $signature,
            $expected,
            "did not return expected signature"
        );
    }

    public function testInvalidFloatTimestamp()
    {
        $this->expectException(\Svix\Exception\WebhookSigningException::class);
        $key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";
        $msgId = "msg_p5jXN8AQM9LWM0D4loKWxJek";
        $timestamp = "161426533.0";
        $payload = '{"test": 2432232314}';
        $expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=";

        $wh = new \Svix\Webhook($key);

        $signature = $wh->sign($msgId, $timestamp, $payload);
    }

    public function testInvalidStringTimestamp()
    {
        $this->expectException(\Svix\Exception\WebhookSigningException::class);
        $key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";
        $msgId = "msg_p5jXN8AQM9LWM0D4loKWxJek";
        $timestamp = "invalid timestamp";
        $payload = '{"test": 2432232314}';
        $expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=";

        $wh = new \Svix\Webhook($key);

        $signature = $wh->sign($msgId, $timestamp, $payload);
    }

    public function testInvalidNegativeTimestamp()
    {
        $this->expectException(\Svix\Exception\WebhookSigningException::class);
        $key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";
        $msgId = "msg_p5jXN8AQM9LWM0D4loKWxJek";
        $timestamp = "-161426533";
        $payload = '{"test": 2432232314}';
        $expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=";

        $wh = new \Svix\Webhook($key);

        $signature = $wh->sign($msgId, $timestamp, $payload);
    }
}
