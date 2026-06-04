// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.svix.Utils;

import lombok.*;

import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
public abstract class StreamSinkPatchConfig {
    @JsonIgnore
    public String getVariantName() {
        VariantName annotation = this.getClass().getAnnotation(VariantName.class);
        return annotation != null ? annotation.value() : null;
    }

    public abstract JsonNode toJsonNode();

    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("poller")
    public static class Poller extends StreamSinkPatchConfig {
        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().createObjectNode();
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("azureBlobStorage")
    public static class AzureBlobStorage extends StreamSinkPatchConfig {
        private final AzureBlobStoragePatchConfig azureBlobStorage;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(azureBlobStorage);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("otelTracing")
    public static class OtelTracing extends StreamSinkPatchConfig {
        private final OtelTracingPatchConfig otelTracing;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(otelTracing);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("http")
    public static class Http extends StreamSinkPatchConfig {
        private final HttpPatchConfig http;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(http);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("amazonS3")
    public static class AmazonS3 extends StreamSinkPatchConfig {
        private final AmazonS3PatchConfig amazonS3;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(amazonS3);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("googleCloudStorage")
    public static class GoogleCloudStorage extends StreamSinkPatchConfig {
        private final GoogleCloudStoragePatchConfig googleCloudStorage;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(googleCloudStorage);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("googleCloudPubSub")
    public static class GoogleCloudPubSub extends StreamSinkPatchConfig {
        private final GoogleCloudPubSubPatchConfig googleCloudPubSub;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(googleCloudPubSub);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("sqs")
    public static class Sqs extends StreamSinkPatchConfig {
        private final SqsPatchConfig sqs;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(sqs);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("sns")
    public static class Sns extends StreamSinkPatchConfig {
        private final SnsPatchConfig sns;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(sns);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("bigQuery")
    public static class BigQuery extends StreamSinkPatchConfig {
        private final BigQueryPatchConfig bigQuery;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(bigQuery);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("clickhouse")
    public static class Clickhouse extends StreamSinkPatchConfig {
        private final ClickhousePatchConfig clickhouse;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(clickhouse);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("eventBridge")
    public static class EventBridge extends StreamSinkPatchConfig {
        private final EventBridgePatchConfig eventBridge;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(eventBridge);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("snowflake")
    public static class Snowflake extends StreamSinkPatchConfig {
        private final SnowflakePatchConfig snowflake;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(snowflake);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("rabbitMq")
    public static class RabbitMq extends StreamSinkPatchConfig {
        private final RabbitMqPatchConfig rabbitMq;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(rabbitMq);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("redshift")
    public static class Redshift extends StreamSinkPatchConfig {
        private final RedshiftPatchConfig redshift;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(redshift);
        }
    }

    @FunctionalInterface
    private interface TypeFactory {
        StreamSinkPatchConfig create(JsonNode config);
    }

    private static final Map<String, TypeFactory> TY_M = new HashMap<>();
    private static final ObjectMapper m = Utils.getObjectMapper();

    static {
        TY_M.put("poller", c -> new Poller());
        TY_M.put(
                "azureBlobStorage",
                c -> new AzureBlobStorage(m.convertValue(c, AzureBlobStoragePatchConfig.class)));
        TY_M.put(
                "otelTracing",
                c -> new OtelTracing(m.convertValue(c, OtelTracingPatchConfig.class)));
        TY_M.put("http", c -> new Http(m.convertValue(c, HttpPatchConfig.class)));
        TY_M.put("amazonS3", c -> new AmazonS3(m.convertValue(c, AmazonS3PatchConfig.class)));
        TY_M.put(
                "googleCloudStorage",
                c ->
                        new GoogleCloudStorage(
                                m.convertValue(c, GoogleCloudStoragePatchConfig.class)));
        TY_M.put(
                "googleCloudPubSub",
                c -> new GoogleCloudPubSub(m.convertValue(c, GoogleCloudPubSubPatchConfig.class)));
        TY_M.put("sqs", c -> new Sqs(m.convertValue(c, SqsPatchConfig.class)));
        TY_M.put("sns", c -> new Sns(m.convertValue(c, SnsPatchConfig.class)));
        TY_M.put("bigQuery", c -> new BigQuery(m.convertValue(c, BigQueryPatchConfig.class)));
        TY_M.put("clickhouse", c -> new Clickhouse(m.convertValue(c, ClickhousePatchConfig.class)));
        TY_M.put(
                "eventBridge",
                c -> new EventBridge(m.convertValue(c, EventBridgePatchConfig.class)));
        TY_M.put("snowflake", c -> new Snowflake(m.convertValue(c, SnowflakePatchConfig.class)));
        TY_M.put("rabbitMq", c -> new RabbitMq(m.convertValue(c, RabbitMqPatchConfig.class)));
        TY_M.put("redshift", c -> new Redshift(m.convertValue(c, RedshiftPatchConfig.class)));
    }

    public static StreamSinkPatchConfig fromTypeAndConfig(String type, JsonNode config) {
        TypeFactory factory = TY_M.get(type);
        if (factory == null) {
            throw new IllegalArgumentException("Unknown type: " + type);
        }
        return factory.create(config);
    }
}
