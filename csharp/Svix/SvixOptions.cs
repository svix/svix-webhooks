namespace Svix
{
    public class SvixOptions
    {
        [Obsolete("BaseUrl is deprecated, please use ServerUrl instead.")]
        public string BaseUrl { get; }
        public string ServerUrl { get; }
        public List<int> RetryScheduleMilliseconds { get; } = [50, 100, 200];
        public int TimeoutMilliseconds { get; } = 15000;

        public SvixOptions(
            string serverUrl,
            int timeoutMilliseconds = 15000,
            List<int>? retryScheduleMilliseconds = null
        )
        {
            ServerUrl = serverUrl;
#pragma warning disable CS0618
            BaseUrl = serverUrl;
#pragma warning restore CS0618
            TimeoutMilliseconds = timeoutMilliseconds;
            retryScheduleMilliseconds ??= [50, 100, 200];
            if (retryScheduleMilliseconds.Count > 5)
            {
                throw new ArgumentException("number of retries must not exceed 5");
            }
            RetryScheduleMilliseconds = retryScheduleMilliseconds;
        }
    }
}
