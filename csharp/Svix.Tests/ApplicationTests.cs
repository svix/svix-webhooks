using System;
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
    public sealed class ApplicationTests
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";

        private readonly Mock<IApplicationApi> _mockApplicationApi;

        private readonly Mock<ISvixOptions> _mockOptions;

        private readonly SvixClient _svixClient;

        public ApplicationTests()
        {
            _mockApplicationApi = new Mock<IApplicationApi>();
            _mockOptions = new Mock<ISvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                applicationApi: _mockApplicationApi.Object);
        }

        [Fact]
        public void ApplicationCreate_WithoutApplication_ThrowsException()
        {
            // Assert
            Assert.Throws<ArgumentNullException>(() => _svixClient.Application.Create(null, null));
        }

        [Fact]
        public void ApplicationCreateAsync_WithoutApplication_ThrowsException()
        {
            // Assert
            Assert.ThrowsAsync<ArgumentNullException>(() => _svixClient.Application.CreateAsync(null, null, null, default));
        }

        [Fact]
        public void ApplicationCreate_WithoutOptions_CallsApi_WithoutOptions()
        {
            // Arrange
            var lName = "app_name_08q73yhrngv";
            var lUid = "08273gh45";
            var lRateLimit = 30;

            ApplicationCreateOptions lOptions = null;
            var lApplication = new ApplicationIn(lName, lRateLimit, lUid);

            // Act
            var lApplicationOut = _svixClient.Application.Create(lApplication, lOptions, null);

            // Assert
            _mockApplicationApi.Verify(x => x.CreateApplicationApiV1AppPost(lApplication, It.IsAny<bool>(), null));
        }

        [Fact]
        public void ApplicationCreateAsync_WithoutOptions_CallsApi_WithoutOptions()
        {
            // Arrange
            var lName = "app_name_08q73yhrngv";
            var lUid = "08273gh45";
            var lRateLimit = 30;

            ApplicationCreateOptions lOptions = null;
            var lApplication = new ApplicationIn(lName, lRateLimit, lUid);

            // Act
            var lResult = _svixClient.Application.CreateAsync(lApplication, lOptions, null, default);

            // Assert
            _mockApplicationApi.Verify(x => x.CreateApplicationApiV1AppPostAsync(lApplication, It.IsAny<bool>(), null, default));
        }

        [Fact]
        public void ApplicationCreate_WithOptions_CallsApi_WithOptions()
        {
            // Arrange
            var lName = "app_name_08q73yhrngv";
            var lUid = "08273gh45";
            var lRateLimit = 30;

            var lApplication = new ApplicationIn(lName, lRateLimit, lUid);
            ApplicationCreateOptions lOptions = new ApplicationCreateOptions
            {
                GetIfExists = true
            };

            // Act
            var lApplicationOut = _svixClient.Application.Create(lApplication, lOptions, null);

            // Assert
            _mockApplicationApi.Verify(x => x.CreateApplicationApiV1AppPost(lApplication, true, null));
        }

        [Fact]
        public void ApplicationCreateAsync_WithOptions_CallsApi_WithOptions()
        {
            // Arrange
            var lName = "app_name_08q73yhrngv";
            var lUid = "08273gh45";
            var lRateLimit = 30;

            var lApplication = new ApplicationIn(lName, lRateLimit, lUid);
            ApplicationCreateOptions lOptions = new ApplicationCreateOptions
            {
                GetIfExists = true
            };

            // Act
            var lApplicationOut = _svixClient.Application.CreateAsync(lApplication, lOptions, null, default);

            // Assert
            _mockApplicationApi.Verify(x => x.CreateApplicationApiV1AppPostAsync(lApplication, true, null, default));
        }

        [Fact]
        public void ApplicationDelete_CallsAPi_WithParams()
        {
            // Arrange 
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockApplicationApi.Setup(x => x.DeleteApplicationApiV1AppAppIdDeleteWithHttpInfo(It.IsAny<string>(), It.IsAny<string>()))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act 
            var lResult = _svixClient.Application.Delete(lApplicationId, lIdempotencyKey);

            // Assert
            _mockApplicationApi.Verify(x => x.DeleteApplicationApiV1AppAppIdDeleteWithHttpInfo(lApplicationId, lIdempotencyKey));
        }

        [Fact]
        public void ApplicationDeleteAsync_CallsAPi_WithParams()
        {
            // Arrange 
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockApplicationApi.Setup(x => x.DeleteApplicationApiV1AppAppIdDeleteWithHttpInfoAsync(It.IsAny<string>(), It.IsAny<string>(), default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act 
            var lResult = _svixClient.Application.DeleteAsync(lApplicationId, lIdempotencyKey, default);

            // Assert
            _mockApplicationApi.Verify(x => x.DeleteApplicationApiV1AppAppIdDeleteWithHttpInfoAsync(lApplicationId, lIdempotencyKey, default));
        }

        [Fact]
        public void ApplicationGet_CallsAPi_WithParams()
        {
            // Arrange 
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act 
            var lResult = _svixClient.Application.Get(lApplicationId, lIdempotencyKey);

            // Assert
            _mockApplicationApi.Verify(x => x.GetApplicationApiV1AppAppIdGet(lApplicationId, lIdempotencyKey));
        }

        [Fact]
        public void ApplicationGetAsync_CallsAPi_WithParams()
        {
            // Arrange 
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act 
            var lResult = _svixClient.Application.GetAsync(lApplicationId, lIdempotencyKey, default);

            // Assert
            _mockApplicationApi.Verify(x => x.GetApplicationApiV1AppAppIdGetAsync(lApplicationId, lIdempotencyKey, default));
        }

        [Fact]
        public void ApplicationList_WithoutOptions_CallsApi_WithoutOptions()
        {
            // Arrange
            ListOptions lOptions = null;

            // Act
            var lResult = _svixClient.Application
                .List(lOptions);

            // Assert
            _mockApplicationApi.Verify(x => x.ListApplicationsApiV1AppGet(null, null, null));
        }

        [Fact]
        public void ApplicationListAsync_WithoutOptions_CallsApi_WithoutOptions()
        {
            // Arrange
            ListOptions lOptions = null;

            // Act
            var lResult = _svixClient.Application
                .ListAsync(lOptions);

            // Assert
            _mockApplicationApi.Verify(x => x.ListApplicationsApiV1AppGetAsync(null, null, null, default));
        }

        [Fact]
        public void ApplicationList_WithOptions_CallsApi_WithOptions()
        {
            // Arrange
            var lIterator = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            var lLimit = 30;

            var lOptions = new ListOptions
            {
                Iterator = lIterator,
                Limit = lLimit
            };

            // Act
            var lResult = _svixClient.Application
                .List(lOptions);

            // Assert
            _mockApplicationApi.Verify(x => x.ListApplicationsApiV1AppGet(lIterator, lLimit, null));
        }

        [Fact]
        public void ApplicationListAsync_WithOptions_CallsApi_WithOptions()
        {
            // Arrange
            var lIterator = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            var lLimit = 30;

            var lOptions = new ListOptions
            {
                Iterator = lIterator,
                Limit = lLimit
            };

            // Act
            var lResult = _svixClient.Application
                .ListAsync(lOptions);

            // Assert
            _mockApplicationApi.Verify(x => x.ListApplicationsApiV1AppGetAsync(lIterator, lLimit, null, default));
        }

        [Fact]
        public void ApplicationUpdate_CallsAPi_WithParams()
        {
            // Arrange 
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lApplication = new ApplicationIn("MyApplication");

            // Act 
            var lResult = _svixClient.Application.Update(lApplicationId, lApplication, lIdempotencyKey);

            // Assert
            _mockApplicationApi.Verify(x => x.UpdateApplicationApiV1AppAppIdPut(lApplicationId, lApplication, lIdempotencyKey));
        }

        [Fact]
        public void ApplicationUpdateAsync_CallsAPi_WithParams()
        {
            // Arrange 
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lApplication = new ApplicationIn("MyApplication");

            // Act 
            var lResult = _svixClient.Application.UpdateAsync(lApplicationId, lApplication, lIdempotencyKey, default);

            // Assert
            _mockApplicationApi.Verify(x => x.UpdateApplicationApiV1AppAppIdPutAsync(lApplicationId, lApplication, lIdempotencyKey, default));
        }
    }
}