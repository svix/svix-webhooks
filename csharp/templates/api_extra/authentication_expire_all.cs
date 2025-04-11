[Obsolete("Please use `AppPortalAccessAsync` instead")]
public async Task<DashboardAccessOut> DashboardAccessAsync(
    string appId,
    AuthenticationDashboardAccessOptions? options = null,
    CancellationToken cancellationToken = default
)
{
    try
    {
        var response = await _client.SvixHttpClient.SendRequestAsync<DashboardAccessOut>(
            method: HttpMethod.Post,
            path: "/api/v1/auth/dashboard-access/{app_id}",
            pathParams: new Dictionary<string, string> { { "app_id", appId } },
            queryParams: options?.QueryParams(),
            headerParams: options?.HeaderParams(),
            cancellationToken: cancellationToken
        );
        return response.Data;
    }
    catch (ApiException e)
    {
        _client.Logger?.LogError(e, $"{nameof(DashboardAccessAsync)} failed");

        throw;
    }
}

[Obsolete("Please use `AppPortalAccess` instead")]
public DashboardAccessOut DashboardAccess(
    string appId,
    AuthenticationDashboardAccessOptions? options = null
)
{
    try
    {
        var response = _client.SvixHttpClient.SendRequest<DashboardAccessOut>(
            method: HttpMethod.Post,
            path: "/api/v1/auth/dashboard-access/{app_id}",
            pathParams: new Dictionary<string, string> { { "app_id", appId } },
            queryParams: options?.QueryParams(),
            headerParams: options?.HeaderParams()
        );
        return response.Data;
    }
    catch (ApiException e)
    {
        _client.Logger?.LogError(e, $"{nameof(DashboardAccess)} failed");

        throw;
    }
}
