using System;
using System.Collections.Generic;
using System.Net;
using Moq;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Model;
using Svix.Models;
using Xunit;

namespace Svix.Tests
{
    public sealed class EndpointTests
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";

        private readonly Mock<IEndpointApi> _mockEndpointApi;

        private readonly Mock<ISvixOptions> _mockOptions;

        private readonly SvixClient _svixClient;

        public EndpointTests()
        {
            _mockEndpointApi = new Mock<IEndpointApi>();
            _mockOptions = new Mock<ISvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                endpointApi: _mockEndpointApi.Object);
        }

        [Fact]
        public void EndpointCreate_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();
            string lUrl = "https://myEndpointUrl.com";
            var lEndpoint = new EndpointIn(url: lUrl)
            {
                Version = 1
            };

            // Act
            var lResult = _svixClient.Endpoint.Create(lApplicationId, lEndpoint, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.CreateEndpointApiV1AppAppIdEndpointPost(lApplicationId, lEndpoint, lIdempotencyKey));
        }

        [Fact]
        public void EndpointCreateAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();
            string lUrl = "https://myEndpointUrl.com";
            var lEndpoint = new EndpointIn(url: lUrl)
            {
                Version = 1
            };

            // Act
            var lResult = _svixClient.Endpoint.CreateAsync(lApplicationId, lEndpoint, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.CreateEndpointApiV1AppAppIdEndpointPostAsync(lApplicationId, lEndpoint, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointDelete_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockEndpointApi.Setup(x => x.DeleteEndpointApiV1AppAppIdEndpointEndpointIdDeleteWithHttpInfo(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>()))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.Delete(lEndpointId, lApplicationId, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.DeleteEndpointApiV1AppAppIdEndpointEndpointIdDeleteWithHttpInfo(lApplicationId, lEndpointId, lIdempotencyKey));
        }

        [Fact]
        public void EndpointDeleteAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockEndpointApi.Setup(x => x.DeleteEndpointApiV1AppAppIdEndpointEndpointIdDeleteWithHttpInfoAsync(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.DeleteAsync(lEndpointId, lApplicationId, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.DeleteEndpointApiV1AppAppIdEndpointEndpointIdDeleteWithHttpInfoAsync(lApplicationId, lEndpointId, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointGet_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Endpoint.Get(lEndpointId, lApplicationId, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.GetEndpointApiV1AppAppIdEndpointEndpointIdGet(lApplicationId, lEndpointId, lIdempotencyKey));
        }

        [Fact]
        public void EndpointGetAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Endpoint.GetAsync(lEndpointId, lApplicationId, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.GetEndpointApiV1AppAppIdEndpointEndpointIdGetAsync(lApplicationId, lEndpointId, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointGetHeaders_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Endpoint.GetHeaders(lEndpointId, lApplicationId, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.GetEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersGet(lApplicationId, lEndpointId, lIdempotencyKey));
        }

        [Fact]
        public void EndpointGetHeadersAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Endpoint.GetHeadersAsync(lEndpointId, lApplicationId, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.GetEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersGetAsync(lApplicationId, lEndpointId, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointGetSecret_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Endpoint.GetSecret(lEndpointId, lApplicationId, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.GetEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(lApplicationId, lEndpointId, lIdempotencyKey));
        }

        [Fact]
        public void EndpointGetSecretAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Endpoint.GetSecretAsync(lEndpointId, lApplicationId, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.GetEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGetAsync(lApplicationId, lEndpointId, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointList_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lIterator = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            var lLimit = 30;
            var lOptions = new ListOptions
            {
                Iterator = lIterator,
                Limit = lLimit
            };

            // Act
            var lResult = _svixClient.Endpoint.List(lApplicationId, lOptions, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.ListEndpointsApiV1AppAppIdEndpointGet(lApplicationId, lIterator, lLimit, lIdempotencyKey));
        }

        [Fact]
        public void EndpointListAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lIterator = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            var lLimit = 30;
            var lOptions = new ListOptions
            {
                Iterator = lIterator,
                Limit = lLimit
            };

            // Act
            var lResult = _svixClient.Endpoint.ListAsync(lApplicationId, lOptions, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.ListEndpointsApiV1AppAppIdEndpointGetAsync(lApplicationId, lIterator, lLimit, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointPatchHeaders_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lHeaderKey = "MyHeaderKey";
            var lHeaderValue = "MyHeaderValue";
            var lHeaders = new EndpointHeadersPatchIn(new Dictionary<string, string>
            {
                { lHeaderKey, lHeaderValue }
            });

            _mockEndpointApi.Setup(x => x.PatchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatchWithHttpInfo(It.IsAny<string>(), It.IsAny<string>(), lHeaders, lIdempotencyKey))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.PatchHeaders(lApplicationId, lEndpointId, lHeaders, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.PatchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatchWithHttpInfo(lApplicationId, lEndpointId, lHeaders, lIdempotencyKey));
        }

        [Fact]
        public void EndpointPatchHeadersAsync_CallsAPi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lHeaderKey = "MyHeaderKey";
            var lHeaderValue = "MyHeaderValue";
            var lHeaders = new EndpointHeadersPatchIn(new Dictionary<string, string>
            {
                { lHeaderKey, lHeaderValue }
            });

            _mockEndpointApi.Setup(x => x.PatchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatchWithHttpInfoAsync(It.IsAny<string>(), It.IsAny<string>(), lHeaders, lIdempotencyKey, default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.PatchHeadersAsync(lApplicationId, lEndpointId, lHeaders, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.PatchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatchWithHttpInfoAsync(lApplicationId, lEndpointId, lHeaders, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointRecover_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lRecover = new RecoverIn();

            _mockEndpointApi.Setup(x => x.RecoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPostWithHttpInfo(It.IsAny<string>(), It.IsAny<string>(), lRecover, lIdempotencyKey))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.Recover(lApplicationId, lEndpointId, lRecover, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.RecoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPostWithHttpInfo(lApplicationId, lEndpointId, lRecover, lIdempotencyKey));
        }

        [Fact]
        public void EndpointRecoverAsync_CallsAPi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lRecover = new RecoverIn();

            _mockEndpointApi.Setup(x => x.RecoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPostWithHttpInfoAsync(It.IsAny<string>(), It.IsAny<string>(), lRecover, lIdempotencyKey, default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.RecoverAsync(lApplicationId, lEndpointId, lRecover, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.RecoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPostWithHttpInfoAsync(lApplicationId, lEndpointId, lRecover, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointRotateSecret_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lSecret = new EndpointSecretRotateIn();

            _mockEndpointApi.Setup(x => x.RotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePostWithHttpInfo(It.IsAny<string>(), It.IsAny<string>(), lSecret, lIdempotencyKey))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.RotateSecret(lApplicationId, lEndpointId, lSecret, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.RotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePostWithHttpInfo(lEndpointId, lApplicationId, lSecret, lIdempotencyKey));
        }

        [Fact]
        public void EndpointRotateSecretAsync_CallsAPi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lSecret = new EndpointSecretRotateIn();

            _mockEndpointApi.Setup(x => x.RotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePostWithHttpInfoAsync(It.IsAny<string>(), It.IsAny<string>(), lSecret, lIdempotencyKey, default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.RotateSecretAsync(lApplicationId, lEndpointId, lSecret, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.RotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePostWithHttpInfoAsync(lEndpointId, lApplicationId, lSecret, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointUpdate_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            string lUrl = "https://myEndpointUrl.com";
            var lEndpoint = new EndpointUpdate(url: lUrl)
            {
                Version = 1
            };

            // Act
            var lResult = _svixClient.Endpoint.Update(lApplicationId, lEndpointId, lEndpoint, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.UpdateEndpointApiV1AppAppIdEndpointEndpointIdPut(lEndpointId, lApplicationId, lEndpoint, lIdempotencyKey));
        }

        [Fact]
        public void EndpointUpdateAsync_CallsAPi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            string lUrl = "https://myEndpointUrl.com";
            var lEndpoint = new EndpointUpdate(url: lUrl)
            {
                Version = 1
            };

            // Act
            var lResult = _svixClient.Endpoint.UpdateAsync(lApplicationId, lEndpointId, lEndpoint, lIdempotencyKey, default);

            // Assert
            _mockEndpointApi.Verify(x => x.UpdateEndpointApiV1AppAppIdEndpointEndpointIdPutAsync(lEndpointId, lApplicationId, lEndpoint, lIdempotencyKey, default));
        }

        [Fact]
        public void EndpointUpdateHeaders_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lHeaderKey = "MyHeaderKey";
            var lHeaderValue = "MyHeaderValue";
            var lHeaders = new EndpointHeadersIn(new Dictionary<string, string>
            {
                { lHeaderKey, lHeaderValue }
            });

            _mockEndpointApi.Setup(x => x.UpdateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPutWithHttpInfo(It.IsAny<string>(), It.IsAny<string>(), lHeaders, lIdempotencyKey))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.UpdateHeaders(lApplicationId, lEndpointId, lHeaders, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.UpdateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPutWithHttpInfo(lApplicationId, lEndpointId, lHeaders, lIdempotencyKey));
        }

        [Fact]
        public void EndpointUpdateHeadersAsync_CallsAPi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lHeaderKey = "MyHeaderKey";
            var lHeaderValue = "MyHeaderValue";
            var lHeaders = new EndpointHeadersIn(new Dictionary<string, string>
            {
                { lHeaderKey, lHeaderValue }
            });

            _mockEndpointApi.Setup(x => x.UpdateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPutWithHttpInfoAsync(It.IsAny<string>(), It.IsAny<string>(), lHeaders, lIdempotencyKey, default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Endpoint.UpdateHeadersAsync(lApplicationId, lEndpointId, lHeaders, lIdempotencyKey);

            // Assert
            _mockEndpointApi.Verify(x => x.UpdateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPutWithHttpInfoAsync(lApplicationId, lEndpointId, lHeaders, lIdempotencyKey, default));
        }
    }
}