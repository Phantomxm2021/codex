"""Materialize the stable release packages selected before running Bazel."""

_BGS_RELEASE_BUILD_FILE = """\
package(default_visibility = ["//visibility:public"])

filegroup(
    name = "bgs",
    srcs = [{entrypoint}],
)

filegroup(
    name = "package",
    srcs = glob([
        "bgs-package.json",
        {binaries},
        {resources},
        {path},
    ]),
)
"""

def _bgs_release_repository_impl(repository_ctx):
    releases = repository_ctx.getenv("BGS_COMPAT_RELEASES")
    if not releases:
        fail("Set BGS_COMPAT_RELEASES to the JSON from resolve_exec_server_releases.py before building release compatibility targets.")
    release = json.decode(releases)[repository_ctx.attr.release]
    asset = "bgs-package-x86_64-unknown-linux-musl.tar.gz"
    version = release["version"]
    repository_ctx.download_and_extract(
        url = [
            "https://releases.openai.com/codex/releases/{}/{}".format(version, asset),
            "https://github.com/openai/codex/releases/download/rust-v{}/{}".format(version, asset),
        ],
        sha256 = release["sha256"],
    )
    manifest = json.decode(repository_ctx.read("bgs-package.json"))
    entrypoint = manifest["entrypoint"]
    binaries = entrypoint.rpartition("/")[0] + "/**"
    repository_ctx.file(
        "BUILD.bazel",
        _BGS_RELEASE_BUILD_FILE.format(
            binaries = json.encode(binaries),
            entrypoint = json.encode(entrypoint),
            path = json.encode(manifest["pathDir"] + "/**"),
            resources = json.encode(manifest["resourcesDir"] + "/**"),
        ),
        executable = False,
    )
    return repository_ctx.repo_metadata(reproducible = True)

bgs_release_archive = repository_rule(
    implementation = _bgs_release_repository_impl,
    attrs = {
        "release": attr.string(mandatory = True, values = ["latest", "historical"]),
    },
)
