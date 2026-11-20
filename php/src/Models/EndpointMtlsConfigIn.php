<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointMtlsConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $identity A PEM encoded private key and X.509 certificate to identify the webhook sender.
     */
    private function __construct(
        public readonly string $identity,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointMtlsConfigIn with required fields.
     */
    public static function create(
        string $identity,
    ): self {
        return new self(
            identity: $identity,
            setFields: ['identity' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'identity' => $this->identity];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            identity: \Svix\Utils::deserializeString($data, 'identity', true, 'EndpointMtlsConfigIn')
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
