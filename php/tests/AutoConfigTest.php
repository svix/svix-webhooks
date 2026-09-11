<?php

declare(strict_types=1);

namespace Svix\Tests;

use GuzzleHttp\Client;
use GuzzleHttp\Handler\MockHandler;
use GuzzleHttp\HandlerStack;
use GuzzleHttp\Middleware;
use GuzzleHttp\Psr7\Response;
use PHPUnit\Framework\TestCase;
use Svix\AutoConfig;
use Svix\AutoConfigConsumer;
use Svix\Models\AutoConfigSinkType;
use Svix\Models\AutoConfigSinkTypeConfig;
use Svix\Models\EndpointIn;
use Svix\Models\SinkInCommon;
use Svix\Request\SvixHttpClient;
use Svix\SvixOptions;

final class AutoConfigTest extends TestCase
{
    private static function encodeAutoConfigToken(array $content, string $prefix): string
    {
        return $prefix . base64_encode(json_encode($content, JSON_THROW_ON_ERROR));
    }

    private static function sampleV1TokenContent(): array
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

    private static function sampleV2TokenContent(): array
    {
        return [
            'aid' => 'app_1srOrx2ZWZBpBUvZwXKQmoEYga2',
            'sid' => 'acfg_2',
            'surl' => 'https://api.example.test',
            'esec' => 'MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw',
            'tok' => 'appsk_test_token',
        ];
    }

    public function testValidTokenConstructsAutoConfig(): void
    {
        $token = self::encodeAutoConfigToken(self::sampleV1TokenContent(), 'auto_v1_');

        $ac = new AutoConfig($token, EndpointIn::create('https://consumer.example/webhook'));

        $this->assertInstanceOf(AutoConfig::class, $ac);
    }

    public function testValidV2TokenConstructsAutoConfig(): void
    {
        $payload = self::sampleV2TokenContent();
        $token = self::encodeAutoConfigToken($payload, 'auto_v2_');

        $ac = new AutoConfig($token, EndpointIn::create('https://consumer.example/webhook'));

        $this->assertInstanceOf(AutoConfig::class, $ac);
        $decoded = AutoConfig::decodeToken($token);
        $this->assertSame('v2', $decoded['version']);
        $this->assertSame('acfg_2', $decoded['autoconfig_id']);
    }

    public function testWrongPrefixThrowsInvalidArgument(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage(
            'Unsupported token version. You might need to update the Svix SDK to use this token',
        );

        new AutoConfig(
            'auto_v341_' . base64_encode(json_encode(self::sampleV1TokenContent())),
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

    public function testAutoConfigSinkTypeRoundtripsAsTypeAndConfig(): void
    {
        $sink = AutoConfigSinkType::create(
            AutoConfigSinkTypeConfig::poller(SinkInCommon::create()->withEventTypes(['a'])),
        );

        $encoded = json_encode($sink, JSON_THROW_ON_ERROR);
        $this->assertJsonStringEqualsJsonString(
            '{"type":"poller","config":{"eventTypes":["a"]}}',
            $encoded,
        );

        $decoded = AutoConfigSinkType::fromJson($encoded);
        $this->assertInstanceOf(\Svix\Models\AutoConfigSinkTypeConfig\Poller::class, $decoded->config);
        $this->assertSame(['a'], $decoded->config->poller->eventTypes);
    }

    public function testV2ConsumerSubscribePutsDestination(): void
    {
        $history = [];
        $http = self::mockHttp([
            new Response(200, [], '{"type":"pollingEndpoint","config":{},"id":"dst_123","status":"enabled","currentIterator":"0","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","batchSize":100,"maxWaitSecs":10,"metadata":{}}'),
        ], $history);

        $token = self::encodeAutoConfigToken(self::sampleV2TokenContent(), 'auto_v2_');
        $consumer = new AutoConfigConsumer(
            $token,
            SinkInCommon::create()->withChannels(['ch1']),
        );
        self::injectHttpClient($consumer, $http);
        $out = $consumer->subscribe();

        $this->assertSame('dst_123', $out->id);
        $this->assertCount(1, $history);
        $req = $history[0]['request'];
        $this->assertSame('PUT', $req->getMethod());
        $this->assertSame('/api/v1/app/app_1srOrx2ZWZBpBUvZwXKQmoEYga2/autoconfig/acfg_2/destination', $req->getUri()->getPath());
        $this->assertJsonStringEqualsJsonString(
            '{"type":"pollingEndpoint","config":{},"uid":null,"channels":["ch1"]}',
            $req->getBody()->getContents(),
        );
    }

    public function testV2ConsumerReceivePolls(): void
    {
        $history = [];
        $http = self::mockHttp([
            new Response(200, [], '{"id":"auto_1srOrx2ZWZBpBUvZwXKQmoEYga2","tokenCensored":"***","createdAt":"2019-08-24T14:15:22Z","status":"active","destId":"dst_123"}'),
            new Response(200, [], '{"data":[],"done":true}'),
        ], $history);

        $token = self::encodeAutoConfigToken(self::sampleV2TokenContent(), 'auto_v2_');
        $consumer = new AutoConfigConsumer($token, SinkInCommon::create());
        self::injectHttpClient($consumer, $http);
        $out = $consumer->receive('c1');

        $this->assertTrue($out->done);
        $this->assertCount(2, $history);
        $this->assertSame('GET', $history[1]['request']->getMethod());
        $this->assertSame('/api/v1/app/app_1srOrx2ZWZBpBUvZwXKQmoEYga2/polling-endpoint/dst_123/consumer/c1', $history[1]['request']->getUri()->getPath());
    }

    private static function mockHttp(array $responses, array &$history): Client
    {
        $stack = HandlerStack::create(new MockHandler($responses));
        $stack->push(Middleware::history($history));

        return new Client(['handler' => $stack]);
    }

    // Injects the guzzle client in AutoConfigConsumer for testing
    private static function injectHttpClient(AutoConfigConsumer $consumer, Client $http): void
    {
        $opts = SvixOptions::newDefault('sk_test_xyz');
        $opts->serverUrl = 'https://api.example.test';

        $client = new SvixHttpClient(
            baseUrl: 'https://api.example.test',
            token: 'sk_test_xyz',
            guzzleClient: $http,
            opts: $opts,
        );

        $prop = new \ReflectionProperty(AutoConfigConsumer::class, 'client');
        $prop->setValue($consumer, $client);
    }
}
