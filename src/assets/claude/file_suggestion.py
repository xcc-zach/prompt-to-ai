import json
import os
import sys
from pathlib import Path

TOP_N = 15


def stripdot(p: str) -> str:
    """Remove leading './' to match the shell script output style."""
    return p[2:] if p.startswith("./") else p


def has_uppercase(s: str) -> int:
    """Return 1 if the path contains any uppercase letters, else 0."""
    return 1 if any("A" <= ch <= "Z" for ch in s) else 0


def folder_priority(path: str) -> int:
    """
    Folder priority (smaller is earlier):
      0: under 'src/'
      1: under 'docs/'
      2: under '.github/'
      3: everything else
    Notes:
      - This uses the normalized relative path without leading './'.
      - Exact folder match means prefix match on path segments.
    """
    if path == "src" or path.startswith("src/"):
        return 0
    if path == "docs" or path.startswith("docs/"):
        return 1
    if path == ".github" or path.startswith(".github/"):
        return 2
    return 3


def main() -> int:
    # Read JSON from stdin, same contract as: jq -r '.query'
    raw = sys.stdin.read()
    try:
        obj = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError:
        obj = {}

    query = obj.get("query") or ""
    ql = query.lower()

    results = []

    # Recursively scan files + directories under current folder (mindepth 1).
    for root, dirs, files in os.walk(".", topdown=True):
        root_path = Path(root)

        # Collect directories.
        for dname in dirs:
            p = stripdot((root_path / dname).as_posix())
            if p in ("", "."):
                continue

            b = Path(p).name
            parent = Path(p).parent.as_posix()
            if parent == ".":
                parent = ""

            bl = b.lower()
            dl = parent.lower()

            # Buckets (same as the awk logic):
            # 1: basename hit + file  (files first)
            # 2: basename hit + dir   (dirs later)
            # 3: dirname hit + dir    (dirs first)
            # 4: dirname hit + file   (files later)
            if ql and (ql in bl):
                bucket = 2
            elif ql and parent and (ql in dl):
                bucket = 3
            else:
                continue

            results.append(
                (
                    folder_priority(p),
                    bucket,
                    has_uppercase(p),
                    p.lower(),
                    p,
                )
            )

        # Collect files.
        for fname in files:
            p = stripdot((root_path / fname).as_posix())
            if p in ("", "."):
                continue

            b = Path(p).name
            parent = Path(p).parent.as_posix()
            if parent == ".":
                parent = ""

            bl = b.lower()
            dl = parent.lower()

            if ql and (ql in bl):
                bucket = 1
            elif ql and parent and (ql in dl):
                bucket = 4
            else:
                continue

            results.append(
                (
                    folder_priority(p),
                    bucket,
                    has_uppercase(p),
                    p.lower(),
                    p,
                )
            )

    # Sort by:
    # - preferred folder group: src -> docs -> .github -> others
    # - bucket: basename-hit files -> basename-hit dirs -> dirname-hit dirs -> dirname-hit files
    # - lowercase preference: paths without uppercase letters first
    # - case-insensitive path order
    # - original path as tie-breaker
    results.sort(key=lambda x: (x[0], x[1], x[2], x[3], x[4]))

    for item in results[:TOP_N]:
        sys.stdout.write(item[4] + "\n")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
