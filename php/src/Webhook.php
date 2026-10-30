<?php

namespace Svix;

class Webhook
{
    public const SECRET_PREFIX = "whsec_";
    private const DEFAULT_TOLERANCE = 5 * 60;
    private $secret;
    private $tolerance = self::DEFAULT_TOLERANCE;

    /**
     * @param string|null $secret    the endpoint's signing secret
     * @param int         $tolerance maximum difference allowed, in seconds, between the
     *                               webhook's timestamp and the current time; defaults to 5 minutes
     * @param string|null $rawSecret the signing secret as raw bytes; pass either this or
     *                               $secret, not both (see fromRaw())
     */
    public function __construct($secret = null, $tolerance = self::DEFAULT_TOLERANCE, $rawSecret = null)
    {
        if (($secret === null) === ($rawSecret === null)) {
            throw new \InvalidArgumentException("exactly one of secret and rawSecret must be given");
        }
        if ($secret !== null) {
            if (substr($secret, 0, strlen(Webhook::SECRET_PREFIX)) === Webhook::SECRET_PREFIX) {
                $secret = substr($secret, strlen(Webhook::SECRET_PREFIX));
            }
            $secret = base64_decode($secret, true);
            if ($secret === false || $secret === '') {
                throw new \InvalidArgumentException("secret must be a non-empty base64 string");
            }
        } else {
            if (!is_string($rawSecret) || $rawSecret === '') {
                throw new \InvalidArgumentException("secret must not be empty");
            }
            $secret = $rawSecret;
        }
        $this->secret = $secret;

        if (!is_int($tolerance) || $tolerance < 0) {
            throw new \InvalidArgumentException("tolerance must be a non-negative integer");
        }
        $this->tolerance = $tolerance;
    }

    /**
     * @param string $secret    the signing secret as raw bytes
     * @param int    $tolerance see __construct()
     */
    public static function fromRaw($secret, $tolerance = self::DEFAULT_TOLERANCE)
    {
        return new self(null, $tolerance, $secret);
    }

    /**
     * @throws Exception\WebhookVerificationException
     * @throws Exception\WebhookSigningException
     */
    public function verify($payload, $headers)
    {
        if (
            isset($headers['svix-id'])
            && isset($headers['svix-timestamp'])
            && isset($headers['svix-signature'])
        ) {
            $msgId = $headers['svix-id'];
            $msgTimestamp = $headers['svix-timestamp'];
            $msgSignature = $headers['svix-signature'];
        } elseif (
            isset($headers['webhook-id'])
            && isset($headers['webhook-timestamp'])
            && isset($headers['webhook-signature'])
        ) {
            $msgId = $headers['webhook-id'];
            $msgTimestamp = $headers['webhook-timestamp'];
            $msgSignature = $headers['webhook-signature'];
        } else {
            throw new Exception\WebhookVerificationException("Missing required headers");
        }


        $timestamp = $this->verifyTimestamp($msgTimestamp);

        $signature = $this->sign($msgId, $timestamp, $payload);
        $expectedSignature = explode(',', $signature, 2)[1];

        $passedSignatures = explode(' ', $msgSignature);
        foreach ($passedSignatures as $versionedSignature) {
            $sigParts = explode(',', $versionedSignature, 2);

            if (count($sigParts) != 2) {
                continue;
            }

            $version = $sigParts[0];
            $passedSignature = $sigParts[1];

            if (strcmp($version, "v1") != 0) {
                continue;
            }

            if (hash_equals($expectedSignature, $passedSignature)) {
                return;
            }
        }
        throw new Exception\WebhookVerificationException("No matching signature found");
    }

    /**
     * @throws Exception\WebhookSigningException
     */
    public function sign($msgId, $timestamp, $payload)
    {
        if (!$this->isPositiveInteger($timestamp)) {
            throw new Exception\WebhookSigningException("Invalid timestamp");
        }
        $toSign = "{$msgId}.{$timestamp}.{$payload}";
        $hex_hash = hash_hmac('sha256', $toSign, $this->secret);
        $signature = base64_encode(pack('H*', $hex_hash));
        return "v1,{$signature}";
    }

    /**
     * @throws Exception\WebhookVerificationException
     */
    private function verifyTimestamp($timestampHeader)
    {
        $now = time();
        $timestamp = intval($timestampHeader, 10);

        if ($timestamp < ($now - $this->tolerance)) {
            throw new Exception\WebhookVerificationException("Message timestamp too old");
        }
        if ($timestamp > ($now + $this->tolerance)) {
            throw new Exception\WebhookVerificationException("Message timestamp too new");
        }
        if ($timestamp <= 0) {
            // Like the Rust SDK, timestamps before 1970 are not honored even when
            // the tolerance window would allow them. sign() also refuses them, so
            // reject the malformed header as a verification failure here.
            throw new Exception\WebhookVerificationException("Invalid Signature Headers");
        }

        return $timestamp;
    }

    private function isPositiveInteger($v)
    {
        return is_numeric($v) && !is_float($v + 0) && (int) $v == $v && (int) $v > 0;
    }
}
