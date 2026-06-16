<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class PollerV2PollOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<PollerV2MessageOut> $data
     */
    private function __construct(
        public readonly array $data,
        public readonly bool $done,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PollerV2PollOut with required fields.
     */
    public static function create(
        array $data,
        bool $done,
    ): self {
        return new self(
            data: $data,
            done: $done,
            setFields: ['data' => true, 'done' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'data' => $this->data,
            'done' => $this->done];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::deserializeObjectArray($data, 'data', true, 'PollerV2PollOut', [PollerV2MessageOut::class, 'fromMixed']),
            done: \Svix\Utils::deserializeBool($data, 'done', true, 'PollerV2PollOut')
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
