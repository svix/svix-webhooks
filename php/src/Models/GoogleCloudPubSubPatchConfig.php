<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class GoogleCloudPubSubPatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $projectId = null,
        public readonly ?string $topicId = null,
        public readonly ?string $credentials = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of GoogleCloudPubSubPatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            projectId: null,
            topicId: null,
            credentials: null,
            setFields: []
        );
    }

    public function withProjectId(?string $projectId): self
    {
        $setFields = $this->setFields;
        $setFields['projectId'] = true;

        return new self(
            projectId: $projectId,
            topicId: $this->topicId,
            credentials: $this->credentials,
            setFields: $setFields
        );
    }

    public function withTopicId(?string $topicId): self
    {
        $setFields = $this->setFields;
        $setFields['topicId'] = true;

        return new self(
            projectId: $this->projectId,
            topicId: $topicId,
            credentials: $this->credentials,
            setFields: $setFields
        );
    }

    public function withCredentials(?string $credentials): self
    {
        $setFields = $this->setFields;
        $setFields['credentials'] = true;

        return new self(
            projectId: $this->projectId,
            topicId: $this->topicId,
            credentials: $credentials,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->projectId) {
            $data['projectId'] = $this->projectId;
        }
        if (null !== $this->topicId) {
            $data['topicId'] = $this->topicId;
        }
        if (null !== $this->credentials) {
            $data['credentials'] = $this->credentials;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            projectId: \Svix\Utils::deserializeString($data, 'projectId', false, 'GoogleCloudPubSubPatchConfig'),
            topicId: \Svix\Utils::deserializeString($data, 'topicId', false, 'GoogleCloudPubSubPatchConfig'),
            credentials: \Svix\Utils::deserializeString($data, 'credentials', false, 'GoogleCloudPubSubPatchConfig')
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
