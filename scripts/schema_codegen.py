#!/usr/bin/env python3
"""Run the approved generator matrix for the workspace example schemas."""

import argparse
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
SCHEMAS = (
    (
        "crates/schemas/telemetry_core",
        "mbt/example/telemetry/v1/telemetry.proto",
        "mbt.example.telemetry.v1.TelemetryResponseV1",
        "telemetry_v1",
    ),
    (
        "crates/schemas/test_compatibility_core",
        "mbt/test_compatibility/v1/all_fields.proto",
        "mbt.test_compatibility.v1.TestCompatibilityResponseV1",
        "test_compatibility_v1",
    ),
)
ADAPTERS = ("json", "protobuf", "csv", "transponding", "arrow", "arrow-ipc", "parquet")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    actions = parser.add_mutually_exclusive_group(required=True)
    for action in ("write", "check", "inspect"):
        actions.add_argument(f"--{action}", dest="action", action="store_const", const=action)
    args = parser.parse_args()
    for crate, schema, root, module in SCHEMAS:
        base = [
            "cargo", "run", "-p", "mbt_codegen", "--bin", "mbt_codegen", "--",
            f"--{args.action}", "--proto-root", f"{crate}/proto", "--proto-root", "proto",
            "--schema", schema, "--root", root, "--module", module,
        ]
        for adapter in (None, *ADAPTERS):
            command = base + ["--surface", "metamorphose" if adapter else "projection"]
            suffix = ""
            if adapter:
                command += ["--adapter", adapter]
                suffix = "_" + adapter.replace("-", "_")
            if args.action != "inspect":
                command += ["--out", f"{crate}/src/{module}{suffix}.rs"]
            subprocess.run(command, cwd=ROOT, check=True)


if __name__ == "__main__":
    try:
        main()
    except (OSError, subprocess.CalledProcessError) as error:
        print(error, file=sys.stderr)
        sys.exit(1)
