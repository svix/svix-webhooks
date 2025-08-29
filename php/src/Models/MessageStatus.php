<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum MessageStatus: int implements \JsonSerializable
{
    case SUCCESS = 0;
    case PENDING = 1;
    case FAIL = 2;
    case SENDING = 3;

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
