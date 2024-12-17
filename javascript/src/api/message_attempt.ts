import {
    Configuration,
    MessageAttemptApi,
    ListResponseEndpointMessageOut,
    ListResponseMessageEndpointOut,
    ListResponseMessageAttemptEndpointOut,
    ListResponseMessageAttemptOut,
    MessageAttemptOut,
    MessageStatus,
    StatusCodeClass,
} from "../openapi";
import { PostOptions } from "../util";

export interface MessageAttemptListOptions {
    iterator?: string | null;
    limit?: number;
    status?: MessageStatus;
    eventTypes?: string[];
    before?: Date | null;
    after?: Date | null;
    statusCodeClass?: StatusCodeClass;
    channel?: string;
    withContent?: boolean;
}

export interface MessageAttemptListByEndpointOptions {
    /// Limit the number of returned items
    limit?: number;
    /// The iterator returned from a prior invocation
    iterator?: string | null;
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    status?: MessageStatus;
    /// Filter response based on the HTTP status code
    statusCodeClass?: StatusCodeClass;
    /// Filter response based on the channel
    channel?: string;
    /// Filter response based on the tag
    tag?: string;
    /// Only include items created before a certain date
    before?: Date | null;
    /// Only include items created after a certain date
    after?: Date | null;
    /// When `true` attempt content is included in the response
    withContent?: boolean;
    /// When `true`, the message information is included in the response
    withMsg?: boolean;
    /// Filter response based on the event type
    eventTypes?: string[];
}

export interface MessageAttemptListByMsgOptions {
    /// Limit the number of returned items
    limit?: number;
    /// The iterator returned from a prior invocation
    iterator?: string | null;
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    status?: MessageStatus;
    /// Filter response based on the HTTP status code
    statusCodeClass?: StatusCodeClass;
    /// Filter response based on the channel
    channel?: string;
    /// Filter response based on the tag
    tag?: string;
    /// Filter the attempts based on the attempted endpoint
    endpointId?: string;
    /// Only include items created before a certain date
    before?: Date | null;
    /// Only include items created after a certain date
    after?: Date | null;
    /// When `true` attempt content is included in the response
    withContent?: boolean;
    /// Filter response based on the event type
    eventTypes?: string[];
}

export interface MessageAttemptListAttemptedMessagesOptions {
    /// Limit the number of returned items
    limit?: number;
    /// The iterator returned from a prior invocation
    iterator?: string | null;
    /// Filter response based on the channel
    channel?: string;
    /// Filter response based on the message tags
    tag?: string;
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    status?: MessageStatus;
    /// Only include items created before a certain date
    before?: Date | null;
    /// Only include items created after a certain date
    after?: Date | null;
    /// When `true` message payloads are included in the response
    withContent?: boolean;
    /// Filter response based on the event type
    eventTypes?: string[];
}

export interface MessageAttemptListAttemptedDestinationsOptions {
    /// Limit the number of returned items
    limit?: number;
    /// The iterator returned from a prior invocation
    iterator?: string | null;
}

export class MessageAttempt {
    private readonly api: MessageAttemptApi;

    public constructor(config: Configuration) {
        this.api = new MessageAttemptApi(config);
    }

    /**
     * @deprecated Since version 0.48.0. Use listByMsg or listByEndpoint instead.
     */
    public list(
        appId: string,
        msgId: string,
        options?: MessageAttemptListByMsgOptions
    ): Promise<ListResponseMessageAttemptOut> {
        return this.listByMsg(appId, msgId, options);
    }

    public listByMsg(
        appId: string,
        msgId: string,
        options?: MessageAttemptListByMsgOptions
    ): Promise<ListResponseMessageAttemptOut> {
        return this.api.v1MessageAttemptListByMsg({
            appId,
            msgId,
            ...options,
            iterator: options?.iterator ?? undefined,
            before: options?.before ?? undefined,
            after: options?.after ?? undefined,
        });
    }

    public listByEndpoint(
        appId: string,
        endpointId: string,
        options?: MessageAttemptListByEndpointOptions
    ): Promise<ListResponseMessageAttemptOut> {
        return this.api.v1MessageAttemptListByEndpoint({
            appId,
            endpointId,
            ...options,
            iterator: options?.iterator ?? undefined,
            before: options?.before ?? undefined,
            after: options?.after ?? undefined,
        });
    }

    public get(
        appId: string,
        msgId: string,
        attemptId: string
    ): Promise<MessageAttemptOut> {
        return this.api.v1MessageAttemptGet({
            attemptId,
            msgId,
            appId,
        });
    }

    public resend(
        appId: string,
        msgId: string,
        endpointId: string,
        options?: PostOptions
    ): Promise<void> {
        return this.api.v1MessageAttemptResend({
            endpointId,
            msgId,
            appId,
            ...options,
        });
    }

    public listAttemptedMessages(
        appId: string,
        endpointId: string,
        options?: MessageAttemptListAttemptedMessagesOptions
    ): Promise<ListResponseEndpointMessageOut> {
        return this.api.v1MessageAttemptListAttemptedMessages({
            appId,
            endpointId,
            ...options,
            iterator: options?.iterator ?? undefined,
            before: options?.before ?? undefined,
            after: options?.after ?? undefined,
        });
    }

    public listAttemptedDestinations(
        appId: string,
        msgId: string,
        options?: MessageAttemptListAttemptedDestinationsOptions
    ): Promise<ListResponseMessageEndpointOut> {
        return this.api.v1MessageAttemptListAttemptedDestinations({
            appId,
            msgId,
            ...options,
            iterator: options?.iterator ?? undefined,
        });
    }

    public listAttemptsForEndpoint(
        appId: string,
        msgId: string,
        endpointId: string,
        options?: MessageAttemptListOptions
    ): Promise<ListResponseMessageAttemptEndpointOut> {
        return this.api.v1MessageAttemptListByEndpointDeprecated({
            appId,
            msgId,
            endpointId,
            ...options,
            iterator: options?.iterator ?? undefined,
            before: options?.before ?? undefined,
            after: options?.after ?? undefined,
        });
    }

    public expungeContent(appId: string, msgId: string, attemptId: string): Promise<void> {
        return this.api.v1MessageAttemptExpungeContent({
            appId,
            msgId,
            attemptId,
        });
    }
}
