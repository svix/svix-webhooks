import {
    Configuration,
    MessageApi,
    MessageOut,
    MessageIn,
    ListResponseMessageOut,
} from "../openapi";
import { ListOptions, PostOptions } from "../util";

export interface MessageListOptions extends ListOptions {
    eventTypes?: string[];
    before?: Date | null;
    after?: Date | null;
    channel?: string;
    withContent?: boolean;
    tag?: string;
}

export class Message {
    private readonly api: MessageApi;

    public constructor(config: Configuration) {
        this.api = new MessageApi(config);
    }

    public list(
        appId: string,
        options?: MessageListOptions
    ): Promise<ListResponseMessageOut> {
        return this.api.v1MessageList({
            appId,
            ...options,
            iterator: options?.iterator ?? undefined,
            before: options?.before ?? undefined,
            after: options?.after ?? undefined,
        });
    }

    public create(
        appId: string,
        messageIn: MessageIn,
        options?: PostOptions
    ): Promise<MessageOut> {
        return this.api.v1MessageCreate({ appId, messageIn, ...options });
    }

    public get(appId: string, msgId: string): Promise<MessageOut> {
        return this.api.v1MessageGet({ msgId, appId });
    }

    public expungeContent(appId: string, msgId: string): Promise<void> {
        return this.api.v1MessageExpungeContent({ appId, msgId });
    }
}

/**
 * Creates a `MessageIn` with a pre-serialized payload.
 *
 * The payload is not normalized on the server. Normally, payloads are
 * required to be JSON, and Svix will minify the payload before sending the
 * webhooks (for example, by removing extraneous whitespace or unnecessarily
 * escaped characters in strings). With this function, the payload will be
 * sent "as is", without any minification or other processing.
 *
 * @param payload Serialized message payload
 * @param contentType The value to use for the Content-Type header of the webhook sent by Svix, overwriting the default of `application/json` if specified
 *
 * See the class documentation for details about the other parameters.
 */
export function messageInRaw(
    eventType: string,
    payload: string,
    contentType?: string
): MessageIn {
    const headers = contentType ? { "content-type": contentType } : undefined;

    return {
        eventType,
        payload: {},
        transformationsParams: {
            rawPayload: payload,
            headers,
        },
    };
}
