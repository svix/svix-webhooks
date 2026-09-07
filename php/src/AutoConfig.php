<?php

declare(strict_types=1);

namespace Svix;

use GuzzleHttp\Client;
use Svix\ApiInternal\EndpointAutoconfig;
use Svix\ApiInternal\EndpointAutoConfigDeprecated;
use Svix\Exception\ApiException;
use Svix\Models\EndpointIn;
use Svix\Models\EndpointOut;
use Svix\Models\SubscribeIn;
use Svix\Request\SvixHttpClient;

final class AutoConfig
{
    private const AUTOCONFIG_TOKEN_PREFIX_V1 = 'auto_v1_';
    private const AUTOCONFIG_TOKEN_PREFIX_V2 = 'auto_v2_';
    private const UNSUPPORTED_TOKEN_VERSION = 'Unsupported token version. You might need to update the Svix SDK to use this token';

    private string $appId;

    private ?string $endpointId = null;

    private ?string $autoconfigId = null;

    private EndpointIn $endpoint;

    private Webhook $webhook;

    private SvixHttpClient $client;

    /**
     * @throws \InvalidArgumentException if the token is invalid
     */
    public function __construct(string $token, EndpointIn $endpoint)
    {
        $content = self::decodeToken($token);
        $this->appId = $content['app_id'];
        $this->endpointId = $content['endpoint_id'] ?? null;
        $this->autoconfigId = $content['autoconfig_id'] ?? null;
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

    /**
     * @throws ApiException
     */
    public function subscribe(): EndpointOut
    {
        if ($this->autoconfigId !== null) {
            return (new EndpointAutoconfig($this->client))->subscribe(
                $this->appId,
                $this->autoconfigId,
                $this->endpoint,
            );
        }

        return (new EndpointAutoConfigDeprecated($this->client))->update(
            $this->appId,
            $this->endpointId ?? '',
            SubscribeIn::create()->withEndpoint($this->endpoint),
        );
    }

    /**
     * @throws Exception\WebhookVerificationException
     * @throws Exception\WebhookSigningException
     */
    public function verify(string $payload, array $headers): mixed
    {
        return $this->webhook->verify($payload, $headers);
    }

    /**
     * @internal
     *
     * @return array{
     *     version: string,
     *     app_id: string,
     *     endpoint_id?: string,
     *     autoconfig_id?: string,
     *     server_url: string,
     *     endpoint_secret: string,
     *     token_plaintext: string
     * }
     *
     * @throws \InvalidArgumentException if the token is invalid
     */
    public static function decodeToken(string $token): array
    {
        if (str_starts_with($token, self::AUTOCONFIG_TOKEN_PREFIX_V1)) {
            $data = self::parseTokenPayload($token, self::AUTOCONFIG_TOKEN_PREFIX_V1);

            return self::tokenContentFromPayload($data, 'v1', 'eid', 'endpoint_id');
        }
        if (str_starts_with($token, self::AUTOCONFIG_TOKEN_PREFIX_V2)) {
            $data = self::parseTokenPayload($token, self::AUTOCONFIG_TOKEN_PREFIX_V2);

            return self::tokenContentFromPayload($data, 'v2', 'sid', 'autoconfig_id');
        }

        throw new \InvalidArgumentException(self::UNSUPPORTED_TOKEN_VERSION);
    }

    /**
     * @return array<string, mixed>
     */
    private static function parseTokenPayload(string $token, string $prefix): array
    {
        $encoded = substr($token, strlen($prefix));
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

        return $data;
    }

    /**
     * @param array<string, mixed> $data
     *
     * @return array{
     *     version: string,
     *     app_id: string,
     *     endpoint_id?: string,
     *     autoconfig_id?: string,
     *     server_url: string,
     *     endpoint_secret: string,
     *     token_plaintext: string
     * }
     */
    private static function tokenContentFromPayload(
        array $data,
        string $version,
        string $idKey,
        string $idField,
    ): array {
        try {
            return [
                'version' => $version,
                'app_id' => (string) $data['aid'],
                $idField => (string) $data[$idKey],
                'server_url' => (string) $data['surl'],
                'endpoint_secret' => (string) $data['esec'],
                'token_plaintext' => (string) $data['tok'],
            ];
        } catch (\Throwable $e) {
            throw new \InvalidArgumentException('invalid token');
        }
    }
}
