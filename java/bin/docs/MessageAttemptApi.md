# MessageAttemptApi

All URIs are relative to *https://api.svix.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet**](MessageAttemptApi.md#getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet) | **GET** /api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/ | Get Attempt
[**listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet**](MessageAttemptApi.md#listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet) | **GET** /api/v1/app/{app_id}/msg/{msg_id}/endpoint/ | List Attempted Destinations
[**listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet**](MessageAttemptApi.md#listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet) | **GET** /api/v1/app/{app_id}/endpoint/{endpoint_id}/msg/ | List Attempted Messages
[**listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet**](MessageAttemptApi.md#listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet) | **GET** /api/v1/app/{app_id}/msg/{msg_id}/attempt/ | List Attempts
[**listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet**](MessageAttemptApi.md#listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet) | **GET** /api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/attempt/ | List Attempts For Endpoint
[**resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost**](MessageAttemptApi.md#resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost) | **POST** /api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/resend/ | Resend Webhook


<a name="getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet"></a>
# **getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet**
> MessageAttemptOut getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId)

Get Attempt

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.MessageAttemptApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    MessageAttemptApi apiInstance = new MessageAttemptApi(defaultClient);
    String attemptId = "atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2"; // String | 
    String msgId = "msgId_example"; // String | 
    String appId = "appId_example"; // String | 
    try {
      MessageAttemptOut result = apiInstance.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling MessageAttemptApi#getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet");
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
 **attemptId** | **String**|  |
 **msgId** | **String**|  |
 **appId** | **String**|  |

### Return type

[**MessageAttemptOut**](MessageAttemptOut.md)

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

<a name="listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet"></a>
# **listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet**
> ListResponseMessageEndpointOut listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(msgId, appId, iterator, limit)

List Attempted Destinations

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.MessageAttemptApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    MessageAttemptApi apiInstance = new MessageAttemptApi(defaultClient);
    String msgId = "msgId_example"; // String | 
    String appId = "appId_example"; // String | 
    String iterator = "iterator_example"; // String | 
    Integer limit = 50; // Integer | 
    try {
      ListResponseMessageEndpointOut result = apiInstance.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(msgId, appId, iterator, limit);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling MessageAttemptApi#listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet");
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
 **iterator** | **String**|  | [optional]
 **limit** | **Integer**|  | [optional] [default to 50]

### Return type

[**ListResponseMessageEndpointOut**](ListResponseMessageEndpointOut.md)

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

<a name="listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet"></a>
# **listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet**
> ListResponseEndpointMessageOut listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(endpointId, appId, iterator, limit, status)

List Attempted Messages

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.MessageAttemptApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    MessageAttemptApi apiInstance = new MessageAttemptApi(defaultClient);
    String endpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2"; // String | 
    String appId = "appId_example"; // String | 
    String iterator = "iterator_example"; // String | 
    Integer limit = 50; // Integer | 
    MessageStatus status = MessageStatus.fromValue("0"); // MessageStatus | 
    try {
      ListResponseEndpointMessageOut result = apiInstance.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(endpointId, appId, iterator, limit, status);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling MessageAttemptApi#listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet");
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
 **iterator** | **String**|  | [optional]
 **limit** | **Integer**|  | [optional] [default to 50]
 **status** | [**MessageStatus**](.md)|  | [optional] [enum: 0, 1, 2]

### Return type

[**ListResponseEndpointMessageOut**](ListResponseEndpointMessageOut.md)

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

<a name="listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet"></a>
# **listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet**
> ListResponseMessageAttemptOut listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(msgId, appId, iterator, limit, status)

List Attempts

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.MessageAttemptApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    MessageAttemptApi apiInstance = new MessageAttemptApi(defaultClient);
    String msgId = "msgId_example"; // String | 
    String appId = "appId_example"; // String | 
    String iterator = "iterator_example"; // String | 
    Integer limit = 50; // Integer | 
    MessageStatus status = MessageStatus.fromValue("0"); // MessageStatus | 
    try {
      ListResponseMessageAttemptOut result = apiInstance.listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(msgId, appId, iterator, limit, status);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling MessageAttemptApi#listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet");
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
 **iterator** | **String**|  | [optional]
 **limit** | **Integer**|  | [optional] [default to 50]
 **status** | [**MessageStatus**](.md)|  | [optional] [enum: 0, 1, 2]

### Return type

[**ListResponseMessageAttemptOut**](ListResponseMessageAttemptOut.md)

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

<a name="listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet"></a>
# **listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet**
> ListResponseMessageAttemptEndpointOut listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(msgId, appId, endpointId, iterator, limit, status)

List Attempts For Endpoint

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.MessageAttemptApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    MessageAttemptApi apiInstance = new MessageAttemptApi(defaultClient);
    String msgId = "msgId_example"; // String | 
    String appId = "appId_example"; // String | 
    String endpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2"; // String | 
    String iterator = "iterator_example"; // String | 
    Integer limit = 50; // Integer | 
    MessageStatus status = MessageStatus.fromValue("0"); // MessageStatus | 
    try {
      ListResponseMessageAttemptEndpointOut result = apiInstance.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(msgId, appId, endpointId, iterator, limit, status);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling MessageAttemptApi#listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet");
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
 **endpointId** | **String**|  |
 **iterator** | **String**|  | [optional]
 **limit** | **Integer**|  | [optional] [default to 50]
 **status** | [**MessageStatus**](.md)|  | [optional] [enum: 0, 1, 2]

### Return type

[**ListResponseMessageAttemptEndpointOut**](ListResponseMessageAttemptEndpointOut.md)

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

<a name="resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost"></a>
# **resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost**
> Object resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId)

Resend Webhook

Resend a message to the specified endpoint.

### Example
```java
// Import classes:
import svix.ApiClient;
import svix.ApiException;
import svix.Configuration;
import svix.auth.*;
import svix.models.*;
import svix.openapi.MessageAttemptApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://api.svix.com");
    
    // Configure HTTP bearer authorization: HTTPBearer
    HttpBearerAuth HTTPBearer = (HttpBearerAuth) defaultClient.getAuthentication("HTTPBearer");
    HTTPBearer.setBearerToken("BEARER TOKEN");

    MessageAttemptApi apiInstance = new MessageAttemptApi(defaultClient);
    String endpointId = "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2"; // String | 
    String msgId = "msgId_example"; // String | 
    String appId = "appId_example"; // String | 
    try {
      Object result = apiInstance.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId);
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling MessageAttemptApi#resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost");
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
 **msgId** | **String**|  |
 **appId** | **String**|  |

### Return type

**Object**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

 - **Content-Type**: Not defined
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

