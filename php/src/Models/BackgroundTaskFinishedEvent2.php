<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class BackgroundTaskFinishedEvent2 implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $taskId the QueueBackgroundTask's ID
     */
    private function __construct(
        public readonly Data $data,
        public readonly BackgroundTaskStatus $status,
        public readonly BackgroundTaskType $task,
        public readonly string $taskId,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BackgroundTaskFinishedEvent2 with required fields.
     */
    public static function create(
        Data $data,
        BackgroundTaskStatus $status,
        BackgroundTaskType $task,
        string $taskId,
    ): self {
        return new self(
            data: $data,
            status: $status,
            task: $task,
            taskId: $taskId,
            setFields: ['data' => true, 'status' => true, 'task' => true, 'taskId' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'data' => $this->data,
            'status' => $this->status,
            'task' => $this->task,
            'taskId' => $this->taskId];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::deserializeObject($data, 'data', true, 'BackgroundTaskFinishedEvent2', [Data::class, 'fromMixed']),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'BackgroundTaskFinishedEvent2', [BackgroundTaskStatus::class, 'fromMixed']),
            task: \Svix\Utils::deserializeObject($data, 'task', true, 'BackgroundTaskFinishedEvent2', [BackgroundTaskType::class, 'fromMixed']),
            taskId: \Svix\Utils::deserializeString($data, 'taskId', true, 'BackgroundTaskFinishedEvent2')
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
