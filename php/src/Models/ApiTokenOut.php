<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ApiTokenOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string            $id     the GlobalApplicationToken's ID
     * @param list<string>|null $scopes
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $id,
        public readonly string $token,
        public readonly ?\DateTimeImmutable $expiresAt = null,
        public readonly ?string $name = null,
        public readonly ?array $scopes = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ApiTokenOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        string $id,
        string $token,
    ): self {
        return new self(
            createdAt: $createdAt,
            expiresAt: null,
            id: $id,
            name: null,
            scopes: null,
            token: $token,
            setFields: ['createdAt' => true, 'id' => true, 'token' => true]
        );
    }

    public function withExpiresAt(?\DateTimeImmutable $expiresAt): self
    {
        $setFields = $this->setFields;
        $setFields['expiresAt'] = true;

        return new self(
            createdAt: $this->createdAt,
            expiresAt: $expiresAt,
            id: $this->id,
            name: $this->name,
            scopes: $this->scopes,
            token: $this->token,
            setFields: $setFields
        );
    }

    public function withName(?string $name): self
    {
        $setFields = $this->setFields;
        $setFields['name'] = true;

        return new self(
            createdAt: $this->createdAt,
            expiresAt: $this->expiresAt,
            id: $this->id,
            name: $name,
            scopes: $this->scopes,
            token: $this->token,
            setFields: $setFields
        );
    }

    public function withScopes(?array $scopes): self
    {
        $setFields = $this->setFields;
        $setFields['scopes'] = true;

        return new self(
            createdAt: $this->createdAt,
            expiresAt: $this->expiresAt,
            id: $this->id,
            name: $this->name,
            scopes: $scopes,
            token: $this->token,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'createdAt' => $this->createdAt->format('c'),
            'id' => $this->id,
            'token' => $this->token];

        if (isset($this->setFields['expiresAt'])) {
            $data['expiresAt'] = $this->expiresAt->format('c');
        }
        if (isset($this->setFields['name'])) {
            $data['name'] = $this->name;
        }
        if (isset($this->setFields['scopes'])) {
            $data['scopes'] = $this->scopes;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'ApiTokenOut'),
            expiresAt: \Svix\Utils::deserializeDt($data, 'expiresAt', false, 'ApiTokenOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'ApiTokenOut'),
            name: \Svix\Utils::deserializeString($data, 'name', false, 'ApiTokenOut'),
            scopes: \Svix\Utils::getValFromJson($data, 'scopes', false, 'ApiTokenOut'),
            token: \Svix\Utils::deserializeString($data, 'token', true, 'ApiTokenOut')
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
