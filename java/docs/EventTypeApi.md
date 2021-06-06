# EventTypeApi

All URIs are relative to *https://api.svix.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createEventTypeApiV1EventTypePost**](EventTypeApi.md#createEventTypeApiV1EventTypePost) | **POST** /api/v1/event-type/ | Create Event Type
[**deleteEventTypeApiV1EventTypeEventTypeNameDelete**](EventTypeApi.md#deleteEventTypeApiV1EventTypeEventTypeNameDelete) | **DELETE** /api/v1/event-type/{event_type_name}/ | Delete Event Type
[**listEventTypesApiV1EventTypeGet**](EventTypeApi.md#listEventTypesApiV1EventTypeGet) | **GET** /api/v1/event-type/ | List Event Types
[**updateEventTypeApiV1EventTypeEventTypeNamePut**](EventTypeApi.md#updateEventTypeApiV1EventTypeEventTypeNamePut) | **PUT** /api/v1/event-type/{event_type_name}/ | Update Event Type


<a name="createEventTypeApiV1EventTypePost"></a>
# **createEventTypeApiV1EventTypePost**
> EventTypeInOut createEventTypeApiV1EventTypePost(eventTypeInOut)

Create Event Type

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EventTypeApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EventTypeApi apiInstance = new EventTypeApi(defaultClient);
    EventTypeInOut eventTypeInOut = new EventTypeInOut(); // EventTypeInOut | 
    try {
      EventTypeInOut result = apiInstance.createEventTypeApiV1EventTypePost(eventTypeInOut);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling EventTypeApi#createEventTypeApiV1EventTypePost");
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
 **eventTypeInOut** | [**EventTypeInOut**](EventTypeInOut.md)|  |

### Return type

[**EventTypeInOut**](EventTypeInOut.md)

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

<a name="deleteEventTypeApiV1EventTypeEventTypeNameDelete"></a>
# **deleteEventTypeApiV1EventTypeEventTypeNameDelete**
> deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName)

Delete Event Type

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EventTypeApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EventTypeApi apiInstance = new EventTypeApi(defaultClient);
    String eventTypeName = "eventTypeName_example"; // String | 
    try {
      apiInstance.deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName);
    } catch (ApiException e) {
      System.err.println("Exception when calling EventTypeApi#deleteEventTypeApiV1EventTypeEventTypeNameDelete");
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
 **eventTypeName** | **String**|  |

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

<a name="listEventTypesApiV1EventTypeGet"></a>
# **listEventTypesApiV1EventTypeGet**
> ListResponseEventTypeInOut listEventTypesApiV1EventTypeGet(iterator, limit)

List Event Types

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EventTypeApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EventTypeApi apiInstance = new EventTypeApi(defaultClient);
    String iterator = "iterator_example"; // String | 
    Integer limit = 50; // Integer | 
    try {
      ListResponseEventTypeInOut result = apiInstance.listEventTypesApiV1EventTypeGet(iterator, limit);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling EventTypeApi#listEventTypesApiV1EventTypeGet");
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

[**ListResponseEventTypeInOut**](ListResponseEventTypeInOut.md)

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

<a name="updateEventTypeApiV1EventTypeEventTypeNamePut"></a>
# **updateEventTypeApiV1EventTypeEventTypeNamePut**
> EventTypeInOut updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate)

Update Event Type

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.EventTypeApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    EventTypeApi apiInstance = new EventTypeApi(defaultClient);
    String eventTypeName = "eventTypeName_example"; // String | 
    EventTypeUpdate eventTypeUpdate = new EventTypeUpdate(); // EventTypeUpdate | 
    try {
      EventTypeInOut result = apiInstance.updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling EventTypeApi#updateEventTypeApiV1EventTypeEventTypeNamePut");
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
 **eventTypeName** | **String**|  |
 **eventTypeUpdate** | [**EventTypeUpdate**](EventTypeUpdate.md)|  |

### Return type

[**EventTypeInOut**](EventTypeInOut.md)

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

