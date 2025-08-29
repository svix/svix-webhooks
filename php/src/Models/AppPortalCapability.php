<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum AppPortalCapability: string implements \JsonSerializable
{
    case VIEW_BASE = 'ViewBase';
    case VIEW_ENDPOINT_SECRET = 'ViewEndpointSecret';
    case MANAGE_ENDPOINT_SECRET = 'ManageEndpointSecret';
    case MANAGE_TRANSFORMATIONS = 'ManageTransformations';
    case CREATE_ATTEMPTS = 'CreateAttempts';
    case MANAGE_ENDPOINT = 'ManageEndpoint';

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return self::from($data);
    }

    public function jsonSerialize(): mixed
    {
        return $this->value;
    }
}
