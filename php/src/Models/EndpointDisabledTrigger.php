<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum EndpointDisabledTrigger: string implements \JsonSerializable
{
    case MANUAL = 'manual';
    case AUTOMATIC = 'automatic';

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
