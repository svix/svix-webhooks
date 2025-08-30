<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null          $channels    list of message channels this endpoint listens to (omit for all)
     * @param list<string>|null          $filterTypes
     * @param array<string, string>|null $headers
     * @param array<string, string>|null $metadata
     * @param string|null                $secret      The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
     * It is recommended to not set this and let the server generate the secret.
     * @param string|null $uid optional unique identifier for the endpoint
     */
    private function __construct(
        public readonly string $url,
        public readonly ?array $channels = null,
        public readonly ?string $description = null,
        public readonly ?bool $disabled = null,
        public readonly ?array $filterTypes = null,
        public readonly ?array $headers = null,
        public readonly ?array $metadata = null,
        public readonly ?int $rateLimit = null,
        public readonly ?string $secret = null,
        public readonly ?string $uid = null,
        public readonly ?int $version = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointIn with required fields.
     */
    public static function create(
        string $url,
    ): self {
        return new self(
            channels: null,
            description: null,
            disabled: null,
            filterTypes: null,
            headers: null,
            metadata: null,
            rateLimit: null,
            secret: null,
            uid: null,
            url: $url,
            version: null,
            setFields: ['url' => true]
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            channels: $channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            headers: $this->headers,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            secret: $this->secret,
            uid: $this->uid,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            channels: $this->channels,
            description: $description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            headers: $this->headers,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            secret: $this->secret,
            uid: $this->uid,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withDisabled(?bool $disabled): self
    {
        $setFields = $this->setFields;
        $setFields['disabled'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $disabled,
            filterTypes: $this->filterTypes,
            headers: $this->headers,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            secret: $this->secret,
            uid: $this->uid,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withFilterTypes(?array $filterTypes): self
    {
        $setFields = $this->setFields;
        $setFields['filterTypes'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $filterTypes,
            headers: $this->headers,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            secret: $this->secret,
            uid: $this->uid,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withHeaders(?array $headers): self
    {
        $setFields = $this->setFields;
        $setFields['headers'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            headers: $headers,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            secret: $this->secret,
            uid: $this->uid,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            headers: $this->headers,
            metadata: $metadata,
            rateLimit: $this->rateLimit,
            secret: $this->secret,
            uid: $this->uid,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withRateLimit(?int $rateLimit): self
    {
        $setFields = $this->setFields;
        $setFields['rateLimit'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            headers: $this->headers,
            metadata: $this->metadata,
            rateLimit: $rateLimit,
            secret: $this->secret,
            uid: $this->uid,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withSecret(?string $secret): self
    {
        $setFields = $this->setFields;
        $setFields['secret'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            headers: $this->headers,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            secret: $secret,
            uid: $this->uid,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            headers: $this->headers,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            secret: $this->secret,
            uid: $uid,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withVersion(?int $version): self
    {
        $setFields = $this->setFields;
        $setFields['version'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            headers: $this->headers,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            secret: $this->secret,
            uid: $this->uid,
            url: $this->url,
            version: $version,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['url' => $this->url];

        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }
        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }
        if (isset($this->setFields['filterTypes'])) {
            $data['filterTypes'] = $this->filterTypes;
        }
        if (isset($this->setFields['headers'])) {
            $data['headers'] = $this->headers;
        }
        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
        }
        if (isset($this->setFields['rateLimit'])) {
            $data['rateLimit'] = $this->rateLimit;
        }
        if (isset($this->setFields['secret'])) {
            $data['secret'] = $this->secret;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (isset($this->setFields['version'])) {
            $data['version'] = $this->version;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'EndpointIn'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'EndpointIn'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'EndpointIn'),
            filterTypes: \Svix\Utils::getValFromJson($data, 'filterTypes', false, 'EndpointIn'),
            headers: \Svix\Utils::getValFromJson($data, 'headers', false, 'EndpointIn'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'EndpointIn'),
            rateLimit: \Svix\Utils::deserializeInt($data, 'rateLimit', false, 'EndpointIn'),
            secret: \Svix\Utils::deserializeString($data, 'secret', false, 'EndpointIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'EndpointIn'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'EndpointIn'),
            version: \Svix\Utils::deserializeInt($data, 'version', false, 'EndpointIn')
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
