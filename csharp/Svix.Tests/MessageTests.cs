using System;
using System.Collections.Generic;
using Moq;
using Svix.Abstractions;
using Svix.Api;
using Svix.Model;
using Svix.Models;
using Xunit;

namespace Svix.Tests
{
    public sealed class MessageTests
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";

        private readonly Mock<IMessageApi> _mockMessageApi;

        private readonly Mock<ISvixOptions> _mockOptions;

        private readonly SvixClient _svixClient;

        public MessageTests()
        {
            _mockMessageApi = new Mock<IMessageApi>();
            _mockOptions = new Mock<ISvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                messageApi: _mockMessageApi.Object);
        }

        [Fact]
        public void MessageCreate_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lMessage = new MessageIn(eventType: "user.signup", payload: new { });

            var lOptions = new MessageCreateOptions
            {
                WithContent = true
            };

            // Act
            var lResult = _svixClient.Message.Create(lApplicationId, lMessage, lOptions, lIdempotencyKey);

            // Assert
            _mockMessageApi.Verify(x => x.CreateMessageApiV1AppAppIdMsgPost(lApplicationId, lMessage, lOptions.WithContent, lIdempotencyKey));
        }

        [Fact]
        public void MessageCreateAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lMessage = new MessageIn(eventType: "user.signup", payload: new { });

            var lOptions = new MessageCreateOptions
            {
                WithContent = true
            };

            // Act
            var lResult = _svixClient.Message.CreateAsync(lApplicationId, lMessage, lOptions, lIdempotencyKey, default);

            // Assert
            _mockMessageApi.Verify(x => x.CreateMessageApiV1AppAppIdMsgPostAsync(lApplicationId, lMessage, lOptions.WithContent, lIdempotencyKey, default));
        }

        [Fact]
        public void MessageGet_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Message.Get(lApplicationId, lMessageId, lIdempotencyKey);

            // Assert
            _mockMessageApi.Verify(x => x.GetMessageApiV1AppAppIdMsgMsgIdGet(lMessageId, lApplicationId, lIdempotencyKey));
        }

        [Fact]
        public void MessageGetAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lMessageId = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Message.GetAsync(lApplicationId, lMessageId, lIdempotencyKey, default);

            // Assert
            _mockMessageApi.Verify(x => x.GetMessageApiV1AppAppIdMsgMsgIdGetAsync(lMessageId, lApplicationId, lIdempotencyKey, default));
        }

        [Fact]
        public void MessageList_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new MessageListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Before = DateTime.Today,
                After = DateTime.Today.Subtract(TimeSpan.FromDays(7)),
                Channel = "project_1337",
                Limit = 50,
                EventTypes = new List<string> { "user.signup" }
            };

            // Act
            var lResult = _svixClient.Message.List(lApplicationId, lOptions, lIdempotencyKey);

            // Assert
            _mockMessageApi.Verify(x => x.ListMessagesApiV1AppAppIdMsgGet(lApplicationId, lOptions.Iterator, lOptions.Limit, lOptions.EventTypes, lOptions.Channel, lOptions.Before, lOptions.After, lIdempotencyKey));
        }

        [Fact]
        public void MessageListAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new MessageListOptions
            {
                Iterator = "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Before = DateTime.Today,
                After = DateTime.Today.Subtract(TimeSpan.FromDays(7)),
                Channel = "project_1337",
                Limit = 50,
                EventTypes = new List<string> { "user.signup" }
            };

            // Act
            var lResult = _svixClient.Message.ListAsync(lApplicationId, lOptions, lIdempotencyKey, default);

            // Assert
            _mockMessageApi.Verify(x => x.ListMessagesApiV1AppAppIdMsgGetAsync(lApplicationId, lOptions.Iterator, lOptions.Limit, lOptions.EventTypes, lOptions.Channel, lOptions.Before, lOptions.After, lIdempotencyKey, default));
        }
    }
}