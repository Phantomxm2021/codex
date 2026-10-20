# BGS distribution

This branch is BeeGameStudio's distribution of OpenAI Codex, maintained by PhantomLab.
The upstream source, licenses, protocol identifiers, model/provider identities and
agent execution behavior are retained. Display branding and product update entry
points are modified. The upstream LICENSE and NOTICE continue to apply.

Maintain branding as ordinary Git commits. Merge a selected upstream release tag
into a temporary branch based on this BGS branch; never reset the branch to upstream.
Build/package through `bun run bgs:build` in the parent BGS repository. The binary
is shipped as `bgs`; the internal Cargo target remains `codex` for compatibility.
The BGS repository's `docs/bgs-runtime-upgrades.md` is the upgrade/runbook authority.

Official package-manager self-update is disabled because it would replace this
build with an unbranded binary. Updating upstream functionality requires merging
source, checking the BGS branding and integration, and producing a new BGS build.
