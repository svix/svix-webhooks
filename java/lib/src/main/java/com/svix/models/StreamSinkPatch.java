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
@JsonSerialize(using = StreamSinkPatchSerializer.class)
@JsonDeserialize(using = StreamSinkPatchDeserializer.class)
public class StreamSinkPatch {
    private Long batchSize;
    private List<String> eventTypes;
    private Long maxWaitSecs;
    private Map<String, String> metadata;
    private SinkStatusIn status;
    private String uid;
    private StreamSinkPatchConfig config;

    public StreamSinkPatch batchSize(Long batchSize) {
        this.batchSize = batchSize;
        return this;
    }

    public StreamSinkPatch eventTypes(List<String> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public StreamSinkPatch maxWaitSecs(Long maxWaitSecs) {
        this.maxWaitSecs = maxWaitSecs;
        return this;
    }

    public StreamSinkPatch metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public StreamSinkPatch status(SinkStatusIn status) {
        this.status = status;
        return this;
    }

    public StreamSinkPatch uid(String uid) {
        this.uid = uid;
        return this;
    }

    public StreamSinkPatch config(StreamSinkPatchConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static StreamSinkPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, StreamSinkPatch.class);
    }
}

@Getter
@NoArgsConstructor
class StreamSinkPatchSurrogate {
    @JsonProperty("batchSize")
    Long batchSize;

    @JsonProperty("eventTypes")
    List<String> eventTypes;

    @JsonProperty("maxWaitSecs")
    Long maxWaitSecs;

    @JsonProperty("metadata")
    Map<String, String> metadata;

    @JsonProperty("status")
    SinkStatusIn status;

    @JsonProperty("uid")
    String uid;

    @JsonProperty("type")
    String type;

    @JsonProperty("config")
    JsonNode config;

    StreamSinkPatchSurrogate(StreamSinkPatch o, String type, JsonNode config) {
        this.batchSize = o.getBatchSize();
        this.eventTypes = o.getEventTypes();
        this.maxWaitSecs = o.getMaxWaitSecs();
        this.metadata = o.getMetadata();
        this.status = o.getStatus();
        this.uid = o.getUid();
        this.type = type;
        this.config = config;
    }
}

class StreamSinkPatchSerializer extends StdSerializer<StreamSinkPatch> {
    public StreamSinkPatchSerializer() {
        this(null);
    }

    public StreamSinkPatchSerializer(Class<StreamSinkPatch> t) {
        super(t);
    }

    @Override
    public void serialize(StreamSinkPatch value, JsonGenerator gen, SerializerProvider provider)
            throws IOException {
        StreamSinkPatchSurrogate surrogate =
                new StreamSinkPatchSurrogate(
                        value, value.getConfig().getVariantName(), value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}

class StreamSinkPatchDeserializer extends StdDeserializer<StreamSinkPatch> {
    public StreamSinkPatchDeserializer() {
        this(null);
    }

    public StreamSinkPatchDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public StreamSinkPatch deserialize(JsonParser p, DeserializationContext ctxt)
            throws IOException {
        StreamSinkPatchSurrogate surrogate =
                p.getCodec().readValue(p, StreamSinkPatchSurrogate.class);
        Long batchSize = surrogate.getBatchSize();
        List<String> eventTypes = surrogate.getEventTypes();
        Long maxWaitSecs = surrogate.getMaxWaitSecs();
        Map<String, String> metadata = surrogate.getMetadata();
        SinkStatusIn status = surrogate.getStatus();
        String uid = surrogate.getUid();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        StreamSinkPatchConfig sourceType = StreamSinkPatchConfig.fromTypeAndConfig(type, config);
        return new StreamSinkPatch(
                batchSize, eventTypes, maxWaitSecs, metadata, status, uid, sourceType);
    }
}
