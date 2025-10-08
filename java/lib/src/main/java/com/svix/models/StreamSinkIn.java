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
@JsonSerialize(using = StreamSinkInSerializer.class)
@JsonDeserialize(using = StreamSinkInDeserializer.class)
public class StreamSinkIn {
    private Long batchSize;
    private List<String> eventTypes;
    private Long maxWaitSecs;
    private Map<String, String> metadata;
    private SinkStatusIn status;
    private String uid;
    private StreamSinkInConfig config;

    public StreamSinkIn batchSize(Long batchSize) {
        this.batchSize = batchSize;
        return this;
    }

    public StreamSinkIn eventTypes(List<String> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public StreamSinkIn maxWaitSecs(Long maxWaitSecs) {
        this.maxWaitSecs = maxWaitSecs;
        return this;
    }

    public StreamSinkIn metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public StreamSinkIn status(SinkStatusIn status) {
        this.status = status;
        return this;
    }

    public StreamSinkIn uid(String uid) {
        this.uid = uid;
        return this;
    }

    public StreamSinkIn config(StreamSinkInConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static StreamSinkIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, StreamSinkIn.class);
    }
}

@Getter
@NoArgsConstructor
class StreamSinkInSurrogate {
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

    StreamSinkInSurrogate(StreamSinkIn o, String type, JsonNode config) {
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

class StreamSinkInSerializer extends StdSerializer<StreamSinkIn> {
    public StreamSinkInSerializer() {
        this(null);
    }

    public StreamSinkInSerializer(Class<StreamSinkIn> t) {
        super(t);
    }

    @Override
    public void serialize(StreamSinkIn value, JsonGenerator gen, SerializerProvider provider)
            throws IOException {
        StreamSinkInSurrogate surrogate =
                new StreamSinkInSurrogate(
                        value, value.getConfig().getVariantName(), value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}

class StreamSinkInDeserializer extends StdDeserializer<StreamSinkIn> {
    public StreamSinkInDeserializer() {
        this(null);
    }

    public StreamSinkInDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public StreamSinkIn deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        StreamSinkInSurrogate surrogate = p.getCodec().readValue(p, StreamSinkInSurrogate.class);
        Long batchSize = surrogate.getBatchSize();
        List<String> eventTypes = surrogate.getEventTypes();
        Long maxWaitSecs = surrogate.getMaxWaitSecs();
        Map<String, String> metadata = surrogate.getMetadata();
        SinkStatusIn status = surrogate.getStatus();
        String uid = surrogate.getUid();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        StreamSinkInConfig sourceType = StreamSinkInConfig.fromTypeAndConfig(type, config);
        return new StreamSinkIn(
                batchSize, eventTypes, maxWaitSecs, metadata, status, uid, sourceType);
    }
}
