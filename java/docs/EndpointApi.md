# EndpointApi

All URIs are relative to *https://api.svix.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createEndpointApiV1AppAppIdEndpointPost**](EndpointApi.md#createEndpointApiV1AppAppIdEndpointPost) | **POST** /api/v1/app/{app_id}/endpoint/ | Create Endpoint
[**deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete**](EndpointApi.md#deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete) | **DELETE** /api/v1/app/{app_id}/endpoint/{endpoint_id}/ | Delete Endpoint
[**getEndpointApiV1AppAppIdEndpointEndpointIdGet**](EndpointApi.md#getEndpointApiV1AppAppIdEndpointEndpointIdGet) | **GET** /api/v1/app/{app_id}/endpoint/{endpoint_id}/ | Get Endpoint
[**getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet**](EndpointApi.md#getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet) | **GET** /api/v1/app/{app_id}/endpoint/{endpoint_id}/secret/ | Get Endpoint Secret
[**getEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet**](EndpointApi.md#getEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet) | **GET** /api/v1/app/{app_id}/endpoint/{endpoint_id}/stats/ | Get Endpoint Stats
[**listEndpointsApiV1AppAppIdEndpointGet**](EndpointApi.md#listEndpointsApiV1AppAppIdEndpointGet) | **GET** /api/v1/app/{app_id}/endpoint/ | List Endpoints
[**updateEndpointApiV1AppAppIdEndpointEndpointIdPut**](EndpointApi.md#updateEndpointApiV1AppAppIdEndpointEndpointIdPut) | **PUT** /api/v1/app/{app_id}/endpoint/{endpoint_id}/ | Update Endpoint


<a name="createEndpointApiV1AppAppIdEndpointPost"></a>
# **createEndpointApiV1AppAppIdEndpointPost**
> EndpointOut createEndpointApiV1AppAppIdEndpointPost(appId, endpointIn)

Create Endpoint

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EndpointApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EndpointApi apiInstance = new EndpointApi(defaultClient);
    String appId = "appId_example"; // String | 
    EndpointIn endpointIn = new EndpointIn(); // EndpointIn | 
    try {
      EndpointOut result = apiInstance.createEndpointApiV1AppAppIdEndpointPost(appId, endpointIn);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling EndpointApi#createEndpointApiV1AppAppIdEndpointPost");
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
 **endpointIn** | [**EndpointIn**](EndpointIn.md)|  |

### Return type

[**EndpointOut**](EndpointOut.md)

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

<a name="deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete"></a>
# **deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete**
> deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(endpointId, appId)

Delete Endpoint

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EndpointApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EndpointApi apiInstance = new EndpointApi(defaultClient);
    String endpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2"; // String | 
    String appId = "appId_example"; // String | 
    try {
      apiInstance.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(endpointId, appId);
    } catch (ApiException e) {
      System.err.println("Exception when calling EndpointApi#deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete");
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
 **endpointId** | **String**|  |
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

<a name="getEndpointApiV1AppAppIdEndpointEndpointIdGet"></a>
# **getEndpointApiV1AppAppIdEndpointEndpointIdGet**
> EndpointOut getEndpointApiV1AppAppIdEndpointEndpointIdGet(endpointId, appId)

Get Endpoint

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EndpointApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EndpointApi apiInstance = new EndpointApi(defaultClient);
    String endpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2"; // String | 
    String appId = "appId_example"; // String | 
    try {
      EndpointOut result = apiInstance.getEndpointApiV1AppAppIdEndpointEndpointIdGet(endpointId, appId);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling EndpointApi#getEndpointApiV1AppAppIdEndpointEndpointIdGet");
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
 **endpointId** | **String**|  |
 **appId** | **String**|  |

### Return type

[**EndpointOut**](EndpointOut.md)

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

<a name="getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet"></a>
# **getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet**
> EndpointSecret getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(endpointId, appId)

Get Endpoint Secret

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EndpointApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EndpointApi apiInstance = new EndpointApi(defaultClient);
    String endpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2"; // String | 
    String appId = "appId_example"; // String | 
    try {
      EndpointSecret result = apiInstance.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(endpointId, appId);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling EndpointApi#getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet");
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
 **endpointId** | **String**|  |
 **appId** | **String**|  |

### Return type

[**EndpointSecret**](EndpointSecret.md)

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

<a name="getEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet"></a>
# **getEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet**
> EndpointStats getEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet(endpointId, appId)

Get Endpoint Stats

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EndpointApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EndpointApi apiInstance = new EndpointApi(defaultClient);
    String endpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2"; // String | 
    String appId = "appId_example"; // String | 
    try {
      EndpointStats result = apiInstance.getEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet(endpointId, appId);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling EndpointApi#getEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet");
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
 **endpointId** | **String**|  |
 **appId** | **String**|  |

### Return type

[**EndpointStats**](EndpointStats.md)

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

<a name="listEndpointsApiV1AppAppIdEndpointGet"></a>
# **listEndpointsApiV1AppAppIdEndpointGet**
> ListResponseEndpointOut listEndpointsApiV1AppAppIdEndpointGet(appId, iterator, limit)

List Endpoints

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EndpointApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EndpointApi apiInstance = new EndpointApi(defaultClient);
    String appId = "appId_example"; // String | 
    String iterator = "iterator_example"; // String | 
    Integer limit = 50; // Integer | 
    try {
      ListResponseEndpointOut result = apiInstance.listEndpointsApiV1AppAppIdEndpointGet(appId, iterator, limit);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling EndpointApi#listEndpointsApiV1AppAppIdEndpointGet");
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
 **iterator** | **String**|  | [optional]
 **limit** | **Integer**|  | [optional] [default to 50]

### Return type

[**ListResponseEndpointOut**](ListResponseEndpointOut.md)

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

<a name="updateEndpointApiV1AppAppIdEndpointEndpointIdPut"></a>
# **updateEndpointApiV1AppAppIdEndpointEndpointIdPut**
> EndpointOut updateEndpointApiV1AppAppIdEndpointEndpointIdPut(endpointId, appId, endpointIn)

Update Endpoint

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EndpointApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EndpointApi apiInstance = new EndpointApi(defaultClient);
    String endpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2"; // String | 
    String appId = "appId_example"; // String | 
    EndpointIn endpointIn = new EndpointIn(); // EndpointIn | 
    try {
      EndpointOut result = apiInstance.updateEndpointApiV1AppAppIdEndpointEndpointIdPut(endpointId, appId, endpointIn);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling EndpointApi#updateEndpointApiV1AppAppIdEndpointEndpointIdPut");
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
 **endpointId** | **String**|  |
 **appId** | **String**|  |
 **endpointIn** | [**EndpointIn**](EndpointIn.md)|  |

### Return type

[**EndpointOut**](EndpointOut.md)

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

