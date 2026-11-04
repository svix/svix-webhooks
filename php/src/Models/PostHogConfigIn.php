<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration parameters for defining a PostHog sink. */
class PostHogConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $domain The PostHog ingestion domain.
     *
     * This is https://us.i.posthog.com for US Cloud, https://eu.i.posthog.com for EU Cloud,
     * and your self-hosted domain for self-hosted instances. Note the `.i.` in the Cloud hosts
     * (`us.posthog.com` and `eu.posthog.com` are different, private endpoints).
     * @param string $projectToken The PostHog project API key.
     *
     * Sent as the `api_key` field in the body of each `/batch/` request.
     */
    private function __construct(
        public readonly string $domain,
        public readonly string $projectToken,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PostHogConfigIn with required fields.
     */
    public static function create(
        string $domain,
        string $projectToken,
    ): self {
        return new self(
            domain: $domain,
            projectToken: $projectToken,
            setFields: ['domain' => true, 'projectToken' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'domain' => $this->domain,
            'projectToken' => $this->projectToken];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            domain: \Svix\Utils::getValFromJson($data, 'domain', true, 'PostHogConfigIn'),
            projectToken: \Svix\Utils::deserializeString($data, 'projectToken', true, 'PostHogConfigIn')
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
