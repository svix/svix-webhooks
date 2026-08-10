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
        private final AzureBlobStorageConfigPatch azureBlobStorage;

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
        private final OtelTracingConfigPatch otelTracing;

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
        private final HttpConfigPatch http;

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
        private final AmazonS3ConfigPatch amazonS3;

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
        private final GoogleCloudStorageConfigPatch googleCloudStorage;

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
        private final GoogleCloudPubSubConfigPatch googleCloudPubSub;

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
        private final SqsConfigPatch sqs;

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
        private final SnsConfigPatch sns;

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
        private final BigQueryConfigPatch bigQuery;

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
        private final ClickhouseConfigPatch clickhouse;

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
        private final EventBridgeConfigPatch eventBridge;

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
        private final SnowflakeConfigPatch snowflake;

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
        private final RabbitMqConfigPatch rabbitMq;

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
        private final RedshiftConfigPatch redshift;

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
                c -> new AzureBlobStorage(m.convertValue(c, AzureBlobStorageConfigPatch.class)));
        TY_M.put(
                "otelTracing",
                c -> new OtelTracing(m.convertValue(c, OtelTracingConfigPatch.class)));
        TY_M.put("http", c -> new Http(m.convertValue(c, HttpConfigPatch.class)));
        TY_M.put("amazonS3", c -> new AmazonS3(m.convertValue(c, AmazonS3ConfigPatch.class)));
        TY_M.put(
                "googleCloudStorage",
                c ->
                        new GoogleCloudStorage(
                                m.convertValue(c, GoogleCloudStorageConfigPatch.class)));
        TY_M.put(
                "googleCloudPubSub",
                c -> new GoogleCloudPubSub(m.convertValue(c, GoogleCloudPubSubConfigPatch.class)));
        TY_M.put("sqs", c -> new Sqs(m.convertValue(c, SqsConfigPatch.class)));
        TY_M.put("sns", c -> new Sns(m.convertValue(c, SnsConfigPatch.class)));
        TY_M.put("bigQuery", c -> new BigQuery(m.convertValue(c, BigQueryConfigPatch.class)));
        TY_M.put("clickhouse", c -> new Clickhouse(m.convertValue(c, ClickhouseConfigPatch.class)));
        TY_M.put(
                "eventBridge",
                c -> new EventBridge(m.convertValue(c, EventBridgeConfigPatch.class)));
        TY_M.put("snowflake", c -> new Snowflake(m.convertValue(c, SnowflakeConfigPatch.class)));
        TY_M.put("rabbitMq", c -> new RabbitMq(m.convertValue(c, RabbitMqConfigPatch.class)));
        TY_M.put("redshift", c -> new Redshift(m.convertValue(c, RedshiftConfigPatch.class)));
    }

    public static StreamSinkPatchConfig fromTypeAndConfig(String type, JsonNode config) {
        TypeFactory factory = TY_M.get(type);
        if (factory == null) {
            throw new IllegalArgumentException("Unknown type: " + type);
        }
        return factory.create(config);
    }
}
