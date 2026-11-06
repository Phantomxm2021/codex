#!/usr/bin/env -S NODE_NO_WARNINGS=1 pnpm ts-node-esm --files

import { Bgs } from "@openai/bgs-sdk";
import { bgsPathOverride } from "./helpers.ts";
import z from "zod";
import zodToJsonSchema from "zod-to-json-schema";

const bgs = new Bgs({ bgsPathOverride: bgsPathOverride() });
const thread = bgs.startThread();

const schema = z.object({
  summary: z.string(),
  status: z.enum(["ok", "action_required"]),
});

const turn = await thread.run("Summarize repository status", {
  outputSchema: zodToJsonSchema(schema, { target: "openAi" }),
});
console.log(turn.finalResponse);
