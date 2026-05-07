<?php

declare(strict_types=1);

namespace Svix\Tests;

use PHPUnit\Framework\TestCase;
use Svix\AutoConfig;
use Svix\Models\EndpointIn;

final class AutoConfigTest extends TestCase
{
    private static function encodeAutoConfigToken(array $payload): string
    {
        return 'auto_v1_' . base64_encode(json_encode($payload, JSON_THROW_ON_ERROR));
    }

    private static function minimalValidPayload(): array
    {
        return [
            'aid' => 'app_1srOrx2ZWZBpBUvZwXKQmoEYga2',
            'eid' => 'ep_1srOrx2ZWZBpBUvZwXKQmoEYga2',
            'surl' => 'https://api.example.test',
            // Same shape as WebhookTest secrets (base64 key material, no prefix required).
            'esec' => 'MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw',
            'tok' => 'appsk_test_token',
        ];
    }

    public function testValidTokenConstructsAutoConfig(): void
    {
        $token = self::encodeAutoConfigToken(self::minimalValidPayload());

        $ac = new AutoConfig($token, EndpointIn::create('https://consumer.example/webhook'));

        $this->assertInstanceOf(AutoConfig::class, $ac);
    }

    public function testWrongPrefixThrowsInvalidArgument(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('invalid token');

        new AutoConfig(
            'not_auto_v1_' . base64_encode(json_encode(self::minimalValidPayload())),
            EndpointIn::create('https://consumer.example/webhook'),
        );
    }

    public function testNothingAfterPrefixThrowsInvalidArgument(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('invalid token');

        new AutoConfig('auto_v1_', EndpointIn::create('https://consumer.example/webhook'));
    }

    public function testInvalidBase64ThrowsInvalidArgument(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('invalid token');

        new AutoConfig(
            'auto_v1_' . '!not-valid-base64!',
            EndpointIn::create('https://consumer.example/webhook'),
        );
    }

    public function testInvalidJsonThrowsInvalidArgument(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('invalid token');

        new AutoConfig(
            'auto_v1_' . base64_encode('{not json'),
            EndpointIn::create('https://consumer.example/webhook'),
        );
    }
}
