def get_or_create(application_in, options = {})
    options = options.transform_keys(&:to_s)
    path = "/api/v1/app"
    res = @client.execute_request(
    "POST",
    path,
    query_params:{
        "get_if_exists"=> true
    },
    headers: {
        "idempotency-key" => options["idempotency-key"]
    },
    body: application_in
    )
    ApplicationOut.deserialize(res)
end
