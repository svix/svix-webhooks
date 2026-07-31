<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AppPortalAccessOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $url,
        public readonly string $token,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AppPortalAccessOut with required fields.
     */
    public static function create(
        string $url,
        string $token,
    ): self {
        return new self(
            url: $url,
            token: $token,
            setFields: ['url' => true, 'token' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'url' => $this->url,
            'token' => $this->token];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'AppPortalAccessOut'),
            token: \Svix\Utils::deserializeString($data, 'token', true, 'AppPortalAccessOut')
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
