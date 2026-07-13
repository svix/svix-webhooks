// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import com.fasterxml.jackson.databind.ser.std.StdSerializer;
import com.svix.Utils;

import lombok.AllArgsConstructor;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import lombok.ToString;

import java.io.IOException;
import java.net.URI;
import java.time.OffsetDateTime;
import java.util.Map;

@Setter
@Getter
@ToString
@NoArgsConstructor
@EqualsAndHashCode
@AllArgsConstructor
@JsonSerialize(using = IngestSourceOutSerializer.class)
@JsonDeserialize(using = IngestSourceOutDeserializer.class)
public class IngestSourceOut {
    private String id;
    private String uid;
    private String name;
    private URI ingestUrl;
    private OffsetDateTime createdAt;
    private OffsetDateTime updatedAt;
    private Map<String, String> metadata;
    private IngestSourceOutConfig config;

    public IngestSourceOut id(String id) {
        this.id = id;
        return this;
    }

    public IngestSourceOut uid(String uid) {
        this.uid = uid;
        return this;
    }

    public IngestSourceOut name(String name) {
        this.name = name;
        return this;
    }

    public IngestSourceOut ingestUrl(URI ingestUrl) {
        this.ingestUrl = ingestUrl;
        return this;
    }

    public IngestSourceOut createdAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
        return this;
    }

    public IngestSourceOut updatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
        return this;
    }

    public IngestSourceOut metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public IngestSourceOut config(IngestSourceOutConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static IngestSourceOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, IngestSourceOut.class);
    }
}

@Getter
@NoArgsConstructor
class IngestSourceOutSurrogate {
    @JsonProperty("id")
    String id;

    @JsonProperty("uid")
    String uid;

    @JsonProperty("name")
    String name;

    @JsonProperty("ingestUrl")
    URI ingestUrl;

    @JsonProperty("createdAt")
    OffsetDateTime createdAt;

    @JsonProperty("updatedAt")
    OffsetDateTime updatedAt;

    @JsonProperty("metadata")
    Map<String, String> metadata;

    @JsonProperty("type")
    String type;

    @JsonProperty("config")
    JsonNode config;

    IngestSourceOutSurrogate(IngestSourceOut o, String type, JsonNode config) {
        this.id = o.getId();
        this.uid = o.getUid();
        this.name = o.getName();
        this.ingestUrl = o.getIngestUrl();
        this.createdAt = o.getCreatedAt();
        this.updatedAt = o.getUpdatedAt();
        this.metadata = o.getMetadata();
        this.type = type;
        this.config = config;
    }
}

class IngestSourceOutSerializer extends StdSerializer<IngestSourceOut> {
    public IngestSourceOutSerializer() {
        this(null);
    }

    public IngestSourceOutSerializer(Class<IngestSourceOut> t) {
        super(t);
    }

    @Override
    public void serialize(IngestSourceOut value, JsonGenerator gen, SerializerProvider provider)
            throws IOException {
        IngestSourceOutSurrogate surrogate =
                new IngestSourceOutSurrogate(
                        value, value.getConfig().getVariantName(), value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}

class IngestSourceOutDeserializer extends StdDeserializer<IngestSourceOut> {
    public IngestSourceOutDeserializer() {
        this(null);
    }

    public IngestSourceOutDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public IngestSourceOut deserialize(JsonParser p, DeserializationContext ctxt)
            throws IOException {
        IngestSourceOutSurrogate surrogate =
                p.getCodec().readValue(p, IngestSourceOutSurrogate.class);
        String id = surrogate.getId();
        String uid = surrogate.getUid();
        String name = surrogate.getName();
        URI ingestUrl = surrogate.getIngestUrl();
        OffsetDateTime createdAt = surrogate.getCreatedAt();
        OffsetDateTime updatedAt = surrogate.getUpdatedAt();
        Map<String, String> metadata = surrogate.getMetadata();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        IngestSourceOutConfig sourceType = IngestSourceOutConfig.fromTypeAndConfig(type, config);
        return new IngestSourceOut(
                id, uid, name, ingestUrl, createdAt, updatedAt, metadata, sourceType);
    }
}
