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
public abstract class StreamSinkInConfig {
    @JsonIgnore
    public String getVariantName() {
        VariantName annotation = this.getClass().getAnnotation(VariantName.class);
        return annotation != null ? annotation.value() : null;
    }

    public abstract JsonNode toJsonNode();

    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("poller")
    public static class Poller extends StreamSinkInConfig {
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
    public static class AzureBlobStorage extends StreamSinkInConfig {
        private final AzureBlobStorageConfig azureBlobStorage;

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
    public static class OtelTracing extends StreamSinkInConfig {
        private final SinkOtelV1Config otelTracing;

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
    public static class Http extends StreamSinkInConfig {
        private final SinkHttpConfig http;

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
    public static class AmazonS3 extends StreamSinkInConfig {
        private final S3Config amazonS3;

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
    public static class GoogleCloudStorage extends StreamSinkInConfig {
        private final GoogleCloudStorageConfig googleCloudStorage;

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
    public static class GoogleCloudPubSub extends StreamSinkInConfig {
        private final GoogleCloudPubSubConfig googleCloudPubSub;

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
    public static class Sqs extends StreamSinkInConfig {
        private final SqsConfig sqs;

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
    public static class Sns extends StreamSinkInConfig {
        private final SnsConfig sns;

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
    public static class BigQuery extends StreamSinkInConfig {
        private final BigQueryConfig bigQuery;

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
    public static class Clickhouse extends StreamSinkInConfig {
        private final ClickhouseConfig clickhouse;

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
    public static class EventBridge extends StreamSinkInConfig {
        private final EventBridgeConfig eventBridge;

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
    public static class Snowflake extends StreamSinkInConfig {
        private final SnowflakeConfig snowflake;

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
    public static class RabbitMq extends StreamSinkInConfig {
        private final RabbitMqConfig rabbitMq;

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
    public static class Redshift extends StreamSinkInConfig {
        private final RedshiftConfig redshift;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(redshift);
        }
    }

    @FunctionalInterface
    private interface TypeFactory {
        StreamSinkInConfig create(JsonNode config);
    }

    private static final Map<String, TypeFactory> TY_M = new HashMap<>();
    private static final ObjectMapper m = Utils.getObjectMapper();

    static {
        TY_M.put("poller", c -> new Poller());
        TY_M.put(
                "azureBlobStorage",
                c -> new AzureBlobStorage(m.convertValue(c, AzureBlobStorageConfig.class)));
        TY_M.put("otelTracing", c -> new OtelTracing(m.convertValue(c, SinkOtelV1Config.class)));
        TY_M.put("http", c -> new Http(m.convertValue(c, SinkHttpConfig.class)));
        TY_M.put("amazonS3", c -> new AmazonS3(m.convertValue(c, S3Config.class)));
        TY_M.put(
                "googleCloudStorage",
                c -> new GoogleCloudStorage(m.convertValue(c, GoogleCloudStorageConfig.class)));
        TY_M.put(
                "googleCloudPubSub",
                c -> new GoogleCloudPubSub(m.convertValue(c, GoogleCloudPubSubConfig.class)));
        TY_M.put("sqs", c -> new Sqs(m.convertValue(c, SqsConfig.class)));
        TY_M.put("sns", c -> new Sns(m.convertValue(c, SnsConfig.class)));
        TY_M.put("bigQuery", c -> new BigQuery(m.convertValue(c, BigQueryConfig.class)));
        TY_M.put("clickhouse", c -> new Clickhouse(m.convertValue(c, ClickhouseConfig.class)));
        TY_M.put("eventBridge", c -> new EventBridge(m.convertValue(c, EventBridgeConfig.class)));
        TY_M.put("snowflake", c -> new Snowflake(m.convertValue(c, SnowflakeConfig.class)));
        TY_M.put("rabbitMq", c -> new RabbitMq(m.convertValue(c, RabbitMqConfig.class)));
        TY_M.put("redshift", c -> new Redshift(m.convertValue(c, RedshiftConfig.class)));
    }

    public static StreamSinkInConfig fromTypeAndConfig(String type, JsonNode config) {
        TypeFactory factory = TY_M.get(type);
        if (factory == null) {
            throw new IllegalArgumentException("Unknown type: " + type);
        }
        return factory.create(config);
    }
}
