import os
from pathlib import Path

PACKAGE_NAME = "openai-bgs-cli-bin"
PACKAGE_METADATA_FILENAME = "bgs-package.json"


def bundled_package_dir() -> Path:
    path = Path(__file__).resolve().parent
    metadata_path = path / PACKAGE_METADATA_FILENAME
    if not metadata_path.is_file():
        raise FileNotFoundError(
            f"{PACKAGE_NAME} is installed but missing its package metadata at {metadata_path}"
        )
    return path


def bundled_bgs_path() -> Path:
    exe = "bgs.exe" if os.name == "nt" else "bgs"
    path = bundled_package_dir() / "bin" / exe
    if not path.is_file():
        raise FileNotFoundError(
            f"{PACKAGE_NAME} is installed but missing its packaged bgs binary at {path}"
        )
    return path


def bundled_path_dir() -> Path | None:
    path = bundled_package_dir() / "bgs-path"
    return path if path.is_dir() else None


__all__ = [
    "PACKAGE_NAME",
    "bundled_bgs_path",
    "bundled_package_dir",
    "bundled_path_dir",
]
