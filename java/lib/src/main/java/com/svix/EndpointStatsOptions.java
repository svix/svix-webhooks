package com.svix;

import java.time.OffsetDateTime;

public class EndpointStatsOptions {
    private OffsetDateTime since;
    private OffsetDateTime until;

    public void setSince(final OffsetDateTime since) {
        this.since = since;
    }

    public OffsetDateTime getSince() {
        return since;
    }

    public void setUntil(final OffsetDateTime until) {
        this.until = until;
    }

    public OffsetDateTime getUntil() {
        return until;
    }
}
