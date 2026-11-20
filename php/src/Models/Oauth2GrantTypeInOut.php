<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum Oauth2GrantTypeInOut: string implements \JsonSerializable
{
    case CLIENT_CREDENTIALS = 'clientCredentials';
    case REFRESH_TOKEN = 'refreshToken';

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
