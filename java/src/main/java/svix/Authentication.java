package svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.AuthenticationApi;
import com.svix.generated.model.DashboardAccessOut;

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
