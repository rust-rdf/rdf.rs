#!/usr/bin/env python3
"""Check packages separately so Cargo feature unification cannot hide leaks.

Run from the repository: python3 .config/check-features.py core|interop|adapters|no-std
Install the bare-metal target first: rustup target add thumbv7em-none-eabihf
The no-std consumer is compile-only; an application using alloc supplies its
own allocator. Database and I/O adapter APIs require std even when their
disabled-feature configurations compile.
"""

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["core", "interop", "adapters", "no-std"])
    parser.add_argument("--offline", action="store_true")
    parser.add_argument("--target", default="thumbv7em-none-eabihf")
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    env = os.environ.copy()
    if env.get("CARGO_RESOLVER_LOCKFILE_PATH"):
        # Build scripts may invoke Cargo from a crate subdirectory.
        env["CARGO_RESOLVER_LOCKFILE_PATH"] = str((root / env["CARGO_RESOLVER_LOCKFILE_PATH"]).resolve())
    cargo = os.environ.get("CARGO", "cargo")
    network = ["--offline"] if args.offline else []
    metadata = json.loads(subprocess.check_output(
        [cargo, "metadata", "--no-deps", "--format-version", "1", *network],
        cwd=root,
        env=env,
    ))
    defaults = set(metadata["workspace_default_members"])
    core = [p for p in metadata["packages"] if p["id"] in defaults or p["name"] == "rdf-borsh"]

    def check(package, features=None, target=None, all_targets=False):
        command = [cargo, "check", "-p", package, "--all-targets" if all_targets else "--lib"]
        if features is not None:
            command += ["--no-default-features"]
            if features:
                command += ["--features", features]
        if target:
            command += ["--target", target]
        command += network
        print("+ " + " ".join(command), flush=True)
        subprocess.run(command, cwd=root, env=env, check=True)

    if args.mode == "core":
        for package in core:
            check(package["name"], all_targets=True)
            check(package["name"], "")
            check(package["name"], "alloc")
    elif args.mode == "interop":
        for package in core:
            for feature in ("serde", "borsh", "blake3", "zeroize", "oxrdf", "bson", "sophia"):
                if feature in package["features"]:
                    check(package["name"], feature)
    elif args.mode == "adapters":
        for package in metadata["packages"]:
            if package["name"].startswith(("rdf-reader-", "rdf-writer-", "rdf-store-")):
                check(package["name"], "")
                check(package["name"], "alloc")
                if package["name"].startswith(("rdf-reader-", "rdf-writer-")) and "oxrdf" in package["features"]:
                    check(package["name"], all_targets=True)
        # Exercise the std API with the native RocksDB backend switched off.
        check("rdf-store-oxigraph", "std", all_targets=True)
    else:
        for features in ("", "alloc", "serde", "hash", "borsh", "datetime", "alloc,serde,hash,borsh,datetime"):
            check("rdf-no-std-check", features, target=args.target)


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as error:
        sys.exit(error.returncode)
