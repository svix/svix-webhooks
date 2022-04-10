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
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptedMessagesAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptsForEndpoint_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptsForEndpointAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageAttempt_ListAttempts_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptsAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptedDestinations_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageAttempt_ListAttemptedDestinationsAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageAttempt_ResendWebhook_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageAttempt_ResendWebhookAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
    }
}