// this file is @generated
#nullable enable
using System;
using System.Collections.Generic;
using System.Net;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Model;
using Svix.Models;

namespace Svix
{
    public partial class BackgroundTaskListOptions
    {
        public BackgroundTaskStatus? Status { get; set; }
        public BackgroundTaskType? Task { get; set; }
        public int? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }
    }

    public sealed class BackgroundTask : SvixResourceBase
    {
        private readonly BackgroundTasksApi _backgroundTaskApi;

        public BackgroundTask(ISvixClient svixClient, BackgroundTasksApi backgroundTaskApi)
            : base(svixClient)
        {
            _backgroundTaskApi =
                backgroundTaskApi ?? throw new ArgumentNullException(nameof(backgroundTaskApi));
        }

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
                var response = await _backgroundTaskApi.V1BackgroundTaskListWithHttpInfoAsync(
                    status: options?.Status,
                    task: options?.Task,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    order: options?.Order,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;
                return new ListResponseBackgroundTaskOut();
            }
        }

        /// <summary>
        /// List background tasks executed in the past 90 days.
        /// </summary>
        public ListResponseBackgroundTaskOut List(BackgroundTaskListOptions? options = null)
        {
            try
            {
                var response = _backgroundTaskApi.V1BackgroundTaskListWithHttpInfo(
                    status: options?.Status,
                    task: options?.Task,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    order: options?.Order
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;
                return new ListResponseBackgroundTaskOut();
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
                var response = await _backgroundTaskApi.V1BackgroundTaskGetWithHttpInfoAsync(
                    taskId: taskId,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;
                return new BackgroundTaskOut();
            }
        }

        /// <summary>
        /// Get a background task by ID.
        /// </summary>
        public BackgroundTaskOut Get(string taskId)
        {
            try
            {
                var response = _backgroundTaskApi.V1BackgroundTaskGetWithHttpInfo(taskId: taskId);
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;
                return new BackgroundTaskOut();
            }
        }
    }
}
