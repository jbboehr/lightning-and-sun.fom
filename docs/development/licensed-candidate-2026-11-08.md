# Licensed candidate, 2026-11-08

The owner chose AGPL-3.0-only on 2026-11-08 and authorized applying it and
resealing the private candidate. No publication, tag or game launch occurred.

## Changes

- `LICENSE` is the SPDX License List 3.28.0 text of AGPL-3.0-only, from nixpkgs
  `spdx-license-list-data`.
- `Cargo.toml` declares `license = "AGPL-3.0-only"`; the Nix package sets
  `meta.license` and installs the text at `share/licenses/mistria-palette/LICENSE`.
- The README license section scopes the grant to this project's tool, recipes,
  masks and mod scripts, excluding the game, its assets and images generated
  from them. The [candidate notes](../releases/linux-candidate.md) point to it.
- The [owner playthrough guide](owner-playthrough.md) prepares the
  ordinary-gameplay run.

## Verification

The rebuilt root package passed all 469 regular tests in its Nix build. Compared
with the binding of the verified
[Stillwell/Terithia package](muted-tone-rollout.md), all 356 previous files are
byte-identical, including the `mistria-palette` binary
(`94dfb922401c476c6284fddf79f639bc67f30facb0b61f965b560659b2909c56`); the only
addition is the license text. The local
[comparison receipt](../../tmp/licensed-candidate/package-comparison.json)
records the file lists.

Because the tool, recipes and mod scripts that installation writes are
unchanged, the rollout's strict install/refusal/removal, compatibility and
native-control results apply unchanged; they were not rerun. The new source
snapshot was sealed with the previous text-only allowlist plus `LICENSE`,
extracted into a fresh tree, and built standalone from its locked flake. The
external [summary](../../tmp/licensed-candidate/source-candidate-summary.json)
and [standalone receipt](../../tmp/licensed-candidate/standalone-verification.json)
own the archive checksum and verdict, since an archive cannot contain its own
checksum. Limits match earlier candidates: same machine and store, no fresh-OS
or uncached build, and no visual or live acceptance.
