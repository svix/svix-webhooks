import { defineComponent } from "convex/server";
import { v } from "convex/values";

export default defineComponent("svix", {
  env: {
    SVIX_API_KEY: v.string(),
    SVIX_SERVER_URL: v.optional(v.string()),
    // How many days to keep sent and failed messages for. "0" keeps them
    // forever. Defaults to `DEFAULT_RETENTION_DAYS`.
    SVIX_MESSAGE_RETENTION_DAYS: v.optional(v.string()),
  },
});
