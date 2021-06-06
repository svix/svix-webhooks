# ApplicationApi

All URIs are relative to *https://api.svix.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createApplicationApiV1AppPost**](ApplicationApi.md#createApplicationApiV1AppPost) | **POST** /api/v1/app/ | Create Application
[**deleteApplicationApiV1AppAppIdDelete**](ApplicationApi.md#deleteApplicationApiV1AppAppIdDelete) | **DELETE** /api/v1/app/{app_id}/ | Delete Application
[**getApplicationApiV1AppAppIdGet**](ApplicationApi.md#getApplicationApiV1AppAppIdGet) | **GET** /api/v1/app/{app_id}/ | Get Application
[**listApplicationsApiV1AppGet**](ApplicationApi.md#listApplicationsApiV1AppGet) | **GET** /api/v1/app/ | List Applications
[**updateApplicationApiV1AppAppIdPut**](ApplicationApi.md#updateApplicationApiV1AppAppIdPut) | **PUT** /api/v1/app/{app_id}/ | Update Application


<a name="createApplicationApiV1AppPost"></a>
# **createApplicationApiV1AppPost**
> ApplicationOut createApplicationApiV1AppPost(applicationIn)

Create Application

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.ApplicationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    ApplicationApi apiInstance = new ApplicationApi(defaultClient);
    ApplicationIn applicationIn = new ApplicationIn(); // ApplicationIn | 
    try {
      ApplicationOut result = apiInstance.createApplicationApiV1AppPost(applicationIn);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ApplicationApi#createApplicationApiV1AppPost");
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
 **applicationIn** | [**ApplicationIn**](ApplicationIn.md)|  |

### Return type

[**ApplicationOut**](ApplicationOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Successful Response |  -  |
**401** | Unauthorized |  -  |
**403** | Forbidden |  -  |
**404** | Not Found |  -  |
**409** | Conflict |  -  |
**422** | Validation Error |  -  |

<a name="deleteApplicationApiV1AppAppIdDelete"></a>
# **deleteApplicationApiV1AppAppIdDelete**
> deleteApplicationApiV1AppAppIdDelete(appId)

Delete Application

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.ApplicationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    ApplicationApi apiInstance = new ApplicationApi(defaultClient);
    String appId = "appId_example"; // String | 
    try {
      apiInstance.deleteApplicationApiV1AppAppIdDelete(appId);
    } catch (ApiException e) {
      System.err.println("Exception when calling ApplicationApi#deleteApplicationApiV1AppAppIdDelete");
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
**422** | Validation Error |  -  |

<a name="getApplicationApiV1AppAppIdGet"></a>
# **getApplicationApiV1AppAppIdGet**
> ApplicationOut getApplicationApiV1AppAppIdGet(appId)

Get Application

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.ApplicationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    ApplicationApi apiInstance = new ApplicationApi(defaultClient);
    String appId = "appId_example"; // String | 
    try {
      ApplicationOut result = apiInstance.getApplicationApiV1AppAppIdGet(appId);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ApplicationApi#getApplicationApiV1AppAppIdGet");
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

[**ApplicationOut**](ApplicationOut.md)

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

<a name="listApplicationsApiV1AppGet"></a>
# **listApplicationsApiV1AppGet**
> ListResponseApplicationOut listApplicationsApiV1AppGet(iterator, limit)

List Applications

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.ApplicationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    ApplicationApi apiInstance = new ApplicationApi(defaultClient);
    String iterator = "iterator_example"; // String | 
    Integer limit = 50; // Integer | 
    try {
      ListResponseApplicationOut result = apiInstance.listApplicationsApiV1AppGet(iterator, limit);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ApplicationApi#listApplicationsApiV1AppGet");
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
 **iterator** | **String**|  | [optional]
 **limit** | **Integer**|  | [optional] [default to 50]

### Return type

[**ListResponseApplicationOut**](ListResponseApplicationOut.md)

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

<a name="updateApplicationApiV1AppAppIdPut"></a>
# **updateApplicationApiV1AppAppIdPut**
> ApplicationOut updateApplicationApiV1AppAppIdPut(appId, applicationIn)

Update Application

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.ApplicationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    ApplicationApi apiInstance = new ApplicationApi(defaultClient);
    String appId = "appId_example"; // String | 
    ApplicationIn applicationIn = new ApplicationIn(); // ApplicationIn | 
    try {
      ApplicationOut result = apiInstance.updateApplicationApiV1AppAppIdPut(appId, applicationIn);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ApplicationApi#updateApplicationApiV1AppAppIdPut");
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
 **applicationIn** | [**ApplicationIn**](ApplicationIn.md)|  |

### Return type

[**ApplicationOut**](ApplicationOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

 - **Content-Type**: application/json
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

