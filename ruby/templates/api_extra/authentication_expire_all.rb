def dashboard_access(app_id, options = {})
  options = options.transform_keys(&:to_s)
  res = @client.execute_request(
    "POST",
    "/api/v1/auth/dashboard-access/#{app_id}",
    headers: {
      "idempotency-key" => options["idempotency-key"]
    }
  )
  DashboardAccessOut.deserialize(res)
end
