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
    public sealed class EventTypeTests
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";

        private readonly Mock<IEventTypeApi> _mockEventTypeApi;

        private readonly Mock<ISvixOptions> _mockOptions;

        private readonly SvixClient _svixClient;

        public EventTypeTests()
        {
            _mockEventTypeApi = new Mock<IEventTypeApi>();
            _mockOptions = new Mock<ISvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                eventTypeApi: _mockEventTypeApi.Object);
        }

        [Fact]
        public void EventTypeArchive_CallsApi_WithParams()
        {
            // Arrange
            var lEventType = "user.signup";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockEventTypeApi.Setup(x => x.DeleteEventTypeApiV1EventTypeEventTypeNameDeleteWithHttpInfo(It.IsAny<string>(), It.IsAny<string>()))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.EventType.Archive(lEventType, lIdempotencyKey);

            // Assert
            _mockEventTypeApi.Verify(x => x.DeleteEventTypeApiV1EventTypeEventTypeNameDeleteWithHttpInfo(lEventType, lIdempotencyKey));
        }

        [Fact]
        public void EventTypeArchiveAsync_CallsApi_WithParams()
        {
            // Arrange
            var lEventType = "user.signup";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockEventTypeApi.Setup(x => x.DeleteEventTypeApiV1EventTypeEventTypeNameDeleteWithHttpInfoAsync(It.IsAny<string>(), It.IsAny<string>(), default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.EventType.ArchiveAsync(lEventType, lIdempotencyKey, default);

            // Assert
            _mockEventTypeApi.Verify(x => x.DeleteEventTypeApiV1EventTypeEventTypeNameDeleteWithHttpInfoAsync(lEventType, lIdempotencyKey, default));
        }

        [Fact]
        public void EventTypeCreate_CallsApi_WithParams()
        {
            // Arrange
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lEventType = new EventTypeIn(name: "user.signup", description: "Invoked when a new user is created");

            // Act
            var lResult = _svixClient.EventType.Create(lEventType, lIdempotencyKey);

            // Assert
            _mockEventTypeApi.Verify(x => x.CreateEventTypeApiV1EventTypePost(lEventType, lIdempotencyKey));
        }

        [Fact]
        public void EventTypeCreateAsync_CallsApi_WithParams()
        {
            // Arrange
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lEventType = new EventTypeIn(name: "user.signup", description: "Invoked when a new user is created");

            // Act
            var lResult = _svixClient.EventType.CreateAsync(lEventType, lIdempotencyKey, default);

            // Assert
            _mockEventTypeApi.Verify(x => x.CreateEventTypeApiV1EventTypePostAsync(lEventType, lIdempotencyKey, default));
        }

        [Fact]
        public void EventTypeGet_CallsApi_WithParams()
        {
            // Arrange
            string lEventType = "user.signup";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.EventType.Get(lEventType, lIdempotencyKey);

            // Assert
            _mockEventTypeApi.Verify(x => x.GetEventTypeApiV1EventTypeEventTypeNameGet(lEventType, lIdempotencyKey));
        }

        [Fact]
        public void EventTypeGetAsync_CallsApi_WithParams()
        {
            // Arrange
            string lEventType = "user.signup";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.EventType.GetAsync(lEventType, lIdempotencyKey);

            // Assert
            _mockEventTypeApi.Verify(x => x.GetEventTypeApiV1EventTypeEventTypeNameGetAsync(lEventType, lIdempotencyKey, default));
        }

        [Fact]
        public void EventTypeList_CallsApi_WithParams()
        {
            // Arrange
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new EventTypeListOptions
            {
                Iterator = "user.signup",
                Limit = 50,
                IncludeArchived = true,
                WithContent = true
            };

            // Act
            var lResult = _svixClient.EventType.List(lOptions, lIdempotencyKey);

            // Assert
            _mockEventTypeApi.Verify(x => x.ListEventTypesApiV1EventTypeGet(lOptions.Iterator, lOptions.Limit, lOptions.WithContent, lOptions.IncludeArchived, lIdempotencyKey));
        }

        [Fact]
        public void EventTypeListAsync_CallsApi_WithParams()
        {
            // Arrange
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new EventTypeListOptions
            {
                Iterator = "user.signup",
                Limit = 50,
                IncludeArchived = true,
                WithContent = true
            };

            // Act
            var lResult = _svixClient.EventType.ListAsync(lOptions, lIdempotencyKey, default);

            // Assert
            _mockEventTypeApi.Verify(x => x.ListEventTypesApiV1EventTypeGetAsync(lOptions.Iterator, lOptions.Limit, lOptions.WithContent, lOptions.IncludeArchived, lIdempotencyKey, default));
        }

        [Fact]
        public void EventTypeUpdate_CallsApi_WithParams()
        {
            // Arrange
            string lEventType = "user.signup";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lUpdate = new EventTypeUpdate(description: "Updated description");

            // Act
            var lResult = _svixClient.EventType.Update(lEventType, lUpdate, lIdempotencyKey);

            // Assert
            _mockEventTypeApi.Verify(x => x.UpdateEventTypeApiV1EventTypeEventTypeNamePut(lEventType, lUpdate, lIdempotencyKey));
        }

        [Fact]
        public void EventTypeUpdateAsync_CallsApi_WithParams()
        {
            // Arrange
            string lEventType = "user.signup";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lUpdate = new EventTypeUpdate(description: "Updated description");

            // Act
            var lResult = _svixClient.EventType.UpdateAsync(lEventType, lUpdate, lIdempotencyKey, default);

            // Assert
            _mockEventTypeApi.Verify(x => x.UpdateEventTypeApiV1EventTypeEventTypeNamePutAsync(lEventType, lUpdate, lIdempotencyKey, default));
        }
    }
}