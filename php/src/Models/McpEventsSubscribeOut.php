<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class McpEventsSubscribeOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                  $id            the subscription's ID, which is also the UID of its endpoint
     * @param string                  $endpointId    the Endpoint's ID
     * @param \DateTimeImmutable|null $refreshBefore when the subscription expires, or `null` if it never does
     * @param string|null             $cursor        always `null`, since events can't be replayed
     * @param bool                    $truncated     always `false`, since events can't be replayed
     */
    private function __construct(
        public readonly string $id,
        public readonly string $endpointId,
        public readonly bool $truncated,
        public readonly ?\DateTimeImmutable $refreshBefore = null,
        public readonly ?string $cursor = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of McpEventsSubscribeOut with required fields.
     */
    public static function create(
        string $id,
        string $endpointId,
        bool $truncated,
    ): self {
        return new self(
            id: $id,
            endpointId: $endpointId,
            refreshBefore: null,
            cursor: null,
            truncated: $truncated,
            setFields: ['id' => true, 'endpointId' => true, 'truncated' => true]
        );
    }

    public function withRefreshBefore(?\DateTimeImmutable $refreshBefore): self
    {
        $setFields = $this->setFields;
        $setFields['refreshBefore'] = true;

        return new self(
            id: $this->id,
            endpointId: $this->endpointId,
            refreshBefore: $refreshBefore,
            cursor: $this->cursor,
            truncated: $this->truncated,
            setFields: $setFields
        );
    }

    public function withCursor(?string $cursor): self
    {
        $setFields = $this->setFields;
        $setFields['cursor'] = true;

        return new self(
            id: $this->id,
            endpointId: $this->endpointId,
            refreshBefore: $this->refreshBefore,
            cursor: $cursor,
            truncated: $this->truncated,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'id' => $this->id,
            'endpointId' => $this->endpointId,
            'truncated' => $this->truncated];

        if (isset($this->setFields['refreshBefore'])) {
            $data['refreshBefore'] = $this->refreshBefore->format('c');
        }
        if (isset($this->setFields['cursor'])) {
            $data['cursor'] = $this->cursor;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            id: \Svix\Utils::deserializeString($data, 'id', true, 'McpEventsSubscribeOut'),
            endpointId: \Svix\Utils::deserializeString($data, 'endpointId', true, 'McpEventsSubscribeOut'),
            refreshBefore: \Svix\Utils::deserializeDt($data, 'refreshBefore', false, 'McpEventsSubscribeOut'),
            cursor: \Svix\Utils::deserializeString($data, 'cursor', false, 'McpEventsSubscribeOut'),
            truncated: \Svix\Utils::deserializeBool($data, 'truncated', true, 'McpEventsSubscribeOut')
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
