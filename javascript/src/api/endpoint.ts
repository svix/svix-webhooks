import {
    Configuration,
    EndpointApi,
    ListResponseEndpointOut,
    EndpointOut,
    EndpointIn,
    EndpointUpdate,
    EndpointSecretOut,
    EndpointSecretRotateIn,
    EndpointTransformationIn,
    EndpointTransformationOut,
    EndpointHeadersIn,
    EndpointHeadersPatchIn,
    EndpointHeadersOut,
    EndpointStats,
    EventExampleIn,
    RecoverIn,
    ReplayIn,
    MessageOut,
    Ordering,
    EndpointPatch,
    EndpointOauthConfigIn,
} from "../openapi";
import { ListOptions, PostOptions } from "../util";

export interface EndpointListOptions extends ListOptions {
    order?: Ordering;
}

export interface EndpointStatsOptions {
    since?: Date | null;
    until?: Date | null;
}

export class Endpoint {
    private readonly api: EndpointApi;

    public constructor(config: Configuration) {
        this.api = new EndpointApi(config);
    }

    public list(
        appId: string,
        options?: EndpointListOptions
    ): Promise<ListResponseEndpointOut> {
        const iterator = options?.iterator ?? undefined;
        return this.api.v1EndpointList({ appId, ...options, iterator });
    }

    public create(
        appId: string,
        endpointIn: EndpointIn,
        options?: PostOptions
    ): Promise<EndpointOut> {
        return this.api.v1EndpointCreate({
            appId,
            endpointIn,
            ...options,
        });
    }

    public get(appId: string, endpointId: string): Promise<EndpointOut> {
        return this.api.v1EndpointGet({ endpointId, appId });
    }

    public update(
        appId: string,
        endpointId: string,
        endpointUpdate: EndpointUpdate
    ): Promise<EndpointOut> {
        return this.api.v1EndpointUpdate({
            appId,
            endpointId,
            endpointUpdate,
        });
    }

    public patch(
        appId: string,
        endpointId: string,
        endpointPatch: EndpointPatch
    ): Promise<EndpointOut> {
        return this.api.v1EndpointPatch({
            appId,
            endpointId,
            endpointPatch,
        });
    }

    public delete(appId: string, endpointId: string): Promise<void> {
        return this.api.v1EndpointDelete({
            endpointId,
            appId,
        });
    }

    public getSecret(appId: string, endpointId: string): Promise<EndpointSecretOut> {
        return this.api.v1EndpointGetSecret({
            endpointId,
            appId,
        });
    }

    public rotateSecret(
        appId: string,
        endpointId: string,
        endpointSecretRotateIn: EndpointSecretRotateIn,
        options?: PostOptions
    ): Promise<void> {
        return this.api.v1EndpointRotateSecret({
            endpointId,
            appId,
            endpointSecretRotateIn,
            ...options,
        });
    }

    public recover(
        appId: string,
        endpointId: string,
        recoverIn: RecoverIn,
        options?: PostOptions
    ): Promise<void> {
        return this.api
            .v1EndpointRecover({
                appId,
                endpointId,
                recoverIn,
                ...options,
            })
            .then(() => Promise.resolve());
    }

    public replayMissing(
        appId: string,
        endpointId: string,
        replayIn: ReplayIn,
        options?: PostOptions
    ): Promise<void> {
        return this.api
            .v1EndpointReplay({
                appId,
                endpointId,
                replayIn,
                ...options,
            })
            .then(() => Promise.resolve());
    }

    public getHeaders(appId: string, endpointId: string): Promise<EndpointHeadersOut> {
        return this.api.v1EndpointGetHeaders({
            appId,
            endpointId,
        });
    }

    /**
     * @deprecated Since version 1.30.0. Use `headersUpdate` instead.
     */
    public updateHeaders(
        appId: string,
        endpointId: string,
        endpointHeadersIn: EndpointHeadersIn
    ): Promise<void> {
        return this.api.v1EndpointUpdateHeaders({
            appId,
            endpointId,
            endpointHeadersIn,
        });
    }

    public headersUpdate(
        appId: string,
        endpointId: string,
        endpointHeadersIn: EndpointHeadersIn
    ): Promise<void> {
        return this.api.v1EndpointUpdateHeaders({
            appId,
            endpointId,
            endpointHeadersIn,
        });
    }

    /**
     * @deprecated Since version 1.30.0. Use `headersPatch` instead.
     */
    public patchHeaders(
        appId: string,
        endpointId: string,
        endpointHeadersPatchIn: EndpointHeadersPatchIn
    ): Promise<void> {
        return this.api.v1EndpointPatchHeaders({
            appId,
            endpointId,
            endpointHeadersPatchIn,
        });
    }

    public headersPatch(
        appId: string,
        endpointId: string,
        endpointHeadersPatchIn: EndpointHeadersPatchIn
    ): Promise<void> {
        return this.api.v1EndpointPatchHeaders({
            appId,
            endpointId,
            endpointHeadersPatchIn,
        });
    }

    public getStats(
        appId: string,
        endpointId: string,
        options?: EndpointStatsOptions
    ): Promise<EndpointStats> {
        return this.api.v1EndpointGetStats({
            appId,
            endpointId,
            ...options,
            since: options?.since ?? undefined,
            until: options?.until ?? undefined,
        });
    }

    public transformationGet(
        appId: string,
        endpointId: string
    ): Promise<EndpointTransformationOut> {
        return this.api.v1EndpointTransformationGet({ endpointId, appId });
    }

    public transformationPartialUpdate(
        appId: string,
        endpointId: string,
        endpointTransformationIn: EndpointTransformationIn
    ): Promise<void> {
        return this.api.v1EndpointTransformationPartialUpdate({
            appId,
            endpointId,
            endpointTransformationIn,
        });
    }

    public sendExample(
        appId: string,
        endpointId: string,
        eventExampleIn: EventExampleIn,
        options?: PostOptions
    ): Promise<MessageOut> {
        return this.api.v1EndpointSendExample({
            appId,
            endpointId,
            eventExampleIn,
            ...options,
        });
    }

    public oauthUpdate(
        appId: string,
        endpointId: string,
        endpointOauthConfigIn: EndpointOauthConfigIn
    ): Promise<void> {
        return this.api.v1EndpointUpdateOauthConfig({
            appId,
            endpointId,
            endpointOauthConfigIn,
        });
    }

    public oauthDelete(appId: string, endpointId: string): Promise<void> {
        return this.api.v1EndpointDeleteOauthConfig({
            appId,
            endpointId,
        });
    }
}
