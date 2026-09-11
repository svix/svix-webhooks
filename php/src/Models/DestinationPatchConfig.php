<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

abstract class DestinationPatchConfig implements \JsonSerializable
{
    abstract public function variantName(): string;

    abstract public function configPayload(): mixed;

    public static function pollingEndpoint(): DestinationPatchConfig\PollingEndpoint
    {
        return new DestinationPatchConfig\PollingEndpoint();
    }

    public static function azureBlobStorage(AzureBlobStorageConfigPatch $azureBlobStorage): DestinationPatchConfig\AzureBlobStorage
    {
        return new DestinationPatchConfig\AzureBlobStorage($azureBlobStorage);
    }

    public static function otelTracing(OtelTracingConfigPatch $otelTracing): DestinationPatchConfig\OtelTracing
    {
        return new DestinationPatchConfig\OtelTracing($otelTracing);
    }

    public static function fifoEndpoint(SinkHttpConfigPatch $fifoEndpoint): DestinationPatchConfig\FifoEndpoint
    {
        return new DestinationPatchConfig\FifoEndpoint($fifoEndpoint);
    }

    public static function amazonS3(S3ConfigPatch $amazonS3): DestinationPatchConfig\AmazonS3
    {
        return new DestinationPatchConfig\AmazonS3($amazonS3);
    }

    public static function googleCloudStorage(GoogleCloudStorageConfigPatch $googleCloudStorage): DestinationPatchConfig\GoogleCloudStorage
    {
        return new DestinationPatchConfig\GoogleCloudStorage($googleCloudStorage);
    }

    public static function googleCloudPubSub(GoogleCloudPubSubConfigPatch $googleCloudPubSub): DestinationPatchConfig\GoogleCloudPubSub
    {
        return new DestinationPatchConfig\GoogleCloudPubSub($googleCloudPubSub);
    }

    public static function sqs(SqsConfigPatch $sqs): DestinationPatchConfig\Sqs
    {
        return new DestinationPatchConfig\Sqs($sqs);
    }

    public static function sns(SnsConfigPatch $sns): DestinationPatchConfig\Sns
    {
        return new DestinationPatchConfig\Sns($sns);
    }

    public static function bigQuery(BigQueryConfigPatch $bigQuery): DestinationPatchConfig\BigQuery
    {
        return new DestinationPatchConfig\BigQuery($bigQuery);
    }

    public static function clickhouse(ClickhouseConfigPatch $clickhouse): DestinationPatchConfig\Clickhouse
    {
        return new DestinationPatchConfig\Clickhouse($clickhouse);
    }

    public static function eventBridge(EventBridgeConfigPatch $eventBridge): DestinationPatchConfig\EventBridge
    {
        return new DestinationPatchConfig\EventBridge($eventBridge);
    }

    public static function snowflake(SnowflakeConfigPatch $snowflake): DestinationPatchConfig\Snowflake
    {
        return new DestinationPatchConfig\Snowflake($snowflake);
    }

    public static function rabbitMq(RabbitMqConfigPatch $rabbitMq): DestinationPatchConfig\RabbitMq
    {
        return new DestinationPatchConfig\RabbitMq($rabbitMq);
    }

    public static function redshift(RedshiftConfigPatch $redshift): DestinationPatchConfig\Redshift
    {
        return new DestinationPatchConfig\Redshift($redshift);
    }

    public static function postgres(PostgresConfigPatch $postgres): DestinationPatchConfig\Postgres
    {
        return new DestinationPatchConfig\Postgres($postgres);
    }

    public static function fromTypeAndConfig(string $type, mixed $config): self
    {
        $config ??= [];

        return match ($type) {
            'pollingEndpoint' => self::pollingEndpoint(),

            'azureBlobStorage' => self::azureBlobStorage(AzureBlobStorageConfigPatch::fromMixed($config)),

            'otelTracing' => self::otelTracing(OtelTracingConfigPatch::fromMixed($config)),

            'fifoEndpoint' => self::fifoEndpoint(SinkHttpConfigPatch::fromMixed($config)),

            'amazonS3' => self::amazonS3(S3ConfigPatch::fromMixed($config)),

            'googleCloudStorage' => self::googleCloudStorage(GoogleCloudStorageConfigPatch::fromMixed($config)),

            'googleCloudPubSub' => self::googleCloudPubSub(GoogleCloudPubSubConfigPatch::fromMixed($config)),

            'sqs' => self::sqs(SqsConfigPatch::fromMixed($config)),

            'sns' => self::sns(SnsConfigPatch::fromMixed($config)),

            'bigQuery' => self::bigQuery(BigQueryConfigPatch::fromMixed($config)),

            'clickhouse' => self::clickhouse(ClickhouseConfigPatch::fromMixed($config)),

            'eventBridge' => self::eventBridge(EventBridgeConfigPatch::fromMixed($config)),

            'snowflake' => self::snowflake(SnowflakeConfigPatch::fromMixed($config)),

            'rabbitMq' => self::rabbitMq(RabbitMqConfigPatch::fromMixed($config)),

            'redshift' => self::redshift(RedshiftConfigPatch::fromMixed($config)),

            'postgres' => self::postgres(PostgresConfigPatch::fromMixed($config)),

            default => throw new \InvalidArgumentException("Unknown type: {$type}"),
        };
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return self::fromTypeAndConfig(
            \Svix\Utils::deserializeString($data, 'type', true, 'DestinationPatchConfig'),
            \Svix\Utils::getValFromJson($data, 'config', false, 'DestinationPatchConfig') ?? []
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

namespace Svix\Models\DestinationPatchConfig;

final class PollingEndpoint extends \Svix\Models\DestinationPatchConfig
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

final class AzureBlobStorage extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\AzureBlobStorageConfigPatch $azureBlobStorage)
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

final class OtelTracing extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\OtelTracingConfigPatch $otelTracing)
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

final class FifoEndpoint extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\SinkHttpConfigPatch $fifoEndpoint)
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

final class AmazonS3 extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\S3ConfigPatch $amazonS3)
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

final class GoogleCloudStorage extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\GoogleCloudStorageConfigPatch $googleCloudStorage)
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

final class GoogleCloudPubSub extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\GoogleCloudPubSubConfigPatch $googleCloudPubSub)
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

final class Sqs extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\SqsConfigPatch $sqs)
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

final class Sns extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\SnsConfigPatch $sns)
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

final class BigQuery extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\BigQueryConfigPatch $bigQuery)
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

final class Clickhouse extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\ClickhouseConfigPatch $clickhouse)
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

final class EventBridge extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\EventBridgeConfigPatch $eventBridge)
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

final class Snowflake extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\SnowflakeConfigPatch $snowflake)
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

final class RabbitMq extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\RabbitMqConfigPatch $rabbitMq)
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

final class Redshift extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\RedshiftConfigPatch $redshift)
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

final class Postgres extends \Svix\Models\DestinationPatchConfig
{
    public function __construct(public readonly \Svix\Models\PostgresConfigPatch $postgres)
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
