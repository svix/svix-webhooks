<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AppPortalAccessOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $token,
        public readonly string $url,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AppPortalAccessOut with required fields.
     */
    public static function create(
        string $token,
        string $url,
    ): self {
        return new self(
            token: $token,
            url: $url,
            setFields: ['token' => true, 'url' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'token' => $this->token,
            'url' => $this->url];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            token: \Svix\Utils::deserializeString($data, 'token', true, 'AppPortalAccessOut'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'AppPortalAccessOut')
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
