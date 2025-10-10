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
import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;

@Setter
@Getter
@ToString
@NoArgsConstructor
@EqualsAndHashCode
@AllArgsConstructor
@JsonSerialize(using = StreamSinkOutSerializer.class)
@JsonDeserialize(using = StreamSinkOutDeserializer.class)
public class StreamSinkOut {
    private Integer batchSize;
    private OffsetDateTime createdAt;
    private String currentIterator;
    private List<String> eventTypes;
    private String failureReason;
    private String id;
    private Integer maxWaitSecs;
    private Map<String, String> metadata;
    private OffsetDateTime nextRetryAt;
    private SinkStatus status;
    private String uid;
    private OffsetDateTime updatedAt;
    private StreamSinkOutConfig config;

    public StreamSinkOut batchSize(Integer batchSize) {
        this.batchSize = batchSize;
        return this;
    }

    public StreamSinkOut createdAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
        return this;
    }

    public StreamSinkOut currentIterator(String currentIterator) {
        this.currentIterator = currentIterator;
        return this;
    }

    public StreamSinkOut eventTypes(List<String> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public StreamSinkOut failureReason(String failureReason) {
        this.failureReason = failureReason;
        return this;
    }

    public StreamSinkOut id(String id) {
        this.id = id;
        return this;
    }

    public StreamSinkOut maxWaitSecs(Integer maxWaitSecs) {
        this.maxWaitSecs = maxWaitSecs;
        return this;
    }

    public StreamSinkOut metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public StreamSinkOut nextRetryAt(OffsetDateTime nextRetryAt) {
        this.nextRetryAt = nextRetryAt;
        return this;
    }

    public StreamSinkOut status(SinkStatus status) {
        this.status = status;
        return this;
    }

    public StreamSinkOut uid(String uid) {
        this.uid = uid;
        return this;
    }

    public StreamSinkOut updatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
        return this;
    }

    public StreamSinkOut config(StreamSinkOutConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static StreamSinkOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, StreamSinkOut.class);
    }
}

@Getter
@NoArgsConstructor
class StreamSinkOutSurrogate {
    @JsonProperty("batchSize")
    Integer batchSize;

    @JsonProperty("createdAt")
    OffsetDateTime createdAt;

    @JsonProperty("currentIterator")
    String currentIterator;

    @JsonProperty("eventTypes")
    List<String> eventTypes;

    @JsonProperty("failureReason")
    String failureReason;

    @JsonProperty("id")
    String id;

    @JsonProperty("maxWaitSecs")
    Integer maxWaitSecs;

    @JsonProperty("metadata")
    Map<String, String> metadata;

    @JsonProperty("nextRetryAt")
    OffsetDateTime nextRetryAt;

    @JsonProperty("status")
    SinkStatus status;

    @JsonProperty("uid")
    String uid;

    @JsonProperty("updatedAt")
    OffsetDateTime updatedAt;

    @JsonProperty("type")
    String type;

    @JsonProperty("config")
    JsonNode config;

    StreamSinkOutSurrogate(StreamSinkOut o, String type, JsonNode config) {
        this.batchSize = o.getBatchSize();
        this.createdAt = o.getCreatedAt();
        this.currentIterator = o.getCurrentIterator();
        this.eventTypes = o.getEventTypes();
        this.failureReason = o.getFailureReason();
        this.id = o.getId();
        this.maxWaitSecs = o.getMaxWaitSecs();
        this.metadata = o.getMetadata();
        this.nextRetryAt = o.getNextRetryAt();
        this.status = o.getStatus();
        this.uid = o.getUid();
        this.updatedAt = o.getUpdatedAt();
        this.type = type;
        this.config = config;
    }
}

class StreamSinkOutSerializer extends StdSerializer<StreamSinkOut> {
    public StreamSinkOutSerializer() {
        this(null);
    }

    public StreamSinkOutSerializer(Class<StreamSinkOut> t) {
        super(t);
    }

    @Override
    public void serialize(StreamSinkOut value, JsonGenerator gen, SerializerProvider provider)
            throws IOException {
        StreamSinkOutSurrogate surrogate =
                new StreamSinkOutSurrogate(
                        value, value.getConfig().getVariantName(), value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}

class StreamSinkOutDeserializer extends StdDeserializer<StreamSinkOut> {
    public StreamSinkOutDeserializer() {
        this(null);
    }

    public StreamSinkOutDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public StreamSinkOut deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        StreamSinkOutSurrogate surrogate = p.getCodec().readValue(p, StreamSinkOutSurrogate.class);
        Integer batchSize = surrogate.getBatchSize();
        OffsetDateTime createdAt = surrogate.getCreatedAt();
        String currentIterator = surrogate.getCurrentIterator();
        List<String> eventTypes = surrogate.getEventTypes();
        String failureReason = surrogate.getFailureReason();
        String id = surrogate.getId();
        Integer maxWaitSecs = surrogate.getMaxWaitSecs();
        Map<String, String> metadata = surrogate.getMetadata();
        OffsetDateTime nextRetryAt = surrogate.getNextRetryAt();
        SinkStatus status = surrogate.getStatus();
        String uid = surrogate.getUid();
        OffsetDateTime updatedAt = surrogate.getUpdatedAt();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        StreamSinkOutConfig sourceType = StreamSinkOutConfig.fromTypeAndConfig(type, config);
        return new StreamSinkOut(
                batchSize,
                createdAt,
                currentIterator,
                eventTypes,
                failureReason,
                id,
                maxWaitSecs,
                metadata,
                nextRetryAt,
                status,
                uid,
                updatedAt,
                sourceType);
    }
}
