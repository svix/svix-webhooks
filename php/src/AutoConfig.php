<?php

declare(strict_types=1);

namespace Svix;

use GuzzleHttp\Client;
use Svix\ApiInternal\EndpointAutoConfig;
use Svix\Models\EndpointIn;
use Svix\Models\EndpointOut;
use Svix\Models\SubscribeIn;
use Svix\Request\SvixHttpClient;

final class AutoConfig
{
    private const AUTOCONFIG_TOKEN_PREFIX_V1 = 'auto_v1_';

    private string $appId;

    private string $endpointId;

    private EndpointIn $endpoint;

    private Webhook $webhook;

    private SvixHttpClient $client;

    public function __construct(string $token, EndpointIn $endpoint)
    {
        $content = self::decodeToken($token);
        $this->appId = $content['app_id'];
        $this->endpointId = $content['endpoint_id'];
        $this->endpoint = $endpoint;

        try {
            $this->webhook = new Webhook($content['endpoint_secret']);
        } catch (\Throwable $e) {
            throw new \InvalidArgumentException('invalid token');
        }

        $opts = SvixOptions::newDefault($content['token_plaintext']);
        $opts->serverUrl = $content['server_url'];

        $this->client = new SvixHttpClient(
            baseUrl: $content['server_url'],
            token: $content['token_plaintext'],
            guzzleClient: new Client(),
            opts: $opts,
        );
    }

    public function subscribe(): EndpointOut
    {
        return (new EndpointAutoConfig($this->client))->update(
            $this->appId,
            $this->endpointId,
            SubscribeIn::create($this->endpoint),
        );
    }

    public function verify(string $payload, array $headers): mixed
    {
        return $this->webhook->verify($payload, $headers);
    }

    /**
     * @return array{app_id: string, endpoint_id: string, server_url: string, endpoint_secret: string, token_plaintext: string}
     */
    private static function decodeToken(string $token): array
    {
        if (!str_starts_with($token, self::AUTOCONFIG_TOKEN_PREFIX_V1)) {
            throw new \InvalidArgumentException('invalid token');
        }

        $encoded = substr($token, strlen(self::AUTOCONFIG_TOKEN_PREFIX_V1));
        if ($encoded === false || $encoded === '') {
            throw new \InvalidArgumentException('invalid token');
        }

        $decoded = base64_decode($encoded, true);
        if ($decoded === false) {
            throw new \InvalidArgumentException('invalid token');
        }

        try {
            $data = json_decode($decoded, true, 512, JSON_THROW_ON_ERROR);
        } catch (\JsonException $e) {
            throw new \InvalidArgumentException('invalid token');
        }

        if (!is_array($data)) {
            throw new \InvalidArgumentException('invalid token');
        }

        try {
            return [
                'app_id' => (string) $data['aid'],
                'endpoint_id' => (string) $data['eid'],
                'server_url' => (string) $data['surl'],
                'endpoint_secret' => (string) $data['esec'],
                'token_plaintext' => (string) $data['tok'],
            ];
        } catch (\Throwable $e) {
            throw new \InvalidArgumentException('invalid token');
        }
    }
}
