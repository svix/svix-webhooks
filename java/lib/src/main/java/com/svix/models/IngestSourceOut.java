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

@Setter
@Getter
@ToString
@NoArgsConstructor
@EqualsAndHashCode
@AllArgsConstructor
@JsonSerialize(using = IngestSourceOutSerializer.class)
@JsonDeserialize(using = IngestSourceOutDeserializer.class)
public class IngestSourceOut {
    private OffsetDateTime createdAt;
    private String id;
    private URI ingestUrl;
    private String name;
    private String uid;
    private OffsetDateTime updatedAt;
    private IngestSourceOutConfig config;

    public IngestSourceOut createdAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
        return this;
    }

    public IngestSourceOut id(String id) {
        this.id = id;
        return this;
    }

    public IngestSourceOut ingestUrl(URI ingestUrl) {
        this.ingestUrl = ingestUrl;
        return this;
    }

    public IngestSourceOut name(String name) {
        this.name = name;
        return this;
    }

    public IngestSourceOut uid(String uid) {
        this.uid = uid;
        return this;
    }

    public IngestSourceOut updatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
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
    @JsonProperty("createdAt")
    OffsetDateTime createdAt;

    @JsonProperty("id")
    String id;

    @JsonProperty("ingestUrl")
    URI ingestUrl;

    @JsonProperty("name")
    String name;

    @JsonProperty("uid")
    String uid;

    @JsonProperty("updatedAt")
    OffsetDateTime updatedAt;

    @JsonProperty("type")
    String type;

    @JsonProperty("config")
    JsonNode config;

    IngestSourceOutSurrogate(IngestSourceOut o, String type, JsonNode config) {
        this.createdAt = o.getCreatedAt();
        this.id = o.getId();
        this.ingestUrl = o.getIngestUrl();
        this.name = o.getName();
        this.uid = o.getUid();
        this.updatedAt = o.getUpdatedAt();
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
        OffsetDateTime createdAt = surrogate.getCreatedAt();
        String id = surrogate.getId();
        URI ingestUrl = surrogate.getIngestUrl();
        String name = surrogate.getName();
        String uid = surrogate.getUid();
        OffsetDateTime updatedAt = surrogate.getUpdatedAt();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        IngestSourceOutConfig sourceType = IngestSourceOutConfig.fromTypeAndConfig(type, config);
        return new IngestSourceOut(createdAt, id, ingestUrl, name, uid, updatedAt, sourceType);
    }
}
