<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

abstract class DestinationInConfig implements \JsonSerializable
{
    abstract public function variantName(): string;

    abstract public function configPayload(): mixed;

    public static function pollingEndpoint(): DestinationInConfig\PollingEndpoint
    {
        return new DestinationInConfig\PollingEndpoint();
    }

    public static function azureBlobStorage(AzureBlobStorageConfigIn $azureBlobStorage): DestinationInConfig\AzureBlobStorage
    {
        return new DestinationInConfig\AzureBlobStorage($azureBlobStorage);
    }

    public static function otelTracing(OtelTracingConfigIn $otelTracing): DestinationInConfig\OtelTracing
    {
        return new DestinationInConfig\OtelTracing($otelTracing);
    }

    public static function fifoEndpoint(FifoEndpointConfigIn $fifoEndpoint): DestinationInConfig\FifoEndpoint
    {
        return new DestinationInConfig\FifoEndpoint($fifoEndpoint);
    }

    public static function amazonS3(S3ConfigIn $amazonS3): DestinationInConfig\AmazonS3
    {
        return new DestinationInConfig\AmazonS3($amazonS3);
    }

    public static function googleCloudStorage(GoogleCloudStorageConfigIn $googleCloudStorage): DestinationInConfig\GoogleCloudStorage
    {
        return new DestinationInConfig\GoogleCloudStorage($googleCloudStorage);
    }

    public static function googleCloudPubSub(GoogleCloudPubSubConfigIn $googleCloudPubSub): DestinationInConfig\GoogleCloudPubSub
    {
        return new DestinationInConfig\GoogleCloudPubSub($googleCloudPubSub);
    }

    public static function sqs(SqsConfigIn $sqs): DestinationInConfig\Sqs
    {
        return new DestinationInConfig\Sqs($sqs);
    }

    public static function sns(SnsConfigIn $sns): DestinationInConfig\Sns
    {
        return new DestinationInConfig\Sns($sns);
    }

    public static function bigQuery(BigQueryConfigIn $bigQuery): DestinationInConfig\BigQuery
    {
        return new DestinationInConfig\BigQuery($bigQuery);
    }

    public static function clickhouse(ClickhouseConfigIn $clickhouse): DestinationInConfig\Clickhouse
    {
        return new DestinationInConfig\Clickhouse($clickhouse);
    }

    public static function eventBridge(EventBridgeConfigIn $eventBridge): DestinationInConfig\EventBridge
    {
        return new DestinationInConfig\EventBridge($eventBridge);
    }

    public static function snowflake(SnowflakeConfigIn $snowflake): DestinationInConfig\Snowflake
    {
        return new DestinationInConfig\Snowflake($snowflake);
    }

    public static function rabbitMq(RabbitMqConfigIn $rabbitMq): DestinationInConfig\RabbitMq
    {
        return new DestinationInConfig\RabbitMq($rabbitMq);
    }

    public static function redshift(RedshiftConfigIn $redshift): DestinationInConfig\Redshift
    {
        return new DestinationInConfig\Redshift($redshift);
    }

    public static function postgres(PostgresConfigIn $postgres): DestinationInConfig\Postgres
    {
        return new DestinationInConfig\Postgres($postgres);
    }

    public static function fromTypeAndConfig(string $type, mixed $config): self
    {
        $config ??= [];

        return match ($type) {
            'pollingEndpoint' => self::pollingEndpoint(),

            'azureBlobStorage' => self::azureBlobStorage(AzureBlobStorageConfigIn::fromMixed($config)),

            'otelTracing' => self::otelTracing(OtelTracingConfigIn::fromMixed($config)),

            'fifoEndpoint' => self::fifoEndpoint(FifoEndpointConfigIn::fromMixed($config)),

            'amazonS3' => self::amazonS3(S3ConfigIn::fromMixed($config)),

            'googleCloudStorage' => self::googleCloudStorage(GoogleCloudStorageConfigIn::fromMixed($config)),

            'googleCloudPubSub' => self::googleCloudPubSub(GoogleCloudPubSubConfigIn::fromMixed($config)),

            'sqs' => self::sqs(SqsConfigIn::fromMixed($config)),

            'sns' => self::sns(SnsConfigIn::fromMixed($config)),

            'bigQuery' => self::bigQuery(BigQueryConfigIn::fromMixed($config)),

            'clickhouse' => self::clickhouse(ClickhouseConfigIn::fromMixed($config)),

            'eventBridge' => self::eventBridge(EventBridgeConfigIn::fromMixed($config)),

            'snowflake' => self::snowflake(SnowflakeConfigIn::fromMixed($config)),

            'rabbitMq' => self::rabbitMq(RabbitMqConfigIn::fromMixed($config)),

            'redshift' => self::redshift(RedshiftConfigIn::fromMixed($config)),

            'postgres' => self::postgres(PostgresConfigIn::fromMixed($config)),

            default => throw new \InvalidArgumentException("Unknown type: {$type}"),
        };
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return self::fromTypeAndConfig(
            \Svix\Utils::deserializeString($data, 'type', true, 'DestinationInConfig'),
            \Svix\Utils::getValFromJson($data, 'config', false, 'DestinationInConfig') ?? []
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
        return \Svix\Utils::newStdClassIfArrayIsEmpty($this->configPayload());
    }
}

namespace Svix\Models\DestinationInConfig;

final class PollingEndpoint extends \Svix\Models\DestinationInConfig
{
    public function variantName(): string
    {
        return 'pollingEndpoint';
    }

