<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamPortalAccessIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags the set of feature flags the created token will have access to
     * @param int|null          $expiry       How long the token will be valid for, in seconds.
     *
     * Valid values are between 1 hour and 7 days. The default is 7 days.
     * @param string|null $sessionId An optional session ID to attach to the token.
     *
     * When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID.
     */
    private function __construct(
        public readonly ?array $featureFlags = null,
        public readonly ?int $expiry = null,
        public readonly ?string $sessionId = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamPortalAccessIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            featureFlags: null,
            expiry: null,
            sessionId: null,
            setFields: []
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            featureFlags: $featureFlags,
            expiry: $this->expiry,
            sessionId: $this->sessionId,
            setFields: $setFields
        );
    }

    public function withExpiry(?int $expiry): self
    {
        $setFields = $this->setFields;
        $setFields['expiry'] = true;

        return new self(
            featureFlags: $this->featureFlags,
            expiry: $expiry,
            sessionId: $this->sessionId,
            setFields: $setFields
        );
    }

    public function withSessionId(?string $sessionId): self
    {
        $setFields = $this->setFields;
        $setFields['sessionId'] = true;

        return new self(
            featureFlags: $this->featureFlags,
            expiry: $this->expiry,
            sessionId: $sessionId,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->featureFlags) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (isset($this->setFields['expiry'])) {
            $data['expiry'] = $this->expiry;
        }
        if (isset($this->setFields['sessionId'])) {
            $data['sessionId'] = $this->sessionId;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'StreamPortalAccessIn'),
            expiry: \Svix\Utils::deserializeInt($data, 'expiry', false, 'StreamPortalAccessIn'),
            sessionId: \Svix\Utils::deserializeString($data, 'sessionId', false, 'StreamPortalAccessIn')
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
