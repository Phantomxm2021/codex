# BGS CLI Runtime for Python SDK

Platform-specific runtime package consumed by the published `openai-bgs`.

This package is staged during release so the SDK can pin an exact BGS CLI
version without checking platform binaries into the repo.

`openai-bgs-cli-bin` is intentionally wheel-only. Do not build or publish an
sdist for this package.
