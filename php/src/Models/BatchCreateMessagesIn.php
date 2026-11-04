<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class BatchCreateMessagesIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<BatchMessageIn> $messages
     */
    private function __construct(
        public readonly array $messages,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BatchCreateMessagesIn with required fields.
     */
    public static function create(
        array $messages,
    ): self {
        return new self(
            messages: $messages,
            setFields: ['messages' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'messages' => $this->messages];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            messages: \Svix\Utils::deserializeObjectArray($data, 'messages', true, 'BatchCreateMessagesIn', [BatchMessageIn::class, 'fromMixed'])
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
