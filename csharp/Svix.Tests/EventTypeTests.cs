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

        public void Dispose()
        {
            
        }
        
        [Fact]
        public void EventTypeArchive_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void EventTypeArchiveAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void EventTypeCreate_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void EventTypeCreateAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void EventTypeGet_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void EventTypeGetAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void EventTypeList_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void EventTypeListAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void EventTypeUpdate_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void EventTypeUpdateAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
    }
}