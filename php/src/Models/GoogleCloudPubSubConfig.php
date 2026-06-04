<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class GoogleCloudPubSubConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $credentials google Cloud Credentials JSON Object as a string
     */
    private function __construct(
        public readonly string $credentials,
        public readonly string $projectId,
        public readonly string $topicId,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of GoogleCloudPubSubConfig with required fields.
     */
    public static function create(
        string $credentials,
        string $projectId,
        string $topicId,
    ): self {
        return new self(
            credentials: $credentials,
            projectId: $projectId,
            topicId: $topicId,
            setFields: ['credentials' => true, 'projectId' => true, 'topicId' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'credentials' => $this->credentials,
            'projectId' => $this->projectId,
            'topicId' => $this->topicId];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            credentials: \Svix\Utils::deserializeString($data, 'credentials', true, 'GoogleCloudPubSubConfig'),
            projectId: \Svix\Utils::deserializeString($data, 'projectId', true, 'GoogleCloudPubSubConfig'),
            topicId: \Svix\Utils::deserializeString($data, 'topicId', true, 'GoogleCloudPubSubConfig')
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
