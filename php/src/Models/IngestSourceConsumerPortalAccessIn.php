<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class IngestSourceConsumerPortalAccessIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param int|null $expiry How long the token will be valid for, in seconds.
     *
     * Valid values are between 1 hour and 7 days. The default is 7 days.
     * @param bool|null $readOnly whether the app portal should be in read-only mode
     */
    private function __construct(
        public readonly ?int $expiry = null,
        public readonly ?bool $readOnly = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IngestSourceConsumerPortalAccessIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            expiry: null,
            readOnly: null,
            setFields: []
        );
    }

    public function withExpiry(?int $expiry): self
    {
        $setFields = $this->setFields;
        $setFields['expiry'] = true;

        return new self(
            expiry: $expiry,
            readOnly: $this->readOnly,
            setFields: $setFields
        );
    }

    public function withReadOnly(?bool $readOnly): self
    {
        $setFields = $this->setFields;
        $setFields['readOnly'] = true;

        return new self(
            expiry: $this->expiry,
            readOnly: $readOnly,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [];

        if (isset($this->setFields['expiry'])) {
            $data['expiry'] = $this->expiry;
        }
        if (isset($this->setFields['readOnly'])) {
            $data['readOnly'] = $this->readOnly;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            expiry: \Svix\Utils::deserializeInt($data, 'expiry', false, 'IngestSourceConsumerPortalAccessIn'),
            readOnly: \Svix\Utils::deserializeBool($data, 'readOnly', false, 'IngestSourceConsumerPortalAccessIn')
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
