<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class MessagePrecheckOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param bool $active whether there are any active endpoint that would get sent such a message
     */
    private function __construct(
        public readonly bool $active,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessagePrecheckOut with required fields.
     */
    public static function create(
        bool $active,
    ): self {
        return new self(
            active: $active,
            setFields: ['active' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'active' => $this->active];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            active: \Svix\Utils::deserializeBool($data, 'active', true, 'MessagePrecheckOut')
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
