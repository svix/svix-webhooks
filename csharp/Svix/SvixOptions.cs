namespace Svix
{
    public class SvixOptions
    {
        public string BaseUrl;
        public List<int> RetryScheduleMilliseconds = [50, 100, 200];
        public int TimeoutMilliseconds = 15000;

        public SvixOptions(
            string baseUrl,
            List<int>? retryScheduleMilliseconds = null,
            int timeoutMilliseconds = 15000
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
