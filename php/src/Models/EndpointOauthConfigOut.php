<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointOauthConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null          $scopes
     * @param array<string, string>|null $extraParams
     * @param array<string, string>|null $extraHeaders
     */
    private function __construct(
        public readonly Oauth2GrantTypeInOut $grantType,
        public readonly Oauth2AuthMethodInOut $authMethod,
        public readonly string $clientId,
        public readonly string $tokenUrl,
        public readonly ?array $scopes = null,
        public readonly ?array $extraParams = null,
        public readonly ?array $extraHeaders = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointOauthConfigOut with required fields.
     */
    public static function create(
        Oauth2GrantTypeInOut $grantType,
        Oauth2AuthMethodInOut $authMethod,
        string $clientId,
        string $tokenUrl,
    ): self {
        return new self(
            grantType: $grantType,
            authMethod: $authMethod,
            clientId: $clientId,
            tokenUrl: $tokenUrl,
            scopes: null,
            extraParams: null,
            extraHeaders: null,
            setFields: ['grantType' => true, 'authMethod' => true, 'clientId' => true, 'tokenUrl' => true]
        );
    }

    public function withScopes(?array $scopes): self
    {
        $setFields = $this->setFields;
        $setFields['scopes'] = true;

        return new self(
            grantType: $this->grantType,
            authMethod: $this->authMethod,
            clientId: $this->clientId,
            tokenUrl: $this->tokenUrl,
            scopes: $scopes,
            extraParams: $this->extraParams,
            extraHeaders: $this->extraHeaders,
            setFields: $setFields
        );
    }

    public function withExtraParams(?array $extraParams): self
    {
        $setFields = $this->setFields;
        $setFields['extraParams'] = true;

        return new self(
            grantType: $this->grantType,
            authMethod: $this->authMethod,
            clientId: $this->clientId,
            tokenUrl: $this->tokenUrl,
            scopes: $this->scopes,
            extraParams: $extraParams,
            extraHeaders: $this->extraHeaders,
            setFields: $setFields
        );
    }

    public function withExtraHeaders(?array $extraHeaders): self
    {
        $setFields = $this->setFields;
        $setFields['extraHeaders'] = true;

        return new self(
            grantType: $this->grantType,
            authMethod: $this->authMethod,
            clientId: $this->clientId,
            tokenUrl: $this->tokenUrl,
            scopes: $this->scopes,
            extraParams: $this->extraParams,
            extraHeaders: $extraHeaders,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'grantType' => $this->grantType,
            'authMethod' => $this->authMethod,
            'clientId' => $this->clientId,
            'tokenUrl' => $this->tokenUrl];

        if (isset($this->setFields['scopes'])) {
            $data['scopes'] = $this->scopes;
        }
        if (isset($this->setFields['extraParams'])) {
            $data['extraParams'] = $this->extraParams;
        }
        if (isset($this->setFields['extraHeaders'])) {
            $data['extraHeaders'] = $this->extraHeaders;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            grantType: \Svix\Utils::deserializeObject($data, 'grantType', true, 'EndpointOauthConfigOut', [Oauth2GrantTypeInOut::class, 'fromMixed']),
            authMethod: \Svix\Utils::deserializeObject($data, 'authMethod', true, 'EndpointOauthConfigOut', [Oauth2AuthMethodInOut::class, 'fromMixed']),
            clientId: \Svix\Utils::deserializeString($data, 'clientId', true, 'EndpointOauthConfigOut'),
            tokenUrl: \Svix\Utils::deserializeString($data, 'tokenUrl', true, 'EndpointOauthConfigOut'),
            scopes: \Svix\Utils::getValFromJson($data, 'scopes', false, 'EndpointOauthConfigOut'),
            extraParams: \Svix\Utils::getValFromJson($data, 'extraParams', false, 'EndpointOauthConfigOut'),
            extraHeaders: \Svix\Utils::getValFromJson($data, 'extraHeaders', false, 'EndpointOauthConfigOut')
        );
    }

    /**
     * Create an instance from a json string.
     */
    public static function fromJson(string $json): self
    {
        $data = json_decode(json: $json, associative: true, depth: 512, flags: JSON_THROW_ON_ERROR);

        return self::fromMixed($data);
    }
}
