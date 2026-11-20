<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ClientSecretJwtParamsIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                   $secretBase64     the base64-encoded secret used for signing the JWT
     * @param OauthJwsSigningAlgorithm $signingAlgorithm the algorithm used to sign the JWT
     * @param string|null              $secretId         Optional secret identifier.
     *
     * If supplied, this will be populated in the JWT header in the `kid` field.
     * @param int|null $tokenExpirySecs Optional number of seconds after which the JWT should expire.
     *
     * Defaults to 300 seconds.
     */
    private function __construct(
        public readonly string $secretBase64,
        public readonly OauthJwsSigningAlgorithm $signingAlgorithm,
        public readonly ?string $secretId = null,
        public readonly ?int $tokenExpirySecs = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ClientSecretJwtParamsIn with required fields.
     */
    public static function create(
        string $secretBase64,
        OauthJwsSigningAlgorithm $signingAlgorithm,
    ): self {
        return new self(
            secretBase64: $secretBase64,
            signingAlgorithm: $signingAlgorithm,
            secretId: null,
            tokenExpirySecs: null,
            setFields: ['secretBase64' => true, 'signingAlgorithm' => true]
        );
    }

    public function withSecretId(?string $secretId): self
    {
        $setFields = $this->setFields;
        $setFields['secretId'] = true;

        return new self(
            secretBase64: $this->secretBase64,
            signingAlgorithm: $this->signingAlgorithm,
            secretId: $secretId,
            tokenExpirySecs: $this->tokenExpirySecs,
            setFields: $setFields
        );
    }

    public function withTokenExpirySecs(?int $tokenExpirySecs): self
    {
        $setFields = $this->setFields;
        $setFields['tokenExpirySecs'] = true;

        return new self(
            secretBase64: $this->secretBase64,
            signingAlgorithm: $this->signingAlgorithm,
            secretId: $this->secretId,
            tokenExpirySecs: $tokenExpirySecs,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'secretBase64' => $this->secretBase64,
            'signingAlgorithm' => $this->signingAlgorithm];

        if (isset($this->setFields['secretId'])) {
            $data['secretId'] = $this->secretId;
        }
        if (isset($this->setFields['tokenExpirySecs'])) {
            $data['tokenExpirySecs'] = $this->tokenExpirySecs;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            secretBase64: \Svix\Utils::deserializeString($data, 'secretBase64', true, 'ClientSecretJwtParamsIn'),
            signingAlgorithm: \Svix\Utils::deserializeObject($data, 'signingAlgorithm', true, 'ClientSecretJwtParamsIn', [OauthJwsSigningAlgorithm::class, 'fromMixed']),
            secretId: \Svix\Utils::deserializeString($data, 'secretId', false, 'ClientSecretJwtParamsIn'),
            tokenExpirySecs: \Svix\Utils::deserializeInt($data, 'tokenExpirySecs', false, 'ClientSecretJwtParamsIn')
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
