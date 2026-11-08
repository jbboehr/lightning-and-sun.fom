# Private Linux candidate finalization

2026-11-07. **CPU verification complete; private source snapshot prepared.**
The owner approved Astra planning, then separately approved the returned
[implementation plan](../plans/linux-candidate-finalization.md), replying
"go for it" at each checkpoint. This record owns the refreshed private source
candidate and bounded current compatibility evidence. Earlier
[support evidence](linux-candidate-support-evidence.md) and historical candidates
retain their original dates, hashes and verification limits.

## Scope and preservation

The batch refreshes the private text-only source snapshot and tests the current
twelve-choice collection alongside two synthetic independent folder mods.
Production code, recipes, artwork, dependencies, defaults and saved-choice IDs
remain unchanged. No local GPU, model inference or live game is involved.

Release files include tooling, text recipes/masks and mod scripts only.
Original and recolored game-derived images, archives, galleries, screenshots,
clips and user state remain excluded. Users generate replacements locally from
their own game copy. Historical local review evidence is preserved separately.

The [baseline](../../tmp/linux-candidate-finalization/baseline.json) identifies
base commit `ad24bda54e92d2e9a0370c89a252a458e97531a3`, the initial documentation
diff, 1,182 production inputs, 31 historical receipts, eight seed files and
additional historical artifact/helper pins. It records the three original lab
files and no visible active handles before mutation. The task uses only
`tmp/finish-batch-install-lab`; the real installation and user state are outside
the batch.

The preserved original game archive SHA256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

## Package identity

The fresh locked root flake check and build pass. The package remains
`/nix/store/il1nhcfkdfsvlg2dp9d7a8acxxnhl151-mistria-palette-0.1.0`.
The [package checks](../../tmp/linux-candidate-finalization/package-checks.json)
and [binding](../../tmp/linux-candidate-finalization/package-binding.json)
verify all 356 files against retained Player27 hashes and all 355 recipes against
current source. All 439 recipe references stay inside the package. Each of the
36 selected sets retains eleven generated choices plus implicit Vanilla:
6,312 selected strips and 69,432 generated variants.

All 1,175 filtered package-source files match current source. The runtime closure
contains the exact pinned MOMI binary and runner with strict linting,
fail-on-skip and required compilation. Root/install/uninstall help passes from
`/tmp` with a cleared environment and no MOMI overrides. These are same-store
resolution and parity results, not a fresh uncached build, offline bootstrap or
fresh operating-system test.

## Current compatibility results

The [preparation receipt](../../tmp/linux-candidate-finalization/folder-mod-prior-install.json)
records the strict pinned-MOMI installation of Alpha and Zulu synthetic folder
mods. Both unique payloads exist and Zulu wins the shared payload. Six source
files, the pristine backup and generated selection list are pinned.

The new harness derives a one-asset fixture from packaged
`caldarus-native-tones.json`, restricting both its shared and facial profiles
consistently. It preserves all eleven IDs, labels, color mappings and regions
for that asset. Its positive export/build control passes. An invalid source-pin
copy refuses with the expected checksum diagnostic while preserving archive,
backup, sources and config and publishing no recovery state.

The full collection installation, installed-output audits, recovery refusals
and final restoration pass. The
[compatibility receipt](../../tmp/linux-candidate-finalization/packaged-install-checks.json)
owns command logs, timings and before/after hashes. The full strict install took
466.333 seconds on this machine.

| Check | Result |
| --- | --- |
| Complete collection with Alpha/Zulu | All 36 characters and 69,432 unique generated PNG bindings match the retained current Player27 installation. Both unique mod payloads survive; Zulu remains the shared-payload winner. |
| Original assets and runtime | All 6,312 original PNG/metadata pairs remain byte-exact. The production verifier checks 219,780 native frames including variants; both runtime scripts and the generated 36-helper table match current evidence. Atlas comparison checks alpha and visible RGBA; trimmed transparent texels may have different hidden RGB. |
| Intentional no-change forms | Bound existing harnesses verify loose PNG bytes, metadata and installed visible atlas pixels for 18 spectral strips/48 frames and 30 shadow strips/149 frames on this same installed archive. |
| External archive modification | Uninstall refuses and preserves the externally modified bytes and both recovery files exactly. The known appended marker is reversed only after verifying the complete original prefix. |
| Normal uninstall | Restores the exact pre-palette modded archive, retaining both mods, six source files, config and pristine backup; palette recovery state is absent. |
| Reversed existing mod order | The reduced twelve-choice installation refuses with the expected precedence diagnostic; archive, source files, config and backup remain unchanged. |
| Final fixture restoration | Separately restores the pristine archive and all three starting lab files exactly. Tiny synthetic mod sources/config remain with the new evidence outside the lab. |

