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
import { ListOptions, PostOptions } from "../util";

export interface MessageAttemptListOptions extends ListOptions {
    status?: MessageStatus;
    eventTypes?: string[];
    before?: Date | null;
    after?: Date | null;
    statusCodeClass?: StatusCodeClass;
    channel?: string;
    withContent?: boolean;
}

export interface MessageAttemptListByEndpointOptions extends MessageAttemptListOptions {
    withMsg?: boolean;
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
        options?: MessageAttemptListOptions
    ): Promise<ListResponseMessageAttemptOut> {
        return this.listByMsg(appId, msgId, options);
    }

    public listByMsg(
        appId: string,
        msgId: string,
        options?: MessageAttemptListOptions
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
        options?: MessageAttemptListOptions
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
        options?: MessageAttemptListOptions
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
