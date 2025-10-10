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
    }

    public static StreamSinkPatchConfig fromTypeAndConfig(String type, JsonNode config) {
        TypeFactory factory = TY_M.get(type);
        if (factory == null) {
            throw new IllegalArgumentException("Unknown type: " + type);
        }
        return factory.create(config);
    }
}
