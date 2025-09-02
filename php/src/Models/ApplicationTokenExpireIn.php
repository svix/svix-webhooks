<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ApplicationTokenExpireIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param int|null          $expiry     how many seconds until the old key is expired
     * @param list<string>|null $sessionIds An optional list of session ids.
     *
     * If any session ids are specified, only Application tokens created with that session id will be expired.
     */
    private function __construct(
        public readonly ?int $expiry = null,
        public readonly ?array $sessionIds = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ApplicationTokenExpireIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            expiry: null,
            sessionIds: null,
            setFields: []
        );
    }

    public function withExpiry(?int $expiry): self
    {
        $setFields = $this->setFields;
        $setFields['expiry'] = true;

        return new self(
            expiry: $expiry,
            sessionIds: $this->sessionIds,
            setFields: $setFields
        );
    }

    public function withSessionIds(?array $sessionIds): self
    {
        $setFields = $this->setFields;
        $setFields['sessionIds'] = true;

        return new self(
            expiry: $this->expiry,
            sessionIds: $sessionIds,
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
        if (null !== $this->sessionIds) {
            $data['sessionIds'] = $this->sessionIds;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            expiry: \Svix\Utils::deserializeInt($data, 'expiry', false, 'ApplicationTokenExpireIn'),
            sessionIds: \Svix\Utils::getValFromJson($data, 'sessionIds', false, 'ApplicationTokenExpireIn')
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
