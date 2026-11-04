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
import java.util.List;
import java.util.Map;

@Setter
@Getter
@ToString
@NoArgsConstructor
@EqualsAndHashCode
@AllArgsConstructor
@JsonSerialize(using = DestinationPatchSerializer.class)
@JsonDeserialize(using = DestinationPatchDeserializer.class)
public class DestinationPatch {
    private String uid;
    private DestinationStatusIn status;
    private Short batchSize;
    private Short maxWaitSecs;
    private List<String> eventTypes;
    private List<String> channels;
    private Map<String, String> metadata;
    private DestinationPatchConfig config;

    public DestinationPatch uid(String uid) {
        this.uid = uid;
        return this;
    }

    public DestinationPatch status(DestinationStatusIn status) {
        this.status = status;
        return this;
    }

    public DestinationPatch batchSize(Short batchSize) {
        this.batchSize = batchSize;
        return this;
    }

    public DestinationPatch maxWaitSecs(Short maxWaitSecs) {
        this.maxWaitSecs = maxWaitSecs;
        return this;
    }

    public DestinationPatch eventTypes(List<String> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public DestinationPatch channels(List<String> channels) {
        this.channels = channels;
        return this;
    }

    public DestinationPatch metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public DestinationPatch config(DestinationPatchConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static DestinationPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, DestinationPatch.class);
    }
}

@Getter
@NoArgsConstructor
class DestinationPatchSurrogate {
    @JsonProperty("uid")
    String uid;

    @JsonProperty("status")
    DestinationStatusIn status;

    @JsonProperty("batchSize")
    Short batchSize;

    @JsonProperty("maxWaitSecs")
    Short maxWaitSecs;

    @JsonProperty("eventTypes")
    List<String> eventTypes;

    @JsonProperty("channels")
    List<String> channels;

    @JsonProperty("metadata")
    Map<String, String> metadata;

    @JsonProperty("type")
    String type;

    @JsonProperty("config")
    JsonNode config;

    DestinationPatchSurrogate(DestinationPatch o, String type, JsonNode config) {
        this.uid = o.getUid();
        this.status = o.getStatus();
        this.batchSize = o.getBatchSize();
        this.maxWaitSecs = o.getMaxWaitSecs();
        this.eventTypes = o.getEventTypes();
        this.channels = o.getChannels();
        this.metadata = o.getMetadata();
        this.type = type;
        this.config = config;
    }
}

class DestinationPatchSerializer extends StdSerializer<DestinationPatch> {
    public DestinationPatchSerializer() {
        this(null);
    }

    public DestinationPatchSerializer(Class<DestinationPatch> t) {
        super(t);
    }

    @Override
    public void serialize(DestinationPatch value, JsonGenerator gen, SerializerProvider provider)
            throws IOException {
        DestinationPatchSurrogate surrogate =
                new DestinationPatchSurrogate(
                        value, value.getConfig().getVariantName(), value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}

class DestinationPatchDeserializer extends StdDeserializer<DestinationPatch> {
    public DestinationPatchDeserializer() {
        this(null);
    }

    public DestinationPatchDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public DestinationPatch deserialize(JsonParser p, DeserializationContext ctxt)
            throws IOException {
        DestinationPatchSurrogate surrogate =
                p.getCodec().readValue(p, DestinationPatchSurrogate.class);
        String uid = surrogate.getUid();
        DestinationStatusIn status = surrogate.getStatus();
        Short batchSize = surrogate.getBatchSize();
        Short maxWaitSecs = surrogate.getMaxWaitSecs();
        List<String> eventTypes = surrogate.getEventTypes();
        List<String> channels = surrogate.getChannels();
        Map<String, String> metadata = surrogate.getMetadata();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        DestinationPatchConfig sourceType = DestinationPatchConfig.fromTypeAndConfig(type, config);
        return new DestinationPatch(
                uid, status, batchSize, maxWaitSecs, eventTypes, channels, metadata, sourceType);
    }
}
