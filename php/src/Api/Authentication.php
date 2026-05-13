<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\ApiTokenOut;
use Svix\Models\ApplicationTokenExpireIn;
use Svix\Models\AppPortalAccessIn;
use Svix\Models\AppPortalAccessOut;
use Svix\Models\RotatePollerTokenIn;
use Svix\Models\StreamPortalAccessIn;
use Svix\Models\StreamTokenExpireIn;
use Svix\Request\SvixHttpClient;

class Authentication
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
     *
     * @throws ApiException
     */
    public function appPortalAccess(
        string $appId,
        AppPortalAccessIn $appPortalAccessIn,
        ?AuthenticationAppPortalAccessOptions $options = null,
    ): AppPortalAccessOut {
        $request = $this->client->newReq('POST', "/api/v1/auth/app-portal-access/{$appId}");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($appPortalAccessIn));
        $res = $this->client->send($request);

        return AppPortalAccessOut::fromJson($res);
    }

    /**
     * Expire all of the tokens associated with a specific application.
     *
     * @throws ApiException
     */
    public function expireAll(
        string $appId,
        ApplicationTokenExpireIn $applicationTokenExpireIn,
        ?AuthenticationExpireAllOptions $options = null,
    ): void {
        $request = $this->client->newReq('POST', "/api/v1/auth/app/{$appId}/expire-all");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($applicationTokenExpireIn));
        $res = $this->client->sendNoResponseBody($request);
    }

    /**
     * Logout an app token.
     *
     * Trying to log out other tokens will fail.
     *
     * @throws ApiException
     */
    public function logout(
        ?AuthenticationLogoutOptions $options = null,
    ): void {
        $request = $this->client->newReq('POST', '/api/v1/auth/logout');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $res = $this->client->sendNoResponseBody($request);
    }

    /**
     * Logout a stream token.
     *
     * Trying to log out other tokens will fail.
     *
     * @throws ApiException
     */
    public function streamLogout(
        ?AuthenticationStreamLogoutOptions $options = null,
    ): void {
        $request = $this->client->newReq('POST', '/api/v1/auth/stream-logout');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $res = $this->client->sendNoResponseBody($request);
    }

    /**
     * Use this function to get magic links (and authentication codes) for connecting your users to the Stream Consumer Portal.
     *
     * @throws ApiException
     */
    public function streamPortalAccess(
        string $streamId,
        StreamPortalAccessIn $streamPortalAccessIn,
        ?AuthenticationStreamPortalAccessOptions $options = null,
    ): AppPortalAccessOut {
        $request = $this->client->newReq('POST', "/api/v1/auth/stream-portal-access/{$streamId}");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($streamPortalAccessIn));
        $res = $this->client->send($request);

        return AppPortalAccessOut::fromJson($res);
    }

    /**
     * Expire all of the tokens associated with a specific stream.
     *
     * @throws ApiException
     */
    public function streamExpireAll(
        string $streamId,
        StreamTokenExpireIn $streamTokenExpireIn,
        ?AuthenticationStreamExpireAllOptions $options = null,
    ): void {
        $request = $this->client->newReq('POST', "/api/v1/auth/stream/{$streamId}/expire-all");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($streamTokenExpireIn));
        $res = $this->client->sendNoResponseBody($request);
    }

    /**
     * Get the current auth token for the stream poller.
     *
     * @throws ApiException
     */
    public function getStreamPollerToken(
        string $streamId,
        string $sinkId,
    ): ApiTokenOut {
        $request = $this->client->newReq('GET', "/api/v1/auth/stream/{$streamId}/sink/{$sinkId}/poller/token");
        $res = $this->client->send($request);

        return ApiTokenOut::fromJson($res);
    }

    /**
     * Create a new auth token for the stream poller API.
     *
     * @throws ApiException
     */
    public function rotateStreamPollerToken(
        string $streamId,
        string $sinkId,
        RotatePollerTokenIn $rotatePollerTokenIn,
        ?AuthenticationRotateStreamPollerTokenOptions $options = null,
    ): ApiTokenOut {
        $request = $this->client->newReq('POST', "/api/v1/auth/stream/{$streamId}/sink/{$sinkId}/poller/token/rotate");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($rotatePollerTokenIn));
        $res = $this->client->send($request);

        return ApiTokenOut::fromJson($res);
    }
}
