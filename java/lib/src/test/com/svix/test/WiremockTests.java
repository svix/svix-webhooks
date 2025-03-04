package com.svix.test;

import static com.github.tomakehurst.wiremock.client.WireMock.*;
import static com.github.tomakehurst.wiremock.core.WireMockConfiguration.wireMockConfig;

import static org.junit.Assert.*;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.github.tomakehurst.wiremock.client.WireMock;
import com.github.tomakehurst.wiremock.junit.WireMockRule;
import com.github.tomakehurst.wiremock.stubbing.ServeEvent;
import com.github.tomakehurst.wiremock.verification.LoggedRequest;
import com.svix.Svix;
import com.svix.SvixOptions;
import com.svix.api.*;
import com.svix.exceptions.ApiException;
import com.svix.models.*;

import org.junit.*;

import java.time.Instant;
import java.time.OffsetDateTime;
import java.time.ZoneId;
import java.time.ZoneOffset;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;

public class WiremockTests {
    @Rule
    public WireMockRule wireMockRule =
            new WireMockRule(
                    wireMockConfig().dynamicPort().withRootDirectory("lib/src/test/resources"));

    @After
    public void afterAll() {
        assertEquals(0, wireMockRule.findAllUnmatchedRequests().size());
        wireMockRule.resetAll();
    }

    private Svix testClient() {
        SvixOptions opts = new SvixOptions();
        opts.setServerUrl(wireMockRule.baseUrl());
        return new Svix("testtk_asd12.eu", opts);
    }

