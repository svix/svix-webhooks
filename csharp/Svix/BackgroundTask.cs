// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class BackgroundTaskListOptions : SvixOptionsBase
    {
        public BackgroundTaskStatus? Status { get; set; }
        public BackgroundTaskType? Task { get; set; }
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "status", Status },
                    { "task", Task },
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "order", Order },
                }
            );
        }
    }

    public class BackgroundTask(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List background tasks executed in the past 90 days.
        /// </summary>
        public async Task<ListResponseBackgroundTaskOut> ListAsync(
            BackgroundTaskListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseBackgroundTaskOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/background-task",
                        queryParams: options?.QueryParams(),
                        headerParams: options?.HeaderParams(),
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// List background tasks executed in the past 90 days.
        /// </summary>
        public ListResponseBackgroundTaskOut List(BackgroundTaskListOptions? options = null)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseBackgroundTaskOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/background-task",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(List)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get a background task by ID.
        /// </summary>
        public async Task<BackgroundTaskOut> GetAsync(
            string taskId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<BackgroundTaskOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/background-task/{task_id}",
                    pathParams: new Dictionary<string, string> { { "task_id", taskId } },
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get a background task by ID.
        /// </summary>
        public BackgroundTaskOut Get(string taskId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<BackgroundTaskOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/background-task/{task_id}",
                    pathParams: new Dictionary<string, string> { { "task_id", taskId } }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Get)} failed");

                throw;
            }
        }
    }
}
