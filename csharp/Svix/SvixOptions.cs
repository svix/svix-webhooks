namespace Svix
{
    public class SvixOptions
    {
        public string BaseUrl { get; }
        public List<int> RetryScheduleMilliseconds { get; } = [50, 100, 200];
        public int TimeoutMilliseconds { get; } = 15000;

        public SvixOptions(
            string baseUrl,
            int timeoutMilliseconds = 15000,
            List<int>? retryScheduleMilliseconds = null
        )
        {
            BaseUrl = baseUrl;
            TimeoutMilliseconds = timeoutMilliseconds;
            retryScheduleMilliseconds ??= [50, 100, 200];
            if (retryScheduleMilliseconds.Count > 5)
            {
                throw new ArgumentException("number of retries must not exceed 5");
            }
            RetryScheduleMilliseconds = retryScheduleMilliseconds;
        }

        public SvixOptions() { }
    }
}
