<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ApiTokenOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $scopes
     */
    private function __construct(
        public readonly string $token,
        public readonly string $id,
        public readonly \DateTimeImmutable $createdAt,
        public readonly ?string $name = null,
        public readonly ?\DateTimeImmutable $expiresAt = null,
        public readonly ?array $scopes = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ApiTokenOut with required fields.
     */
    public static function create(
        string $token,
        string $id,
        \DateTimeImmutable $createdAt,
    ): self {
        return new self(
            token: $token,
            id: $id,
            name: null,
            createdAt: $createdAt,
            expiresAt: null,
            scopes: null,
            setFields: ['token' => true, 'id' => true, 'createdAt' => true]
        );
    }

    public function withName(?string $name): self
    {
        $setFields = $this->setFields;
        $setFields['name'] = true;

        return new self(
            token: $this->token,
            id: $this->id,
            name: $name,
            createdAt: $this->createdAt,
            expiresAt: $this->expiresAt,
            scopes: $this->scopes,
            setFields: $setFields
        );
    }

    public function withExpiresAt(?\DateTimeImmutable $expiresAt): self
    {
        $setFields = $this->setFields;
        $setFields['expiresAt'] = true;

        return new self(
            token: $this->token,
            id: $this->id,
            name: $this->name,
            createdAt: $this->createdAt,
            expiresAt: $expiresAt,
            scopes: $this->scopes,
            setFields: $setFields
        );
    }

    public function withScopes(?array $scopes): self
    {
        $setFields = $this->setFields;
        $setFields['scopes'] = true;

        return new self(
            token: $this->token,
            id: $this->id,
            name: $this->name,
            createdAt: $this->createdAt,
            expiresAt: $this->expiresAt,
            scopes: $scopes,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'token' => $this->token,
            'id' => $this->id,
            'createdAt' => $this->createdAt->format('c')];

        if (isset($this->setFields['name'])) {
            $data['name'] = $this->name;
        }
        if (isset($this->setFields['expiresAt'])) {
            $data['expiresAt'] = $this->expiresAt->format('c');
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
            token: \Svix\Utils::deserializeString($data, 'token', true, 'ApiTokenOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'ApiTokenOut'),
            name: \Svix\Utils::deserializeString($data, 'name', false, 'ApiTokenOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'ApiTokenOut'),
            expiresAt: \Svix\Utils::deserializeDt($data, 'expiresAt', false, 'ApiTokenOut'),
            scopes: \Svix\Utils::getValFromJson($data, 'scopes', false, 'ApiTokenOut')
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
