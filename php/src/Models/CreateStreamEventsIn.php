<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class CreateStreamEventsIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<EventIn> $events
     * @param StreamIn|null $stream Optionally creates a new Stream alongside the events.
     *
     * If the stream id or uid that is used in the path already exists, this argument is ignored.
     */
    private function __construct(
        public readonly array $events,
        public readonly ?StreamIn $stream = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of CreateStreamEventsIn with required fields.
     */
    public static function create(
        array $events,
    ): self {
        return new self(
            events: $events,
            stream: null,
            setFields: ['events' => true]
        );
    }

    public function withStream(?StreamIn $stream): self
    {
        $setFields = $this->setFields;
        $setFields['stream'] = true;

        return new self(
            events: $this->events,
            stream: $stream,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'events' => $this->events];

        if (isset($this->setFields['stream'])) {
            $data['stream'] = $this->stream;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            events: \Svix\Utils::deserializeObjectArray($data, 'events', true, 'CreateStreamEventsIn', [EventIn::class, 'fromMixed']),
            stream: \Svix\Utils::deserializeObject($data, 'stream', false, 'CreateStreamEventsIn', [StreamIn::class, 'fromMixed'])
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
