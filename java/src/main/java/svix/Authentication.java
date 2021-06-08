package svix;

import openapi.svix.ApiException;
import openapi.svix.api.AuthenticationApi;
import openapi.svix.model.DashboardAccessOut;

public class Authentication {
	private final AuthenticationApi api;

	Authentication() {
		api = new AuthenticationApi();
	}

	public DashboardAccessOut dashboardAccess(String appId) throws ApiException {
		return api.getDashboardAccessApiV1AuthDashboardAccessAppIdPost(appId);
	}

	public void logout() throws ApiException {
		api.logoutApiV1AuthLogoutPost();
	}
}
