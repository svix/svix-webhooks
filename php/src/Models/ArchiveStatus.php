<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum ArchiveStatus: string implements \JsonSerializable
{
    case ACTIVE = 'active';
    case ARCHIVED = 'archived';
    case ALL = 'all';

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
