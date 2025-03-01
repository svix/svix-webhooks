/// <summary>
/// Gets or creates a new application.
/// </summary>
public async Task<ApplicationOut> GetOrCreateAsync(
    ApplicationIn applicationIn,
    ApplicationCreateOptions? options = null,
    CancellationToken cancellationToken = default
)
{
    applicationIn = applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));
    try
    {
        var queryParams = options?.QueryParams() ?? [];
        queryParams?.Add("get_if_exists", "true");

        var response = await _client.SvixHttpClient.SendRequestAsync<ApplicationOut>(
            method: HttpMethod.Post,
            path: "/api/v1/app",
            queryParams: queryParams,
            headerParams: options?.HeaderParams(),
            content: applicationIn,
            cancellationToken: cancellationToken
        );
        return response.Data;
    }
    catch (ApiException e)
    {
        _client.Logger?.LogError(e, $"{nameof(GetOrCreateAsync)} failed");

        throw;
    }
}

/// <summary>
/// Gets or creates a new application.
/// </summary>
public ApplicationOut GetOrCreate(
    ApplicationIn applicationIn,
    ApplicationCreateOptions? options = null
)
{
    applicationIn = applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));
    try
    {
        var queryParams = options?.QueryParams() ?? [];
        queryParams?.Add("get_if_exists", "true");

        var response = _client.SvixHttpClient.SendRequest<ApplicationOut>(
            method: HttpMethod.Post,
            path: "/api/v1/app",
            queryParams: queryParams,
            headerParams: options?.HeaderParams(),
            content: applicationIn
        );
        return response.Data;
    }
    catch (ApiException e)
    {
        _client.Logger?.LogError(e, $"{nameof(GetOrCreate)} failed");

        throw;
    }
}
