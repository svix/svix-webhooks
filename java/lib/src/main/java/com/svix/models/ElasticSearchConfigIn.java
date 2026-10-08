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
public class ElasticSearchConfigIn {
    @JsonProperty private String indexName;
    @JsonProperty private URI url;
    @JsonProperty private String apiKey;
    @JsonProperty private Boolean refresh;

    public ElasticSearchConfigIn() {}

    public ElasticSearchConfigIn indexName(String indexName) {
        this.indexName = indexName;
        return this;
    }

    /**
     * Name of the index to write to
     *
     * <p>This can also be a data stream on sufficiently-new versions of ElasticSearch/OpenSearch
     *
     * @return indexName
     */
    @javax.annotation.Nonnull
    public String getIndexName() {
        return indexName;
    }

    public void setIndexName(String indexName) {
        this.indexName = indexName;
    }

    public ElasticSearchConfigIn url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Base URL to send indexing requests
     *
     * <p>This should not include the /{index}/_bulk suffix.
     *
     * @return url
     */
    @javax.annotation.Nonnull
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    public ElasticSearchConfigIn apiKey(String apiKey) {
        this.apiKey = apiKey;
        return this;
    }

    /**
     * API key for authentication for ElasticSearch
     *
     * <p>If not passed, any username:password embedded in the URL will be used. If none is passed,
     * the indexing will be done unauthenticated.
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

    public ElasticSearchConfigIn refresh(Boolean refresh) {
        this.refresh = refresh;
        return this;
    }

    /**
     * If true, Elasticsearch refreshes the affected shards to make this operation visible to
     * search.
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
     * Create an instance of ElasticSearchConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ElasticSearchConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ElasticSearchConfigIn
     */
    public static ElasticSearchConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ElasticSearchConfigIn.class);
    }

    /**
     * Convert an instance of ElasticSearchConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
