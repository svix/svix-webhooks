<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class RecoverOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $id the QueueBackgroundTask's ID
     */
    private function __construct(
        public readonly string $id,
        public readonly BackgroundTaskStatus $status,
        public readonly BackgroundTaskType $task,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RecoverOut with required fields.
     */
    public static function create(
        string $id,
        BackgroundTaskStatus $status,
        BackgroundTaskType $task,
    ): self {
        return new self(
            id: $id,
            status: $status,
            task: $task,
            setFields: ['id' => true, 'status' => true, 'task' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'id' => $this->id,
            'status' => $this->status,
            'task' => $this->task];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            id: \Svix\Utils::deserializeString($data, 'id', true, 'RecoverOut'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'RecoverOut', [BackgroundTaskStatus::class, 'fromMixed']),
            task: \Svix\Utils::deserializeObject($data, 'task', true, 'RecoverOut', [BackgroundTaskType::class, 'fromMixed'])
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
