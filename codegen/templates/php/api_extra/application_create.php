/** Get or create a new application */
public function getOrCreate(
    ApplicationIn $applicationIn,
    ?ApplicationCreateOptions $options = null,
): ApplicationOut {
    $request = $this->client->newReq('POST', '/api/v1/app');
    if (null !== $options) {
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
    }
    $request->setQueryParam("get_if_exists", "true");
    $request->setBody(json_encode($applicationIn));
    $res = $this->client->send($request);

    return ApplicationOut::fromJson($res);
}
