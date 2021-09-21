package com.svix;

public class ApplicationListOptions extends ListOptions {
    @Override
    public ApplicationListOptions iterator(final String iterator) {
        return (ApplicationListOptions) super.iterator(iterator);
    }

    @Override
    public ApplicationListOptions limit(final Integer limit) {
        return (ApplicationListOptions) super.limit(limit);
    }
}
