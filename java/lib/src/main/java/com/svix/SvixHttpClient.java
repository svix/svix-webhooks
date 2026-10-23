package com.svix;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.svix.exceptions.ApiException;

import okhttp3.*;

import java.io.IOException;
import java.time.Duration;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.locks.LockSupport;
import java.util.UUID;

public class SvixHttpClient {
    private final HttpUrl baseUrl;
    private final Map<String, String> defaultHeaders;
    private final List<Long> retrySchedule;
    private final OkHttpClient client;
    private final ObjectMapper objectMapper;

    public SvixHttpClient(
            HttpUrl rawBaseUrl, Map<String, String> defaultHeaders, List<Long> retrySchedule) {
        String path = rawBaseUrl.encodedPath();
        if (path.length() > 1 && path.endsWith("/")) {
            String trimmedPath = path.replaceAll("/+$", "");
            this.baseUrl =
                    rawBaseUrl.newBuilder()
                            .encodedPath(trimmedPath.isEmpty() ? "/" : trimmedPath)
                            .build();
        } else {
            this.baseUrl = rawBaseUrl;
        }
        this.defaultHeaders = defaultHeaders;
        this.retrySchedule = retrySchedule;
        this.client = new OkHttpClient.Builder().callTimeout(Duration.ofSeconds(15)).build();

        this.objectMapper = Utils.getObjectMapper();
    }

    public HttpUrl.Builder newUrlBuilder() {
        return new HttpUrl.Builder()
                .scheme(baseUrl.scheme())
                .host(baseUrl.host())
                .port(baseUrl.port());
    }

    public HttpUrl getBaseUrl() {
        return baseUrl;
    }

    public <Req, Res> Res executeRequest(
            String method, HttpUrl url, Headers headers, Req reqBody, Class<Res> responseClass)
            throws ApiException, IOException {
        Request.Builder reqBuilder = new Request.Builder().url(url);

        // Handle request body
        String jsonBody = "";
        if (reqBody != null) {
            jsonBody = objectMapper.writeValueAsString(reqBody);
            RequestBody body = RequestBody.create(jsonBody, MediaType.parse("application/json"));
            reqBuilder.method(method, body);
        } else if (method.equals("POST") || method.equals("PUT") || method.equals("PATCH")) {
            reqBuilder.method(method, RequestBody.create(new byte[0]));
        } else {
            reqBuilder.method(method, null);
        }

        // Add default headers
        defaultHeaders.forEach(reqBuilder::addHeader);

        String idempotencyKey = headers == null ? null : headers.get("idempotency-key");
        if ((idempotencyKey == null || idempotencyKey.isEmpty()) && method.toUpperCase() == "POST") {
                reqBuilder.addHeader("idempotency-key", "auto_" + UUID.randomUUID().toString());
        }

        // Add custom headers if present
        if (headers != null) {
            headers.forEach(pair -> reqBuilder.addHeader(pair.getFirst(), pair.getSecond()));
        }

        reqBuilder.addHeader(
                "svix-req-id",
                String.valueOf(ThreadLocalRandom.current().nextLong(0, Long.MAX_VALUE)));

        Request request = reqBuilder.build();
        Response response = executeRequestWithRetry(request, jsonBody);

        if (response.body() == null) {
            throw new ApiException("Body is null", response.code(), "");
        }

        String bodyString = response.body().string();

        if (response.code() == 204) {
            return null;
        }

        if (response.code() >= 200 && response.code() < 300) {
            return objectMapper.readValue(bodyString, responseClass);
        }

        throw new ApiException(
                "Non 200 status code: `" + response.code() + "`", response.code(), bodyString);
    }

    private Response executeRequestWithRetry(Request request, String body) throws IOException {
        Request currentRequest = request;
        for (int retryCount = 0; retryCount < retrySchedule.size(); retryCount++) {
            try {
                Response response = client.newCall(currentRequest).execute();
                if (response.code() < 500) {
                    return response;
                }
                response.close();
            } catch (IOException e) {
                // Network errors (timeouts, connection failures, etc.) are retried
            }

            // Use LockSupport for precise parking instead of Thread.sleep
            LockSupport.parkNanos(retrySchedule.get(retryCount) * 1_000_000); // Convert ms to ns

            currentRequest =
                    request.newBuilder()
                            .header("svix-retry-count", String.valueOf(retryCount + 1))
                            .build();
        }
        return client.newCall(currentRequest).execute();
    }
}
