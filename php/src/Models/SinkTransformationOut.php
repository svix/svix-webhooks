<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SinkTransformationOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly bool $enabled,
        public readonly ?string $code = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SinkTransformationOut with required fields.
     */
    public static function create(
        bool $enabled,
    ): self {
        return new self(
            code: null,
            enabled: $enabled,
            setFields: ['enabled' => true]
        );
    }

    public function withCode(?string $code): self
    {
        $setFields = $this->setFields;
        $setFields['code'] = true;

        return new self(
            code: $code,
            enabled: $this->enabled,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'enabled' => $this->enabled];

        if (isset($this->setFields['code'])) {
            $data['code'] = $this->code;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            code: \Svix\Utils::deserializeString($data, 'code', false, 'SinkTransformationOut'),
            enabled: \Svix\Utils::deserializeBool($data, 'enabled', true, 'SinkTransformationOut')
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
