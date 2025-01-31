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
    public partial class OperationalWebhookEndpointListOptions
    {
        public int? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }
    }

    public partial class OperationalWebhookEndpointCreateOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class OperationalWebhookEndpointRotateSecretOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public sealed class OperationalWebhookEndpoint : SvixResourceBase
    {
        private readonly WebhookEndpointApi _operationalWebhookEndpointApi;

        public OperationalWebhookEndpoint(
            ISvixClient svixClient,
            WebhookEndpointApi operationalWebhookEndpointApi
        )
            : base(svixClient)
        {
            _operationalWebhookEndpointApi =
                operationalWebhookEndpointApi
                ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointApi));
        }

        /// <summary>
        /// List operational webhook endpoints.
        /// </summary>
        public async Task<ListResponseOperationalWebhookEndpointOut> ListAsync(
            OperationalWebhookEndpointListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _operationalWebhookEndpointApi.V1OperationalWebhookEndpointListWithHttpInfoAsync(
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
                return new ListResponseOperationalWebhookEndpointOut();
            }
        }

        /// <summary>
        /// List operational webhook endpoints.
        /// </summary>
        public ListResponseOperationalWebhookEndpointOut List(
            OperationalWebhookEndpointListOptions? options = null
        )
        {
            try
            {
                var response =
                    _operationalWebhookEndpointApi.V1OperationalWebhookEndpointListWithHttpInfo(
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
                return new ListResponseOperationalWebhookEndpointOut();
            }
        }

        /// <summary>
        /// Create an operational webhook endpoint.
        /// </summary>
        public async Task<OperationalWebhookEndpointOut> CreateAsync(
            OperationalWebhookEndpointIn operationalWebhookEndpointIn,
            OperationalWebhookEndpointCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                operationalWebhookEndpointIn =
                    operationalWebhookEndpointIn
                    ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointIn));
                var response =
                    await _operationalWebhookEndpointApi.V1OperationalWebhookEndpointCreateWithHttpInfoAsync(
                        operationalWebhookEndpointIn: operationalWebhookEndpointIn,
                        idempotencyKey: options?.IdempotencyKey,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointOut();
            }
        }

        /// <summary>
        /// Create an operational webhook endpoint.
        /// </summary>
        public OperationalWebhookEndpointOut Create(
            OperationalWebhookEndpointIn operationalWebhookEndpointIn,
            OperationalWebhookEndpointCreateOptions? options = null
        )
        {
            try
            {
                operationalWebhookEndpointIn =
                    operationalWebhookEndpointIn
                    ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointIn));
                var response =
                    _operationalWebhookEndpointApi.V1OperationalWebhookEndpointCreateWithHttpInfo(
                        operationalWebhookEndpointIn: operationalWebhookEndpointIn,
                        idempotencyKey: options?.IdempotencyKey
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointOut();
            }
        }

        /// <summary>
        /// Get an operational webhook endpoint.
        /// </summary>
        public async Task<OperationalWebhookEndpointOut> GetAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _operationalWebhookEndpointApi.V1OperationalWebhookEndpointGetWithHttpInfoAsync(
                        endpointId: endpointId,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointOut();
            }
        }

        /// <summary>
        /// Get an operational webhook endpoint.
        /// </summary>
        public OperationalWebhookEndpointOut Get(string endpointId)
        {
            try
            {
                var response =
                    _operationalWebhookEndpointApi.V1OperationalWebhookEndpointGetWithHttpInfo(
                        endpointId: endpointId
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointOut();
            }
        }

        /// <summary>
        /// Update an operational webhook endpoint.
        /// </summary>
        public async Task<OperationalWebhookEndpointOut> UpdateAsync(
            string endpointId,
            OperationalWebhookEndpointUpdate operationalWebhookEndpointUpdate,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                operationalWebhookEndpointUpdate =
                    operationalWebhookEndpointUpdate
                    ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointUpdate));
                var response =
                    await _operationalWebhookEndpointApi.V1OperationalWebhookEndpointUpdateWithHttpInfoAsync(
                        endpointId: endpointId,
                        operationalWebhookEndpointUpdate: operationalWebhookEndpointUpdate,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointOut();
            }
        }

        /// <summary>
        /// Update an operational webhook endpoint.
        /// </summary>
        public OperationalWebhookEndpointOut Update(
            string endpointId,
            OperationalWebhookEndpointUpdate operationalWebhookEndpointUpdate
        )
        {
            try
            {
                operationalWebhookEndpointUpdate =
                    operationalWebhookEndpointUpdate
                    ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointUpdate));
                var response =
                    _operationalWebhookEndpointApi.V1OperationalWebhookEndpointUpdateWithHttpInfo(
                        endpointId: endpointId,
                        operationalWebhookEndpointUpdate: operationalWebhookEndpointUpdate
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointOut();
            }
        }

        /// <summary>
        /// Delete an operational webhook endpoint.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _operationalWebhookEndpointApi.V1OperationalWebhookEndpointDeleteWithHttpInfoAsync(
                        endpointId: endpointId,
                        cancellationToken: cancellationToken
                    );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Delete an operational webhook endpoint.
        /// </summary>
        public bool Delete(string endpointId)
        {
            try
            {
                var response =
                    _operationalWebhookEndpointApi.V1OperationalWebhookEndpointDeleteWithHttpInfo(
                        endpointId: endpointId
                    );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Delete)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Get the additional headers to be sent with the operational webhook.
        /// </summary>
        public async Task<OperationalWebhookEndpointHeadersOut> GetHeadersAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _operationalWebhookEndpointApi.V1OperationalWebhookEndpointGetHeadersWithHttpInfoAsync(
                        endpointId: endpointId,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetHeadersAsync)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointHeadersOut();
            }
        }

        /// <summary>
        /// Get the additional headers to be sent with the operational webhook.
        /// </summary>
        public OperationalWebhookEndpointHeadersOut GetHeaders(string endpointId)
        {
            try
            {
                var response =
                    _operationalWebhookEndpointApi.V1OperationalWebhookEndpointGetHeadersWithHttpInfo(
                        endpointId: endpointId
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetHeaders)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointHeadersOut();
            }
        }

        /// <summary>
        /// Set the additional headers to be sent with the operational webhook.
        /// </summary>
        public async Task<bool> UpdateHeadersAsync(
            string endpointId,
            OperationalWebhookEndpointHeadersIn operationalWebhookEndpointHeadersIn,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                operationalWebhookEndpointHeadersIn =
                    operationalWebhookEndpointHeadersIn
                    ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointHeadersIn));
                var response =
                    await _operationalWebhookEndpointApi.V1OperationalWebhookEndpointUpdateHeadersWithHttpInfoAsync(
                        endpointId: endpointId,
                        operationalWebhookEndpointHeadersIn: operationalWebhookEndpointHeadersIn,
                        cancellationToken: cancellationToken
                    );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateHeadersAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Set the additional headers to be sent with the operational webhook.
        /// </summary>
        public bool UpdateHeaders(
            string endpointId,
            OperationalWebhookEndpointHeadersIn operationalWebhookEndpointHeadersIn
        )
        {
            try
            {
                operationalWebhookEndpointHeadersIn =
                    operationalWebhookEndpointHeadersIn
                    ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointHeadersIn));
                var response =
                    _operationalWebhookEndpointApi.V1OperationalWebhookEndpointUpdateHeadersWithHttpInfo(
                        endpointId: endpointId,
                        operationalWebhookEndpointHeadersIn: operationalWebhookEndpointHeadersIn
                    );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateHeaders)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Get an operational webhook endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public async Task<OperationalWebhookEndpointSecretOut> GetSecretAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _operationalWebhookEndpointApi.V1OperationalWebhookEndpointGetSecretWithHttpInfoAsync(
                        endpointId: endpointId,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetSecretAsync)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointSecretOut();
            }
        }

        /// <summary>
        /// Get an operational webhook endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public OperationalWebhookEndpointSecretOut GetSecret(string endpointId)
        {
            try
            {
                var response =
                    _operationalWebhookEndpointApi.V1OperationalWebhookEndpointGetSecretWithHttpInfo(
                        endpointId: endpointId
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetSecret)} failed");

                if (Throw)
                    throw;
                return new OperationalWebhookEndpointSecretOut();
            }
        }

        /// <summary>
        /// Rotates an operational webhook endpoint's signing secret.
        ///
        /// The previous secret will remain valid for the next 24 hours.
        /// </summary>
        public async Task<bool> RotateSecretAsync(
            string endpointId,
            OperationalWebhookEndpointSecretIn operationalWebhookEndpointSecretIn,
            OperationalWebhookEndpointRotateSecretOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                operationalWebhookEndpointSecretIn =
                    operationalWebhookEndpointSecretIn
                    ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointSecretIn));
                var response =
                    await _operationalWebhookEndpointApi.V1OperationalWebhookEndpointRotateSecretWithHttpInfoAsync(
                        endpointId: endpointId,
                        operationalWebhookEndpointSecretIn: operationalWebhookEndpointSecretIn,
                        idempotencyKey: options?.IdempotencyKey,
                        cancellationToken: cancellationToken
                    );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateSecretAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Rotates an operational webhook endpoint's signing secret.
        ///
        /// The previous secret will remain valid for the next 24 hours.
        /// </summary>
        public bool RotateSecret(
            string endpointId,
            OperationalWebhookEndpointSecretIn operationalWebhookEndpointSecretIn,
            OperationalWebhookEndpointRotateSecretOptions? options = null
        )
        {
            try
            {
                operationalWebhookEndpointSecretIn =
                    operationalWebhookEndpointSecretIn
                    ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointSecretIn));
                var response =
                    _operationalWebhookEndpointApi.V1OperationalWebhookEndpointRotateSecretWithHttpInfo(
                        endpointId: endpointId,
                        operationalWebhookEndpointSecretIn: operationalWebhookEndpointSecretIn,
                        idempotencyKey: options?.IdempotencyKey
                    );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateSecret)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }
    }
}
