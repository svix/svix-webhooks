<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum Oauth2AuthMethodInOut: string implements \JsonSerializable
{
    case CLIENT_SECRET_JWT = 'clientSecretJwt';
    case CLIENT_SECRET_BASIC = 'clientSecretBasic';
    case CLIENT_SECRET_POST = 'clientSecretPost';

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
