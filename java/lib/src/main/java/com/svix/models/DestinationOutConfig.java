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
public abstract class DestinationOutConfig {
    @JsonIgnore
    public String getVariantName() {
        VariantName annotation = this.getClass().getAnnotation(VariantName.class);
        return annotation != null ? annotation.value() : null;
    }

    public abstract JsonNode toJsonNode();

    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("pollingEndpoint")
    public static class PollingEndpoint extends DestinationOutConfig {
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
    public static class AzureBlobStorage extends DestinationOutConfig {
        private final AzureBlobStorageConfigOut azureBlobStorage;

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
    public static class OtelTracing extends DestinationOutConfig {
        private final OtelTracingConfigOut otelTracing;

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
    @VariantName("fifoEndpoint")
    public static class FifoEndpoint extends DestinationOutConfig {
        private final SinkHttpConfigOut fifoEndpoint;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(fifoEndpoint);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("amazonS3")
    public static class AmazonS3 extends DestinationOutConfig {
        private final S3ConfigOut amazonS3;

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
    @VariantName("snowflake")
    public static class Snowflake extends DestinationOutConfig {
        private final SnowflakeConfigOut snowflake;

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
    @VariantName("googleCloudStorage")
    public static class GoogleCloudStorage extends DestinationOutConfig {
        private final GoogleCloudStorageConfigOut googleCloudStorage;

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
    public static class GoogleCloudPubSub extends DestinationOutConfig {
        private final GoogleCloudPubSubConfigOut googleCloudPubSub;

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
    @VariantName("redshift")
    public static class Redshift extends DestinationOutConfig {
        private final RedshiftConfigOut redshift;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(redshift);
        }
    }

    @Getter
    @Setter
    @AllArgsConstructor
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("bigQuery")
    public static class BigQuery extends DestinationOutConfig {
        private final BigQueryConfigOut bigQuery;

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
    public static class Clickhouse extends DestinationOutConfig {
        private final ClickhouseConfigOut clickhouse;

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
    @VariantName("rabbitMq")
    public static class RabbitMq extends DestinationOutConfig {
        private final RabbitMqConfigOut rabbitMq;

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
    @VariantName("sqs")
    public static class Sqs extends DestinationOutConfig {
        private final SqsConfigOut sqs;

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
    @VariantName("eventBridge")
    public static class EventBridge extends DestinationOutConfig {
        private final EventBridgeConfigOut eventBridge;

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
    @VariantName("sns")
    public static class Sns extends DestinationOutConfig {
        private final SnsConfigOut sns;

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
    @VariantName("postgres")
    public static class Postgres extends DestinationOutConfig {
        private final PostgresConfigOut postgres;

        @Override
        public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(postgres);
        }
    }

    @FunctionalInterface
    private interface TypeFactory {
        DestinationOutConfig create(JsonNode config);
    }

    private static final Map<String, TypeFactory> TY_M = new HashMap<>();
    private static final ObjectMapper m = Utils.getObjectMapper();

    static {
        TY_M.put("pollingEndpoint", c -> new PollingEndpoint());
        TY_M.put(
                "azureBlobStorage",
                c -> new AzureBlobStorage(m.convertValue(c, AzureBlobStorageConfigOut.class)));
        TY_M.put(
                "otelTracing", c -> new OtelTracing(m.convertValue(c, OtelTracingConfigOut.class)));
        TY_M.put("fifoEndpoint", c -> new FifoEndpoint(m.convertValue(c, SinkHttpConfigOut.class)));
        TY_M.put("amazonS3", c -> new AmazonS3(m.convertValue(c, S3ConfigOut.class)));
        TY_M.put("snowflake", c -> new Snowflake(m.convertValue(c, SnowflakeConfigOut.class)));
        TY_M.put(
                "googleCloudStorage",
                c -> new GoogleCloudStorage(m.convertValue(c, GoogleCloudStorageConfigOut.class)));
        TY_M.put(
                "googleCloudPubSub",
                c -> new GoogleCloudPubSub(m.convertValue(c, GoogleCloudPubSubConfigOut.class)));
        TY_M.put("redshift", c -> new Redshift(m.convertValue(c, RedshiftConfigOut.class)));
        TY_M.put("bigQuery", c -> new BigQuery(m.convertValue(c, BigQueryConfigOut.class)));
        TY_M.put("clickhouse", c -> new Clickhouse(m.convertValue(c, ClickhouseConfigOut.class)));
        TY_M.put("rabbitMq", c -> new RabbitMq(m.convertValue(c, RabbitMqConfigOut.class)));
        TY_M.put("sqs", c -> new Sqs(m.convertValue(c, SqsConfigOut.class)));
        TY_M.put(
                "eventBridge", c -> new EventBridge(m.convertValue(c, EventBridgeConfigOut.class)));
        TY_M.put("sns", c -> new Sns(m.convertValue(c, SnsConfigOut.class)));
        TY_M.put("postgres", c -> new Postgres(m.convertValue(c, PostgresConfigOut.class)));
    }

    public static DestinationOutConfig fromTypeAndConfig(String type, JsonNode config) {
        TypeFactory factory = TY_M.get(type);
        if (factory == null) {
            throw new IllegalArgumentException("Unknown type: " + type);
        }
        return factory.create(config);
    }
}
