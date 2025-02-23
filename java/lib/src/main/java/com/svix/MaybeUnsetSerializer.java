package com.svix;

import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.databind.JsonSerializer;
import com.fasterxml.jackson.databind.SerializerProvider;

import java.io.IOException;

public class MaybeUnsetSerializer<T> extends JsonSerializer<MaybeUnset<T>> {
    @Override
    public void serialize(MaybeUnset<T> field, JsonGenerator gen, SerializerProvider provider)
            throws IOException {

        if (!field.isSet()) {
            return;
        }

        gen.writeObject(field.getValue());
    }

    @Override
    public boolean isEmpty(SerializerProvider provider, MaybeUnset<T> value) {
        return !value.isSet();
    }
}
