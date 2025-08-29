<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum BackgroundTaskType: string implements \JsonSerializable
{
    case ENDPOINT_REPLAY = 'endpoint.replay';
    case ENDPOINT_RECOVER = 'endpoint.recover';
    case APPLICATION_STATS = 'application.stats';
    case MESSAGE_BROADCAST = 'message.broadcast';
    case SDK_GENERATE = 'sdk.generate';
    case EVENT_TYPE_AGGREGATE = 'event-type.aggregate';
    case APPLICATION_PURGE_CONTENT = 'application.purge_content';

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
