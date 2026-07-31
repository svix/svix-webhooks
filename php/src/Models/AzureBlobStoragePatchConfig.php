<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AzureBlobStoragePatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $container = null,
        public readonly ?string $account = null,
        public readonly ?string $accessKey = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AzureBlobStoragePatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            container: null,
            account: null,
            accessKey: null,
            setFields: []
        );
    }

    public function withContainer(?string $container): self
    {
        $setFields = $this->setFields;
        $setFields['container'] = true;

        return new self(
            container: $container,
            account: $this->account,
            accessKey: $this->accessKey,
            setFields: $setFields
        );
    }

    public function withAccount(?string $account): self
    {
        $setFields = $this->setFields;
        $setFields['account'] = true;

        return new self(
            container: $this->container,
            account: $account,
            accessKey: $this->accessKey,
            setFields: $setFields
        );
    }

    public function withAccessKey(?string $accessKey): self
    {
        $setFields = $this->setFields;
        $setFields['accessKey'] = true;

        return new self(
            container: $this->container,
            account: $this->account,
            accessKey: $accessKey,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->container) {
            $data['container'] = $this->container;
        }
        if (null !== $this->account) {
            $data['account'] = $this->account;
        }
        if (null !== $this->accessKey) {
            $data['accessKey'] = $this->accessKey;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            container: \Svix\Utils::deserializeString($data, 'container', false, 'AzureBlobStoragePatchConfig'),
            account: \Svix\Utils::deserializeString($data, 'account', false, 'AzureBlobStoragePatchConfig'),
            accessKey: \Svix\Utils::deserializeString($data, 'accessKey', false, 'AzureBlobStoragePatchConfig')
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
