<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AppUsageStatsOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string       $id               the QueueBackgroundTask's ID
     * @param list<string> $unresolvedAppIds Any app IDs or UIDs received in the request that weren't found.
     *
     * Stats will be produced for all the others.
     */
    private function __construct(
        public readonly string $id,
        public readonly BackgroundTaskStatus $status,
        public readonly BackgroundTaskType $task,
        public readonly array $unresolvedAppIds,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AppUsageStatsOut with required fields.
     */
    public static function create(
        string $id,
        BackgroundTaskStatus $status,
        BackgroundTaskType $task,
        array $unresolvedAppIds,
    ): self {
        return new self(
            id: $id,
            status: $status,
            task: $task,
            unresolvedAppIds: $unresolvedAppIds,
            setFields: ['id' => true, 'status' => true, 'task' => true, 'unresolvedAppIds' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['id' => $this->id,
            'status' => $this->status,
            'task' => $this->task,
            'unresolvedAppIds' => $this->unresolvedAppIds];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            id: \Svix\Utils::deserializeString($data, 'id', true, 'AppUsageStatsOut'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'AppUsageStatsOut', [BackgroundTaskStatus::class, 'fromMixed']),
            task: \Svix\Utils::deserializeObject($data, 'task', true, 'AppUsageStatsOut', [BackgroundTaskType::class, 'fromMixed']),
            unresolvedAppIds: \Svix\Utils::getValFromJson($data, 'unresolvedAppIds', true, 'AppUsageStatsOut')
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
