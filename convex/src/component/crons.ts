import { cronJobs } from "convex/server";
import { internal } from "./_generated/api.js";

const crons = cronJobs();

crons.daily(
  "delete expired messages",
  { hourUTC: 4, minuteUTC: 0 },
  internal.messages.deleteExpired,
  {},
);

export default crons;
