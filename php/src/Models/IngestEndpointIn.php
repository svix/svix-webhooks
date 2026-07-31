<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class IngestEndpointIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param int|null $throttleRate Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null $uid    optional unique identifier for the endpoint
     * @param string|null $secret The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
     * It is recommended to not set this and let the server generate the secret.
     * @param array<string, string>|null $metadata
     */
    private function __construct(
        public readonly string $url,
        public readonly ?string $description = null,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        public readonly ?bool $disabled = null,
        public readonly ?string $secret = null,
        public readonly ?array $metadata = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IngestEndpointIn with required fields.
     */
    public static function create(
        string $url,
    ): self {
        return new self(
            url: $url,
            description: null,
            throttleRate: null,
            uid: null,
            disabled: null,
            secret: null,
            metadata: null,
            setFields: ['url' => true]
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            url: $this->url,
            description: $description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            disabled: $this->disabled,
            secret: $this->secret,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            url: $this->url,
            description: $this->description,
            throttleRate: $throttleRate,
            uid: $this->uid,
            disabled: $this->disabled,
            secret: $this->secret,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $uid,
            disabled: $this->disabled,
            secret: $this->secret,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withDisabled(?bool $disabled): self
    {
        $setFields = $this->setFields;
        $setFields['disabled'] = true;

        return new self(
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            disabled: $disabled,
            secret: $this->secret,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withSecret(?string $secret): self
    {
        $setFields = $this->setFields;
        $setFields['secret'] = true;

        return new self(
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            disabled: $this->disabled,
            secret: $secret,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            disabled: $this->disabled,
            secret: $this->secret,
            metadata: $metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'url' => $this->url];

        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }
        if (isset($this->setFields['secret'])) {
            $data['secret'] = $this->secret;
        }
        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'IngestEndpointIn'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'IngestEndpointIn'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'IngestEndpointIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'IngestEndpointIn'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'IngestEndpointIn'),
            secret: \Svix\Utils::deserializeString($data, 'secret', false, 'IngestEndpointIn'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'IngestEndpointIn')
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
