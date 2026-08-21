<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AzureBlobStorageConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $accessKey Access key.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     */
    private function __construct(
        public readonly string $container,
        public readonly string $account,
        public readonly ?string $accessKey = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AzureBlobStorageConfigIn with required fields.
     */
    public static function create(
        string $container,
        string $account,
    ): self {
        return new self(
            container: $container,
            account: $account,
            accessKey: null,
            setFields: ['container' => true, 'account' => true]
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
            'container' => $this->container,
            'account' => $this->account];

        if (isset($this->setFields['accessKey'])) {
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
            container: \Svix\Utils::deserializeString($data, 'container', true, 'AzureBlobStorageConfigIn'),
            account: \Svix\Utils::deserializeString($data, 'account', true, 'AzureBlobStorageConfigIn'),
            accessKey: \Svix\Utils::deserializeString($data, 'accessKey', false, 'AzureBlobStorageConfigIn')
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
