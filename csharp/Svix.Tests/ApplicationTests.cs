using System;
using System.Net;
using Moq;
using Svix.Models;
using Svix;
using Xunit;

namespace Svix.Tests
{
    public sealed class ApplicationTests
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";

        private readonly Mock<Application> _mockApplication;

        private readonly Mock<SvixOptions> _mockOptions;

        private readonly SvixClient _svixClient;

        public ApplicationTests()
        {
            _mockApplication = new Mock<Application>();
            _mockOptions = new Mock<SvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                application: _mockApplication.Object);
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
            Assert.ThrowsAsync<ArgumentNullException>(() => _svixClient.Application.CreateAsync(null, null, default));
        }
    }
}