def get_or_create(
    self,
    application_in: ApplicationIn,
    options: ApplicationCreateOptions = ApplicationCreateOptions(),
) -> ApplicationOut:
    response = self._request_sync(
        method="post",
        path="/api/v1/app",
        path_params={},
        query_params={"get_if_exists": "true"},
        header_params=options._header_params(),
        json_body=application_in.model_dump_json(exclude_unset=True, by_alias=True),
    )
    return ApplicationOut.model_validate(response.json())
