<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class PostHogConfigPatch implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $domain = null,
        public readonly ?string $projectToken = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PostHogConfigPatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            domain: null,
            projectToken: null,
            setFields: []
        );
    }

    public function withDomain(?string $domain): self
    {
        $setFields = $this->setFields;
        $setFields['domain'] = true;

        return new self(
            domain: $domain,
            projectToken: $this->projectToken,
            setFields: $setFields
        );
    }

    public function withProjectToken(?string $projectToken): self
    {
        $setFields = $this->setFields;
        $setFields['projectToken'] = true;

        return new self(
            domain: $this->domain,
            projectToken: $projectToken,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->domain) {
            $data['domain'] = $this->domain;
        }
        if (null !== $this->projectToken) {
            $data['projectToken'] = $this->projectToken;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            domain: \Svix\Utils::getValFromJson($data, 'domain', false, 'PostHogConfigPatch'),
            projectToken: \Svix\Utils::deserializeString($data, 'projectToken', false, 'PostHogConfigPatch')
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
