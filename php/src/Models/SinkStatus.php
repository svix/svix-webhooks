<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum SinkStatus: string implements \JsonSerializable
{
    case ENABLED = 'enabled';
    case PAUSED = 'paused';
    case DISABLED = 'disabled';
    case RETRYING = 'retrying';

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
