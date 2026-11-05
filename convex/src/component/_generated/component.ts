/* eslint-disable */
/**
 * Generated `ComponentApi` utility.
 *
 * THIS CODE IS AUTOMATICALLY GENERATED.
 *
 * To regenerate, run `npx convex dev`.
 * @module
 */

import type { FunctionReference } from "convex/server";

/**
 * A utility for referencing a Convex component's exposed API.
 *
 * Useful when expecting a parameter like `components.myComponent`.
 * Usage:
 * ```ts
 * async function myFunction(ctx: QueryCtx, component: ComponentApi) {
 *   return ctx.runQuery(component.someFile.someQuery, { ...args });
 * }
 * ```
 */
export type ComponentApi<Name extends string | undefined = string | undefined> =
  {
    applications: {
      portalAccess: FunctionReference<
        "action",
        "internal",
        {
          appId: string;
          application?: { name: string; uid?: string };
          capabilities?: Array<
            | "ViewBase"
            | "ViewEndpointSecret"
            | "ManageEndpointSecret"
            | "ManageTransformations"
            | "CreateAttempts"
            | "ManageEndpoint"
          >;
          expiry?: number;
          featureFlags?: Array<string>;
          readOnly?: boolean;
        },
        { token: string; url: string },
        Name
      >;
    };
    messages: {
      get: FunctionReference<
        "query",
        "internal",
        { messageId: string },
        null | {
          _creationTime: number;
          _id: string;
          appId: string;
          application?: { name: string; uid?: string };
          attempts: number;
          channels?: Array<string>;
          eventId?: string;
          eventType: string;
          lastError?: string;
          payload: string;
          status: "pending" | "sent" | "failed";
          svixMessageId?: string;
          tags?: Array<string>;
        },
        Name
      >;
      list: FunctionReference<
        "query",
        "internal",
        { appId: string; limit?: number },
        Array<{
          _creationTime: number;
          _id: string;
          appId: string;
          application?: { name: string; uid?: string };
          attempts: number;
          channels?: Array<string>;
          eventId?: string;
          eventType: string;
          lastError?: string;
          status: "pending" | "sent" | "failed";
          svixMessageId?: string;
          tags?: Array<string>;
        }>,
        Name
      >;
      retry: FunctionReference<
        "mutation",
        "internal",
        { messageId: string },
        boolean,
        Name
      >;
      retryFailed: FunctionReference<
        "mutation",
        "internal",
        { after?: number; appId?: string },
        null,
        Name
      >;
      send: FunctionReference<
        "mutation",
        "internal",
        {
          appId: string;
          application?: { name: string; uid?: string };
          channels?: Array<string>;
          eventId?: string;
          eventType: string;
          payload: string;
          tags?: Array<string>;
        },
        string,
        Name
      >;
    };
  };
