<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class BulkReplayIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $eventTypes
     */
    private function __construct(
        public readonly \DateTimeImmutable $since,
        public readonly ?\DateTimeImmutable $until = null,
        public readonly ?array $eventTypes = null,
        public readonly ?string $channel = null,
        public readonly ?string $tag = null,
        public readonly ?MessageStatus $status = null,
        public readonly ?StatusCodeClass $statusCodeClass = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BulkReplayIn with required fields.
     */
    public static function create(
        \DateTimeImmutable $since,
    ): self {
        return new self(
            since: $since,
            until: null,
            eventTypes: null,
            channel: null,
            tag: null,
            status: null,
            statusCodeClass: null,
            setFields: ['since' => true]
        );
    }

    public function withUntil(?\DateTimeImmutable $until): self
    {
        $setFields = $this->setFields;
        $setFields['until'] = true;

        return new self(
            since: $this->since,
            until: $until,
            eventTypes: $this->eventTypes,
            channel: $this->channel,
            tag: $this->tag,
            status: $this->status,
            statusCodeClass: $this->statusCodeClass,
            setFields: $setFields
        );
    }

    public function withEventTypes(?array $eventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['eventTypes'] = true;

        return new self(
            since: $this->since,
            until: $this->until,
            eventTypes: $eventTypes,
            channel: $this->channel,
            tag: $this->tag,
            status: $this->status,
            statusCodeClass: $this->statusCodeClass,
            setFields: $setFields
        );
    }

    public function withChannel(?string $channel): self
    {
        $setFields = $this->setFields;
        $setFields['channel'] = true;

        return new self(
            since: $this->since,
            until: $this->until,
            eventTypes: $this->eventTypes,
            channel: $channel,
            tag: $this->tag,
            status: $this->status,
            statusCodeClass: $this->statusCodeClass,
            setFields: $setFields
        );
    }

    public function withTag(?string $tag): self
    {
        $setFields = $this->setFields;
        $setFields['tag'] = true;

        return new self(
            since: $this->since,
            until: $this->until,
            eventTypes: $this->eventTypes,
            channel: $this->channel,
            tag: $tag,
            status: $this->status,
            statusCodeClass: $this->statusCodeClass,
            setFields: $setFields
        );
    }

    public function withStatus(?MessageStatus $status): self
    {
        $setFields = $this->setFields;
        $setFields['status'] = true;

        return new self(
            since: $this->since,
            until: $this->until,
            eventTypes: $this->eventTypes,
            channel: $this->channel,
            tag: $this->tag,
            status: $status,
            statusCodeClass: $this->statusCodeClass,
            setFields: $setFields
        );
    }

    public function withStatusCodeClass(?StatusCodeClass $statusCodeClass): self
    {
        $setFields = $this->setFields;
        $setFields['statusCodeClass'] = true;

        return new self(
            since: $this->since,
            until: $this->until,
            eventTypes: $this->eventTypes,
            channel: $this->channel,
            tag: $this->tag,
            status: $this->status,
            statusCodeClass: $statusCodeClass,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'since' => $this->since->format('c')];

        if (isset($this->setFields['until'])) {
            $data['until'] = $this->until->format('c');
        }
        if (isset($this->setFields['eventTypes'])) {
            $data['eventTypes'] = $this->eventTypes;
        }
        if (isset($this->setFields['channel'])) {
            $data['channel'] = $this->channel;
        }
        if (isset($this->setFields['tag'])) {
            $data['tag'] = $this->tag;
        }
        if (isset($this->setFields['status'])) {
            $data['status'] = $this->status;
        }
        if (isset($this->setFields['statusCodeClass'])) {
            $data['statusCodeClass'] = $this->statusCodeClass;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            since: \Svix\Utils::deserializeDt($data, 'since', true, 'BulkReplayIn'),
            until: \Svix\Utils::deserializeDt($data, 'until', false, 'BulkReplayIn'),
            eventTypes: \Svix\Utils::getValFromJson($data, 'eventTypes', false, 'BulkReplayIn'),
            channel: \Svix\Utils::deserializeString($data, 'channel', false, 'BulkReplayIn'),
            tag: \Svix\Utils::deserializeString($data, 'tag', false, 'BulkReplayIn'),
            status: \Svix\Utils::deserializeObject($data, 'status', false, 'BulkReplayIn', [MessageStatus::class, 'fromMixed']),
            statusCodeClass: \Svix\Utils::deserializeObject($data, 'statusCodeClass', false, 'BulkReplayIn', [StatusCodeClass::class, 'fromMixed'])
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
