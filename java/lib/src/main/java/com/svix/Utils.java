package com.svix;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;

import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

public class Utils {
    private static boolean isEnum(Object v) {
        return v != null && v.getClass().isEnum();
    }

    private static boolean isList(Object v) {
        return v instanceof List;
    }

    private static boolean isSet(Object v) {
        return v instanceof Set;
    }

    public static String serializeQueryParam(Object v) {
        if (isEnum(v)) {
            return ((ToQueryParam) v).toQueryParam();
        } else if (isList(v)) {
            return ((List<?>) v)
                    .stream()
                            .map(Utils::serializeQueryParam)
                            .sorted()
                            .collect(Collectors.joining(","));
        } else if (isSet(v)) {
            return ((Set<?>) v)
                    .stream()
                            .map(Utils::serializeQueryParam)
                            .sorted()
                            .collect(Collectors.joining(","));
        } else {
            return v.toString();
        }
    }

    public interface ToQueryParam {
        // Used to get the enums correct representation as a query param
        // does not url encode the returned string
        String toQueryParam();
    }

    public static ObjectMapper getObjectMapper() {
        ObjectMapper mapper = new ObjectMapper();
        mapper.disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS);
        mapper.enable(JsonParser.Feature.INCLUDE_SOURCE_IN_LOCATION);
        mapper.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
        mapper.registerModule(new JavaTimeModule());
        mapper.registerModule(new Jdk8Module());
        return mapper;
    }
}
