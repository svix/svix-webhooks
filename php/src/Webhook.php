<?php

namespace Svix;

class Webhook
{
    public const SECRET_PREFIX = "whsec_";
    public const TOLERANCE = 5 * 60;
    private $secret;

    public function __construct($secret)
    {
        if (substr($secret, 0, strlen(Webhook::SECRET_PREFIX)) === Webhook::SECRET_PREFIX) {
            $secret = substr($secret, strlen(Webhook::SECRET_PREFIX));
        }
        $this->secret = base64_decode($secret);
    }

    public static function fromRaw($secret)
    {
        $obj = new self();
        $obj->secret = $secret;
        return $obj;
    }

    /**
     * Validates the payload against the svix signature headers using the
     * webhook's signing secret.
     *
     * @throws Exception\WebhookVerificationException
     * @throws Exception\WebhookSigningException
     */
    public function verify($payload, $headers)
    {
        return $this->verifyInternal($payload, $headers, true);
    }

    /**
     * Validates the payload against the svix signature headers using the
     * webhook's signing secret.
     *
     * WARNING: This method does not check the signature's timestamp.
     * We recommend using the `verify` method instead.
     *
     * @throws Exception\WebhookVerificationException
     * @throws Exception\WebhookSigningException
     */
    public function verifyIgnoringTimestamp($payload, $headers)
    {
        return $this->verifyInternal($payload, $headers, false);
    }

    /**
     * @throws Exception\WebhookVerificationException
     * @throws Exception\WebhookSigningException
     */
    private function verifyInternal($payload, $headers, $enforceTolerance)
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


        $timestamp = $this->parseTimestampHeader($msgTimestamp);
        if ($enforceTolerance) {
            $this->verifyTimestamp($timestamp);
        } elseif ($timestamp <= 0) {
            // sign() refuses non-positive timestamps with a WebhookSigningException;
            // surface malformed header input as a verification error instead.
            throw new Exception\WebhookVerificationException("Invalid Signature Headers");
        }

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
                return json_decode($payload, true);
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

    private function parseTimestampHeader($timestampHeader)
    {
        return intval($timestampHeader, 10);
    }

    /**
     * @throws Exception\WebhookVerificationException
     */
    private function verifyTimestamp($timestamp)
    {
        $now = time();

        if ($timestamp < ($now - Webhook::TOLERANCE)) {
            throw new Exception\WebhookVerificationException("Message timestamp too old");
        }
        if ($timestamp > ($now + Webhook::TOLERANCE)) {
            throw new Exception\WebhookVerificationException("Message timestamp too new");
        }
        return $timestamp;
    }

    private function isPositiveInteger($v)
    {
        return is_numeric($v) && !is_float($v + 0) && (int) $v == $v && (int) $v > 0;
    }
}
