"""Adapt an extracted signed Windows package to WinGet's existing root filenames."""

import argparse
import json
import shutil
from pathlib import Path


def prepare_winget_package(package: Path) -> None:
    metadata_path = package / "bgs-package.json"
    metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
    target = metadata["target"]
    if target not in ("x86_64-pc-windows-msvc", "aarch64-pc-windows-msvc"):
        raise ValueError("WinGet requires a Windows package")
    if metadata["entrypoint"] != "bin/bgs.exe":
        raise ValueError("WinGet requires the canonical Bgs entrypoint")
    entrypoint = f"bgs-{target}.exe"
    manifest_path = package / "bgs-resources/voice/manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    manifest["sha256"][entrypoint] = manifest["sha256"].pop("bin/bgs.exe")
    (package / "bin/bgs.exe").rename(package / entrypoint)
    (package / "bin/bgs-code-mode-host.exe").rename(
        package / "bgs-code-mode-host.exe"
    )
    # Keep resources in place for package-aware discovery and provide the root
    # filenames declared by the existing WinGet portable installer manifest.
    for helper in ("bgs-command-runner.exe", "bgs-windows-sandbox-setup.exe"):
        shutil.copy2(package / "bgs-resources" / helper, package / helper)
    metadata["entrypoint"] = entrypoint
    for path, value in ((metadata_path, metadata), (manifest_path, manifest)):
        path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("package", type=Path)
    prepare_winget_package(parser.parse_args().package)
