# BGS runtime

BGS is a PhantomLab distribution based on OpenAI Codex. The upstream LICENSE
and NOTICE remain applicable. Product names, source modules, configuration,
environment variables and storage paths are branded; agent execution semantics
and external service contracts are retained.

Build from the parent BeeGameStudio repository with `bun run bgs:build` and
verify with `bun run bgs:verify`. The internal Cargo package is `bgs-cli`, target
`bgs`, under `bgs-rs/`. No public npm BGS package is assumed or required.

The runtime reads `BGS_HOME`, defaulting to `~/.bgs`. Skills are under
`$BGS_HOME/skills`; installed plugins and caches are under `$BGS_HOME/plugins`.
BGS servers use an owner-private `.runtime/app/bgs` home. Old home directories
require the offline migration described in the parent runbook; do not rename
active account stores or rewrite saved sessions.

Official binary self-updates are disabled. Keep `bgs/source-VERSION` as the
upstream-compatible source branch with the small product patches. Merge the
next official release there, then run the parent's `scripts/brand-bgs.py`
on a separate distribution branch. Never merge upstream directly into the
mechanically renamed tree. See `docs/bgs-runtime-upgrades.md` in BeeGameStudio.

External URLs/model IDs, protocol headers, third-party `.codex-plugin` manifests,
and original attribution are deliberate compatibility exceptions. A local
successful build does not validate untested platforms or live provider logins.
