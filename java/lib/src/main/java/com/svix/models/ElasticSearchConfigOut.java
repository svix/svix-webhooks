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
public class ElasticSearchConfigOut {
    @JsonProperty private String indexName;
    @JsonProperty private URI url;
    @JsonProperty private Boolean refresh;

    public ElasticSearchConfigOut() {}

    public ElasticSearchConfigOut indexName(String indexName) {
        this.indexName = indexName;
        return this;
    }

    /**
     * Get indexName
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

    public ElasticSearchConfigOut url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
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

    public ElasticSearchConfigOut refresh(Boolean refresh) {
        this.refresh = refresh;
        return this;
    }

    /**
     * Get refresh
     *
     * @return refresh
     */
    @javax.annotation.Nonnull
    public Boolean getRefresh() {
        return refresh;
    }

    public void setRefresh(Boolean refresh) {
        this.refresh = refresh;
    }

    /**
     * Create an instance of ElasticSearchConfigOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ElasticSearchConfigOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ElasticSearchConfigOut
     */
    public static ElasticSearchConfigOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ElasticSearchConfigOut.class);
    }

    /**
     * Convert an instance of ElasticSearchConfigOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
