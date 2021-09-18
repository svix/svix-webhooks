package com.svix;

public class EndpointListOptions extends ListOptions {
    @Override
    public EndpointListOptions iterator(final String iterator) {
        return (EndpointListOptions) super.iterator(iterator);
    }

    @Override
    public EndpointListOptions limit(final Integer limit) {
        return (EndpointListOptions) super.limit(limit);
    }
}