    @Test
    public void applicationGetOrCreate() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.post(urlEqualTo("/api/v1/app?get_if_exists=true"))
                        .willReturn(WireMock.ok().withBodyFile("ApplicationOut.json")));

        svx.getApplication().getOrCreate(new ApplicationIn().name("test"));

        wireMockRule.verify(1, postRequestedFor(urlEqualTo("/api/v1/app?get_if_exists=true")));
    }

    @Test
    public void testDateUTCInQueryParam() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlMatching("/api/v1/app/ap/attempt/endpoint/endp?.*"))
                        .willReturn(
                                WireMock.ok().withBodyFile("ListResponseMessageAttemptOut.json")));

        Instant instant = Instant.ofEpochSecond(1740240455, 514412223);
        OffsetDateTime before = instant.atOffset(ZoneOffset.UTC);

        MessageAttemptListByEndpointOptions opts = new MessageAttemptListByEndpointOptions();
        opts.setBefore(before);
        svx.getMessageAttempt().listByEndpoint("ap", "endp", opts);

        wireMockRule.verify(
                1,
                getRequestedFor(
                        urlEqualTo(
                                "/api/v1/app/ap/attempt/endpoint/endp?before=2025-02-22T16%3A07%3A35.514412223Z")));
    }

    @Test
    public void testDateESTInQueryParam() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlMatching("/api/v1/app/ap/attempt/endpoint/endp?.*"))
                        .willReturn(
                                WireMock.ok().withBodyFile("ListResponseMessageAttemptOut.json")));

        Instant instant = Instant.ofEpochSecond(1740240455, 514412223);
        OffsetDateTime before = instant.atZone(ZoneId.of("America/New_York")).toOffsetDateTime();
        MessageAttemptListByEndpointOptions opts = new MessageAttemptListByEndpointOptions();
        opts.setBefore(before);

        svx.getMessageAttempt().listByEndpoint("ap", "endp", opts);

        wireMockRule.verify(
                1,
                getRequestedFor(
                        urlEqualTo(
                                "/api/v1/app/ap/attempt/endpoint/endp?before=2025-02-22T11%3A07%3A35.514412223-05%3A00")));
    }

    @Test
    public void testDateESTRequestBody() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.post(urlEqualTo("/api/v1/app/ap/endpoint/ep/replay-missing"))
                        .willReturn(WireMock.ok().withBodyFile("ReplayOut.json")));

        Instant instant = Instant.ofEpochSecond(1740240455, 514412223);
        OffsetDateTime since = instant.atZone(ZoneId.of("America/New_York")).toOffsetDateTime();

        ReplayIn in = new ReplayIn();
        in.setSince(since);
        svx.getEndpoint().replayMissing("ap", "ep", in);

        wireMockRule.verify(
                1,
                postRequestedFor(urlEqualTo("/api/v1/app/ap/endpoint/ep/replay-missing"))
                        .withRequestBody(
                                equalTo("{\"since\":\"2025-02-22T11:07:35.514412223-05:00\"}")));
    }

    @Test
    public void testDateUTCRequestBody() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.post(urlEqualTo("/api/v1/app/ap/endpoint/ep/replay-missing"))
                        .willReturn(WireMock.ok().withBodyFile("ReplayOut.json")));
        Instant instant = Instant.ofEpochSecond(1740240455, 514412223);
        OffsetDateTime since = instant.atOffset(ZoneOffset.UTC);

        ReplayIn in = new ReplayIn();
        in.setSince(since);
        svx.getEndpoint().replayMissing("ap", "ep", in);
        wireMockRule.verify(
                1,
                postRequestedFor(urlEqualTo("/api/v1/app/ap/endpoint/ep/replay-missing"))
                        .withRequestBody(
                                equalTo("{\"since\":\"2025-02-22T16:07:35.514412223Z\"}")));
    }

    @Test
    public void testDateResponseBody() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlMatching("/api/v1/app/ap/attempt/endpoint/endp?.*"))
                        .willReturn(
                                WireMock.ok().withBodyFile("ListResponseMessageAttemptOut.json")));

        Instant instant = Instant.ofEpochSecond(1740240455, 514412223);
        OffsetDateTime timestamp = instant.atOffset(ZoneOffset.UTC);
        ListResponseMessageAttemptOut out = svx.getMessageAttempt().listByEndpoint("ap", "endp");

        assertEquals(timestamp, out.getData().get(0).getTimestamp());
    }

    @Test
    public void testListResponseOutDeserializesCorrectly() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlEqualTo("/api/v1/app/ap/attempt/endpoint/endp"))
                        .willReturn(
                                WireMock.ok()
                                        .withBody(
                                                "{\"data\":[],\"iterator\":null,\"prevIterator\":null,\"done\":true}")));

        ListResponseMessageAttemptOut res = svx.getMessageAttempt().listByEndpoint("ap", "endp");
        assertNull(res.getIterator());

        wireMockRule.verify(1, getRequestedFor(urlEqualTo("/api/v1/app/ap/attempt/endpoint/endp")));
    }

    @Test
    public void testEnumAsQueryParam() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlEqualTo("/api/v1/app?order=ascending"))
                        .willReturn(WireMock.ok().withBodyFile("ListResponseApplicationOut.json")));

        ApplicationListOptions opts = new ApplicationListOptions();
        opts.setOrder(Ordering.ASCENDING);

        svx.getApplication().list(opts);

        wireMockRule.verify(1, getRequestedFor(urlEqualTo("/api/v1/app?order=ascending")));
    }

    @Test
    public void testListAsQueryParam() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlEqualTo("/api/v1/app/app1/msg?event_types=val1%2Cval5%2Cval8"))
                        .willReturn(WireMock.ok().withBodyFile("ListResponseMessageOut.json")));
        MessageListOptions opts = new MessageListOptions();
        HashSet<String> eventTypes = new HashSet<>();
        eventTypes.add("val1");
        eventTypes.add("val8");
        eventTypes.add("val5");

        opts.setEventTypes(eventTypes);
        svx.getMessage().list("app1", opts);

        wireMockRule.verify(
                1,
                getRequestedFor(urlEqualTo("/api/v1/app/app1/msg?event_types=val1%2Cval5%2Cval8")));
    }

    @Test
    public void testHeaderParamSent() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.post(urlEqualTo("/api/v1/app"))
                        .willReturn(WireMock.ok().withBodyFile("ApplicationOut.json")));

        ApplicationIn app = new ApplicationIn();
        app.setName("name");
        ApplicationCreateOptions opts = new ApplicationCreateOptions();
        opts.setIdempotencyKey("random-key");
        svx.getApplication().create(app, opts);

        wireMockRule.verify(
                1,
                postRequestedFor(urlEqualTo("/api/v1/app"))
                        .withHeader("idempotency-key", equalTo("random-key")));
    }

    @Test
    public void testRetryForStatus500() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlEqualTo("/api/v1/app")).willReturn(WireMock.status(500)));

        try {
            svx.getApplication().list();
        } catch (ApiException e) {
            assertEquals(500, e.getCode());
        }
        List<ServeEvent> requests = wireMockRule.getAllServeEvents();
        assertEquals(4, requests.size());

        String req_id = requests.get(0).getRequest().header("svix-req-id").firstValue();

        for (int i = 0; i < requests.size(); i++) {
            // all retries have the same req id
            LoggedRequest req = requests.get(0).getRequest();
            assertEquals(req_id, req.header("svix-req-id").firstValue());
            if (i == 0) {
                // first request does not have svix-retry-count
                wireMockRule.verify(
                        1,
                        getRequestedFor(urlEqualTo("/api/v1/app"))
                                .withoutHeader("svix-retry-count"));
            } else {
                wireMockRule.verify(
                        1,
                        getRequestedFor(urlEqualTo("/api/v1/app"))
                                .withHeader("svix-retry-count", equalTo(Integer.toString(i))));
            }
        }

        wireMockRule.verify(
                4,
                getRequestedFor(urlEqualTo("/api/v1/app"))
                        .withHeader("svix-req-id", equalTo(req_id)));
    }

    @Test
    public void noBodyInResponseDoesNotReturnAnything() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.delete(urlEqualTo("/api/v1/app/ap")).willReturn(WireMock.status(204)));

        svx.getApplication().delete("ap");
        wireMockRule.verify(1, deleteRequestedFor(urlEqualTo("/api/v1/app/ap")));
    }

    @Test
    public void subResourceWorks() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.delete(urlEqualTo("/api/v1/operational-webhook/endpoint/ep"))
                        .willReturn(WireMock.status(204)));

        svx.getOperationalWebhookEndpoint().delete("ep");
        wireMockRule.verify(
                1, deleteRequestedFor(urlEqualTo("/api/v1/operational-webhook/endpoint/ep")));
    }

    @Test
    public void integerEnumDeserialization() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlMatching("/api/v1/app/ap/attempt/endpoint/endp"))
                        .willReturn(
                                WireMock.ok().withBodyFile("ListResponseMessageAttemptOut.json")));

        ListResponseMessageAttemptOut res = svx.getMessageAttempt().listByEndpoint("ap", "endp");

        assertEquals(MessageAttemptTriggerType.SCHEDULED, res.getData().get(0).getTriggerType());
        wireMockRule.verify(1, getRequestedFor(urlEqualTo("/api/v1/app/ap/attempt/endpoint/endp")));
    }

    @Test
    public void stringEnumDeSerialization() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlEqualTo("/api/v1/background-task"))
                        .willReturn(
                                WireMock.ok().withBodyFile("ListResponseBackgroundTaskOut.json")));

        ListResponseBackgroundTaskOut res = svx.getBackgroundTask().list();
        assertEquals(BackgroundTaskType.ENDPOINT_REPLAY, res.getData().get(0).getTask());

        wireMockRule.verify(1, getRequestedFor(urlEqualTo("/api/v1/background-task")));
    }

    @Test
    public void nonCamelCaseFieldName() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.post(urlEqualTo("/api/v1/event-type/import/openapi"))
                        .willReturn(WireMock.ok().withBodyFile("EventTypeImportOpenApiOut.json")));
        EventTypeImportOpenApiIn in = new EventTypeImportOpenApiIn();
        in.setDryRun(true);
        EventTypeImportOpenApiOut res = svx.getEventType().importOpenapi(in);

        // toModify is not camelCase
        assertEquals(1, res.getData().getToModify().size());

        wireMockRule.verify(1, postRequestedFor(urlEqualTo("/api/v1/event-type/import/openapi")));
    }

    @Test
    public void patchRequestBodyNullField() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.patch(urlEqualTo("/api/v1/app/ap/endpoint/endp"))
                        .willReturn(WireMock.ok().withBodyFile("EndpointOut.json")));

        svx.getEndpoint().patch("ap", "endp", new EndpointPatch().rateLimit(null));

        wireMockRule.verify(
                1,
                patchRequestedFor(urlEqualTo("/api/v1/app/ap/endpoint/endp"))
                        .withRequestBody(equalTo("{\"rateLimit\":null}")));
    }

    @Test
    public void patchRequestBodyUnsetField() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.patch(urlEqualTo("/api/v1/app/ap/endpoint/endp"))
                        .willReturn(WireMock.ok().withBodyFile("EndpointOut.json")));

        svx.getEndpoint().patch("ap", "endp", new EndpointPatch().description("test"));

        wireMockRule.verify(
                1,
                patchRequestedFor(urlEqualTo("/api/v1/app/ap/endpoint/endp"))
                        .withRequestBody(equalTo("{\"description\":\"test\"}")));
    }

    @Test
    public void patchRequestBodySetField() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.patch(urlEqualTo("/api/v1/app/ap/endpoint/endp"))
                        .willReturn(WireMock.ok().withBodyFile("EndpointOut.json")));

        svx.getEndpoint().patch("ap", "endp", new EndpointPatch().rateLimit(123L));

        wireMockRule.verify(
                1,
                patchRequestedFor(urlEqualTo("/api/v1/app/ap/endpoint/endp"))
                        .withRequestBody(equalTo("{\"rateLimit\":123}")));
    }

    @Test
    public void arbitraryJsonObjectBody() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.post(urlEqualTo("/api/v1/app/app1/msg"))
                        .willReturn(WireMock.ok().withBodyFile("MessageOut.json")));

        MessageIn msg = new MessageIn();
        HashMap<String, String> map = new HashMap<>();
        map.put("key", "val");
        map.put("key1", "val");
        map.put("key2", "val");
        ObjectMapper mapper = new ObjectMapper();
        String json = mapper.writeValueAsString(map);
        msg.setPayload(json);

        svx.getMessage().create("app1", msg);

        // file in java/lib/src/test/resources/__files/ExpectedMsgCreateBody.json
        String expectedBody =
                wireMockRule
                        .getOptions()
                        .filesRoot()
                        .getTextFileNamed("__files/ExpectedMsgCreateBody.json")
                        .readContentsAsString();

        wireMockRule.verify(
                1,
                postRequestedFor(urlEqualTo("/api/v1/app/app1/msg"))
                        .withRequestBody(equalTo(expectedBody)));
    }

    @Test
    public void tokenAndUserAgentIsSent() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.delete(urlEqualTo("/api/v1/app/app1")).willReturn(WireMock.status(204)));

        svx.getApplication().delete("app1");

        wireMockRule.verify(
                1,
                deleteRequestedFor(urlEqualTo("/api/v1/app/app1"))
                        .withHeader("Authorization", equalTo("Bearer testtk_asd12.eu")));
        wireMockRule.verify(
                1,
                deleteRequestedFor(urlEqualTo("/api/v1/app/app1"))
                        .withHeader("User-Agent", matching("svix-libs/.*/java")));
    }

    @Test
    public void msgInRaw() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.post(urlEqualTo("/api/v1/app/app1/msg"))
                        .willReturn(WireMock.ok().withBodyFile("MessageOut.json")));

        MessageIn msg = Message.messageInRaw("<xml>{no json here}").eventType("event.ended");
        svx.getMessage().create("app1", msg);

        String expectedBody =
                "{\"eventType\":\"event.ended\",\"payload\":{},\"transformationsParams\":{\"rawPayload\":\"<xml>{no"
                    + " json here}\"}}";
        wireMockRule.verify(
                1,
                postRequestedFor(urlEqualTo("/api/v1/app/app1/msg"))
                        .withRequestBody(equalTo(expectedBody)));
    }

    @Test
    public void octothorpeInUrlQuery() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.get(urlEqualTo("/api/v1/app/app1/msg?tag=test%23test"))
                        .willReturn(WireMock.ok().withBodyFile("ListResponseMessageOut.json")));

        MessageListOptions opts = new MessageListOptions();
        opts.setTag("test#test");
        svx.getMessage().list("app1", opts);

        wireMockRule.verify(1, getRequestedFor(urlEqualTo("/api/v1/app/app1/msg?tag=test%23test")));
    }

    @Test
    public void headersInTransformationParamsNotOverwritten() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.post(urlEqualTo("/api/v1/app/app1/msg"))
                        .willReturn(WireMock.ok().withBodyFile("MessageOut.json")));

        String jsonPayload = "{\"key\":\"val\",\"key1\":[\"list\"]}";
        MessageIn msg = new MessageIn().payload(jsonPayload).eventType("event.type").transformationsParams(Map.of("headers",Map.of( "header-key","header-val")));
        svx.getMessage().create("app1", msg);

        // file in java/lib/src/test/resources/__files/ExpectedMsgCreateBodyWithHeaders.json
        String expectedBody =
                wireMockRule
                        .getOptions()
                        .filesRoot()
                        .getTextFileNamed("__files/ExpectedMsgCreateBodyWithHeaders.json")
                        .readContentsAsString();
        wireMockRule.verify(
                1,
                postRequestedFor(urlEqualTo("/api/v1/app/app1/msg"))
                        .withRequestBody(equalTo(expectedBody)));
    }
    @Test
    public void jsonEncodedMessageIn() throws Exception {
        Svix svx = testClient();
        wireMockRule.stubFor(
                WireMock.post(urlEqualTo("/api/v1/app/app1/msg"))
                        .willReturn(WireMock.ok().withBodyFile("MessageOut.json")));

        String jsonPayload = "{\"key\":\"val\",\"key1\":[\"list\"]}";
        MessageIn msg = new MessageIn().payload(jsonPayload).eventType("event.type");
        svx.getMessage().create("app1", msg);

        // file in java/lib/src/test/resources/__files/ExpectedMsgCreateBody2.json
        String expectedBody =
                wireMockRule
                        .getOptions()
                        .filesRoot()
                        .getTextFileNamed("__files/ExpectedMsgCreateBody2.json")
                        .readContentsAsString();
        wireMockRule.verify(
                1,
                postRequestedFor(urlEqualTo("/api/v1/app/app1/msg"))
                        .withRequestBody(equalTo(expectedBody)));
    }

}
