// this file is @generated
#nullable enable
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
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseBackgroundTaskOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/background-task",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// List background tasks executed in the past 90 days.
        /// </summary>
        public ListResponseBackgroundTaskOut List(BackgroundTaskListOptions? options = null)
        {
            var response = this._client.SvixHttpClient.SendRequest<ListResponseBackgroundTaskOut>(
                method: HttpMethod.Get,
                path: "/api/v1/background-task",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
        }

        /// <summary>
        /// Get a background task by ID.
        /// </summary>
        public async Task<BackgroundTaskOut> GetAsync(
            string taskId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<BackgroundTaskOut>(
                method: HttpMethod.Get,
                path: "/api/v1/background-task/{task_id}",
                pathParams: new Dictionary<string, string> { { "task_id", taskId } },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Get a background task by ID.
        /// </summary>
        public BackgroundTaskOut Get(string taskId)
        {
            var response = this._client.SvixHttpClient.SendRequest<BackgroundTaskOut>(
                method: HttpMethod.Get,
                path: "/api/v1/background-task/{task_id}",
                pathParams: new Dictionary<string, string> { { "task_id", taskId } }
            );
            return response.Data;
        }
    }
}
