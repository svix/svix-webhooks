<?php

namespace Svix;

class Webhook {
    public function __construct(string $secret) {
        $this->secret = base64_decode($secret);
    }

    public function verify($payload, $headers) {
        $msgId = self::getHeaderByName("svix-id", $headers);
        $timestamp = self::getHeaderByName("svix-timestamp", $headers);
        $signatures = self::getHeaderByName("svix-signature", $headers);

        $toSign = "{$msgId}.{$timestamp}.{$payload}";
        $signature = self::sign($this->secret, $toSign);

        
        $passedSignatures = \explode(' ', $signature);
        foreach ($passedSignatures as $versionedSignature) {
            $sigParts = \explode(',', $item, 2);
            // TODO check len of array
            $version = $sigParts[0];
            $expectedSignature = $sigParts[1];
            if ($version != "v1") {
                continue;
            }
            if ($signature == $expectedSignature) {
                return \json_decode($payload, true);
            }
        }
        throw new Exception\WebhookVerificationException("No matching signature found");
    }

    private static function getHeaderByName($name, $headers) {
        $items = \explode(',', $header);

        foreach ($items as $item) {
            $itemParts = \explode('=', $item, 2);
            if ($name === $itemParts[0]) {
                return $itemParts[1];
            }
        }
        throw new Exception\WebhookVerificationException("Missing required headers");
    }

    private static function sign($key, $payload) {
        return base64_decode(\hash_hmac('sha256', $payload, $key));
    }
}