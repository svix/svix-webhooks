<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class RotatePollerTokenIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param int|null $expiry         How long the token will be valid for, in seconds. Can be up to 31,536,000 seconds (1 year).
     * @param int|null $oldTokenExpiry Updates the previous token's expiration, in seconds.
     *
     * If set to 0, the old token will immediately be revoked. Must be between 0 and 86,400 seconds (1 day).
     *
     * Defaults to 300 seconds (5 minutes).
     */
    private function __construct(
        public readonly ?int $expiry = null,
        public readonly ?int $oldTokenExpiry = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RotatePollerTokenIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            expiry: null,
            oldTokenExpiry: null,
            setFields: []
        );
    }

    public function withExpiry(?int $expiry): self
    {
        $setFields = $this->setFields;
        $setFields['expiry'] = true;

        return new self(
            expiry: $expiry,
            oldTokenExpiry: $this->oldTokenExpiry,
            setFields: $setFields
        );
    }

    public function withOldTokenExpiry(?int $oldTokenExpiry): self
    {
        $setFields = $this->setFields;
        $setFields['oldTokenExpiry'] = true;

        return new self(
            expiry: $this->expiry,
            oldTokenExpiry: $oldTokenExpiry,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (isset($this->setFields['expiry'])) {
            $data['expiry'] = $this->expiry;
        }
        if (null !== $this->oldTokenExpiry) {
            $data['oldTokenExpiry'] = $this->oldTokenExpiry;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            expiry: \Svix\Utils::deserializeInt($data, 'expiry', false, 'RotatePollerTokenIn'),
            oldTokenExpiry: \Svix\Utils::deserializeInt($data, 'oldTokenExpiry', false, 'RotatePollerTokenIn')
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
