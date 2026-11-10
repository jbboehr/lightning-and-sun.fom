# Default-collection candidate, 2026-11-09

The `install` command now installs the complete packaged collection when no
recipe is selected, so the private candidate was resealed. No publication, tag
or game launch occurred.

## Changes

- With none of `--characters`, `--presets` or `--palette`, `install` resolves
  `share/mistria-palette/palettes/sets/characters-trial.json` beside the
  executable and prints its path. Without a packaged collection, as in a plain
  cargo build, it fails and names those options. Previously it applied the
  single-portrait Adeline blue study compiled into the binary; that study now
  requires an explicit `--palette`.
- The README, [tool notes](../TOOLS.md) and
  [playthrough guide](owner-playthrough.md) use the bare command. The
  [release notes](../releases/linux-candidate.md) were rewritten for end users.

## Verification

The package built from the committed change passed all 471 regular tests in its
Nix build, including two new installer tests for the packaged and unpackaged
cases. That package was then installed with the bare command into the isolated
copy of the game archive: it selected the packaged collection, generated all 36
characters and passed the installer's pixel and animation-metadata check.
`uninstall` restored the original archive and `Maybe.toml` byte for byte. The
local [install log](../../tmp/default-collection/install.stderr) and
[before](../../tmp/default-collection/lab-before.sha256) and
[after](../../tmp/default-collection/lab-after.sha256) checksums record the run.

The source snapshot was sealed with the unchanged text-only allowlist, extracted
into a fresh tree, and built standalone from its locked flake. This record and
the planning line are outside the package's filtered source, so the sealed tree
builds the same store path as the package that was installed. The external
[summary](../../tmp/default-collection/source-candidate-summary.json) and
[standalone receipt](../../tmp/default-collection/standalone-verification.json)
own the archive checksum and verdict, since an archive cannot contain its own
checksum.

Limits match earlier candidates: same machine and store, and no fresh-OS or
uncached build. The change selects which recipe is installed and does not alter
the recipes, so nothing was rechecked in the running game; the earlier
[licensed candidate](licensed-candidate-2026-11-08.md) limits on visual and live
acceptance still apply.
