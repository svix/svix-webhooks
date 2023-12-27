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
    }
}