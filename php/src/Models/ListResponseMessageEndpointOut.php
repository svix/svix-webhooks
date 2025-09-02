<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ListResponseMessageEndpointOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<MessageEndpointOut> $data
     */
    private function __construct(
        public readonly array $data,
        public readonly bool $done,
        public readonly string $iterator,
        public readonly ?string $prevIterator = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ListResponseMessageEndpointOut with required fields.
     */
    public static function create(
        array $data,
        bool $done,
        string $iterator,
    ): self {
        return new self(
            data: $data,
            done: $done,
            iterator: $iterator,
            prevIterator: null,
            setFields: ['data' => true, 'done' => true, 'iterator' => true]
        );
    }

    public function withPrevIterator(?string $prevIterator): self
    {
        $setFields = $this->setFields;
        $setFields['prevIterator'] = true;

        return new self(
            data: $this->data,
            done: $this->done,
            iterator: $this->iterator,
            prevIterator: $prevIterator,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'data' => $this->data,
            'done' => $this->done];

        if (isset($this->setFields['prevIterator'])) {
            $data['prevIterator'] = $this->prevIterator;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::deserializeObjectArray($data, 'data', true, 'ListResponseMessageEndpointOut', [MessageEndpointOut::class, 'fromMixed']),
            done: \Svix\Utils::deserializeBool($data, 'done', true, 'ListResponseMessageEndpointOut'),
            iterator: \Svix\Utils::deserializeString($data, 'iterator', true, 'ListResponseMessageEndpointOut'),
            prevIterator: \Svix\Utils::deserializeString($data, 'prevIterator', false, 'ListResponseMessageEndpointOut')
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
