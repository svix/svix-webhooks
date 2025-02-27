/**
 * List background tasks executed in the past 90 days.
 *
 * @deprecated Use list instead.
 * */
public listByEndpoint(
  options?: BackgroundTaskListOptions
): Promise<ListResponseBackgroundTaskOut> {
  return this.list(options);
}
