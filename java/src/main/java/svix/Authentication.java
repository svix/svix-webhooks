package svix;

import svix.ApiException;
import svix.openapi.AuthenticationApi;
import svix.openapi.model.DashboardAccessOut;

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
