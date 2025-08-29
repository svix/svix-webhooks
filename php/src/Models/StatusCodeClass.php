<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum StatusCodeClass: int implements \JsonSerializable
{
    case CODE_NONE = 0;
    case CODE1XX = 100;
    case CODE2XX = 200;
    case CODE3XX = 300;
    case CODE4XX = 400;
    case CODE5XX = 500;

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
