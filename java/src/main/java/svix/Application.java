package svix;

import svix.ApiException;
import svix.openapi.ApplicationApi;
import svix.openapi.model.ApplicationIn;
import svix.openapi.model.ApplicationOut;
import svix.openapi.model.ListResponseApplicationOut;

public class Application {
	private final ApplicationApi api;

	Application() {
		api = new ApplicationApi();
	}

	public ListResponseApplicationOut list(String iterator, Integer limit) throws ApiException {
		return api.listApplicationsApiV1AppGet(iterator, limit);
	}

	public ApplicationOut create(String appId, ApplicationIn applicationIn) throws ApiException {
		return api.createApplicationApiV1AppPost(applicationIn);
	}

	public ApplicationOut get(String appId) throws ApiException {
		return api.getApplicationApiV1AppAppIdGet(appId);
	}

	public ApplicationOut update(String appId, ApplicationIn applicationIn) throws ApiException {
		return api.updateApplicationApiV1AppAppIdPut(appId, applicationIn);
	}

	public void delete(String appId) throws ApiException {
		api.deleteApplicationApiV1AppAppIdDelete(appId);
	}
}
