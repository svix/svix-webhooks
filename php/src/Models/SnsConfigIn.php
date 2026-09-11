<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration for a SNS sink. */
class SnsConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $region The region of the SNS instance.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields in the future.
     * @param string|null $accessKeyId Access key ID.
     *
     * Required (along with `secret_access_key`) if `role_arn` is None
     * @param string|null $secretAccessKey Secret access key.
     *
     * Required (along with `access_key_id`) if `role_arn` is None
     * @param string|null $roleArn    Role ARN for delegated authentication
     * @param string|null $externalId Shared secret passed as the STS ExternalId.
     *
     * Can only be set if `role_arn` is Some
     */
    private function __construct(
        public readonly string $topicArn,
        public readonly ?string $region = null,
        public readonly ?string $accessKeyId = null,
        public readonly ?string $secretAccessKey = null,
        public readonly ?string $endpointUrl = null,
        public readonly ?string $roleArn = null,
        public readonly ?string $externalId = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SnsConfigIn with required fields.
     */
    public static function create(
        string $topicArn,
    ): self {
        return new self(
            topicArn: $topicArn,
            region: null,
            accessKeyId: null,
            secretAccessKey: null,
            endpointUrl: null,
            roleArn: null,
            externalId: null,
            setFields: ['topicArn' => true]
        );
    }

    public function withRegion(?string $region): self
    {
        $setFields = $this->setFields;
        $setFields['region'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $region,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            endpointUrl: $this->endpointUrl,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            setFields: $setFields
        );
    }

    public function withAccessKeyId(?string $accessKeyId): self
    {
        $setFields = $this->setFields;
        $setFields['accessKeyId'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            endpointUrl: $this->endpointUrl,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            setFields: $setFields
        );
    }

    public function withSecretAccessKey(?string $secretAccessKey): self
    {
        $setFields = $this->setFields;
        $setFields['secretAccessKey'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $secretAccessKey,
            endpointUrl: $this->endpointUrl,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            setFields: $setFields
        );
    }

    public function withEndpointUrl(?string $endpointUrl): self
    {
        $setFields = $this->setFields;
        $setFields['endpointUrl'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            endpointUrl: $endpointUrl,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            setFields: $setFields
        );
    }

    public function withRoleArn(?string $roleArn): self
    {
        $setFields = $this->setFields;
        $setFields['roleArn'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            endpointUrl: $this->endpointUrl,
            roleArn: $roleArn,
            externalId: $this->externalId,
            setFields: $setFields
        );
    }

    public function withExternalId(?string $externalId): self
    {
        $setFields = $this->setFields;
        $setFields['externalId'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            endpointUrl: $this->endpointUrl,
            roleArn: $this->roleArn,
            externalId: $externalId,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'topicArn' => $this->topicArn];

        if (isset($this->setFields['region'])) {
            $data['region'] = $this->region;
        }
        if (isset($this->setFields['accessKeyId'])) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (isset($this->setFields['secretAccessKey'])) {
            $data['secretAccessKey'] = $this->secretAccessKey;
        }
        if (isset($this->setFields['endpointUrl'])) {
            $data['endpointUrl'] = $this->endpointUrl;
        }
        if (isset($this->setFields['roleArn'])) {
            $data['roleArn'] = $this->roleArn;
        }
        if (isset($this->setFields['externalId'])) {
            $data['externalId'] = $this->externalId;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            topicArn: \Svix\Utils::deserializeString($data, 'topicArn', true, 'SnsConfigIn'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'SnsConfigIn'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'SnsConfigIn'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'SnsConfigIn'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'SnsConfigIn'),
            roleArn: \Svix\Utils::deserializeString($data, 'roleArn', false, 'SnsConfigIn'),
            externalId: \Svix\Utils::deserializeString($data, 'externalId', false, 'SnsConfigIn')
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
