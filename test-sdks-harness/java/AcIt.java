import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.svix.AutoConfig;
import com.svix.AutoConfigConsumer;
import com.svix.exceptions.ApiException;
import com.svix.internalapi.MessagePollerv2ConsumerPollOptions;
import com.svix.models.DestinationOut;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;
import com.svix.models.PollerV2MessageOut;
import com.svix.models.PollerV2PollOut;
import com.svix.models.SinkInCommon;
import com.svix.models.StartingPosition;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class AcIt {
    public static void main(String[] args) throws Exception {
        String path = System.getenv("AUTOCONFIG_FIXTURES");
        JsonNode root = new ObjectMapper().readTree(Files.readString(Path.of(path)));
        JsonNode lang = root.get("languages").get("java");
        String serverUrl = root.get("serverUrl").asText();
        String orgToken = root.get("orgToken").asText();
        String eventType = root.get("eventType").asText();
        String httpUrl = root.get("httpUrl").asText();
        String consumerId = root.get("consumerId").asText();
        String appId = lang.get("appId").asText();

        System.out.println("LANG: java");
        System.out.println(runHttp("A", lang.get("v1Http").asText(), httpUrl, eventType));
        System.out.println(runHttp("B", lang.get("v2Http").asText(), httpUrl, eventType));
        System.out.println(
                runPoller(
                        "C",
                        lang.get("v1Poller").asText(),
                        eventType,
                        consumerId,
                        serverUrl,
                        orgToken,
                        appId,
                        "java-v1"));
        System.out.println(
                runPoller(
                        "D",
                        lang.get("v2Poller").asText(),
                        eventType,
                        consumerId,
                        serverUrl,
                        orgToken,
                        appId,
                        "java-v2"));
        System.out.println(
                runPollerExisting(
                        "E",
                        lang.get("v2PollerExisting").asText(),
                        eventType,
                        consumerId,
                        serverUrl,
                        orgToken,
                        appId,
                        "java-e"));
        System.out.println(
                "Notes: SinkInCommon with eventTypes; no filled optionals");
    }

    private static String runHttp(String label, String token, String httpUrl, String eventType) {
        try {
            EndpointIn endpoint =
                    new EndpointIn()
                            .url(URI.create(httpUrl))
                            .eventTypes(new LinkedHashSet<>(Set.of(eventType)));
            EndpointOut out = new AutoConfig(token, endpoint).subscribe();
            if (out == null || out.getId() == null || out.getId().isEmpty()) {
                return label + ": FAIL — subscribe returned empty id";
            }
            if (out.getUrl() == null || !httpUrl.equals(out.getUrl().toString())) {
                return label + ": FAIL — url mismatch got=" + out.getUrl() + " id=" + out.getId();
            }
            return label + ": PASS — id=" + out.getId() + " url=" + out.getUrl();
        } catch (ApiException e) {
            return label + ": FAIL — ApiException " + e.getCode() + " " + e.getMessage() + " "
                    + e.getResponseBody();
        } catch (Exception e) {
            return label + ": FAIL — " + e.getClass().getSimpleName() + ": " + e.getMessage();
        }
    }

    private static SinkInCommon minimalSink(String eventType) {
        return new SinkInCommon().eventTypes(Set.of(eventType));
    }

    private static String runPoller(
            String label,
            String token,
            String eventType,
            String consumerId,
            String serverUrl,
            String orgToken,
            String appId,
            String src) {
        DestinationOut dest;
        AutoConfigConsumer consumer;
        try {
            consumer = new AutoConfigConsumer(token, minimalSink(eventType));
            dest = consumer.subscribe();
        } catch (ApiException e) {
            if (e.getCode() == 501) {
                return label + ": FAIL — DIOM_MISSING " + e.getResponseBody();
            }
            return label + ": FAIL — subscribe ApiException " + e.getCode() + " " + e.getMessage()
                    + " " + e.getResponseBody();
        } catch (Exception e) {
            return label + ": FAIL — subscribe " + e.getClass().getSimpleName() + ": "
                    + e.getMessage();
        }

        if (dest == null || dest.getId() == null || dest.getId().isEmpty()) {
            return label + ": FAIL — subscribe returned empty dest id";
        }

        try {
            sendMsg(serverUrl, orgToken, appId, eventType, src);
        } catch (Exception e) {
            return label + ": FAIL — send msg " + e.getClass().getSimpleName() + ": "
                    + e.getMessage();
        }

        MessagePollerv2ConsumerPollOptions opts = new MessagePollerv2ConsumerPollOptions();
        opts.setStartingPosition(StartingPosition.EARLIEST);
        opts.setLeaseDurationMs(2000L);

        PollerV2MessageOut hit = null;
        String lastErr = null;
        for (int i = 0; i < 10; i++) {
            try {
                PollerV2PollOut poll = consumer.receive(consumerId, opts);
                if (poll != null && poll.getData() != null) {
                    for (PollerV2MessageOut msg : poll.getData()) {
                        if (eventType.equals(msg.getEventType()) && srcEquals(msg.getPayload(), src)) {
                            hit = msg;
                            break;
                        }
                    }
                }
                if (hit != null) {
                    break;
                }
                lastErr = "empty/no-match";
            } catch (ApiException e) {
                if (e.getCode() == 501) {
                    return label + ": FAIL — DIOM_MISSING dest=" + dest.getId() + " "
                            + e.getResponseBody();
                }
                lastErr = "receive ApiException " + e.getCode() + " " + e.getResponseBody();
            } catch (Exception e) {
                lastErr = "receive " + e.getClass().getSimpleName() + ": " + e.getMessage();
            }
            try {
                Thread.sleep(2000);
            } catch (InterruptedException ie) {
                Thread.currentThread().interrupt();
                return label + ": FAIL — interrupted";
            }
        }

        if (hit == null) {
            return label + ": FAIL — dest=" + dest.getId()
                    + " no matching message after 10x2s; last=" + lastErr;
        }

        long offset = hit.getOffset();
        try {
            consumer.commit(consumerId, offset);
        } catch (Exception e) {
            return label + ": FAIL — dest=" + dest.getId() + " commit offset=" + offset + " "
                    + e.getClass().getSimpleName() + ": " + e.getMessage();
        }

        try {
            PollerV2PollOut after = consumer.receive(consumerId, opts);
            if (after != null && after.getData() != null) {
                for (PollerV2MessageOut msg : after.getData()) {
                    if (msg.getOffset() != null && msg.getOffset() == offset) {
                        return label + ": FAIL — dest=" + dest.getId() + " committed offset "
                                + offset + " came back";
                    }
                }
            }
        } catch (Exception e) {
            return label + ": FAIL — dest=" + dest.getId() + " post-commit receive "
                    + e.getClass().getSimpleName() + ": " + e.getMessage();
        }

        return label + ": PASS — dest=" + dest.getId() + " offset=" + offset + " src=" + src;
    }

    private static String runPollerExisting(
            String label,
            String token,
            String eventType,
            String consumerId,
            String serverUrl,
            String orgToken,
            String appId,
            String src) {
        DestinationOut dest;
        AutoConfigConsumer consumer;
        try {
            AutoConfigConsumer binder = new AutoConfigConsumer(token, minimalSink(eventType));
            dest = binder.subscribe();
            if (dest == null || dest.getId() == null || dest.getId().isEmpty()) {
                return label + ": FAIL — subscribe returned empty dest id";
            }
            consumer = new AutoConfigConsumer(token, minimalSink(eventType));
        } catch (ApiException e) {
            if (e.getCode() == 501) {
                return label + ": FAIL — DIOM_MISSING " + e.getResponseBody();
            }
            return label + ": FAIL — subscribe ApiException " + e.getCode() + " " + e.getMessage()
                    + " " + e.getResponseBody();
        } catch (Exception e) {
            return label + ": FAIL — " + e.getClass().getSimpleName() + ": " + e.getMessage();
        }

        try {
            sendMsg(serverUrl, orgToken, appId, eventType, src);
        } catch (Exception e) {
            return label + ": FAIL — send msg " + e.getClass().getSimpleName() + ": "
                    + e.getMessage();
        }

        MessagePollerv2ConsumerPollOptions opts = new MessagePollerv2ConsumerPollOptions();
        opts.setStartingPosition(StartingPosition.EARLIEST);
        opts.setLeaseDurationMs(2000L);

        PollerV2MessageOut hit = null;
        String lastErr = null;
        for (int i = 0; i < 10; i++) {
            try {
                PollerV2PollOut poll = consumer.receive(consumerId, opts);
                if (poll != null && poll.getData() != null) {
                    for (PollerV2MessageOut msg : poll.getData()) {
                        if (eventType.equals(msg.getEventType()) && srcEquals(msg.getPayload(), src)) {
                            hit = msg;
                            break;
                        }
                    }
                }
                if (hit != null) {
                    break;
                }
                lastErr = "empty/no-match";
            } catch (ApiException e) {
                if (e.getCode() == 501) {
                    return label + ": FAIL — DIOM_MISSING " + e.getResponseBody();
                }
                lastErr = "receive ApiException " + e.getCode() + " " + e.getResponseBody();
            } catch (Exception e) {
                lastErr = "receive " + e.getClass().getSimpleName() + ": " + e.getMessage();
            }
            try {
                Thread.sleep(2000);
            } catch (InterruptedException ie) {
                Thread.currentThread().interrupt();
                return label + ": FAIL — interrupted";
            }
        }

        if (hit == null) {
            return label + ": FAIL — no matching message after 10x2s; last=" + lastErr;
        }

        long offset = hit.getOffset();
        try {
            consumer.commit(consumerId, offset);
        } catch (Exception e) {
            return label + ": FAIL — commit offset=" + offset + " "
                    + e.getClass().getSimpleName() + ": " + e.getMessage();
        }

        try {
            PollerV2PollOut after = consumer.receive(consumerId, opts);
            if (after != null && after.getData() != null) {
                for (PollerV2MessageOut msg : after.getData()) {
                    if (msg.getOffset() != null && msg.getOffset() == offset) {
                        return label + ": FAIL — committed offset " + offset + " came back";
                    }
                }
            }
        } catch (Exception e) {
            return label + ": FAIL — post-commit receive "
                    + e.getClass().getSimpleName() + ": " + e.getMessage();
        }

        return label + ": PASS — dest=" + dest.getId() + " offset=" + offset + " src=" + src
                + " (no subscribe on receiver)";
    }

    private static void sendMsg(
            String serverUrl, String orgToken, String appId, String eventType, String src)
            throws Exception {
        String body =
                "{\"eventType\":\""
                        + eventType
                        + "\",\"payload\":{\"ok\":true,\"src\":\""
                        + src
                        + "\"}}";
        HttpRequest req =
                HttpRequest.newBuilder()
                        .uri(URI.create(serverUrl + "/api/v1/app/" + appId + "/msg/"))
                        .header("Authorization", "Bearer " + orgToken)
                        .header("Content-Type", "application/json")
                        .POST(HttpRequest.BodyPublishers.ofString(body, StandardCharsets.UTF_8))
                        .build();
        HttpResponse<String> resp =
                HttpClient.newHttpClient().send(req, HttpResponse.BodyHandlers.ofString());
        if (resp.statusCode() < 200 || resp.statusCode() >= 300) {
            throw new RuntimeException("HTTP " + resp.statusCode() + " " + resp.body());
        }
    }

    private static boolean srcEquals(Object payload, String src) {
        if (payload instanceof Map) {
            Object v = ((Map<?, ?>) payload).get("src");
            return src.equals(String.valueOf(v));
        }
        return payload != null && payload.toString().contains("\"src\":\"" + src + "\"");
    }
}
