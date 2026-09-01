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
@JsonSerialize(using = DestinationInSerializer.class)
@JsonDeserialize(using = DestinationInDeserializer.class)
public class DestinationIn {
    private String uid;
    private SinkStatusIn status;
    private Short batchSize;
    private Short maxWaitSecs;
    private List<String> eventTypes;
    private List<String> channels;
    private Map<String, String> metadata;
    private DestinationInConfig config;

    public DestinationIn uid(String uid) {
        this.uid = uid;
        return this;
    }

    public DestinationIn status(SinkStatusIn status) {
        this.status = status;
        return this;
    }

    public DestinationIn batchSize(Short batchSize) {
        this.batchSize = batchSize;
        return this;
    }

    public DestinationIn maxWaitSecs(Short maxWaitSecs) {
        this.maxWaitSecs = maxWaitSecs;
        return this;
    }

    public DestinationIn eventTypes(List<String> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public DestinationIn channels(List<String> channels) {
        this.channels = channels;
        return this;
    }

    public DestinationIn metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public DestinationIn config(DestinationInConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static DestinationIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, DestinationIn.class);
    }
}

@Getter
@NoArgsConstructor
class DestinationInSurrogate {
    @JsonProperty("uid")
    String uid;

    @JsonProperty("status")
    SinkStatusIn status;

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

    DestinationInSurrogate(DestinationIn o, String type, JsonNode config) {
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

class DestinationInSerializer extends StdSerializer<DestinationIn> {
    public DestinationInSerializer() {
        this(null);
    }

    public DestinationInSerializer(Class<DestinationIn> t) {
        super(t);
    }

    @Override
    public void serialize(DestinationIn value, JsonGenerator gen, SerializerProvider provider)
            throws IOException {
        DestinationInSurrogate surrogate =
                new DestinationInSurrogate(
                        value, value.getConfig().getVariantName(), value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}

class DestinationInDeserializer extends StdDeserializer<DestinationIn> {
    public DestinationInDeserializer() {
        this(null);
    }

    public DestinationInDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public DestinationIn deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        DestinationInSurrogate surrogate = p.getCodec().readValue(p, DestinationInSurrogate.class);
        String uid = surrogate.getUid();
        SinkStatusIn status = surrogate.getStatus();
        Short batchSize = surrogate.getBatchSize();
        Short maxWaitSecs = surrogate.getMaxWaitSecs();
        List<String> eventTypes = surrogate.getEventTypes();
        List<String> channels = surrogate.getChannels();
        Map<String, String> metadata = surrogate.getMetadata();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        DestinationInConfig sourceType = DestinationInConfig.fromTypeAndConfig(type, config);
        return new DestinationIn(
                uid, status, batchSize, maxWaitSecs, eventTypes, channels, metadata, sourceType);
    }
}
