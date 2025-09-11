<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AppPortalAccessIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param ApplicationIn|null $application Optionally creates a new application while generating the access link.
     *
     * If the application id or uid that is used in the path already exists, this argument is ignored.
     * @param list<AppPortalCapability>|null $capabilities Custom capabilities attached to the token, You can combine as many capabilities as necessary.
     *
     * The `ViewBase` capability is always required
     *
     * - `ViewBase`: Basic read only permissions, does not allow the user to see the endpoint secret.
     *
     * - `ViewEndpointSecret`: Allows user to view the endpoint secret.
     *
     * - `ManageEndpointSecret`: Allows user to rotate and view the endpoint secret.
     *
     * - `ManageTransformations`: Allows user to modify the endpoint transformations.
     *
     * - `CreateAttempts`: Allows user to replay missing messages and send example messages.
     *
     * - `ManageEndpoint`: Allows user to read/modify any field or configuration of an endpoint (including secrets)
     * @param int|null $expiry How long the token will be valid for, in seconds.
     *
     * Valid values are between 1 hour and 7 days. The default is 7 days.
     * @param list<string>|null $featureFlags the set of feature flags the created token will have access to
     * @param bool|null         $readOnly     whether the app portal should be in read-only mode
     * @param string|null       $sessionId    An optional session ID to attach to the token.
     *
     * When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID.
     */
    private function __construct(
        public readonly ?ApplicationIn $application = null,
        public readonly ?array $capabilities = null,
        public readonly ?int $expiry = null,
        public readonly ?array $featureFlags = null,
        public readonly ?bool $readOnly = null,
        public readonly ?string $sessionId = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AppPortalAccessIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            application: null,
            capabilities: null,
            expiry: null,
            featureFlags: null,
            readOnly: null,
            sessionId: null,
            setFields: []
        );
    }

    public function withApplication(?ApplicationIn $application): self
    {
        $setFields = $this->setFields;
        $setFields['application'] = true;

        return new self(
            application: $application,
            capabilities: $this->capabilities,
            expiry: $this->expiry,
            featureFlags: $this->featureFlags,
            readOnly: $this->readOnly,
            sessionId: $this->sessionId,
            setFields: $setFields
        );
    }

    public function withCapabilities(?array $capabilities): self
    {
        $setFields = $this->setFields;
        $setFields['capabilities'] = true;

        return new self(
            application: $this->application,
            capabilities: $capabilities,
            expiry: $this->expiry,
            featureFlags: $this->featureFlags,
            readOnly: $this->readOnly,
            sessionId: $this->sessionId,
            setFields: $setFields
        );
    }

    public function withExpiry(?int $expiry): self
    {
        $setFields = $this->setFields;
        $setFields['expiry'] = true;

        return new self(
            application: $this->application,
            capabilities: $this->capabilities,
            expiry: $expiry,
            featureFlags: $this->featureFlags,
            readOnly: $this->readOnly,
            sessionId: $this->sessionId,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            application: $this->application,
            capabilities: $this->capabilities,
            expiry: $this->expiry,
            featureFlags: $featureFlags,
            readOnly: $this->readOnly,
            sessionId: $this->sessionId,
            setFields: $setFields
        );
    }

    public function withReadOnly(?bool $readOnly): self
    {
        $setFields = $this->setFields;
        $setFields['readOnly'] = true;

        return new self(
            application: $this->application,
            capabilities: $this->capabilities,
            expiry: $this->expiry,
            featureFlags: $this->featureFlags,
            readOnly: $readOnly,
            sessionId: $this->sessionId,
            setFields: $setFields
        );
    }

    public function withSessionId(?string $sessionId): self
    {
        $setFields = $this->setFields;
        $setFields['sessionId'] = true;

        return new self(
            application: $this->application,
            capabilities: $this->capabilities,
            expiry: $this->expiry,
            featureFlags: $this->featureFlags,
            readOnly: $this->readOnly,
            sessionId: $sessionId,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (isset($this->setFields['application'])) {
            $data['application'] = $this->application;
        }
        if (isset($this->setFields['capabilities'])) {
            $data['capabilities'] = $this->capabilities;
        }
        if (isset($this->setFields['expiry'])) {
            $data['expiry'] = $this->expiry;
        }
        if (null !== $this->featureFlags) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (isset($this->setFields['readOnly'])) {
            $data['readOnly'] = $this->readOnly;
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
            application: \Svix\Utils::deserializeObject($data, 'application', false, 'AppPortalAccessIn', [ApplicationIn::class, 'fromMixed']),
            capabilities: \Svix\Utils::getValFromJson($data, 'capabilities', false, 'AppPortalAccessIn'),
            expiry: \Svix\Utils::deserializeInt($data, 'expiry', false, 'AppPortalAccessIn'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'AppPortalAccessIn'),
            readOnly: \Svix\Utils::deserializeBool($data, 'readOnly', false, 'AppPortalAccessIn'),
            sessionId: \Svix\Utils::deserializeString($data, 'sessionId', false, 'AppPortalAccessIn')
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
