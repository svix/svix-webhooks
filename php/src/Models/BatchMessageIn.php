<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/**
 * A single message in a batch of bulk-created messages.
 *
 * Practically identical to `MessageIn`, but without the `application` field
 */
class BatchMessageIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $eventId Optional unique identifier for the message
     *
     * This should not contain the '.' character
     * @param string $eventType The event type's name
     * @param array  $payload   JSON payload to send as the request body of the webhook.
     *
     * We also support sending non-JSON payloads. Please contact us for more information.
     *
     * The serialized byte-length of this value should be under 1MiB. Please contact us
     * if you need to send payloads large than 1MiB.
     * @param list<string>|null       $channels              List of free-form identifiers that endpoints can filter by
     * @param list<string>|null       $tags                  List of free-form tags that can be filtered by when listing messages
     * @param array|null              $transformationsParams Extra parameters to pass to Transformations (for future use)
     * @param \DateTimeImmutable|null $deliverAt             The date and time at which the message will be delivered.
     *
     * Note that this time is best-effort-only. Must be at least one minute
     * and no more than 24 hours in the future.
     * @param int|null $payloadRetentionPeriod Optional number of days to retain the message payload. Defaults to 90.
     *                                         Note that this is mutually exclusive with `payloadRetentionHours`.
     * @param int|null $payloadRetentionHours  Optional number of hours to retain the message payload. Note that
     *                                         this is mutually exclusive with `payloadRetentionPeriod`.
     */
    private function __construct(
        public readonly string $eventType,
        public readonly array $payload,
        public readonly ?string $eventId = null,
        public readonly ?array $channels = null,
        public readonly ?array $tags = null,
        public readonly ?array $transformationsParams = null,
        public readonly ?\DateTimeImmutable $deliverAt = null,
        public readonly ?int $payloadRetentionPeriod = null,
        public readonly ?int $payloadRetentionHours = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BatchMessageIn with required fields.
     */
    public static function create(
        string $eventType,
        array $payload,
    ): self {
        return new self(
            eventId: null,
            eventType: $eventType,
            payload: $payload,
            channels: null,
            tags: null,
            transformationsParams: null,
            deliverAt: null,
            payloadRetentionPeriod: null,
            payloadRetentionHours: null,
            setFields: ['eventType' => true, 'payload' => true]
        );
    }

    public function withEventId(?string $eventId): self
    {
        $setFields = $this->setFields;
        $setFields['eventId'] = true;

        return new self(
            eventId: $eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            deliverAt: $this->deliverAt,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            payloadRetentionHours: $this->payloadRetentionHours,
            setFields: $setFields
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $channels,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            deliverAt: $this->deliverAt,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            payloadRetentionHours: $this->payloadRetentionHours,
            setFields: $setFields
        );
    }

    public function withTags(?array $tags): self
    {
        $setFields = $this->setFields;
        $setFields['tags'] = true;

        return new self(
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            tags: $tags,
            transformationsParams: $this->transformationsParams,
            deliverAt: $this->deliverAt,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            payloadRetentionHours: $this->payloadRetentionHours,
            setFields: $setFields
        );
    }

    public function withTransformationsParams(?array $transformationsParams): self
    {
        $setFields = $this->setFields;
        $setFields['transformationsParams'] = true;

        return new self(
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            tags: $this->tags,
            transformationsParams: $transformationsParams,
            deliverAt: $this->deliverAt,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            payloadRetentionHours: $this->payloadRetentionHours,
            setFields: $setFields
        );
    }

    public function withDeliverAt(?\DateTimeImmutable $deliverAt): self
    {
        $setFields = $this->setFields;
        $setFields['deliverAt'] = true;

        return new self(
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            deliverAt: $deliverAt,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            payloadRetentionHours: $this->payloadRetentionHours,
            setFields: $setFields
        );
    }

    public function withPayloadRetentionPeriod(?int $payloadRetentionPeriod): self
    {
        $setFields = $this->setFields;
        $setFields['payloadRetentionPeriod'] = true;

        return new self(
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            deliverAt: $this->deliverAt,
            payloadRetentionPeriod: $payloadRetentionPeriod,
            payloadRetentionHours: $this->payloadRetentionHours,
            setFields: $setFields
        );
    }

    public function withPayloadRetentionHours(?int $payloadRetentionHours): self
    {
        $setFields = $this->setFields;
        $setFields['payloadRetentionHours'] = true;

        return new self(
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            tags: $this->tags,
            transformationsParams: $this->transformationsParams,
            deliverAt: $this->deliverAt,
            payloadRetentionPeriod: $this->payloadRetentionPeriod,
            payloadRetentionHours: $payloadRetentionHours,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'eventType' => $this->eventType,
            'payload' => $this->payload];

        if (isset($this->setFields['eventId'])) {
            $data['eventId'] = $this->eventId;
        }
        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }
        if (isset($this->setFields['tags'])) {
            $data['tags'] = $this->tags;
        }
        if (isset($this->setFields['transformationsParams'])) {
            $data['transformationsParams'] = $this->transformationsParams;
        }
        if (isset($this->setFields['deliverAt'])) {
            $data['deliverAt'] = $this->deliverAt->format('c');
        }
        if (isset($this->setFields['payloadRetentionPeriod'])) {
            $data['payloadRetentionPeriod'] = $this->payloadRetentionPeriod;
        }
        if (isset($this->setFields['payloadRetentionHours'])) {
            $data['payloadRetentionHours'] = $this->payloadRetentionHours;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            eventId: \Svix\Utils::deserializeString($data, 'eventId', false, 'BatchMessageIn'),
            eventType: \Svix\Utils::deserializeString($data, 'eventType', true, 'BatchMessageIn'),
            payload: \Svix\Utils::getValFromJson($data, 'payload', true, 'BatchMessageIn'),
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'BatchMessageIn'),
            tags: \Svix\Utils::getValFromJson($data, 'tags', false, 'BatchMessageIn'),
            transformationsParams: \Svix\Utils::getValFromJson($data, 'transformationsParams', false, 'BatchMessageIn'),
            deliverAt: \Svix\Utils::deserializeDt($data, 'deliverAt', false, 'BatchMessageIn'),
            payloadRetentionPeriod: \Svix\Utils::deserializeInt($data, 'payloadRetentionPeriod', false, 'BatchMessageIn'),
            payloadRetentionHours: \Svix\Utils::deserializeInt($data, 'payloadRetentionHours', false, 'BatchMessageIn')
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
