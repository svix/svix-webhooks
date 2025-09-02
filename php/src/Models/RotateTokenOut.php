<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class RotateTokenOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $ingestUrl,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RotateTokenOut with required fields.
     */
    public static function create(
        string $ingestUrl,
    ): self {
        return new self(
            ingestUrl: $ingestUrl,
            setFields: ['ingestUrl' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'ingestUrl' => $this->ingestUrl];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            ingestUrl: \Svix\Utils::deserializeString($data, 'ingestUrl', true, 'RotateTokenOut')
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
