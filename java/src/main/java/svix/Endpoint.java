package svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.EndpointApi;
import com.svix.generated.model.EndpointIn;
import com.svix.generated.model.EndpointOut;
import com.svix.generated.model.EndpointSecret;
import com.svix.generated.model.ListResponseEndpointOut;

public class Endpoint {
	private final EndpointApi api;

	public Endpoint() {
		api = new EndpointApi();
	}

	public ListResponseEndpointOut list(String appId, String iterator, Integer limit) throws ApiException {
		return api.listEndpointsApiV1AppAppIdEndpointGet(appId, iterator, limit);
	}

	public EndpointOut create(String appId, EndpointIn endpointIn) throws ApiException {
		return api.createEndpointApiV1AppAppIdEndpointPost(appId, endpointIn);
	}

	public EndpointOut get(String appId, String endpointId) throws ApiException {
		return api.getEndpointApiV1AppAppIdEndpointEndpointIdGet(endpointId, appId);
	}

	public EndpointOut update(String appId, String endpointId, EndpointIn endpointIn) throws ApiException {
		return api.updateEndpointApiV1AppAppIdEndpointEndpointIdPut(endpointId, appId, endpointIn);
	}

	public void delete(String appId, String endpointId) throws ApiException {
		api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(endpointId, appId);
	}

	public EndpointSecret getSecret(String appId, String endpointId) throws ApiException {
		return api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(endpointId, appId);
	}
}
