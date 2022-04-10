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
    public sealed class IntegrationTests
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";
        
        private readonly Mock<IIntegrationApi> _mockIntegrationApi;

        private readonly Mock<ISvixOptions> _mockOptions;
        
        private readonly SvixClient _svixClient;

        public IntegrationTests()
        {
            _mockIntegrationApi = new Mock<IIntegrationApi>();
            _mockOptions = new Mock<ISvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                integrationApi: _mockIntegrationApi.Object);
        }

        public void Dispose()
        {
            
        }
        
        [Fact]
        public void IntegrationCreate_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationCreateAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationDelete_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationDeleteAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationGet_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationGetAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationGetKey_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationGetKeyAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationList_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationListAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationRotateKey_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationRotateKeyAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationUpdate_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
        
        [Fact]
        public void IntegrationUpdateAsync_CallsApi_WithParams()
        {
            // Arrange
            
            // Act
            
            // Assert
            throw new NotImplementedException();
        }
    }
}