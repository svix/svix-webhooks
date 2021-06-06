# HealthApi

All URIs are relative to *https://api.svix.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**healthApiV1HealthGet**](HealthApi.md#healthApiV1HealthGet) | **GET** /api/v1/health/ | Health


<a name="healthApiV1HealthGet"></a>
# **healthApiV1HealthGet**
> healthApiV1HealthGet()

Health

Verify the API server is up and running.

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.models.*;
import svix.openapi.HealthApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");

    HealthApi apiInstance = new HealthApi(defaultClient);
    try {
      apiInstance.healthApiV1HealthGet();
    } catch (ApiException e) {
      System.err.println("Exception when calling HealthApi#healthApiV1HealthGet");
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

