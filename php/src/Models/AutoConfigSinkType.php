<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

abstract class AutoConfigSinkType implements \JsonSerializable
{
    abstract public function variantName(): string;

    abstract public function configPayload(): mixed;

    public static function poller(SinkInCommon $poller): AutoConfigSinkType\Poller
    {
        return new AutoConfigSinkType\Poller($poller);
    }

    public static function http(EndpointIn $http): AutoConfigSinkType\Http
    {
        return new AutoConfigSinkType\Http($http);
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

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return self::fromTypeAndConfig(
            \Svix\Utils::deserializeString($data, 'type', true, 'AutoConfigSinkType'),
            \Svix\Utils::getValFromJson($data, 'config', false, 'AutoConfigSinkType') ?? []
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

    public function jsonSerialize(): mixed
    {
        return [
            'type' => $this->variantName(),
            'config' => \Svix\Utils::newStdClassIfArrayIsEmpty($this->configPayload()),
        ];
    }
}

namespace Svix\Models\AutoConfigSinkType;

final class Poller extends \Svix\Models\AutoConfigSinkType
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

final class Http extends \Svix\Models\AutoConfigSinkType
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
