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
    public sealed class MessageAttemptTests
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";
        
        private readonly Mock<IMessageAttemptApi> _mockMessageAttemptApi;

        private readonly Mock<ISvixOptions> _mockOptions;
        
        private readonly SvixClient _svixClient;

        public MessageAttemptTests()
        {
            _mockMessageAttemptApi = new Mock<IMessageAttemptApi>();
            _mockOptions = new Mock<ISvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                messageAttemptApi: _mockMessageAttemptApi.Object);
        }

        public void Dispose()
        {
            
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptedMessages_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new MessageAttemptListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Before = DateTime.Today,
                Limit = 50,
                Status = 0
            };

            // Act
            var lResult = _svixClient.MessageAttempt.ListAttemptedMessages(lApplicationId, lEndpointId, lOptions, lIdempotencyKey);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ListAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(lEndpointId, lApplicationId, lOptions.Iterator, lOptions.Limit, (Svix.Model.MessageStatus)lOptions.Status, lOptions.Before, lIdempotencyKey));
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptedMessagesAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new MessageAttemptListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Before = DateTime.Today,
                Limit = 50,
                Status = 0
            };

            // Act
            var lResult = _svixClient.MessageAttempt.ListAttemptedMessagesAsync(lApplicationId, lEndpointId, lOptions, lIdempotencyKey, default);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ListAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGetAsync(lEndpointId, lApplicationId, lOptions.Iterator, lOptions.Limit, (Svix.Model.MessageStatus)lOptions.Status, lOptions.Before, lIdempotencyKey, default));
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptsForEndpoint_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new AttemptsByEndpointListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Before = DateTime.Today,
                Limit = 50,
                Status = 0,
                Channel = "1337",
                EventTypes = new List<string>() { "user.signup" }
            };

            // Act
            var lResult = _svixClient.MessageAttempt.ListAttemptsForEndpoint(lApplicationId, lMessageId, lEndpointId, lOptions, lIdempotencyKey);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ListAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(lMessageId, lApplicationId, lEndpointId, lOptions.Iterator, lOptions.Limit, lOptions.EventTypes, lOptions.Channel, (Svix.Model.MessageStatus)lOptions.Status, lOptions.Before, lIdempotencyKey));
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptsForEndpointAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new AttemptsByEndpointListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Before = DateTime.Today,
                Limit = 50,
                Status = 0,
                Channel = "1337",
                EventTypes = new List<string>() { "user.signup" }
            };

            // Act
            var lResult = _svixClient.MessageAttempt.ListAttemptsForEndpointAsync(lApplicationId, lMessageId, lEndpointId, lOptions, lIdempotencyKey, default);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ListAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGetAsync(lMessageId, lApplicationId, lEndpointId, lOptions.Iterator, lOptions.Limit, lOptions.EventTypes, lOptions.Channel, (Svix.Model.MessageStatus)lOptions.Status, lOptions.Before, lIdempotencyKey, default));
        }
        
        [Fact]
        public void MessageAttempt_ListAttempts_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new MessageAttemptListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Limit = 50,
                Status = 0
            };

            // Act
            var lResult = _svixClient.MessageAttempt.ListAttempts(lApplicationId, lMessageId, lOptions, lIdempotencyKey);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ListAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(lApplicationId, lMessageId, lOptions.Iterator, lOptions.Limit, (Svix.Model.MessageStatus)lOptions.Status, lIdempotencyKey));
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptsAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new MessageAttemptListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Limit = 50,
                Status = 0
            };

            // Act
            var lResult = _svixClient.MessageAttempt.ListAttemptsAsync(lApplicationId, lMessageId, lOptions, lIdempotencyKey, default);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ListAttemptsApiV1AppAppIdMsgMsgIdAttemptGetAsync(lApplicationId, lMessageId, lOptions.Iterator, lOptions.Limit, (Svix.Model.MessageStatus)lOptions.Status, lIdempotencyKey, default));
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptedDestinations_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new ListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Limit = 50
            };

            // Act
            var lResult = _svixClient.MessageAttempt.ListAttemptedDestinations(lApplicationId, lMessageId, lOptions, lIdempotencyKey);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ListAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(lMessageId, lApplicationId, lOptions.Iterator, lOptions.Limit, lIdempotencyKey));
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptedDestinationsAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new ListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Limit = 50
            };

            // Act
            var lResult = _svixClient.MessageAttempt.ListAttemptedDestinationsAsync(lApplicationId, lMessageId, lOptions, lIdempotencyKey, default);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ListAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGetAsync(lMessageId, lApplicationId, lOptions.Iterator, lOptions.Limit, lIdempotencyKey, default));
        }
        
        [Fact]
        public void MessageAttempt_ResendWebhook_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockMessageAttemptApi.Setup(x => x.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPostWithHttpInfo(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>()))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));
            
            // Act
            var lResult = _svixClient.MessageAttempt.ResendWebhook(lApplicationId, lMessageId, lEndpointId, lIdempotencyKey);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPostWithHttpInfo(lEndpointId, lMessageId, lApplicationId, lIdempotencyKey));
        }
        
        [Fact]
        public void MessageAttempt_ResendWebhookAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lEndpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockMessageAttemptApi.Setup(x => x.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPostWithHttpInfoAsync(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));
            
            // Act
            var lResult = _svixClient.MessageAttempt.ResendWebhookAsync(lApplicationId, lMessageId, lEndpointId, lIdempotencyKey, default);
            
            // Assert
            _mockMessageAttemptApi.Verify(x => x.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPostWithHttpInfoAsync(lEndpointId, lMessageId, lApplicationId, lIdempotencyKey, default));
        }
    }
}