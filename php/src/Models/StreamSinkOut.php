<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamSinkOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                $id         the sink's ID
     * @param string|null           $uid        the sink's UID
     * @param list<string>|null     $eventTypes
     * @param list<string>|null     $channels
     * @param array<string, string> $metadata
     */
    private function __construct(
        public readonly StreamSinkOutConfig $config,
        public readonly string $id,
        public readonly SinkStatus $status,
        public readonly string $currentIterator,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly int $batchSize,
        public readonly int $maxWaitSecs,
        public readonly array $metadata,
        public readonly ?string $uid = null,
        public readonly ?string $failureReason = null,
        public readonly ?array $eventTypes = null,
        public readonly ?array $channels = null,
        public readonly ?\DateTimeImmutable $nextRetryAt = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamSinkOut with required fields.
     */
    public static function create(
        StreamSinkOutConfig $config,
        string $id,
        SinkStatus $status,
        string $currentIterator,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
        int $batchSize,
        int $maxWaitSecs,
        array $metadata,
    ): self {
        return new self(
            config: $config,
            id: $id,
            uid: null,
            status: $status,
            currentIterator: $currentIterator,
            failureReason: null,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            batchSize: $batchSize,
            maxWaitSecs: $maxWaitSecs,
            eventTypes: null,
            channels: null,
            nextRetryAt: null,
            metadata: $metadata,
            setFields: [
                'config' => true, 'id' => true, 'status' => true, 'currentIterator' => true, 'createdAt' => true, 'updatedAt' => true, 'batchSize' => true, 'maxWaitSecs' => true, 'metadata' => true, ]
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            config: $this->config,
            id: $this->id,
            uid: $uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $this->failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            nextRetryAt: $this->nextRetryAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withFailureReason(?string $failureReason): self
    {
        $setFields = $this->setFields;
        $setFields['failureReason'] = true;

        return new self(
            config: $this->config,
            id: $this->id,
            uid: $this->uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            nextRetryAt: $this->nextRetryAt,
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
            id: $this->id,
            uid: $this->uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $this->failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $eventTypes,
            channels: $this->channels,
            nextRetryAt: $this->nextRetryAt,
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
            id: $this->id,
            uid: $this->uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $this->failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $channels,
            nextRetryAt: $this->nextRetryAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withNextRetryAt(?\DateTimeImmutable $nextRetryAt): self
    {
        $setFields = $this->setFields;
        $setFields['nextRetryAt'] = true;

        return new self(
            config: $this->config,
            id: $this->id,
            uid: $this->uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $this->failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            nextRetryAt: $nextRetryAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'type' => $this->config->variantName(),
            'config' => $this->config,
            'id' => $this->id,
            'status' => $this->status,
            'currentIterator' => $this->currentIterator,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c'),
            'batchSize' => $this->batchSize,
            'maxWaitSecs' => $this->maxWaitSecs,
            'metadata' => $this->metadata,
        ];

        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (isset($this->setFields['failureReason'])) {
            $data['failureReason'] = $this->failureReason;
        }
        if (isset($this->setFields['nextRetryAt'])) {
            $data['nextRetryAt'] = $this->nextRetryAt->format('c');
        }
        if (null !== $this->eventTypes) {
            $data['eventTypes'] = $this->eventTypes;
        }
        if (null !== $this->channels) {
            $data['channels'] = $this->channels;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            config: StreamSinkOutConfig::fromTypeAndConfig(
                \Svix\Utils::deserializeString($data, 'type', true, 'StreamSinkOut'),
                \Svix\Utils::getValFromJson($data, 'config', false, 'StreamSinkOut') ?? []
            ),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'StreamSinkOut'),

            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'StreamSinkOut'),

            status: \Svix\Utils::deserializeObject($data, 'status', true, 'StreamSinkOut', [SinkStatus::class, 'fromMixed']),

            currentIterator: \Svix\Utils::deserializeString($data, 'currentIterator', true, 'StreamSinkOut'),

            failureReason: \Svix\Utils::deserializeString($data, 'failureReason', false, 'StreamSinkOut'),

            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'StreamSinkOut'),

            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'StreamSinkOut'),

            batchSize: \Svix\Utils::deserializeInt($data, 'batchSize', true, 'StreamSinkOut'),

            maxWaitSecs: \Svix\Utils::deserializeInt($data, 'maxWaitSecs', true, 'StreamSinkOut'),

            eventTypes: \Svix\Utils::getValFromJson($data, 'eventTypes', false, 'StreamSinkOut'),

            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'StreamSinkOut'),

            nextRetryAt: \Svix\Utils::deserializeDt($data, 'nextRetryAt', false, 'StreamSinkOut'),

            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'StreamSinkOut')
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

abstract class StreamSinkOutConfig implements \JsonSerializable
{
    abstract public function variantName(): string;

    abstract public function configPayload(): mixed;

    public static function poller(): StreamSinkOutConfig\Poller
    {
        return new StreamSinkOutConfig\Poller();
    }

    public static function pollingEndpoint(): StreamSinkOutConfig\PollingEndpoint
    {
        return new StreamSinkOutConfig\PollingEndpoint();
    }

    public static function azureBlobStorage(AzureBlobStorageConfigOut $azureBlobStorage): StreamSinkOutConfig\AzureBlobStorage
    {
        return new StreamSinkOutConfig\AzureBlobStorage($azureBlobStorage);
    }

    public static function otelTracing(OtelTracingConfigOut $otelTracing): StreamSinkOutConfig\OtelTracing
    {
        return new StreamSinkOutConfig\OtelTracing($otelTracing);
    }

    public static function http(SinkHttpConfigOut $http): StreamSinkOutConfig\Http
    {
        return new StreamSinkOutConfig\Http($http);
    }

    public static function amazonS3(S3ConfigOut $amazonS3): StreamSinkOutConfig\AmazonS3
    {
        return new StreamSinkOutConfig\AmazonS3($amazonS3);
    }

    public static function snowflake(SnowflakeConfigOut $snowflake): StreamSinkOutConfig\Snowflake
    {
        return new StreamSinkOutConfig\Snowflake($snowflake);
    }

    public static function googleCloudStorage(GoogleCloudStorageConfigOut $googleCloudStorage): StreamSinkOutConfig\GoogleCloudStorage
    {
        return new StreamSinkOutConfig\GoogleCloudStorage($googleCloudStorage);
    }

    public static function googleCloudPubSub(GoogleCloudPubSubConfigOut $googleCloudPubSub): StreamSinkOutConfig\GoogleCloudPubSub
    {
        return new StreamSinkOutConfig\GoogleCloudPubSub($googleCloudPubSub);
    }

    public static function redshift(RedshiftConfigOut $redshift): StreamSinkOutConfig\Redshift
    {
        return new StreamSinkOutConfig\Redshift($redshift);
    }

    public static function bigQuery(BigQueryConfigOut $bigQuery): StreamSinkOutConfig\BigQuery
    {
        return new StreamSinkOutConfig\BigQuery($bigQuery);
    }

    public static function clickhouse(ClickhouseConfigOut $clickhouse): StreamSinkOutConfig\Clickhouse
    {
        return new StreamSinkOutConfig\Clickhouse($clickhouse);
    }

    public static function rabbitMq(RabbitMqConfigOut $rabbitMq): StreamSinkOutConfig\RabbitMq
    {
        return new StreamSinkOutConfig\RabbitMq($rabbitMq);
    }

    public static function sqs(SqsConfigOut $sqs): StreamSinkOutConfig\Sqs
    {
        return new StreamSinkOutConfig\Sqs($sqs);
    }

    public static function eventBridge(EventBridgeConfigOut $eventBridge): StreamSinkOutConfig\EventBridge
    {
        return new StreamSinkOutConfig\EventBridge($eventBridge);
    }

    public static function sns(SnsConfigOut $sns): StreamSinkOutConfig\Sns
    {
        return new StreamSinkOutConfig\Sns($sns);
    }

    public static function postgres(PostgresConfigOut $postgres): StreamSinkOutConfig\Postgres
    {
        return new StreamSinkOutConfig\Postgres($postgres);
    }

    public static function fromTypeAndConfig(string $type, mixed $config): self
    {
        $config ??= [];

        return match ($type) {
            'poller' => self::poller(),
            'pollingEndpoint' => self::pollingEndpoint(),
            'azureBlobStorage' => self::azureBlobStorage(AzureBlobStorageConfigOut::fromMixed($config)),
            'otelTracing' => self::otelTracing(OtelTracingConfigOut::fromMixed($config)),
            'http' => self::http(SinkHttpConfigOut::fromMixed($config)),
            'amazonS3' => self::amazonS3(S3ConfigOut::fromMixed($config)),
            'snowflake' => self::snowflake(SnowflakeConfigOut::fromMixed($config)),
            'googleCloudStorage' => self::googleCloudStorage(GoogleCloudStorageConfigOut::fromMixed($config)),
            'googleCloudPubSub' => self::googleCloudPubSub(GoogleCloudPubSubConfigOut::fromMixed($config)),
            'redshift' => self::redshift(RedshiftConfigOut::fromMixed($config)),
            'bigQuery' => self::bigQuery(BigQueryConfigOut::fromMixed($config)),
            'clickhouse' => self::clickhouse(ClickhouseConfigOut::fromMixed($config)),
            'rabbitMq' => self::rabbitMq(RabbitMqConfigOut::fromMixed($config)),
            'sqs' => self::sqs(SqsConfigOut::fromMixed($config)),
            'eventBridge' => self::eventBridge(EventBridgeConfigOut::fromMixed($config)),
            'sns' => self::sns(SnsConfigOut::fromMixed($config)),
            'postgres' => self::postgres(PostgresConfigOut::fromMixed($config)),
            default => throw new \InvalidArgumentException("Unknown type: {$type}"),
        };
    }

    public function jsonSerialize(): mixed
    {
        return \Svix\Utils::newStdClassIfArrayIsEmpty($this->configPayload());
    }
}

namespace Svix\Models\StreamSinkOutConfig;

final class Poller extends \Svix\Models\StreamSinkOutConfig
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

final class PollingEndpoint extends \Svix\Models\StreamSinkOutConfig
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

final class AzureBlobStorage extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\AzureBlobStorageConfigOut $azureBlobStorage)
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

final class OtelTracing extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\OtelTracingConfigOut $otelTracing)
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

final class Http extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\SinkHttpConfigOut $http)
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

final class AmazonS3 extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\S3ConfigOut $amazonS3)
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

final class Snowflake extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\SnowflakeConfigOut $snowflake)
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

final class GoogleCloudStorage extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\GoogleCloudStorageConfigOut $googleCloudStorage)
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

final class GoogleCloudPubSub extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\GoogleCloudPubSubConfigOut $googleCloudPubSub)
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

final class Redshift extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\RedshiftConfigOut $redshift)
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

final class BigQuery extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\BigQueryConfigOut $bigQuery)
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

final class Clickhouse extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\ClickhouseConfigOut $clickhouse)
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

final class RabbitMq extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\RabbitMqConfigOut $rabbitMq)
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

final class Sqs extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\SqsConfigOut $sqs)
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

final class EventBridge extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\EventBridgeConfigOut $eventBridge)
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

final class Sns extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\SnsConfigOut $sns)
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

final class Postgres extends \Svix\Models\StreamSinkOutConfig
{
    public function __construct(public readonly \Svix\Models\PostgresConfigOut $postgres)
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
