import { cronJobs } from "convex/server";
import { internal } from "./_generated/api.js";

const crons = cronJobs();

crons.interval(
  "requeue stuck messages",
  { minutes: 5 },
  internal.messages.requeueStuck,
  {},
);

crons.daily(
  "delete expired messages",
  { hourUTC: 4, minuteUTC: 0 },
  internal.messages.deleteExpired,
  {},
);

export default crons;
