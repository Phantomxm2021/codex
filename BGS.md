# BGS runtime

BGS is a PhantomLab distribution based on OpenAI Codex. LICENSE and NOTICE
retain upstream attribution. Native execution, Goal accounting, history,
model metadata and wire protocols follow the selected upstream release.

Build from BeeGameStudio with `bun run bgs:build`, then `bun run bgs:verify`.
The public executable is `bgs`; internal Cargo names and the companion
`codex-code-mode-host` retain upstream names under `codex-rs/`.

The runtime reads `BGS_HOME`, defaulting to `~/.bgs`. BeeGameStudio supplies
an owner-private `.runtime/app/bgs` home. Existing sessions are not rewritten.
Protocol fields, MCP metadata, model/service identifiers, plugin formats and
project-level `.codex` configuration retain the upstream contract.

Official binary self-replacement is disabled for this source-built package.
Merge upstream into `bgs/source-VERSION`, review its branding/packaging-only
diff, then run `scripts/brand-bgs.py` to generate the distribution. Do not add
native behavior patches without explicit user authorization. See the parent
`docs/bgs-runtime-upgrades.md` for provenance, validation and deployment.
