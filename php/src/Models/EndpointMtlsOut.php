<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointMtlsOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly bool $enabled,
        public readonly ?EndpointMtlsConfigOut $config = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointMtlsOut with required fields.
     */
    public static function create(
        bool $enabled,
    ): self {
        return new self(
            enabled: $enabled,
            config: null,
            setFields: ['enabled' => true]
        );
    }

    public function withConfig(?EndpointMtlsConfigOut $config): self
    {
        $setFields = $this->setFields;
        $setFields['config'] = true;

        return new self(
            enabled: $this->enabled,
            config: $config,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'enabled' => $this->enabled];

        if (isset($this->setFields['config'])) {
            $data['config'] = $this->config;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            enabled: \Svix\Utils::deserializeBool($data, 'enabled', true, 'EndpointMtlsOut'),
            config: \Svix\Utils::deserializeObject($data, 'config', false, 'EndpointMtlsOut', [EndpointMtlsConfigOut::class, 'fromMixed'])
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
