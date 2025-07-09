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
import java.util.Map;

@Setter
@Getter
@ToString
@NoArgsConstructor
@EqualsAndHashCode
@AllArgsConstructor
@JsonSerialize(using = IngestSourceInSerializer.class)
@JsonDeserialize(using = IngestSourceInDeserializer.class)
public class IngestSourceIn {
    private Map<String, String> metadata;
    private String name;
    private String uid;
    private IngestSourceInConfig config;

    public IngestSourceIn metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public IngestSourceIn name(String name) {
        this.name = name;
        return this;
    }

    public IngestSourceIn uid(String uid) {
        this.uid = uid;
        return this;
    }

    public IngestSourceIn config(IngestSourceInConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static IngestSourceIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, IngestSourceIn.class);
    }
}

@Getter
@NoArgsConstructor
class IngestSourceInSurrogate {
    @JsonProperty("metadata")
    Map<String, String> metadata;

    @JsonProperty("name")
    String name;

    @JsonProperty("uid")
    String uid;

    @JsonProperty("type")
    String type;

    @JsonProperty("config")
    JsonNode config;

    IngestSourceInSurrogate(IngestSourceIn o, String type, JsonNode config) {
        this.metadata = o.getMetadata();
        this.name = o.getName();
        this.uid = o.getUid();
        this.type = type;
        this.config = config;
    }
}

class IngestSourceInSerializer extends StdSerializer<IngestSourceIn> {
    public IngestSourceInSerializer() {
        this(null);
    }

    public IngestSourceInSerializer(Class<IngestSourceIn> t) {
        super(t);
    }

    @Override
    public void serialize(IngestSourceIn value, JsonGenerator gen, SerializerProvider provider)
            throws IOException {
        IngestSourceInSurrogate surrogate =
                new IngestSourceInSurrogate(
                        value, value.getConfig().getVariantName(), value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}

class IngestSourceInDeserializer extends StdDeserializer<IngestSourceIn> {
    public IngestSourceInDeserializer() {
        this(null);
    }

    public IngestSourceInDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public IngestSourceIn deserialize(JsonParser p, DeserializationContext ctxt)
            throws IOException {
        IngestSourceInSurrogate surrogate =
                p.getCodec().readValue(p, IngestSourceInSurrogate.class);
        Map<String, String> metadata = surrogate.getMetadata();
        String name = surrogate.getName();
        String uid = surrogate.getUid();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        IngestSourceInConfig sourceType = IngestSourceInConfig.fromTypeAndConfig(type, config);
        return new IngestSourceIn(metadata, name, uid, sourceType);
    }
}
