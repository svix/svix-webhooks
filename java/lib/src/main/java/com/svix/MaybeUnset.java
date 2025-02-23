package com.svix;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize(using = MaybeUnsetSerializer.class)
public class MaybeUnset<T> {
    private T value;
    private boolean isSet;

    public MaybeUnset() {
        this.isSet = false;
        this.value = null;
    }

    public MaybeUnset(T value) {
        this.value = value;
        this.isSet = true;
    }

    public T getValue() {
        return value;
    }

    public boolean isSet() {
        return isSet;
    }
}