    public function configPayload(): mixed
    {
        return new \stdClass();
    }
}

final class AzureBlobStorage extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\AzureBlobStorageConfigIn $azureBlobStorage)
    {
    }

    public function variantName(): string
    {
        return 'azureBlobStorage';
    }

    public function configPayload(): mixed
    {
        return $this->azureBlobStorage;
    }
}

final class OtelTracing extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\OtelTracingConfigIn $otelTracing)
    {
    }

    public function variantName(): string
    {
        return 'otelTracing';
    }

    public function configPayload(): mixed
    {
        return $this->otelTracing;
    }
}

final class FifoEndpoint extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\FifoEndpointConfigIn $fifoEndpoint)
    {
    }

    public function variantName(): string
    {
        return 'fifoEndpoint';
    }

    public function configPayload(): mixed
    {
        return $this->fifoEndpoint;
    }
}

final class AmazonS3 extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\S3ConfigIn $amazonS3)
    {
    }

    public function variantName(): string
    {
        return 'amazonS3';
    }

    public function configPayload(): mixed
    {
        return $this->amazonS3;
    }
}

final class GoogleCloudStorage extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\GoogleCloudStorageConfigIn $googleCloudStorage)
    {
    }

    public function variantName(): string
    {
        return 'googleCloudStorage';
    }

    public function configPayload(): mixed
    {
        return $this->googleCloudStorage;
    }
}

final class GoogleCloudPubSub extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\GoogleCloudPubSubConfigIn $googleCloudPubSub)
    {
    }

    public function variantName(): string
    {
        return 'googleCloudPubSub';
    }

    public function configPayload(): mixed
    {
        return $this->googleCloudPubSub;
    }
}

final class Sqs extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\SqsConfigIn $sqs)
    {
    }

    public function variantName(): string
    {
        return 'sqs';
    }

    public function configPayload(): mixed
    {
        return $this->sqs;
    }
}

final class Sns extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\SnsConfigIn $sns)
    {
    }

    public function variantName(): string
    {
        return 'sns';
    }

    public function configPayload(): mixed
    {
        return $this->sns;
    }
}

final class BigQuery extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\BigQueryConfigIn $bigQuery)
    {
    }

    public function variantName(): string
    {
        return 'bigQuery';
    }

    public function configPayload(): mixed
    {
        return $this->bigQuery;
    }
}

final class Clickhouse extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\ClickhouseConfigIn $clickhouse)
    {
    }

    public function variantName(): string
    {
        return 'clickhouse';
    }

    public function configPayload(): mixed
    {
        return $this->clickhouse;
    }
}

final class EventBridge extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\EventBridgeConfigIn $eventBridge)
    {
    }

    public function variantName(): string
    {
        return 'eventBridge';
    }

    public function configPayload(): mixed
    {
        return $this->eventBridge;
    }
}

final class Snowflake extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\SnowflakeConfigIn $snowflake)
    {
    }

    public function variantName(): string
    {
        return 'snowflake';
    }

    public function configPayload(): mixed
    {
        return $this->snowflake;
    }
}

final class RabbitMq extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\RabbitMqConfigIn $rabbitMq)
    {
    }

    public function variantName(): string
    {
        return 'rabbitMq';
    }

    public function configPayload(): mixed
    {
        return $this->rabbitMq;
    }
}

final class Redshift extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\RedshiftConfigIn $redshift)
    {
    }

    public function variantName(): string
    {
        return 'redshift';
    }

    public function configPayload(): mixed
    {
        return $this->redshift;
    }
}

final class Postgres extends \Svix\Models\DestinationInConfig
{
    public function __construct(public readonly \Svix\Models\PostgresConfigIn $postgres)
    {
    }

    public function variantName(): string
    {
        return 'postgres';
    }

    public function configPayload(): mixed
    {
        return $this->postgres;
    }
}
