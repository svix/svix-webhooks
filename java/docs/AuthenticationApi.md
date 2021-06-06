# AuthenticationApi

All URIs are relative to *https://api.svix.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**getDashboardAccessApiV1AuthDashboardAccessAppIdPost**](AuthenticationApi.md#getDashboardAccessApiV1AuthDashboardAccessAppIdPost) | **POST** /api/v1/auth/dashboard-access/{app_id}/ | Get Dashboard Access
[**logoutApiV1AuthLogoutPost**](AuthenticationApi.md#logoutApiV1AuthLogoutPost) | **POST** /api/v1/auth/logout/ | Logout


<a name="getDashboardAccessApiV1AuthDashboardAccessAppIdPost"></a>
# **getDashboardAccessApiV1AuthDashboardAccessAppIdPost**
> DashboardAccessOut getDashboardAccessApiV1AuthDashboardAccessAppIdPost(appId)

Get Dashboard Access

Use this function to get magic links (and authentication codes) for connecting your users to the management UIs.

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.AuthenticationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    AuthenticationApi apiInstance = new AuthenticationApi(defaultClient);
    String appId = "appId_example"; // String | 
    try {
      DashboardAccessOut result = apiInstance.getDashboardAccessApiV1AuthDashboardAccessAppIdPost(appId);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling AuthenticationApi#getDashboardAccessApiV1AuthDashboardAccessAppIdPost");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **appId** | **String**|  |

### Return type

[**DashboardAccessOut**](DashboardAccessOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Response |  -  |
**401** | Unauthorized |  -  |
**403** | Forbidden |  -  |
**404** | Not Found |  -  |
**409** | Conflict |  -  |
**422** | Validation Error |  -  |

<a name="logoutApiV1AuthLogoutPost"></a>
# **logoutApiV1AuthLogoutPost**
> logoutApiV1AuthLogoutPost()

Logout

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.AuthenticationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    AuthenticationApi apiInstance = new AuthenticationApi(defaultClient);
    try {
      apiInstance.logoutApiV1AuthLogoutPost();
    } catch (ApiException e) {
      System.err.println("Exception when calling AuthenticationApi#logoutApiV1AuthLogoutPost");
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

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**204** | Successful Response |  -  |
**401** | Unauthorized |  -  |
**403** | Forbidden |  -  |
**404** | Not Found |  -  |
**409** | Conflict |  -  |

