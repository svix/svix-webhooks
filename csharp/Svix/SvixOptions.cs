namespace Svix
{
    public class SvixOptions
    {
        [Obsolete("BaseUrl is deprecated, please use ServerUrl instead.")]
        public string? BaseUrl
        {
            get => ServerUrl;
        }
        public string? ServerUrl { get; }
        public List<int> RetryScheduleMilliseconds { get; } = [50, 100, 200];
        public int TimeoutMilliseconds { get; } = 15000;

        /// <param name="serverUrl">The server URL to connect to.</param>
        /// <param name="baseUrl">[Deprecated] Please use serverUrl parameter instead.</param>
#pragma warning disable CS0618
        public SvixOptions(
            string? serverUrl = null,
            int timeoutMilliseconds = 15000,
            List<int>? retryScheduleMilliseconds = null,
            string? baseUrl = null
        )
        {
            ServerUrl = serverUrl ?? baseUrl;
            TimeoutMilliseconds = timeoutMilliseconds;
            retryScheduleMilliseconds ??= [50, 100, 200];
            if (retryScheduleMilliseconds.Count > 5)
            {
                throw new ArgumentException("number of retries must not exceed 5");
            }
            RetryScheduleMilliseconds = retryScheduleMilliseconds;
        }
#pragma warning restore CS0618
    }
}
