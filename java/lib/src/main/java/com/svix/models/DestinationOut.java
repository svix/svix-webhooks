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
@JsonSerialize(using = DestinationOutSerializer.class)
@JsonDeserialize(using = DestinationOutDeserializer.class)
public class DestinationOut {
    private String id;
    private String uid;
    private SinkStatus status;
    private String currentIterator;
    private String failureReason;
    private OffsetDateTime createdAt;
    private OffsetDateTime updatedAt;
    private Integer batchSize;
    private Integer maxWaitSecs;
    private List<String> eventTypes;
    private List<String> channels;
    private OffsetDateTime nextRetryAt;
    private Map<String, String> metadata;
    private DestinationOutConfig config;

    public DestinationOut id(String id) {
        this.id = id;
        return this;
    }

    public DestinationOut uid(String uid) {
        this.uid = uid;
        return this;
    }

    public DestinationOut status(SinkStatus status) {
        this.status = status;
        return this;
    }

    public DestinationOut currentIterator(String currentIterator) {
        this.currentIterator = currentIterator;
        return this;
    }

    public DestinationOut failureReason(String failureReason) {
        this.failureReason = failureReason;
        return this;
    }

    public DestinationOut createdAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
        return this;
    }

    public DestinationOut updatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
        return this;
    }

    public DestinationOut batchSize(Integer batchSize) {
        this.batchSize = batchSize;
        return this;
    }

    public DestinationOut maxWaitSecs(Integer maxWaitSecs) {
        this.maxWaitSecs = maxWaitSecs;
        return this;
    }

    public DestinationOut eventTypes(List<String> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public DestinationOut channels(List<String> channels) {
        this.channels = channels;
        return this;
    }

    public DestinationOut nextRetryAt(OffsetDateTime nextRetryAt) {
        this.nextRetryAt = nextRetryAt;
        return this;
    }

    public DestinationOut metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public DestinationOut config(DestinationOutConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static DestinationOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, DestinationOut.class);
    }
}

@Getter
@NoArgsConstructor
class DestinationOutSurrogate {
    @JsonProperty("id")
    String id;

    @JsonProperty("uid")
    String uid;

    @JsonProperty("status")
    SinkStatus status;

    @JsonProperty("currentIterator")
    String currentIterator;

    @JsonProperty("failureReason")
    String failureReason;

    @JsonProperty("createdAt")
    OffsetDateTime createdAt;

    @JsonProperty("updatedAt")
    OffsetDateTime updatedAt;

    @JsonProperty("batchSize")
    Integer batchSize;

    @JsonProperty("maxWaitSecs")
    Integer maxWaitSecs;

    @JsonProperty("eventTypes")
    List<String> eventTypes;

    @JsonProperty("channels")
    List<String> channels;

    @JsonProperty("nextRetryAt")
    OffsetDateTime nextRetryAt;

    @JsonProperty("metadata")
    Map<String, String> metadata;

    @JsonProperty("type")
    String type;

    @JsonProperty("config")
    JsonNode config;

    DestinationOutSurrogate(DestinationOut o, String type, JsonNode config) {
        this.id = o.getId();
        this.uid = o.getUid();
        this.status = o.getStatus();
        this.currentIterator = o.getCurrentIterator();
        this.failureReason = o.getFailureReason();
        this.createdAt = o.getCreatedAt();
        this.updatedAt = o.getUpdatedAt();
        this.batchSize = o.getBatchSize();
        this.maxWaitSecs = o.getMaxWaitSecs();
        this.eventTypes = o.getEventTypes();
        this.channels = o.getChannels();
        this.nextRetryAt = o.getNextRetryAt();
        this.metadata = o.getMetadata();
        this.type = type;
        this.config = config;
    }
}

class DestinationOutSerializer extends StdSerializer<DestinationOut> {
    public DestinationOutSerializer() {
        this(null);
    }

    public DestinationOutSerializer(Class<DestinationOut> t) {
        super(t);
    }

    @Override
    public void serialize(DestinationOut value, JsonGenerator gen, SerializerProvider provider)
            throws IOException {
        DestinationOutSurrogate surrogate =
                new DestinationOutSurrogate(
                        value, value.getConfig().getVariantName(), value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}

class DestinationOutDeserializer extends StdDeserializer<DestinationOut> {
    public DestinationOutDeserializer() {
        this(null);
    }

    public DestinationOutDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public DestinationOut deserialize(JsonParser p, DeserializationContext ctxt)
            throws IOException {
        DestinationOutSurrogate surrogate =
                p.getCodec().readValue(p, DestinationOutSurrogate.class);
        String id = surrogate.getId();
        String uid = surrogate.getUid();
        SinkStatus status = surrogate.getStatus();
        String currentIterator = surrogate.getCurrentIterator();
        String failureReason = surrogate.getFailureReason();
        OffsetDateTime createdAt = surrogate.getCreatedAt();
        OffsetDateTime updatedAt = surrogate.getUpdatedAt();
        Integer batchSize = surrogate.getBatchSize();
        Integer maxWaitSecs = surrogate.getMaxWaitSecs();
        List<String> eventTypes = surrogate.getEventTypes();
        List<String> channels = surrogate.getChannels();
        OffsetDateTime nextRetryAt = surrogate.getNextRetryAt();
        Map<String, String> metadata = surrogate.getMetadata();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        DestinationOutConfig sourceType = DestinationOutConfig.fromTypeAndConfig(type, config);
        return new DestinationOut(
                id,
                uid,
                status,
                currentIterator,
                failureReason,
                createdAt,
                updatedAt,
                batchSize,
                maxWaitSecs,
                eventTypes,
                channels,
                nextRetryAt,
                metadata,
                sourceType);
    }
}
