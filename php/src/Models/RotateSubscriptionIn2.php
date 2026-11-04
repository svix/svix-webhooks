<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class RotateSubscriptionIn2 implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?EndpointSecretRotateIn $signingSecret = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RotateSubscriptionIn2 with required fields.
     */
    public static function create(
    ): self {
        return new self(
            signingSecret: null,
            setFields: []
        );
    }

    public function withSigningSecret(?EndpointSecretRotateIn $signingSecret): self
    {
        $setFields = $this->setFields;
        $setFields['signingSecret'] = true;

        return new self(
            signingSecret: $signingSecret,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (isset($this->setFields['signingSecret'])) {
            $data['signingSecret'] = $this->signingSecret;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            signingSecret: \Svix\Utils::deserializeObject($data, 'signingSecret', false, 'RotateSubscriptionIn2', [EndpointSecretRotateIn::class, 'fromMixed'])
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
