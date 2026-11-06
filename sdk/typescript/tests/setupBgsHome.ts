import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, beforeEach } from "@jest/globals";

const originalBgsHome = process.env.BGS_HOME;
let currentBgsHome: string | undefined;

beforeEach(async () => {
  currentBgsHome = await fs.mkdtemp(path.join(os.tmpdir(), "bgs-sdk-test-"));
  process.env.BGS_HOME = currentBgsHome;
});

afterEach(async () => {
  const bgsHomeToDelete = currentBgsHome;
  currentBgsHome = undefined;

  if (originalBgsHome === undefined) {
    delete process.env.BGS_HOME;
  } else {
    process.env.BGS_HOME = originalBgsHome;
  }

  if (bgsHomeToDelete) {
    await fs.rm(bgsHomeToDelete, { recursive: true, force: true });
  }
});
