<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AutoConfigSubscriptionOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $id     the AutoConfigSubscription's ID
     * @param string|null $endpId the Endpoint's ID
     * @param string|null $destId the StreamSink's ID
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $tokenCensored,
        public readonly string $id,
        public readonly Status $status,
        public readonly ?string $endpId = null,
        public readonly ?string $destId = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AutoConfigSubscriptionOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        string $tokenCensored,
        string $id,
        Status $status,
    ): self {
        return new self(
            createdAt: $createdAt,
            tokenCensored: $tokenCensored,
            id: $id,
            endpId: null,
            destId: null,
            status: $status,
            setFields: ['createdAt' => true, 'tokenCensored' => true, 'id' => true, 'status' => true]
        );
    }

    public function withEndpId(?string $endpId): self
    {
        $setFields = $this->setFields;
        $setFields['endpId'] = true;

        return new self(
            createdAt: $this->createdAt,
            tokenCensored: $this->tokenCensored,
            id: $this->id,
            endpId: $endpId,
            destId: $this->destId,
            status: $this->status,
            setFields: $setFields
        );
    }

    public function withDestId(?string $destId): self
    {
        $setFields = $this->setFields;
        $setFields['destId'] = true;

        return new self(
            createdAt: $this->createdAt,
            tokenCensored: $this->tokenCensored,
            id: $this->id,
            endpId: $this->endpId,
            destId: $destId,
            status: $this->status,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'createdAt' => $this->createdAt->format('c'),
            'tokenCensored' => $this->tokenCensored,
            'id' => $this->id,
            'status' => $this->status];

        if (isset($this->setFields['endpId'])) {
            $data['endpId'] = $this->endpId;
        }
        if (isset($this->setFields['destId'])) {
            $data['destId'] = $this->destId;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'AutoConfigSubscriptionOut'),
            tokenCensored: \Svix\Utils::deserializeString($data, 'tokenCensored', true, 'AutoConfigSubscriptionOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'AutoConfigSubscriptionOut'),
            endpId: \Svix\Utils::deserializeString($data, 'endpId', false, 'AutoConfigSubscriptionOut'),
            destId: \Svix\Utils::deserializeString($data, 'destId', false, 'AutoConfigSubscriptionOut'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'AutoConfigSubscriptionOut', [Status::class, 'fromMixed'])
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
