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
public class ElasticSearchConfigPatch {
    @JsonProperty private String indexName;
    @JsonProperty private URI url;
    @JsonProperty private String apiKey;
    @JsonProperty private Boolean refresh;

    public ElasticSearchConfigPatch() {}

    public ElasticSearchConfigPatch indexName(String indexName) {
        this.indexName = indexName;
        return this;
    }

    /**
     * Get indexName
     *
     * @return indexName
     */
    @javax.annotation.Nullable
    public String getIndexName() {
        return indexName;
    }

    public void setIndexName(String indexName) {
        this.indexName = indexName;
    }

    public ElasticSearchConfigPatch url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nullable
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    public ElasticSearchConfigPatch apiKey(String apiKey) {
        this.apiKey = apiKey;
        return this;
    }

    /**
     * Get apiKey
     *
     * @return apiKey
     */
    @javax.annotation.Nullable
    public String getApiKey() {
        return apiKey;
    }

    public void setApiKey(String apiKey) {
        this.apiKey = apiKey;
    }

    public ElasticSearchConfigPatch refresh(Boolean refresh) {
        this.refresh = refresh;
        return this;
    }

    /**
     * Get refresh
     *
     * @return refresh
     */
    @javax.annotation.Nullable
    public Boolean getRefresh() {
        return refresh;
    }

    public void setRefresh(Boolean refresh) {
        this.refresh = refresh;
    }

    /**
     * Create an instance of ElasticSearchConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ElasticSearchConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ElasticSearchConfigPatch
     */
    public static ElasticSearchConfigPatch fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ElasticSearchConfigPatch.class);
    }

    /**
     * Convert an instance of ElasticSearchConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