The pre-palette modded archive SHA256 is
`0c5322d25d3159743873a3a401d940c3a077096d7ebd9f1482a971d97e328f92`.
The full installed archive SHA256 is
`1db55b7e8869eb71128d9d392937798c0bcf5870f9e00cab949cc0eb472c618e`.
Its generated palette table SHA256 is
`5f43b73f33f3803442cc1da3c0918bd39806cf6867c1b6863fb279fd74264cc4`.
The final lab returns to the preserved original fingerprint above. Exact normal
uninstall restoration and later pristine-fixture cleanup are separate results.

These are bounded synthetic-mod results, not compatibility with arbitrary named
mods, other game builds or additional mod managers. Historical acceptance
snapshots remain frozen; intentionally corrected current faces are compared
against current Player27 evidence rather than obsolete artwork expectations.

## Source candidate and final checks

The refreshed [private source archive](../../tmp/linux-candidate-finalization/lightning-and-sun-linux-candidate.tar.gz)
includes this documentation snapshot. The
[per-file manifest](../../tmp/linux-candidate-finalization/source-candidate-manifest.json)
identifies the base commit plus actual bytes of authorized working-tree
documentation changes; it does not claim all bundled bytes are committed HEAD.
The [source summary](../../tmp/linux-candidate-finalization/source-candidate-summary.json)
and external checksum files identify the serialized archive. Its source roots
and text allowlist preserve the established distribution boundary. Git-ignored
caches stay excluded and untouched.

The [source-helper checks](../../tmp/linux-candidate-finalization/source-helper-checks.json)
pass 43 isolated boundary cases, including unsafe/unexpected inputs, symlinks,
binary/embedded-media markers, altered payloads/manifests and extraction
tampering. Deterministic serialization repeats the archive digest without
retaining a redundant archive. The
[standalone-source receipt](../../tmp/linux-candidate-finalization/standalone-source.json)
binds the exact safe extraction, without Git or local game state.

The authoritative sealed-artifact verdict is the external
[final check receipt](../../tmp/linux-candidate-finalization/checks.json).
It records the archive digest, root/standalone/retained package equality, exact
extracted/source membership and final historical/lab preservation checks. These
results and checksums stay outside the archive to avoid self-reference and
rewriting bundled documentation after sealing. Same-store package equality does
not establish a fresh uncached build, offline bootstrap or fresh operating system.

Fresh [standard checks](../../tmp/linux-candidate-finalization/standard-check-summary.json)
pass formatting, all-target Clippy with warnings denied, 466 regular tests and
the release build; 520 tests remain ignored according to the existing suite.
The first Python orchestration launch found no Python in the pure shell and
started no Cargo check. The corrected driver runs orchestration outside that
shell while executing every unchanged Cargo check once inside the pinned pure
environment; dependencies and the shell definition remain unchanged.

The [documentation receipt](../../tmp/linux-candidate-finalization/documentation-checks.json)
separates shipped-document links from explicitly excluded local evidence links.
Historical generated-media references remain unavailable in the source archive;
they are not claimed portable. One pre-existing missing development anchor from
`palette-presets.md` into the native-tone plan remains recorded, with both files
verified unchanged from HEAD. The current documentation scope does not rewrite
those historical records or include their media. New or changed broken links
are rejected separately.

The [installation footprint checkpoint](../../tmp/linux-candidate-finalization/install-footprint-checkpoint.json)
observed about 6.03 GB allocated in the lab during transaction work. This is a
checkpoint, not measured peak usage. The transaction generated the full
collection internally; no separate permanent full-generation tree was created.
Final evidence/source-tree sizes and remaining disk space are recorded in the
final receipt. The tiny refusal fixture, probe sources, command logs, source
archive and exact standalone extraction are retained for reproduction. All old
source candidates, review packets and helpers retain their prior bytes.

## Remaining release gates

This task does not supply broader tone-range, uninspected artwork,
physical-controller, unsampled action or ordinary story/animation-reachability
acceptance. The qualified displayed assessments and original live criterion keep
their recorded scope. Licensing, commit, tagging and publication remain separate.
Finalizing a private candidate does not close the usable Linux release milestone.
