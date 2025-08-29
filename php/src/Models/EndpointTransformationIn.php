<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointTransformationIn implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $code = null,
        public readonly ?bool $enabled = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointTransformationIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            code: null,
            enabled: null,
            setFields: []
        );
    }

    public function withCode(?string $code): self
    {
        $setFields = $this->setFields;
        $setFields['code'] = true;

        return new self(
            code: $code,
            enabled: $this->enabled,
            setFields: $setFields
        );
    }

    public function withEnabled(?bool $enabled): self
    {
        $setFields = $this->setFields;
        $setFields['enabled'] = true;

        return new self(
            code: $this->code,
            enabled: $enabled,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [];

        if (isset($this->setFields['code'])) {
            $data['code'] = $this->code;
        }
        if (null !== $this->enabled) {
            $data['enabled'] = $this->enabled;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            code: \Svix\Utils::deserializeString($data, 'code', false, 'EndpointTransformationIn'),
            enabled: \Svix\Utils::deserializeBool($data, 'enabled', false, 'EndpointTransformationIn')
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
