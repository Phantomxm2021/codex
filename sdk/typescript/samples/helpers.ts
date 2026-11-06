import path from "node:path";

export function bgsPathOverride() {
  return (
    process.env.BGS_EXECUTABLE ??
    path.join(process.cwd(), "..", "..", "bgs-rs", "target", "debug", "bgs")
  );
}
