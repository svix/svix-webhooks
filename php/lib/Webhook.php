<?php

namespace Svix;

class WebhookVerificationException extends \Exception {
    public function __construct($message, $code = 0, Throwable $previous = null) {
            parent::__construct($message, $code, $previous);
    }
}

class Webhook {
    public function __construct(string $secret) {
        $this->secret = base64_decode($secret);
    }

    public function verify($payload, $headers) {
        if (!isset($headers['svix-id']) || !isset($headers['svix-timestamp']) || !isset($headers['svix-signature'])) {
            throw new WebhookVerificationException("Missing required headers");
        }

        $msgId = $headers['svix-id'];
        $timestamp = $headers['svix-timestamp'];
        $msgSignature = $headers['svix-signature'];

        $toSign = "{$msgId}.{$timestamp}.{$payload}";
        $signature = self::sign($this->secret, $toSign);

        $passedSignatures = explode(' ', $msgSignature);
        foreach ($passedSignatures as $versionedSignature) {
            $sigParts = explode(',', $versionedSignature, 2);
            // TODO check len of array
            $version = $sigParts[0];
            $expectedSignature = $sigParts[1];

            if (strcmp($version, "v1") != 0) {
                continue;
            }

            if (strcmp($signature, $expectedSignature) == 0) {
                return json_decode($payload, true);
            }
        }
        throw new WebhookVerificationException("No matching signature found");
    }

    private static function sign($key, $payload) {
        $hex_hash = hash_hmac('sha256', $payload, $key);
        return base64_encode(pack('H*',$hex_hash));
    }
}