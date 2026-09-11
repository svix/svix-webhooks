<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** The destination's type and type-specific configuration. */
class DestinationIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null              $uid    an optional unique identifier for the destination
     * @param DestinationStatusIn|null $status Whether the destination will receive events.
     *
     * If the destination is `enabled`, events sent to the application will be dispatched to the destination in order.
     *
     * If the destination is `disabled`, events will not be dispatched until the destination is reenabled.
     * @param int|null $batchSize   how many events will be batched in a request to the destination
     * @param int|null $maxWaitSecs How long to wait before a batch of events is sent, if the `batchSize` is not reached.
     *
     * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, a request is sent after 10 seconds or 100 events, whichever comes first.
     *
     * Note that an empty batch is never sent to the destination.
     * @param list<string>|null          $eventTypes A list of event types that filter which events are dispatched to the destination. An empty list (or null) will not filter out any events.
     * @param list<string>|null          $channels   A list of channels that filter which events are dispatched to the destination. An empty list (or null) will not filter out any events.
     * @param array<string, string>|null $metadata
     */
    private function __construct(
        public readonly DestinationInConfig $config,
        public readonly ?string $uid = null,
        public readonly ?DestinationStatusIn $status = null,
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
     * Create an instance of DestinationIn with required fields.
     */
    public static function create(
        DestinationInConfig $config,
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

    public function withStatus(?DestinationStatusIn $status): self
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
        if (null !== $this->status) {
            $data['status'] = $this->status;
        }
        if (null !== $this->batchSize) {
            $data['batchSize'] = $this->batchSize;
        }
        if (null !== $this->maxWaitSecs) {
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
            config: DestinationInConfig::fromTypeAndConfig(
                \Svix\Utils::deserializeString($data, 'type', true, 'DestinationIn'),
                \Svix\Utils::getValFromJson($data, 'config', false, 'DestinationIn') ?? []
            ),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'DestinationIn'),

            status: \Svix\Utils::deserializeObject($data, 'status', false, 'DestinationIn', [DestinationStatusIn::class, 'fromMixed']),

            batchSize: \Svix\Utils::deserializeInt($data, 'batchSize', false, 'DestinationIn'),

            maxWaitSecs: \Svix\Utils::deserializeInt($data, 'maxWaitSecs', false, 'DestinationIn'),

            eventTypes: \Svix\Utils::getValFromJson($data, 'eventTypes', false, 'DestinationIn'),

            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'DestinationIn'),

            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'DestinationIn')
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
