using System.Collections.Generic;
using System.Net;
using System.Text;
using System.Text.Json;

namespace Svix
{
    public class SvixHttpClient
    {
        readonly string _baseUrl;
        readonly HttpClient _httpClient;
        private readonly string _token;

        public SvixHttpClient(string baseUrl, string token)
        {
            _baseUrl = baseUrl;
            _httpClient = new HttpClient();
            _token = token;
        }

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
            var url = _baseUrl;

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
            Console.WriteLine(url);
            using var request = new HttpRequestMessage(method, url);
            if (headerParams != null)
            {
                foreach (KeyValuePair<string, string> entry in headerParams)
                {
                    request.Headers.Add(entry.Key, entry.Value);
                }
            }

            request.Headers.Add("Authorization", $"Bearer {_token}");
            if (content != null)
            {
                request.Content = new StringContent(
                    JsonSerializer.Serialize(content),
                    Encoding.UTF8,
                    "application/json"
                );
            }

            var response = await _httpClient.SendAsync(request, cancellationToken);
            Console.WriteLine(response.ToString());
            Console.WriteLine(await response.Content.ReadAsStringAsync());
            if (response.IsSuccessStatusCode)
            {
                var responseContent = await response.Content.ReadAsStringAsync(cancellationToken);
                var data =
                    JsonSerializer.Deserialize<T>(responseContent)
                    ?? throw new ApiException($"Failed to deserialize response body");
                return new ApiResponse<T> { Data = data, StatusCode = response.StatusCode };
            }

            throw new ApiException($"Request failed with status code {response.StatusCode}");
        }
    }

    public class ApiResponse<T>
    {
        public T Data { get; set; } = default!;
        public HttpStatusCode StatusCode { get; set; }
    }

    public class ApiException : Exception
    {
        public ApiException(string message)
            : base(message) { }
    }
}
