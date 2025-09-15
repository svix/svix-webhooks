<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AppUsageStatsIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $appIds Specific app IDs or UIDs to aggregate stats for.
     *
     * Note that if none of the given IDs or UIDs are resolved, a 422 response will be given.
     */
    private function __construct(
        public readonly \DateTimeImmutable $since,
        public readonly \DateTimeImmutable $until,
        public readonly ?array $appIds = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AppUsageStatsIn with required fields.
     */
    public static function create(
        \DateTimeImmutable $since,
        \DateTimeImmutable $until,
    ): self {
        return new self(
            appIds: null,
            since: $since,
            until: $until,
            setFields: ['since' => true, 'until' => true]
        );
    }

    public function withAppIds(?array $appIds): self
    {
        $setFields = $this->setFields;
        $setFields['appIds'] = true;

        return new self(
            appIds: $appIds,
            since: $this->since,
            until: $this->until,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'since' => $this->since->format('c'),
            'until' => $this->until->format('c')];

        if (isset($this->setFields['appIds'])) {
            $data['appIds'] = $this->appIds;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            appIds: \Svix\Utils::getValFromJson($data, 'appIds', false, 'AppUsageStatsIn'),
            since: \Svix\Utils::deserializeDt($data, 'since', true, 'AppUsageStatsIn'),
            until: \Svix\Utils::deserializeDt($data, 'until', true, 'AppUsageStatsIn')
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
