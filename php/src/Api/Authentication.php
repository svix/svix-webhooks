<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\ApplicationTokenExpireIn;
use Svix\Models\AppPortalAccessIn;
use Svix\Models\AppPortalAccessOut;
use Svix\Request\SvixHttpClient;

class Authentication
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal. */
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

    /** Expire all of the tokens associated with a specific application. */
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
}
