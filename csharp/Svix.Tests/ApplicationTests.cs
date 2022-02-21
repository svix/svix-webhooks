using Moq;
using Svix.Abstractions;
using Svix.Api;
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
        
        public void Dispose()
        {
            // empty
        }
        
        [Fact]
        public void ApplicationList_WithoutIterator_CallsApi_WithoutIteratorInfo()
        {
            // Arrange
            var lOptions = new ApplicationListOptions { };
            
            // Act
            var lResult = _svixClient.Application
                .List(lOptions);
            
            // Assert
            _mockApplicationApi.Verify(x => x.ListApplicationsApiV1AppGet(null, null, null));
        }
        
        [Fact]
        public async void ApplicationListAsync_WithoutIterator_CallsApi_WithoutIteratorInfo()
        {
            // Arrange
            var lOptions = new ApplicationListOptions { };
            
            // Act
            var lResult = _svixClient.Application
                .ListAsync(lOptions);
            
            // Assert
            _mockApplicationApi.Verify(x => x.ListApplicationsApiV1AppGetAsync(null, null, null, default));
        }
        
        [Fact]
        public void ApplicationList_WithIterator_CallsApi_WithIteratorInfo()
        {
            // Arrange
            var lIterator = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            var lLimit = 30;
            
            var lOptions = new ApplicationListOptions
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
        public async void ApplicationListAsync_WithIterator_CallsApi_WithIteratorInfo()
        {
            // Arrange
            var lIterator = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            var lLimit = 30;
            
            var lOptions = new ApplicationListOptions
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
    }
}