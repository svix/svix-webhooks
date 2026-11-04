<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AutoConfigSinkType implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly AutoConfigSinkTypeConfig $config,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AutoConfigSinkType with required fields.
     */
    public static function create(
        AutoConfigSinkTypeConfig $config,
    ): self {
        return new self(
            config: $config,
            setFields: [
                'config' => true, ]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'type' => $this->config->variantName(),
            'config' => $this->config,
        ];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            config: AutoConfigSinkTypeConfig::fromTypeAndConfig(
                \Svix\Utils::deserializeString($data, 'type', true, 'AutoConfigSinkType'),
                \Svix\Utils::getValFromJson($data, 'config', false, 'AutoConfigSinkType') ?? []
            ),
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

abstract class AutoConfigSinkTypeConfig implements \JsonSerializable
{
    abstract public function variantName(): string;

    abstract public function configPayload(): mixed;

    public static function poller(SinkInCommon $poller): AutoConfigSinkTypeConfig\Poller
    {
        return new AutoConfigSinkTypeConfig\Poller($poller);
    }

    public static function http(EndpointIn $http): AutoConfigSinkTypeConfig\Http
    {
        return new AutoConfigSinkTypeConfig\Http($http);
    }

    public static function fromTypeAndConfig(string $type, mixed $config): self
    {
        $config ??= [];

        return match ($type) {
            'poller' => self::poller(SinkInCommon::fromMixed($config)),
            'http' => self::http(EndpointIn::fromMixed($config)),
            default => throw new \InvalidArgumentException("Unknown type: {$type}"),
        };
    }

    public function jsonSerialize(): mixed
    {
        return \Svix\Utils::newStdClassIfArrayIsEmpty($this->configPayload());
    }
}

namespace Svix\Models\AutoConfigSinkTypeConfig;

final class Poller extends \Svix\Models\AutoConfigSinkTypeConfig
{
    public function __construct(public readonly \Svix\Models\SinkInCommon $poller)
    {
    }

    public function variantName(): string
    {
        return 'poller';
    }

    public function configPayload(): mixed
    {
        return $this->poller;
    }
}

final class Http extends \Svix\Models\AutoConfigSinkTypeConfig
{
    public function __construct(public readonly \Svix\Models\EndpointIn $http)
    {
    }

    public function variantName(): string
    {
        return 'http';
    }

    public function configPayload(): mixed
    {
        return $this->http;
    }
}
