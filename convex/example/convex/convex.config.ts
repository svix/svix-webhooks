import { defineApp } from "convex/server";
import { v } from "convex/values";
import svix from "@svix/convex/convex.config.js";

const app = defineApp({
  env: {
    SVIX_API_KEY: v.string(),
    SVIX_SERVER_URL: v.optional(v.string()),
  },
});
app.use(svix, {
  env: {
    SVIX_API_KEY: app.env.SVIX_API_KEY,
    SVIX_SERVER_URL: app.env.SVIX_SERVER_URL,
  },
});

export default app;
