<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum BackgroundTaskStatus: string implements \JsonSerializable
{
    case RUNNING = 'running';
    case FINISHED = 'finished';
    case FAILED = 'failed';

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
