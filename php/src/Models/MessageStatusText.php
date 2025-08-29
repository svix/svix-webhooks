<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum MessageStatusText: string implements \JsonSerializable
{
    case SUCCESS = 'success';
    case PENDING = 'pending';
    case FAIL = 'fail';
    case SENDING = 'sending';

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
