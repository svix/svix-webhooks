<?php

namespace Svix;

class Utils
{
    public static function assertNotNull(mixed $value, string $fieldName, string $structName): mixed
    {
        $msg = "Unable to deserialize `{$structName}` missing required field `{$fieldName}`";
        if ($value === null) {
            throw new \InvalidArgumentException($msg);
        }
        return $value;
    }

    // Get a value from a `mixed`
    //
    // 1. Ensure the mixed is an `array`
    // 2. Ensure keys exists
    // 3. If key is $required, raise exception if key does not exists/is null
    //
    public static function getValFromJson(mixed $data, string $key, bool $required, string $structName): mixed
    {
        if (!is_array($data)) {
            throw new \InvalidArgumentException("Error: expected deserialized {$structName} to be an array");
        }

        if (!array_key_exists($key, $data) && $required) {
            throw new \InvalidArgumentException("Unable to deserialize `{$structName}` missing required field `{$key}`");
        } else {
            $val = $data[$key] ?? null;
            return $val;
        }
    }


    // Get a string from a json obj, return null if $required is false and key exists but is null
    static function deserializeString(mixed $data, string $key, bool $required, string $structName): ?string
    {
        $val = Utils::getValFromJson($data, $key, $required, $structName);
        if ($val === null) {
            if ($required) {
                throw new \InvalidArgumentException("Required field '$key' on '{$structName}' is missing or null");
            } else {
                return null;
            }
        }

        if (!is_string($val)) {
            throw new \InvalidArgumentException("Field '$key' on '{$structName}' is not a string");
        }

        return $val;
    }

    static function deserializeDt(mixed $data, string $key, bool $required, string $structName): \DateTimeImmutable
    {

        $val = Utils::getValFromJson($data, $key, $required, $structName);
        return new \DateTimeImmutable($val);
    }

    static function deserializeInt(mixed $data, string $key, bool $required, string $structName): ?int
    {
        $val = Utils::getValFromJson($data, $key, $required, $structName);
        if ($val === null) {
            if ($required) {
                throw new \InvalidArgumentException("Required field '$key' on '{$structName}' is missing or null");
            } else {
                return null;
            }
        }

        if (!is_int($val)) {
            throw new \InvalidArgumentException("Field '$key' on '{$structName}' is not an int");
        }

        return $val;
    }

    static function deserializeBool(mixed $data, string $key, bool $required, string $structName): ?bool
    {
        $val = Utils::getValFromJson($data, $key, $required, $structName);
        if ($val === null) {
            if ($required) {
                throw new \InvalidArgumentException("Required field '$key' on '{$structName}' is missing or null");
            } else {
                return null;
            }
        }

        if (!is_bool($val)) {
            throw new \InvalidArgumentException("Field '$key' on '{$structName}' is not an int");
        }

        return $val;
    }

    static function deserializeObjectArray(mixed $data, string $key, bool $required, string $structName, callable $fromMixed): array
    {
        $rawData = self::getValFromJson($data, $key, $required, $structName);
        $parsedData = [];

        if (is_array($rawData)) {
            foreach ($rawData as $item) {
                $parsedData[] = $fromMixed($item);
            }
        }

        return $parsedData;
    }

    static function deserializeObject(mixed $data, string $key, bool $required, string $structName, callable $fromMixed): ?object
    {
        $val = Utils::getValFromJson($data, $key, $required, $structName);
        if ($val === null) {
            if ($required) {
                throw new \InvalidArgumentException("Required field '$key' on '{$structName}' is missing or null");
            } else {
                return null;
            }
        }

        return $fromMixed($val);
    }


    /**
     * Auto-detect the server URL based on the token region.
     * Extracts the region from the token and maps it to the corresponding API URL.
     */
    static function getServerUrlFromToken(string $token): string
    {
        $tokenParts = explode('.', $token);
        $region = end($tokenParts);

        return match ($region) {
            'us' => 'https://api.us.svix.com',
            'eu' => 'https://api.eu.svix.com',
            'in' => 'https://api.in.svix.com',
            'ca' => 'https://api.ca.svix.com',
            'au' => 'https://api.au.svix.com',
            default => 'https://api.svix.com'
        };
    }


    static function uuid4(): string
    {
        $data = random_bytes(16);

        $data[6] = chr(ord($data[6]) & 0x0f | 0x40); // set version to 0100
        $data[8] = chr(ord($data[8]) & 0x3f | 0x80); // set bits 6-7 to 10

        return vsprintf('%s%s-%s-%s-%s-%s%s%s', str_split(bin2hex($data), 4));
    }


    // if we get in a array type that is empty
    // return a new stdClass instead
    // this will be used in the `jsonSerialize` to ensure empty objects are serialized as `{}` and not as `[]`
    static function newStdClassIfArrayIsEmpty(mixed $val): mixed
    {
        if (is_array($val) && empty($val)) {
            return new \stdClass();
        } else {
            return $val;
        }
    }
}
