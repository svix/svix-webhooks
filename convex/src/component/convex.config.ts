import { defineComponent } from "convex/server";
import { v } from "convex/values";
import workpool from "@convex-dev/workpool/convex.config.js";

const component = defineComponent("svix", {
  env: {
    SVIX_API_KEY: v.string(),
    SVIX_SERVER_URL: v.optional(v.string()),
    // How many days to keep sent and failed messages for. "0" keeps them
    // forever. Defaults to `DEFAULT_RETENTION_DAYS`.
    SVIX_MESSAGE_RETENTION_DAYS: v.optional(v.string()),
  },
});
// Delivers messages to Svix, with retries.
component.use(workpool, { name: "deliveryPool" });

export default component;
