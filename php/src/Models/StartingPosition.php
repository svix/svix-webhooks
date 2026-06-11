<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum StartingPosition: string implements \JsonSerializable
{
    case EARLIEST = 'earliest';
    case LATEST = 'latest';

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
