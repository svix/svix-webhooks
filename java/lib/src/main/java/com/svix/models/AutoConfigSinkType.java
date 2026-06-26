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

@Setter
@Getter
@ToString
@NoArgsConstructor
@EqualsAndHashCode
@AllArgsConstructor
@JsonSerialize(using = AutoConfigSinkTypeSerializer.class)
@JsonDeserialize(using = AutoConfigSinkTypeDeserializer.class)
public class AutoConfigSinkType {
    private AutoConfigSinkTypeConfig config;

    public AutoConfigSinkType config(AutoConfigSinkTypeConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static AutoConfigSinkType fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AutoConfigSinkType.class);
    }
}

@Getter
@NoArgsConstructor
class AutoConfigSinkTypeSurrogate {
    @JsonProperty("type")
    String type;

    @JsonProperty("config")
    JsonNode config;

    AutoConfigSinkTypeSurrogate(AutoConfigSinkType o, String type, JsonNode config) {
        this.type = type;
        this.config = config;
    }
}

class AutoConfigSinkTypeSerializer extends StdSerializer<AutoConfigSinkType> {
    public AutoConfigSinkTypeSerializer() {
        this(null);
    }

    public AutoConfigSinkTypeSerializer(Class<AutoConfigSinkType> t) {
        super(t);
    }

    @Override
    public void serialize(AutoConfigSinkType value, JsonGenerator gen, SerializerProvider provider)
            throws IOException {
        AutoConfigSinkTypeSurrogate surrogate =
                new AutoConfigSinkTypeSurrogate(
                        value, value.getConfig().getVariantName(), value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}

class AutoConfigSinkTypeDeserializer extends StdDeserializer<AutoConfigSinkType> {
    public AutoConfigSinkTypeDeserializer() {
        this(null);
    }

    public AutoConfigSinkTypeDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public AutoConfigSinkType deserialize(JsonParser p, DeserializationContext ctxt)
            throws IOException {
        AutoConfigSinkTypeSurrogate surrogate =
                p.getCodec().readValue(p, AutoConfigSinkTypeSurrogate.class);
        String type = surrogate.getType();
        JsonNode config = surrogate.getConfig();
        AutoConfigSinkTypeConfig sourceType =
                AutoConfigSinkTypeConfig.fromTypeAndConfig(type, config);
        return new AutoConfigSinkType(sourceType);
    }
}
