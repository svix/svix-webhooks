<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AutoConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $id the AutoConfigSubscription's ID
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $token,
        public readonly string $id,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AutoConfigOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        string $token,
        string $id,
    ): self {
        return new self(
            createdAt: $createdAt,
            token: $token,
            id: $id,
            setFields: ['createdAt' => true, 'token' => true, 'id' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'createdAt' => $this->createdAt->format('c'),
            'token' => $this->token,
            'id' => $this->id];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'AutoConfigOut'),
            token: \Svix\Utils::deserializeString($data, 'token', true, 'AutoConfigOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'AutoConfigOut')
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
