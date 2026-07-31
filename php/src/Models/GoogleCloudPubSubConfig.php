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
        public readonly string $projectId,
        public readonly string $topicId,
        public readonly string $credentials,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of GoogleCloudPubSubConfig with required fields.
     */
    public static function create(
        string $projectId,
        string $topicId,
        string $credentials,
    ): self {
        return new self(
            projectId: $projectId,
            topicId: $topicId,
            credentials: $credentials,
            setFields: ['projectId' => true, 'topicId' => true, 'credentials' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'projectId' => $this->projectId,
            'topicId' => $this->topicId,
            'credentials' => $this->credentials];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            projectId: \Svix\Utils::deserializeString($data, 'projectId', true, 'GoogleCloudPubSubConfig'),
            topicId: \Svix\Utils::deserializeString($data, 'topicId', true, 'GoogleCloudPubSubConfig'),
            credentials: \Svix\Utils::deserializeString($data, 'credentials', true, 'GoogleCloudPubSubConfig')
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
