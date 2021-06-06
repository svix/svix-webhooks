# DevelopmentApi

All URIs are relative to *https://api.svix.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**echoApiV1DevelopmentEchoPost**](DevelopmentApi.md#echoApiV1DevelopmentEchoPost) | **POST** /api/v1/development/echo/ | Echo


<a name="echoApiV1DevelopmentEchoPost"></a>
# **echoApiV1DevelopmentEchoPost**
> echoApiV1DevelopmentEchoPost()

Echo

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.models.*;
import svix.openapi.DevelopmentApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");

    DevelopmentApi apiInstance = new DevelopmentApi(defaultClient);
    try {
      apiInstance.echoApiV1DevelopmentEchoPost();
    } catch (ApiException e) {
      System.err.println("Exception when calling DevelopmentApi#echoApiV1DevelopmentEchoPost");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**204** | Successful Response |  -  |

