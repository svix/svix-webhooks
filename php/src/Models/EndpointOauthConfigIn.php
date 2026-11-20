<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointOauthConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param Oauth2GrantTypeInOut         $grantType    the OAuth grant type
     * @param ClientSecretJwtParamsIn|null $jwtParams    Optional JWT parameters. Only required for `clientSecretJwt`
     * @param string                       $clientId     The client ID. Required for all authentication types.
     * @param string|null                  $clientSecret Optional client secret. This is only used for `clientSecretBasic` and `clientSecretPost`.
     *
     * For `clientSecretBasic`, the secret will be appended to the `Authorization` header.
     * For `clientSecretPost`, this will be added to the body in a `client_secret` parameter.
     * @param string                     $tokenUrl     the URL of the authorization server
     * @param list<string>|null          $scopes       optional OAuth scopes added to the request body
     * @param array<string, string>|null $extraParams  extra parameters added to the request body as key-value pairs
     * @param string|null                $refreshToken for `refreshToken` grant type
     * @param array<string, string>|null $extraHeaders optional headers added to the token request
     */
    private function __construct(
        public readonly Oauth2GrantTypeInOut $grantType,
        public readonly Oauth2AuthMethodInOut $authMethod,
        public readonly string $clientId,
        public readonly string $tokenUrl,
        public readonly ?ClientSecretJwtParamsIn $jwtParams = null,
        public readonly ?string $clientSecret = null,
        public readonly ?array $scopes = null,
        public readonly ?array $extraParams = null,
        public readonly ?string $refreshToken = null,
        public readonly ?array $extraHeaders = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointOauthConfigIn with required fields.
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
            jwtParams: null,
            clientId: $clientId,
            clientSecret: null,
            tokenUrl: $tokenUrl,
            scopes: null,
            extraParams: null,
            refreshToken: null,
            extraHeaders: null,
            setFields: ['grantType' => true, 'authMethod' => true, 'clientId' => true, 'tokenUrl' => true]
        );
    }

    public function withJwtParams(?ClientSecretJwtParamsIn $jwtParams): self
    {
        $setFields = $this->setFields;
        $setFields['jwtParams'] = true;

        return new self(
            grantType: $this->grantType,
            authMethod: $this->authMethod,
            jwtParams: $jwtParams,
            clientId: $this->clientId,
            clientSecret: $this->clientSecret,
            tokenUrl: $this->tokenUrl,
            scopes: $this->scopes,
            extraParams: $this->extraParams,
            refreshToken: $this->refreshToken,
            extraHeaders: $this->extraHeaders,
            setFields: $setFields
        );
    }

    public function withClientSecret(?string $clientSecret): self
    {
        $setFields = $this->setFields;
        $setFields['clientSecret'] = true;

        return new self(
            grantType: $this->grantType,
            authMethod: $this->authMethod,
            jwtParams: $this->jwtParams,
            clientId: $this->clientId,
            clientSecret: $clientSecret,
            tokenUrl: $this->tokenUrl,
            scopes: $this->scopes,
            extraParams: $this->extraParams,
            refreshToken: $this->refreshToken,
            extraHeaders: $this->extraHeaders,
            setFields: $setFields
        );
    }

    public function withScopes(?array $scopes): self
    {
        $setFields = $this->setFields;
        $setFields['scopes'] = true;

        return new self(
            grantType: $this->grantType,
            authMethod: $this->authMethod,
            jwtParams: $this->jwtParams,
            clientId: $this->clientId,
            clientSecret: $this->clientSecret,
            tokenUrl: $this->tokenUrl,
            scopes: $scopes,
            extraParams: $this->extraParams,
            refreshToken: $this->refreshToken,
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
            jwtParams: $this->jwtParams,
            clientId: $this->clientId,
            clientSecret: $this->clientSecret,
            tokenUrl: $this->tokenUrl,
            scopes: $this->scopes,
            extraParams: $extraParams,
            refreshToken: $this->refreshToken,
            extraHeaders: $this->extraHeaders,
            setFields: $setFields
        );
    }

    public function withRefreshToken(?string $refreshToken): self
    {
        $setFields = $this->setFields;
        $setFields['refreshToken'] = true;

        return new self(
            grantType: $this->grantType,
            authMethod: $this->authMethod,
            jwtParams: $this->jwtParams,
            clientId: $this->clientId,
            clientSecret: $this->clientSecret,
            tokenUrl: $this->tokenUrl,
            scopes: $this->scopes,
            extraParams: $this->extraParams,
            refreshToken: $refreshToken,
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
            jwtParams: $this->jwtParams,
            clientId: $this->clientId,
            clientSecret: $this->clientSecret,
            tokenUrl: $this->tokenUrl,
            scopes: $this->scopes,
            extraParams: $this->extraParams,
            refreshToken: $this->refreshToken,
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

        if (isset($this->setFields['jwtParams'])) {
            $data['jwtParams'] = $this->jwtParams;
        }
        if (isset($this->setFields['clientSecret'])) {
            $data['clientSecret'] = $this->clientSecret;
        }
        if (isset($this->setFields['scopes'])) {
            $data['scopes'] = $this->scopes;
        }
        if (isset($this->setFields['extraParams'])) {
            $data['extraParams'] = $this->extraParams;
        }
        if (isset($this->setFields['refreshToken'])) {
            $data['refreshToken'] = $this->refreshToken;
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
            grantType: \Svix\Utils::deserializeObject($data, 'grantType', true, 'EndpointOauthConfigIn', [Oauth2GrantTypeInOut::class, 'fromMixed']),
            authMethod: \Svix\Utils::deserializeObject($data, 'authMethod', true, 'EndpointOauthConfigIn', [Oauth2AuthMethodInOut::class, 'fromMixed']),
            jwtParams: \Svix\Utils::deserializeObject($data, 'jwtParams', false, 'EndpointOauthConfigIn', [ClientSecretJwtParamsIn::class, 'fromMixed']),
            clientId: \Svix\Utils::deserializeString($data, 'clientId', true, 'EndpointOauthConfigIn'),
            clientSecret: \Svix\Utils::deserializeString($data, 'clientSecret', false, 'EndpointOauthConfigIn'),
            tokenUrl: \Svix\Utils::getValFromJson($data, 'tokenUrl', true, 'EndpointOauthConfigIn'),
            scopes: \Svix\Utils::getValFromJson($data, 'scopes', false, 'EndpointOauthConfigIn'),
            extraParams: \Svix\Utils::getValFromJson($data, 'extraParams', false, 'EndpointOauthConfigIn'),
            refreshToken: \Svix\Utils::deserializeString($data, 'refreshToken', false, 'EndpointOauthConfigIn'),
            extraHeaders: \Svix\Utils::getValFromJson($data, 'extraHeaders', false, 'EndpointOauthConfigIn')
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
