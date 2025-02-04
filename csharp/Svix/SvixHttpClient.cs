using Newtonsoft.Json;
using System.Net;
using System.Text;
using Newtonsoft.Json.Serialization;

namespace Svix
{
    public class SvixHttpClient(string token, SvixOptions options)
    {
        readonly SvixOptions _options = options;
        readonly HttpClient _httpClient = new();
        private readonly string _token = token;

        public ApiResponse<T> SendRequest<T>(
            HttpMethod method,
            string path,
            IDictionary<string, string>? pathParams = null,
            IDictionary<string, string>? queryParams = null,
            IDictionary<string, string>? headerParams = null,
            object? content = null
        )
        {
            var res = SendRequestAsync<T>(
                    method: method,
                    path: path,
                    pathParams: pathParams,
                    content: content,
                    queryParams: queryParams,
                    headerParams: headerParams
                )
                .GetAwaiter()
                .GetResult();
            return res;
        }

        public async Task<ApiResponse<T>> SendRequestAsync<T>(
            HttpMethod method,
            string path,
            IDictionary<string, string>? pathParams = null,
            IDictionary<string, string>? queryParams = null,
            IDictionary<string, string>? headerParams = null,
            object? content = null,
            CancellationToken cancellationToken = default
        )
        {
            var url = _options.BaseUrl;

            // Apply path parameters if provided
            if (pathParams != null)
            {
                var processedPath = path;
                foreach (var param in pathParams)
                {
                    processedPath = processedPath.Replace(
                        $"{{{param.Key}}}",
                        WebUtility.UrlEncode(param.Value)
                    );
                }
                url += processedPath;
            }
            else
            {
                url += path;
            }

            if (queryParams != null && queryParams.Count > 0)
            {
                url +=
                    "?"
                    + string.Join(
                        "&",
                        queryParams.Select(p =>
                            $"{WebUtility.UrlEncode(p.Key)}={WebUtility.UrlEncode(p.Value)}"
                        )
                    );
            }
            using var request = new HttpRequestMessage(method, url);
            if (headerParams != null)
            {
                foreach (KeyValuePair<string, string> entry in headerParams)
                {
                    request.Headers.Add(entry.Key, entry.Value);
                }
            }

            request.Headers.Add("Authorization", $"Bearer {_token}");
            uint req_id = (uint)new Random().NextInt64(0, (long)uint.MaxValue + 1);
            request.Headers.Add("svix-req-id", req_id.ToString());
            if (content != null)
            {
                var encoded_content = new StringContent(
                    JsonConvert.SerializeObject(content, new JsonSerializerSettings { NullValueHandling = NullValueHandling.Ignore }),
                    Encoding.UTF8,
                    "application/json"
                );
                request.Content = encoded_content;
            }
            var response = await _httpClient.SendAsync(request, cancellationToken);
            if (response.IsSuccessStatusCode)
            {
                if ((int)response.StatusCode == 204){
                    return new ApiResponse<T> { Data = (T)(object)true, StatusCode = response.StatusCode };
                }

                var responseContent = await response.Content.ReadAsStringAsync(cancellationToken);
                var data =
                    JsonConvert.DeserializeObject<T>(responseContent)
                    ?? throw new ApiException((int)response.StatusCode, $"Failed to deserialize response body");
                return new ApiResponse<T> { Data = data, StatusCode = response.StatusCode };
            }

            throw new ApiException((int)response.StatusCode, $"Request failed with status code {response.StatusCode}");
        }
    }

    public class ApiResponse<T>
    {
        public T Data { get; set; } = default!;
        public HttpStatusCode StatusCode { get; set; }
    }

    public class ApiException : Exception
    {
        public ApiException(int errorCode, string message) : base(message)
        {
            ErrorCode = errorCode;
        }
        public ApiException(int errorCode, string message, object errorContent = null, Dictionary<string, string> headers = null) : base(message)
        {
            ErrorCode = errorCode;
            ErrorContent = errorContent;
            Headers = headers;
        }


        public int ErrorCode { get; set; }
        public object? ErrorContent { get; private set; }
        public Dictionary<string, string>? Headers { get; private set; }


    }
}
