# MessageApi

All URIs are relative to *https://api.svix.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createMessageApiV1AppAppIdMsgPost**](MessageApi.md#createMessageApiV1AppAppIdMsgPost) | **POST** /api/v1/app/{app_id}/msg/ | Create Message
[**getMessageApiV1AppAppIdMsgMsgIdGet**](MessageApi.md#getMessageApiV1AppAppIdMsgMsgIdGet) | **GET** /api/v1/app/{app_id}/msg/{msg_id}/ | Get Message
[**listMessagesApiV1AppAppIdMsgGet**](MessageApi.md#listMessagesApiV1AppAppIdMsgGet) | **GET** /api/v1/app/{app_id}/msg/ | List Messages


<a name="createMessageApiV1AppAppIdMsgPost"></a>
# **createMessageApiV1AppAppIdMsgPost**
> MessageOut createMessageApiV1AppAppIdMsgPost(appId, messageIn)

Create Message

Creates a new message and schedules it to be sent. If the message includes an &#x60;event_id&#x60; and a message with this id already exists, a 409 conflict error will be returned.

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.MessageApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    MessageApi apiInstance = new MessageApi(defaultClient);
    String appId = "appId_example"; // String | 
    MessageIn messageIn = new MessageIn(); // MessageIn | 
    try {
      MessageOut result = apiInstance.createMessageApiV1AppAppIdMsgPost(appId, messageIn);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling MessageApi#createMessageApiV1AppAppIdMsgPost");
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
 **messageIn** | [**MessageIn**](MessageIn.md)|  |

### Return type

[**MessageOut**](MessageOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**202** | Successful Response |  -  |
**401** | Unauthorized |  -  |
**403** | Forbidden |  -  |
**404** | Not Found |  -  |
**409** | Conflict |  -  |
**422** | Validation Error |  -  |

<a name="getMessageApiV1AppAppIdMsgMsgIdGet"></a>
# **getMessageApiV1AppAppIdMsgMsgIdGet**
> MessageOut getMessageApiV1AppAppIdMsgMsgIdGet(msgId, appId)

Get Message

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.MessageApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    MessageApi apiInstance = new MessageApi(defaultClient);
    String msgId = "msgId_example"; // String | 
    String appId = "appId_example"; // String | 
    try {
      MessageOut result = apiInstance.getMessageApiV1AppAppIdMsgMsgIdGet(msgId, appId);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling MessageApi#getMessageApiV1AppAppIdMsgMsgIdGet");
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
 **msgId** | **String**|  |
 **appId** | **String**|  |

### Return type

[**MessageOut**](MessageOut.md)

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

<a name="listMessagesApiV1AppAppIdMsgGet"></a>
# **listMessagesApiV1AppAppIdMsgGet**
> ListResponseMessageOut listMessagesApiV1AppAppIdMsgGet(appId, iterator, limit)

List Messages

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.MessageApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    MessageApi apiInstance = new MessageApi(defaultClient);
    String appId = "appId_example"; // String | 
    String iterator = "iterator_example"; // String | 
    Integer limit = 50; // Integer | 
    try {
      ListResponseMessageOut result = apiInstance.listMessagesApiV1AppAppIdMsgGet(appId, iterator, limit);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling MessageApi#listMessagesApiV1AppAppIdMsgGet");
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

[**ListResponseMessageOut**](ListResponseMessageOut.md)

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

