<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Exception\ApiException;
use Svix\Models\AutoConfigSubscriptionOut;
use Svix\Request\SvixHttpClient;

class AutoconfigSubscription
{
    public AutoconfigSubscriptionDestination $destination;
    public AutoconfigSubscriptionEndpoint $endpoint;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->destination = new AutoconfigSubscriptionDestination($client);
        $this->endpoint = new AutoconfigSubscriptionEndpoint($client);
    }

    /**
     * Get an AutoConfig subscription, including the bound endpoint or destination if any.
     *
     * @throws ApiException
     */
    public function get(
        string $appId,
        string $autoconfigId,
    ): AutoConfigSubscriptionOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/autoconfig/{$autoconfigId}");
        $res = $this->client->send($request);

        return AutoConfigSubscriptionOut::fromJson($res);
    }
}
