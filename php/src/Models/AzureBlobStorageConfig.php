<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AzureBlobStorageConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $accessKey,
        public readonly string $account,
        public readonly string $container,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AzureBlobStorageConfig with required fields.
     */
    public static function create(
        string $accessKey,
        string $account,
        string $container,
    ): self {
        return new self(
            accessKey: $accessKey,
            account: $account,
            container: $container,
            setFields: ['accessKey' => true, 'account' => true, 'container' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'accessKey' => $this->accessKey,
            'account' => $this->account,
            'container' => $this->container];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            accessKey: \Svix\Utils::deserializeString($data, 'accessKey', true, 'AzureBlobStorageConfig'),
            account: \Svix\Utils::deserializeString($data, 'account', true, 'AzureBlobStorageConfig'),
            container: \Svix\Utils::deserializeString($data, 'container', true, 'AzureBlobStorageConfig')
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
