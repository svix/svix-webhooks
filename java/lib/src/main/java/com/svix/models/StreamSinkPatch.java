// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonInclude;
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
import com.svix.MaybeUnset;
import com.svix.Utils;

import lombok.AccessLevel;
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
    @Getter(AccessLevel.NONE)
    @Setter(AccessLevel.NONE)
    private MaybeUnset<String> uid;

    @Getter(AccessLevel.NONE)
    @Setter(AccessLevel.NONE)
    private MaybeUnset<SinkStatusIn> status;

    @Getter(AccessLevel.NONE)
    @Setter(AccessLevel.NONE)
    private MaybeUnset<Short> batchSize;

    @Getter(AccessLevel.NONE)
    @Setter(AccessLevel.NONE)
    private MaybeUnset<Short> maxWaitSecs;

    private List<String> eventTypes;
    private List<String> channels;
    private Map<String, String> metadata;
    private StreamSinkPatchConfig config;

    public StreamSinkPatch uid(String uid) {
        this.uid = new MaybeUnset<>(uid);
        return this;
    }

    public String getUid() {
        if (this.uid == null) {
            return null;
        }
        return this.uid.getValue();
    }

    public void setUid(String uid) {
        this.uid = new MaybeUnset<>(uid);
    }

    MaybeUnset<String> uidOrUnset() {
        return this.uid;
    }

    public StreamSinkPatch status(SinkStatusIn status) {
        this.status = new MaybeUnset<>(status);
        return this;
    }

    public SinkStatusIn getStatus() {
        if (this.status == null) {
            return null;
        }
        return this.status.getValue();
    }

    public void setStatus(SinkStatusIn status) {
        this.status = new MaybeUnset<>(status);
    }

    MaybeUnset<SinkStatusIn> statusOrUnset() {
        return this.status;
    }

    public StreamSinkPatch batchSize(Short batchSize) {
        this.batchSize = new MaybeUnset<>(batchSize);
        return this;
    }

    public Short getBatchSize() {
        if (this.batchSize == null) {
            return null;
        }
        return this.batchSize.getValue();
    }

    public void setBatchSize(Short batchSize) {
        this.batchSize = new MaybeUnset<>(batchSize);
    }

    MaybeUnset<Short> batchSizeOrUnset() {
        return this.batchSize;
    }

    public StreamSinkPatch maxWaitSecs(Short maxWaitSecs) {
        this.maxWaitSecs = new MaybeUnset<>(maxWaitSecs);
        return this;
    }

    public Short getMaxWaitSecs() {
        if (this.maxWaitSecs == null) {
            return null;
        }
        return this.maxWaitSecs.getValue();
    }

    public void setMaxWaitSecs(Short maxWaitSecs) {
        this.maxWaitSecs = new MaybeUnset<>(maxWaitSecs);
    }

    MaybeUnset<Short> maxWaitSecsOrUnset() {
        return this.maxWaitSecs;
    }

    public StreamSinkPatch eventTypes(List<String> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public StreamSinkPatch channels(List<String> channels) {
        this.channels = channels;
        return this;
    }

    public StreamSinkPatch metadata(Map<String, String> metadata) {
        this.metadata = metadata;
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
@JsonInclude(JsonInclude.Include.NON_NULL)
class StreamSinkPatchSurrogate {
    @JsonProperty("uid")
    MaybeUnset<String> uid;

    @JsonProperty("status")
    MaybeUnset<SinkStatusIn> status;

    @JsonProperty("batchSize")
    MaybeUnset<Short> batchSize;

    @JsonProperty("maxWaitSecs")
    MaybeUnset<Short> maxWaitSecs;

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

    StreamSinkPatchSurrogate(StreamSinkPatch o, String type, JsonNode config) {
        this.uid = o.uidOrUnset();
        this.status = o.statusOrUnset();
        this.batchSize = o.batchSizeOrUnset();
        this.maxWaitSecs = o.maxWaitSecsOrUnset();
        this.eventTypes = o.getEventTypes();
        this.channels = o.getChannels();
        this.metadata = o.getMetadata();
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
        MaybeUnset<String> uid = surrogate.getUid();
        MaybeUnset<SinkStatusIn> status = surrogate.getStatus();
        MaybeUnset<Short> batchSize = surrogate.getBatchSize();
        MaybeUnset<Short> maxWaitSecs = surrogate.getMaxWaitSecs();
        List<String> eventTypes = surrogate.getEventTypes();
        List<String> channels = surrogate.getChannels();
        Map<String, String> metadata = surrogate.getMetadata();
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        StreamSinkPatchConfig sourceType = StreamSinkPatchConfig.fromTypeAndConfig(type, config);
        return new StreamSinkPatch(
                uid, status, batchSize, maxWaitSecs, eventTypes, channels, metadata, sourceType);
    }
}
