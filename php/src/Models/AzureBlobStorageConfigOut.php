<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AzureBlobStorageConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $container,
        public readonly string $account,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AzureBlobStorageConfigOut with required fields.
     */
    public static function create(
        string $container,
        string $account,
    ): self {
        return new self(
            container: $container,
            account: $account,
            setFields: ['container' => true, 'account' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'container' => $this->container,
            'account' => $this->account];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            container: \Svix\Utils::deserializeString($data, 'container', true, 'AzureBlobStorageConfigOut'),
            account: \Svix\Utils::deserializeString($data, 'account', true, 'AzureBlobStorageConfigOut')
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
