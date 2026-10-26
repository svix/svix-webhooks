// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.Utils;

import lombok.EqualsAndHashCode;
import lombok.ToString;

import java.net.URI;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class PostHogConfigIn {
    @JsonProperty private URI domain;
    @JsonProperty private String projectToken;

    public PostHogConfigIn() {}

    public PostHogConfigIn domain(URI domain) {
        this.domain = domain;
        return this;
    }

    /**
     * The PostHog ingestion domain.
     *
     * <p>This is https://us.i.posthog.com for US Cloud, https://eu.i.posthog.com for EU Cloud, and
     * your self-hosted domain for self-hosted instances. Note the `.i.` in the Cloud hosts
     * (`us.posthog.com` and `eu.posthog.com` are different, private endpoints).
     *
     * @return domain
     */
    @javax.annotation.Nonnull
    public URI getDomain() {
        return domain;
    }

    public void setDomain(URI domain) {
        this.domain = domain;
    }

    public PostHogConfigIn projectToken(String projectToken) {
        this.projectToken = projectToken;
        return this;
    }

    /**
     * The PostHog project API key.
     *
     * <p>Sent as the `api_key` field in the body of each `/batch/` request.
     *
     * @return projectToken
     */
    @javax.annotation.Nonnull
    public String getProjectToken() {
        return projectToken;
    }

    public void setProjectToken(String projectToken) {
        this.projectToken = projectToken;
    }

    /**
     * Create an instance of PostHogConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PostHogConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to PostHogConfigIn
     */
    public static PostHogConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PostHogConfigIn.class);
    }

    /**
     * Convert an instance of PostHogConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
