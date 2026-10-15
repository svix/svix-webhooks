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
      getOrCreate: FunctionReference<
        "action",
        "internal",
        { metadata?: Record<string, string>; name: string; uid?: string },
        { id: string; name: string; uid?: string },
        Name
      >;
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
      remove: FunctionReference<
        "action",
        "internal",
        { appId: string },
        null,
        Name
      >;
    };
    messages: {
      cleanup: FunctionReference<
        "mutation",
        "internal",
        { batchSize?: number; before: number },
        { deleted: number; hasMore: boolean },
        Name
      >;
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
          deliveryJobId?: string;
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
          deliveryJobId?: string;
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
        null,
        Name
      >;
      retryFailed: FunctionReference<
        "mutation",
        "internal",
        { appId?: string; batchSize?: number },
        { hasMore: boolean; retried: number },
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
