<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ReplayIn implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly \DateTimeImmutable $since,
        public readonly ?\DateTimeImmutable $until = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ReplayIn with required fields.
     */
    public static function create(
        \DateTimeImmutable $since,
    ): self {
        return new self(
            since: $since,
            until: null,
            setFields: ['since' => true]
        );
    }

    public function withUntil(?\DateTimeImmutable $until): self
    {
        $setFields = $this->setFields;
        $setFields['until'] = true;

        return new self(
            since: $this->since,
            until: $until,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'since' => $this->since->format('c')];

        if (isset($this->setFields['until'])) {
            $data['until'] = $this->until->format('c');
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            since: \Svix\Utils::deserializeDt($data, 'since', true, 'ReplayIn'),
            until: \Svix\Utils::deserializeDt($data, 'until', false, 'ReplayIn')
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
