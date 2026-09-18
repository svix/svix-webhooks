<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamSinkIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null       $uid    an optional unique identifier for the sink
     * @param SinkStatusIn|null $status Whether the sink will receive events.
     *
     * If the sink is `enabled`, any events posted to the stream will be
     * dispatched to the Sink in the same order that events were posted to the stream.
     *
     * If the sink is `disabled`, events will not be dispatched to the sink until the sink is reenabled.
     * @param int|null $batchSize   how many events will be batched in a request to the Sink
     * @param int|null $maxWaitSecs How long to wait before a batch of events is sent, if the `batchSize` is not reached.
     *
     * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request after 10 seconds or 100 events, whichever comes first.
     *
     * Note that we will never send an empty batch of events to the Sink.
     * @param list<string>|null          $eventTypes A list of event types that filter which events are dispatched to the Sink. An empty list (or null) will not filter out
     *                                               any events.
     * @param list<string>|null          $channels
     * @param array<string, string>|null $metadata
     */
    private function __construct(
        public readonly StreamSinkInConfig $config,
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
     * Create an instance of StreamSinkIn with required fields.
     */
    public static function create(
        StreamSinkInConfig $config,
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
            config: StreamSinkInConfig::fromTypeAndConfig(
                \Svix\Utils::deserializeString($data, 'type', true, 'StreamSinkIn'),
                \Svix\Utils::getValFromJson($data, 'config', false, 'StreamSinkIn') ?? []
            ),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'StreamSinkIn'),

            status: \Svix\Utils::deserializeObject($data, 'status', false, 'StreamSinkIn', [SinkStatusIn::class, 'fromMixed']),

            batchSize: \Svix\Utils::deserializeInt($data, 'batchSize', false, 'StreamSinkIn'),

            maxWaitSecs: \Svix\Utils::deserializeInt($data, 'maxWaitSecs', false, 'StreamSinkIn'),

            eventTypes: \Svix\Utils::getValFromJson($data, 'eventTypes', false, 'StreamSinkIn'),

            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'StreamSinkIn'),

            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'StreamSinkIn')
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

abstract class StreamSinkInConfig implements \JsonSerializable
{
    abstract public function variantName(): string;

    abstract public function configPayload(): mixed;

    public static function poller(): StreamSinkInConfig\Poller
    {
        return new StreamSinkInConfig\Poller();
    }

    public static function azureBlobStorage(AzureBlobStorageConfigIn $azureBlobStorage): StreamSinkInConfig\AzureBlobStorage
    {
        return new StreamSinkInConfig\AzureBlobStorage($azureBlobStorage);
    }

    public static function otelTracing(OtelTracingConfigIn $otelTracing): StreamSinkInConfig\OtelTracing
    {
        return new StreamSinkInConfig\OtelTracing($otelTracing);
    }

    public static function http(SinkHttpConfigIn $http): StreamSinkInConfig\Http
    {
        return new StreamSinkInConfig\Http($http);
    }

    public static function amazonS3(S3ConfigIn $amazonS3): StreamSinkInConfig\AmazonS3
    {
        return new StreamSinkInConfig\AmazonS3($amazonS3);
    }

    public static function googleCloudStorage(GoogleCloudStorageConfigIn $googleCloudStorage): StreamSinkInConfig\GoogleCloudStorage
    {
        return new StreamSinkInConfig\GoogleCloudStorage($googleCloudStorage);
    }

    public static function googleCloudPubSub(GoogleCloudPubSubConfigIn $googleCloudPubSub): StreamSinkInConfig\GoogleCloudPubSub
    {
        return new StreamSinkInConfig\GoogleCloudPubSub($googleCloudPubSub);
    }

    public static function sqs(SqsConfigIn $sqs): StreamSinkInConfig\Sqs
    {
        return new StreamSinkInConfig\Sqs($sqs);
    }

    public static function sns(SnsConfigIn $sns): StreamSinkInConfig\Sns
    {
        return new StreamSinkInConfig\Sns($sns);
    }

    public static function bigQuery(BigQueryConfigIn $bigQuery): StreamSinkInConfig\BigQuery
    {
        return new StreamSinkInConfig\BigQuery($bigQuery);
    }

    public static function clickhouse(ClickhouseConfigIn $clickhouse): StreamSinkInConfig\Clickhouse
    {
        return new StreamSinkInConfig\Clickhouse($clickhouse);
    }

    public static function eventBridge(EventBridgeConfigIn $eventBridge): StreamSinkInConfig\EventBridge
    {
        return new StreamSinkInConfig\EventBridge($eventBridge);
    }

    public static function snowflake(SnowflakeConfigIn $snowflake): StreamSinkInConfig\Snowflake
    {
        return new StreamSinkInConfig\Snowflake($snowflake);
    }

    public static function rabbitMq(RabbitMqConfigIn $rabbitMq): StreamSinkInConfig\RabbitMq
    {
        return new StreamSinkInConfig\RabbitMq($rabbitMq);
    }

    public static function redshift(RedshiftConfigIn $redshift): StreamSinkInConfig\Redshift
    {
        return new StreamSinkInConfig\Redshift($redshift);
    }

    public static function postgres(PostgresConfigIn $postgres): StreamSinkInConfig\Postgres
    {
        return new StreamSinkInConfig\Postgres($postgres);
    }

    public static function fromTypeAndConfig(string $type, mixed $config): self
    {
        $config ??= [];

        return match ($type) {
            'poller' => self::poller(),
            'azureBlobStorage' => self::azureBlobStorage(AzureBlobStorageConfigIn::fromMixed($config)),
            'otelTracing' => self::otelTracing(OtelTracingConfigIn::fromMixed($config)),
            'http' => self::http(SinkHttpConfigIn::fromMixed($config)),
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

namespace Svix\Models\StreamSinkInConfig;

final class Poller extends \Svix\Models\StreamSinkInConfig
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

final class AzureBlobStorage extends \Svix\Models\StreamSinkInConfig
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

final class OtelTracing extends \Svix\Models\StreamSinkInConfig
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

final class Http extends \Svix\Models\StreamSinkInConfig
{
    public function __construct(public readonly \Svix\Models\SinkHttpConfigIn $http)
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

final class AmazonS3 extends \Svix\Models\StreamSinkInConfig
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

final class GoogleCloudStorage extends \Svix\Models\StreamSinkInConfig
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

final class GoogleCloudPubSub extends \Svix\Models\StreamSinkInConfig
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

final class Sqs extends \Svix\Models\StreamSinkInConfig
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

final class Sns extends \Svix\Models\StreamSinkInConfig
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

final class BigQuery extends \Svix\Models\StreamSinkInConfig
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

final class Clickhouse extends \Svix\Models\StreamSinkInConfig
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

final class EventBridge extends \Svix\Models\StreamSinkInConfig
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

final class Snowflake extends \Svix\Models\StreamSinkInConfig
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

final class RabbitMq extends \Svix\Models\StreamSinkInConfig
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

final class Redshift extends \Svix\Models\StreamSinkInConfig
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

final class Postgres extends \Svix\Models\StreamSinkInConfig
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
