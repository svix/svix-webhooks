<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration parameters for defining a Segment sink. */
class SegmentSinkConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $host The Segment ingestion host.
     *
     * This is https://api.segment.io for US workspaces, https://events.eu1.segmentapis.com
     * for EU workspaces, or your custom domain. Events are sent to the `/v1/batch` endpoint
     * under this host.
     * @param string $writeKey The Segment source write key.
     *
     * Sent as HTTP Basic auth (the write key as the username, with an empty password) on each
     * `/v1/batch` request.
     */
    private function __construct(
        public readonly string $host,
        public readonly string $writeKey,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SegmentSinkConfigIn with required fields.
     */
    public static function create(
        string $host,
        string $writeKey,
    ): self {
        return new self(
            host: $host,
            writeKey: $writeKey,
            setFields: ['host' => true, 'writeKey' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'host' => $this->host,
            'writeKey' => $this->writeKey];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            host: \Svix\Utils::getValFromJson($data, 'host', true, 'SegmentSinkConfigIn'),
            writeKey: \Svix\Utils::deserializeString($data, 'writeKey', true, 'SegmentSinkConfigIn')
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
