import {
    Configuration,
    IntegrationApi,
    ListResponseIntegrationOut,
    IntegrationOut,
    IntegrationIn,
    IntegrationUpdate,
    IntegrationKeyOut,
} from "../openapi";
import { ListOptions, PostOptions } from "../util";

export type IntegrationListOptions = ListOptions;

export class Integration {
    private readonly api: IntegrationApi;

    public constructor(config: Configuration) {
        this.api = new IntegrationApi(config);
    }

    public list(
        appId: string,
        options?: IntegrationListOptions
    ): Promise<ListResponseIntegrationOut> {
        const iterator = options?.iterator ?? undefined;
        return this.api.v1IntegrationList({ appId, ...options, iterator });
    }

    public create(
        appId: string,
        integrationIn: IntegrationIn,
        options?: PostOptions
    ): Promise<IntegrationOut> {
        return this.api.v1IntegrationCreate({
            appId,
            integrationIn,
            ...options,
        });
    }

    public get(appId: string, integId: string): Promise<IntegrationOut> {
        return this.api.v1IntegrationGet({ integId, appId });
    }

    public update(
        appId: string,
        integId: string,
        integrationUpdate: IntegrationUpdate
    ): Promise<IntegrationOut> {
        return this.api.v1IntegrationUpdate({
            appId,
            integId,
            integrationUpdate,
        });
    }

    public delete(appId: string, integId: string): Promise<void> {
        return this.api.v1IntegrationDelete({
            integId,
            appId,
        });
    }

    public getKey(appId: string, integId: string): Promise<IntegrationKeyOut> {
        return this.api.v1IntegrationGetKey({
            integId,
            appId,
        });
    }

    public rotateKey(
        appId: string,
        integId: string,
        options?: PostOptions
    ): Promise<IntegrationKeyOut> {
        return this.api.v1IntegrationRotateKey({
            integId,
            appId,
            ...options,
        });
    }
}
