import time
import whsaas.openapi_client
from pprint import pprint
from whsaas.openapi_client.api import application_api
from whsaas.openapi_client.model.application_in import ApplicationIn
from whsaas.openapi_client.model.application_out import ApplicationOut
from whsaas.openapi_client.model.http_validation_error import HTTPValidationError
from whsaas.openapi_client.model.http_error_out import HttpErrorOut
from whsaas.openapi_client.model.list_response_application_out import ListResponseApplicationOut
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = whsaas.openapi_client.Configuration(
        host = "http://localhost:8040"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: APIKeyHeader
configuration.api_key['APIKeyHeader'] = 'qqqqqqqq'

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['APIKeyHeader'] = 'Bearer'


# Enter a context with an instance of the API client
with whsaas.openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = application_api.ApplicationApi(api_client)

    try:
        # Create Application
        api_response = api_instance.list_applications_api_v1_app_get()
        pprint(api_response)
    except whsaas.openapi_client.ApiException as e:
        print("Exception when calling ApplicationApi->create_application_api_v1_app_post: %s\n" % e)
