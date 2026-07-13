<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ListResponseStreamSinkOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<StreamSinkOut> $data
     */
    private function __construct(
        public readonly array $data,
        public readonly string $iterator,
        public readonly bool $done,
        public readonly ?string $prevIterator = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ListResponseStreamSinkOut with required fields.
     */
    public static function create(
        array $data,
        string $iterator,
        bool $done,
    ): self {
        return new self(
            data: $data,
            iterator: $iterator,
            prevIterator: null,
            done: $done,
            setFields: ['data' => true, 'iterator' => true, 'done' => true]
        );
    }

    public function withPrevIterator(?string $prevIterator): self
    {
        $setFields = $this->setFields;
        $setFields['prevIterator'] = true;

        return new self(
            data: $this->data,
            iterator: $this->iterator,
            prevIterator: $prevIterator,
            done: $this->done,
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

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::deserializeObjectArray($data, 'data', true, 'ListResponseStreamSinkOut', [StreamSinkOut::class, 'fromMixed']),
            iterator: \Svix\Utils::deserializeString($data, 'iterator', true, 'ListResponseStreamSinkOut'),
            prevIterator: \Svix\Utils::deserializeString($data, 'prevIterator', false, 'ListResponseStreamSinkOut'),
            done: \Svix\Utils::deserializeBool($data, 'done', true, 'ListResponseStreamSinkOut')
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
