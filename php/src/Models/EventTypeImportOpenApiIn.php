<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/**
 * Import a list of event types from webhooks defined in an OpenAPI spec.
 *
 * The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither or both is invalid, resulting in a `400` **Bad Request**.
 */
class EventTypeImportOpenApiIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param bool|null   $dryRun     if `true`, return the event types that would be modified without actually modifying them
     * @param bool|null   $replaceAll if `true`, all existing event types that are not in the spec will be archived
     * @param array|null  $spec       a pre-parsed JSON spec
     * @param string|null $specRaw    a string, parsed by the server as YAML or JSON
     */
    private function __construct(
        public readonly ?bool $dryRun = null,
        public readonly ?bool $replaceAll = null,
        public readonly ?array $spec = null,
        public readonly ?string $specRaw = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventTypeImportOpenApiIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            dryRun: null,
            replaceAll: null,
            spec: null,
            specRaw: null,
            setFields: []
        );
    }

    public function withDryRun(?bool $dryRun): self
    {
        $setFields = $this->setFields;
        $setFields['dryRun'] = true;

        return new self(
            dryRun: $dryRun,
            replaceAll: $this->replaceAll,
            spec: $this->spec,
            specRaw: $this->specRaw,
            setFields: $setFields
        );
    }

    public function withReplaceAll(?bool $replaceAll): self
    {
        $setFields = $this->setFields;
        $setFields['replaceAll'] = true;

        return new self(
            dryRun: $this->dryRun,
            replaceAll: $replaceAll,
            spec: $this->spec,
            specRaw: $this->specRaw,
            setFields: $setFields
        );
    }

    public function withSpec(?array $spec): self
    {
        $setFields = $this->setFields;
        $setFields['spec'] = true;

        return new self(
            dryRun: $this->dryRun,
            replaceAll: $this->replaceAll,
            spec: $spec,
            specRaw: $this->specRaw,
            setFields: $setFields
        );
    }

    public function withSpecRaw(?string $specRaw): self
    {
        $setFields = $this->setFields;
        $setFields['specRaw'] = true;

        return new self(
            dryRun: $this->dryRun,
            replaceAll: $this->replaceAll,
            spec: $this->spec,
            specRaw: $specRaw,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->dryRun) {
            $data['dryRun'] = $this->dryRun;
        }
        if (null !== $this->replaceAll) {
            $data['replaceAll'] = $this->replaceAll;
        }
        if (isset($this->setFields['spec'])) {
            $data['spec'] = $this->spec;
        }
        if (isset($this->setFields['specRaw'])) {
            $data['specRaw'] = $this->specRaw;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            dryRun: \Svix\Utils::deserializeBool($data, 'dryRun', false, 'EventTypeImportOpenApiIn'),
            replaceAll: \Svix\Utils::deserializeBool($data, 'replaceAll', false, 'EventTypeImportOpenApiIn'),
            spec: \Svix\Utils::getValFromJson($data, 'spec', false, 'EventTypeImportOpenApiIn'),
            specRaw: \Svix\Utils::deserializeString($data, 'specRaw', false, 'EventTypeImportOpenApiIn')
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
