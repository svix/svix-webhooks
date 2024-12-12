import {
    Configuration,
    EventTypeApi,
    ListResponseEventTypeOut,
    EventTypeIn,
    EventTypeOut,
    EventTypeUpdate,
    EventTypePatch,
    EventTypeImportOpenApiIn,
    EventTypeImportOpenApiOut,
} from "../openapi";
import { ListOptions, PostOptions } from "../util";

export interface EventTypeListOptions extends ListOptions {
    withContent?: boolean;
    includeArchived?: boolean;
}

export class EventType {
    private readonly api: EventTypeApi;

    public constructor(config: Configuration) {
        this.api = new EventTypeApi(config);
    }

    public list(options?: EventTypeListOptions): Promise<ListResponseEventTypeOut> {
        const iterator = options?.iterator ?? undefined;
        return this.api.v1EventTypeList({ ...options, iterator });
    }

    public get(eventTypeName: string): Promise<EventTypeOut> {
        return this.api.v1EventTypeGet({ eventTypeName });
    }

    public create(eventTypeIn: EventTypeIn, options?: PostOptions): Promise<EventTypeOut> {
        return this.api.v1EventTypeCreate({ eventTypeIn, ...options });
    }

    public update(
        eventTypeName: string,
        eventTypeUpdate: EventTypeUpdate
    ): Promise<EventTypeOut> {
        return this.api.v1EventTypeUpdate({
            eventTypeName,
            eventTypeUpdate,
        });
    }

    public patch(
        eventTypeName: string,
        eventTypePatch: EventTypePatch
    ): Promise<EventTypeOut> {
        return this.api.v1EventTypePatch({
            eventTypeName,
            eventTypePatch,
        });
    }

    public delete(eventTypeName: string): Promise<void> {
        return this.api.v1EventTypeDelete({ eventTypeName });
    }

    public importOpenApi(
        eventTypeImportOpenApiIn: EventTypeImportOpenApiIn,
        options?: PostOptions
    ): Promise<EventTypeImportOpenApiOut> {
        return this.api.v1EventTypeImportOpenapi({ eventTypeImportOpenApiIn, ...options });
    }
}
