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

        public void Dispose()
        {
            
        }
        
        [Fact]
        public void MessageCreate_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageCreateAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageGet_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageGetAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageList_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void MessageListAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
    }
}