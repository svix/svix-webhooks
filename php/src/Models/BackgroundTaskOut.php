<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class BackgroundTaskOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $id the QueueBackgroundTask's ID
     */
    private function __construct(
        public readonly array $data,
        public readonly string $id,
        public readonly BackgroundTaskStatus $status,
        public readonly BackgroundTaskType $task,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BackgroundTaskOut with required fields.
     */
    public static function create(
        array $data,
        string $id,
        BackgroundTaskStatus $status,
        BackgroundTaskType $task,
    ): self {
        return new self(
            data: $data,
            id: $id,
            status: $status,
            task: $task,
            setFields: ['data' => true, 'id' => true, 'status' => true, 'task' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'data' => $this->data,
            'id' => $this->id,
            'status' => $this->status,
            'task' => $this->task];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::getValFromJson($data, 'data', true, 'BackgroundTaskOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'BackgroundTaskOut'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'BackgroundTaskOut', [BackgroundTaskStatus::class, 'fromMixed']),
            task: \Svix\Utils::deserializeObject($data, 'task', true, 'BackgroundTaskOut', [BackgroundTaskType::class, 'fromMixed'])
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
