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
