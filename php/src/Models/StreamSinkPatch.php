<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamSinkPatch implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null                $uid        the StreamSink's UID
     * @param list<string>|null          $eventTypes
     * @param list<string>|null          $channels
     * @param array<string, string>|null $metadata
     */
    private function __construct(
        public readonly StreamSinkPatchConfig $config,
        public readonly ?string $uid = null,
        public readonly ?SinkStatusIn $status = null,
        public readonly ?int $batchSize = null,
        public readonly ?int $maxWaitSecs = null,
        public readonly ?array $eventTypes = null,
        public readonly ?array $channels = null,
        public readonly ?array $metadata = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamSinkPatch with required fields.
     */
    public static function create(
        StreamSinkPatchConfig $config,
    ): self {
        return new self(
            config: $config,
            uid: null,
            status: null,
            batchSize: null,
            maxWaitSecs: null,
            eventTypes: null,
            channels: null,
            metadata: null,
            setFields: [
                'config' => true, ]
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            config: $this->config,
            uid: $uid,
            status: $this->status,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withStatus(?SinkStatusIn $status): self
    {
        $setFields = $this->setFields;
        $setFields['status'] = true;

        return new self(
            config: $this->config,
            uid: $this->uid,
            status: $status,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withBatchSize(?int $batchSize): self
    {
        $setFields = $this->setFields;
        $setFields['batchSize'] = true;

        return new self(
            config: $this->config,
            uid: $this->uid,
            status: $this->status,
            batchSize: $batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withMaxWaitSecs(?int $maxWaitSecs): self
    {
        $setFields = $this->setFields;
        $setFields['maxWaitSecs'] = true;

        return new self(
            config: $this->config,
            uid: $this->uid,
            status: $this->status,
            batchSize: $this->batchSize,
            maxWaitSecs: $maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withEventTypes(?array $eventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['eventTypes'] = true;

        return new self(
            config: $this->config,
            uid: $this->uid,
            status: $this->status,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            config: $this->config,
            uid: $this->uid,
            status: $this->status,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            config: $this->config,
            uid: $this->uid,
            status: $this->status,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'type' => $this->config->variantName(),
            'config' => $this->config,
        ];

        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (isset($this->setFields['status'])) {
            $data['status'] = $this->status;
        }
        if (isset($this->setFields['batchSize'])) {
            $data['batchSize'] = $this->batchSize;
        }
        if (isset($this->setFields['maxWaitSecs'])) {
            $data['maxWaitSecs'] = $this->maxWaitSecs;
        }
        if (null !== $this->eventTypes) {
            $data['eventTypes'] = $this->eventTypes;
        }
        if (null !== $this->channels) {
            $data['channels'] = $this->channels;
        }
        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            config: StreamSinkPatchConfig::fromTypeAndConfig(
                \Svix\Utils::deserializeString($data, 'type', true, 'StreamSinkPatch'),
                \Svix\Utils::getValFromJson($data, 'config', false, 'StreamSinkPatch') ?? []
            ),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'StreamSinkPatch'),

            status: \Svix\Utils::deserializeObject($data, 'status', false, 'StreamSinkPatch', [SinkStatusIn::class, 'fromMixed']),

            batchSize: \Svix\Utils::deserializeInt($data, 'batchSize', false, 'StreamSinkPatch'),

            maxWaitSecs: \Svix\Utils::deserializeInt($data, 'maxWaitSecs', false, 'StreamSinkPatch'),

            eventTypes: \Svix\Utils::getValFromJson($data, 'eventTypes', false, 'StreamSinkPatch'),

            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'StreamSinkPatch'),

            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'StreamSinkPatch')
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

abstract class StreamSinkPatchConfig implements \JsonSerializable
{
    abstract public function variantName(): string;

    abstract public function configPayload(): mixed;

    public static function poller(): StreamSinkPatchConfig\Poller
    {
        return new StreamSinkPatchConfig\Poller();
    }

    public static function azureBlobStorage(AzureBlobStorageConfigPatch $azureBlobStorage): StreamSinkPatchConfig\AzureBlobStorage
    {
        return new StreamSinkPatchConfig\AzureBlobStorage($azureBlobStorage);
    }

    public static function otelTracing(OtelTracingConfigPatch $otelTracing): StreamSinkPatchConfig\OtelTracing
    {
        return new StreamSinkPatchConfig\OtelTracing($otelTracing);
    }

    public static function http(SinkHttpConfigPatch $http): StreamSinkPatchConfig\Http
    {
        return new StreamSinkPatchConfig\Http($http);
    }

    public static function amazonS3(S3ConfigPatch $amazonS3): StreamSinkPatchConfig\AmazonS3
    {
        return new StreamSinkPatchConfig\AmazonS3($amazonS3);
    }

    public static function googleCloudStorage(GoogleCloudStorageConfigPatch $googleCloudStorage): StreamSinkPatchConfig\GoogleCloudStorage
    {
        return new StreamSinkPatchConfig\GoogleCloudStorage($googleCloudStorage);
    }

    public static function googleCloudPubSub(GoogleCloudPubSubConfigPatch $googleCloudPubSub): StreamSinkPatchConfig\GoogleCloudPubSub
    {
        return new StreamSinkPatchConfig\GoogleCloudPubSub($googleCloudPubSub);
    }

    public static function sqs(SqsConfigPatch $sqs): StreamSinkPatchConfig\Sqs
    {
        return new StreamSinkPatchConfig\Sqs($sqs);
    }

    public static function sns(SnsConfigPatch $sns): StreamSinkPatchConfig\Sns
    {
        return new StreamSinkPatchConfig\Sns($sns);
    }

    public static function bigQuery(BigQueryConfigPatch $bigQuery): StreamSinkPatchConfig\BigQuery
    {
        return new StreamSinkPatchConfig\BigQuery($bigQuery);
    }

    public static function clickhouse(ClickhouseConfigPatch $clickhouse): StreamSinkPatchConfig\Clickhouse
    {
        return new StreamSinkPatchConfig\Clickhouse($clickhouse);
    }

    public static function eventBridge(EventBridgeConfigPatch $eventBridge): StreamSinkPatchConfig\EventBridge
    {
        return new StreamSinkPatchConfig\EventBridge($eventBridge);
    }

    public static function snowflake(SnowflakeConfigPatch $snowflake): StreamSinkPatchConfig\Snowflake
    {
        return new StreamSinkPatchConfig\Snowflake($snowflake);
    }

    public static function rabbitMq(RabbitMqConfigPatch $rabbitMq): StreamSinkPatchConfig\RabbitMq
    {
        return new StreamSinkPatchConfig\RabbitMq($rabbitMq);
    }

    public static function redshift(RedshiftConfigPatch $redshift): StreamSinkPatchConfig\Redshift
    {
        return new StreamSinkPatchConfig\Redshift($redshift);
    }

    public static function postgres(PostgresConfigPatch $postgres): StreamSinkPatchConfig\Postgres
    {
        return new StreamSinkPatchConfig\Postgres($postgres);
    }

    public static function fromTypeAndConfig(string $type, mixed $config): self
    {
        $config ??= [];

        return match ($type) {
            'poller' => self::poller(),
            'azureBlobStorage' => self::azureBlobStorage(AzureBlobStorageConfigPatch::fromMixed($config)),
            'otelTracing' => self::otelTracing(OtelTracingConfigPatch::fromMixed($config)),
            'http' => self::http(SinkHttpConfigPatch::fromMixed($config)),
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

    public function jsonSerialize(): mixed
    {
        return \Svix\Utils::newStdClassIfArrayIsEmpty($this->configPayload());
    }
}

namespace Svix\Models\StreamSinkPatchConfig;

final class Poller extends \Svix\Models\StreamSinkPatchConfig
{
    public function variantName(): string
    {
        return 'poller';
    }

    public function configPayload(): mixed
    {
        return new \stdClass();
    }
}

final class AzureBlobStorage extends \Svix\Models\StreamSinkPatchConfig
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

final class OtelTracing extends \Svix\Models\StreamSinkPatchConfig
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

final class Http extends \Svix\Models\StreamSinkPatchConfig
{
    public function __construct(public readonly \Svix\Models\SinkHttpConfigPatch $http)
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

final class AmazonS3 extends \Svix\Models\StreamSinkPatchConfig
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

final class GoogleCloudStorage extends \Svix\Models\StreamSinkPatchConfig
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

final class GoogleCloudPubSub extends \Svix\Models\StreamSinkPatchConfig
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

final class Sqs extends \Svix\Models\StreamSinkPatchConfig
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

final class Sns extends \Svix\Models\StreamSinkPatchConfig
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

final class BigQuery extends \Svix\Models\StreamSinkPatchConfig
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

final class Clickhouse extends \Svix\Models\StreamSinkPatchConfig
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

final class EventBridge extends \Svix\Models\StreamSinkPatchConfig
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

final class Snowflake extends \Svix\Models\StreamSinkPatchConfig
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

final class RabbitMq extends \Svix\Models\StreamSinkPatchConfig
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

final class Redshift extends \Svix\Models\StreamSinkPatchConfig
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

final class Postgres extends \Svix\Models\StreamSinkPatchConfig
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
