<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class MessageIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param ApplicationIn|null $application Optionally creates a new application alongside the message.
     *
     * If the application id or uid that is used in the path already exists, this argument is ignored.
     * @param list<string>|null       $channels  List of free-form identifiers that endpoints can filter by
     * @param \DateTimeImmutable|null $deliverAt The date and time at which the message will be delivered.
     *
     * Note that this time is best-effort-only. Must be at least one minute and no more than 24 hours in the future.
     * @param string|null $eventId   Optional unique identifier for the message
     * @param string      $eventType The event type's name
     * @param array       $payload   JSON payload to send as the request body of the webhook.
     *
     * We also support sending non-JSON payloads. Please contact us for more information.
     * @param int|null          $payloadRetentionHours  Optional number of hours to retain the message payload. Note that this is mutually exclusive with `payloadRetentionPeriod`.
     * @param int|null          $payloadRetentionPeriod Optional number of days to retain the message payload. Defaults to 90. Note that this is mutually exclusive with `payloadRetentionHours`.
     * @param list<string>|null $tags                   List of free-form tags that can be filtered by when listing messages
     * @param array|null        $transformationsParams  Extra parameters to pass to Transformations (for future use)
     */
    private function __construct(
        public readonly string $eventType,
        public readonly array $payload,
        public readonly ?ApplicationIn $application = null,
        public readonly ?array $channels = null,
        public readonly ?\DateTimeImmutable $deliverAt = null,
        public readonly ?string $eventId = null,
        public readonly ?int $payloadRetentionHours = null,
        public readonly ?int $payloadRetentionPeriod = null,
        public readonly ?array $tags = null,
        public readonly ?array $transformationsParams = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessageIn with required fields.
     */
    public static function create(
        string $eventType,
        array $payload,
    ): self {
        return new self(
            application: null,
            channels: null,
            deliverAt: null,
            eventId: null,
            eventType: $eventType,
            payload: $payload,
            payloadRetentionHours: null,
            payloadRetentionPeriod: null,
            tags: null,
            transformationsParams: null,
            setFields: ['eventType' => true, 'payload' => true]
        );
    }

    public function withApplication(?ApplicationIn $application): self
    {
        $setFields = $this->setFields;
        $setFields['application'] = true;

        return new self(
            application: $application,
            channels: $this->channels,
            deliverAt: $this->deliverAt,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            payloadRetentionHours: $this->payloadRetentionHours,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            setFields: $setFields
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            application: $this->application,
            channels: $channels,
            deliverAt: $this->deliverAt,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            payloadRetentionHours: $this->payloadRetentionHours,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            setFields: $setFields
        );
    }

    public function withDeliverAt(?\DateTimeImmutable $deliverAt): self
    {
        $setFields = $this->setFields;
        $setFields['deliverAt'] = true;

        return new self(
            application: $this->application,
            channels: $this->channels,
            deliverAt: $deliverAt,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            payloadRetentionHours: $this->payloadRetentionHours,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            setFields: $setFields
        );
    }

    public function withEventId(?string $eventId): self
    {
        $setFields = $this->setFields;
        $setFields['eventId'] = true;

        return new self(
            application: $this->application,
            channels: $this->channels,
            deliverAt: $this->deliverAt,
            eventId: $eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            payloadRetentionHours: $this->payloadRetentionHours,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            setFields: $setFields
        );
    }

    public function withPayloadRetentionHours(?int $payloadRetentionHours): self
    {
        $setFields = $this->setFields;
        $setFields['payloadRetentionHours'] = true;

        return new self(
            application: $this->application,
            channels: $this->channels,
            deliverAt: $this->deliverAt,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            payloadRetentionHours: $payloadRetentionHours,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            setFields: $setFields
        );
    }

    public function withPayloadRetentionPeriod(?int $payloadRetentionPeriod): self
    {
        $setFields = $this->setFields;
        $setFields['payloadRetentionPeriod'] = true;

        return new self(
            application: $this->application,
            channels: $this->channels,
            deliverAt: $this->deliverAt,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            payloadRetentionHours: $this->payloadRetentionHours,
            payloadRetentionPeriod: $payloadRetentionPeriod,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            setFields: $setFields
        );
    }

    public function withTags(?array $tags): self
    {
        $setFields = $this->setFields;
        $setFields['tags'] = true;

        return new self(
            application: $this->application,
            channels: $this->channels,
            deliverAt: $this->deliverAt,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            payloadRetentionHours: $this->payloadRetentionHours,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            tags: $tags,
            transformationsParams: $this->transformationsParams,
            setFields: $setFields
        );
    }

    public function withTransformationsParams(?array $transformationsParams): self
    {
        $setFields = $this->setFields;
        $setFields['transformationsParams'] = true;

        return new self(
            application: $this->application,
            channels: $this->channels,
            deliverAt: $this->deliverAt,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            payloadRetentionHours: $this->payloadRetentionHours,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            tags: $this->tags,
            transformationsParams: $transformationsParams,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'eventType' => $this->eventType,
            'payload' => $this->payload];

        if (isset($this->setFields['application'])) {
            $data['application'] = $this->application;
        }
        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }
        if (isset($this->setFields['deliverAt'])) {
            $data['deliverAt'] = $this->deliverAt->format('c');
        }
        if (isset($this->setFields['eventId'])) {
            $data['eventId'] = $this->eventId;
        }
        if (isset($this->setFields['payloadRetentionHours'])) {
            $data['payloadRetentionHours'] = $this->payloadRetentionHours;
        }
        if (isset($this->setFields['payloadRetentionPeriod'])) {
            $data['payloadRetentionPeriod'] = $this->payloadRetentionPeriod;
        }
        if (isset($this->setFields['tags'])) {
            $data['tags'] = $this->tags;
        }
        if (isset($this->setFields['transformationsParams'])) {
            $data['transformationsParams'] = $this->transformationsParams;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            application: \Svix\Utils::deserializeObject($data, 'application', false, 'MessageIn', [ApplicationIn::class, 'fromMixed']),
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'MessageIn'),
            deliverAt: \Svix\Utils::deserializeDt($data, 'deliverAt', false, 'MessageIn'),
            eventId: \Svix\Utils::deserializeString($data, 'eventId', false, 'MessageIn'),
            eventType: \Svix\Utils::deserializeString($data, 'eventType', true, 'MessageIn'),
            payload: \Svix\Utils::getValFromJson($data, 'payload', true, 'MessageIn'),
            payloadRetentionHours: \Svix\Utils::deserializeInt($data, 'payloadRetentionHours', false, 'MessageIn'),
            payloadRetentionPeriod: \Svix\Utils::deserializeInt($data, 'payloadRetentionPeriod', false, 'MessageIn'),
            tags: \Svix\Utils::getValFromJson($data, 'tags', false, 'MessageIn'),
            transformationsParams: \Svix\Utils::getValFromJson($data, 'transformationsParams', false, 'MessageIn')
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
