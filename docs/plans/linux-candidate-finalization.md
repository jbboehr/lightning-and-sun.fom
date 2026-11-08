# Private Linux candidate finalization

2026-11-07. **CPU verification complete; private source snapshot prepared.**
The owner approved Astra planning with "go for it" after Sol proposed refreshing
the source candidate and current twelve-choice compatibility evidence. The
owner separately approved the returned implementation plan with "go for it".
The [overview](../planning.md) owns priority; the
[finish plan](finish.md#p6--one-integrated-release-candidate) owns
P6 scope. Astra returned this approach after read-only investigation; Sol checked
the package filter, preset-profile contract, historical helpers and source builder.
The [execution record](../development/linux-candidate-finalization.md) owns the
completed package, twelve-choice compatibility/refusal/removal and standard-check
results. The external [final receipt](../../tmp/linux-candidate-finalization/checks.json)
owns the exact sealed-source, standalone-build and final preservation verdict.
The steps below preserve the approved implementation approach.

## Intended outcome and boundaries

Prepare a reproducible, unpublished source candidate containing current
documentation, and verify the current full collection alongside the existing
two synthetic folder mods. Refresh bounded refusal and exact-removal evidence.
The [support record](../development/linux-candidate-support-evidence.md) explains
the two gaps: the retained source archive predates later documentation, and the
folder-mod probes cover the older five-choice checkpoint.

Production code, recipes, artwork, dependencies, defaults and saved-choice IDs
remain unchanged. Release contents remain tooling, text recipes/masks and mod
scripts. Original and recolored game-derived assets, galleries, screenshots,
clips and local game state stay outside release files and attachments.
Historical candidates, receipts, helpers, reviews and acceptance records remain
frozen. No gallery rebuild or new visual acceptance is included.

This work needs **no local GPU or local model inference**. It includes no game
launch, licensing selection, commit, tag or publication. Broader tone assessment,
physical-controller acceptance, unsampled actions, ordinary animation reachability
and the original live criterion retain their recorded limits. A private candidate
can be prepared before those gates close; it is not a usable-release declaration.

## Baselines and important contracts

- Planning baseline: clean commit `ad24bda`. The new archive must identify that
  base plus its actual documentation changes, rather than claiming all bundled
  bytes are committed HEAD.
- Current package:
  `/nix/store/il1nhcfkdfsvlg2dp9d7a8acxxnhl151-mistria-palette-0.1.0`.
  The [retained binding](../../tmp/player27-rollout/package-binding.json) covers
  one binary and 355 recipe files. All 36 selected sets have eleven generated
  choices plus Vanilla; the full collection has 69,432 generated variants.
- Preserved source archive SHA256:
  `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
  Reuse only `tmp/finish-batch-install-lab` after checking its identity, starting
  files and absence of active users. The real installation and user state remain
  outside this task.
- [Package source filtering](../../nix/package.nix) excludes documentation.
  Root and standalone builds should preserve the retained package exactly.
  A cached realization establishes same-store reproducible resolution, not a
  fresh uncached build, offline bootstrap or fresh operating-system test.
- [Preset loading](../../src/presets.rs) resolves shared and per-preset profile
  paths relative to the set and requires identical asset inventories. Reduced
  fixtures must restrict both profiles consistently, preserving all eleven
  preset entries, labels, IDs, colors and regions for the chosen asset.
- [Installer](../../src/installer.rs) checks source compatibility, existing mod
  ordering and recovery hashes. [Installed-output verification](../../src/installed.rs)
  checks original assets, generated variants, metadata and scripts before archive
  publication. The pinned MOMI runner must retain strict linting, fail-on-skip
  and required compilation; no runner override or compiler bypass is allowed.
- Normal uninstall first restores the exact pre-palette **modded** archive.
  Restoring the pristine lab archive afterward is separate fixture cleanup.

## Ordered implementation

### 1. Pin inputs and prepare fresh evidence paths

Use `tmp/linux-candidate-finalization/` for outputs and
`local/authoring/linux-candidate-finalization/` for narrowly adapted helpers.
Require fresh destinations. Record HEAD, the documentation diff, lab starting
contents/hashes, current package binding, old source candidates and the existing
1,182 production-input, 31 historical-receipt and eight seed-file bindings.
Recheck available disk space before archive work.

Preserve `Maybe.toml`, the pristine archive and all pre-existing lab files.
Verify no process uses the lab before mutation. If a phase fails, retain its
diagnostics and recovery state; do not run unconditional cleanup over an
unexplained archive. Restore only independently verified expected states.

### 2. Verify the packaged tool and recipe closure

Build the locked root flake into a fresh out-link, then compare all 356 package
files with the retained binding and all packaged recipes with current source.
Validate relative profile/set references stay within the package and confirm
all 36 twelve-choice catalogs.

Adapt `tmp/release-checks/verify-nix-package.py` into the new helper directory.
Replace historical counts and evidence paths; do not require a fresh test log
from a cached Nix build. Check root/install/uninstall help outside the checkout
with a cleared environment, package closure and explicitly verified MOMI paths.
Use the absolute packaged binary from `/tmp` for compatibility probes, with
`PATH=/nonexistent`, `LANG=C.UTF-8`, `TZ=UTC` and no MOMI overrides.

### 3. Adapt and execute the bounded compatibility probes

Reuse the workflow in `local/authoring/linux-release/prepare-compatibility-lab.py`
and `check-packaged-install.py` through bounded copies. Those historical helpers
write old receipts, assume 25,248 variants and use the earlier Caldarus set.
Their historical accepted-snapshot equality loop also predates intentional
facial corrections; preserve those receipts without reinterpreting old snapshots
as current-art expectations.

Derive a one-asset refusal fixture from packaged `caldarus-native-tones.json`.
Copy/restrict both the shared and override profiles, retaining every region for
that asset, and rebase references. Validate the unchanged reduced fixture with
`export` and `build-presets` before creating a separate invalid source-pin copy.

Run compiler/archive phases sequentially:

| Phase | Required evidence |
| --- | --- |
| Prepare Alpha/Zulu folder mods with the pinned runner | Both unique payloads exist, Zulu wins the shared payload, and source/config/pristine-backup hashes are pinned. |
| Invalid source-pin fixture | Expected checksum refusal; archive, backup, mod sources and config unchanged; no recovery publication. |
| One full current collection installation with both mods | Strict installation passes with all 36 characters and 69,432 unique `(character, preset, asset)` bindings matching current Player27 evidence. |
| Installed-output preservation | Production verification passes; all 6,312 original PNGs and metadata remain exact; scripts and generated table agree with the current candidate. |
| Intentional no-change preservation | Bound existing harnesses verify 18 spectral strips/48 frames and 30 shadow strips/149 frames against this installed archive. |
| External-change refusal | Append the known marker; uninstall refuses and preserves external bytes and recovery files. Verify prefix and marker before reversing only that append. |
| Normal removal | Exact pre-palette modded archive restored; both probe mods, config and pristine backup preserved; recovery directory absent. |
| Reversed-order fixture | Reduced twelve-choice installation refuses with the expected precedence diagnostic; pre-attempt bytes unchanged. |
| Final lab restoration | Independently verified pristine archive and starting lab contents restored; tiny probe sources/config retained with new evidence outside the lab. |

Adapt `local/authoring/player27-rollout/bind-install.py` for current report,
original-asset and script checks. Verify no-change executables against
`tmp/player27-rollout/no-change-harness-binding.json` before reuse. Record each
phase's command, status, timing and relevant before/after hashes.

One full strict installation is necessary to establish coexistence at the
current collection size. Source mismatch fails before MOMI; reversed ordering
needs a real strict run but only the reduced fixture. A separate full generation
or review-gallery rebuild adds no required evidence.

### 4. Complete ordinary checks and documentation

Run the existing standard checks once in the pinned shell, retaining logs in the
new evidence directory:

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
```

Do not enable historical ignored installation tests against obsolete labs.
Create `docs/development/linux-candidate-finalization.md` to own current package,
compatibility, recovery and claim-boundary conclusions. Update this plan, the
overview and finish plan. Add a dated pointer from the support record while
preserving its documentation-only result. Update candidate notes to identify
the refreshed private source snapshot. Change README/tool-guide instructions
only if a verified result requires an end-user correction.

Describe compatibility as the two synthetic folder mods and specific refusal
paths. Do not expand that into arbitrary named-mod or game-version support.

### 5. Seal the final source archive and verify the exact extraction

Adapt `local/authoring/player27-rollout/build-source.py` and
`prepare-standalone.py` into the new namespace. Preserve explicit source roots,
text-extension allowlisting, sorted members, normalized permissions and zero
timestamps. Include tracked source plus explicitly authorized new documentation;
reject unexpected untracked inputs, symlinks, unsafe paths and forbidden roots.
Reuse and extend the existing UTF-8 and embedded-media boundary checks.

Finish bundled documents before sealing. `SOURCE-MANIFEST.json` records the base
commit and actual per-file hashes. Keep the archive's checksum and final
validation results in external `source-candidate-summary.json`, `checks.json`
and `SHA256SUMS`, avoiding an archive checksum inside its own contents. Reseal
and reverify if any bundled document changes afterward.

Extract into a fresh standalone tree without `.git`, game assets or local
fixtures. Check every extracted member against the manifest, including current
release notes and acceptance records. Build that exact tree using its locked
flake and require root/standalone/retained package equality. A second archive
serialization must reproduce the digest without retaining a redundant copy.

Check shipped-document links and separately identify historical references to
excluded local evidence. Preserve those references and their limits; do not ship
game-derived media to make historical review links portable.

## Validation and acceptance

Representative build checks, using the new helpers for receipts and fresh paths:

```sh
nix flake check --no-build --all-systems
nix build .#packages.x86_64-linux.default --out-link tmp/linux-candidate-finalization/nix-package --cores 2 --max-jobs 2 --print-build-logs
nix build path:./tmp/linux-candidate-finalization/nix-source#packages.x86_64-linux.default --out-link tmp/linux-candidate-finalization/standalone-package --cores 2 --max-jobs 2 --print-build-logs
git diff --check
```

Completion requires package/recipe parity, current bounded compatibility and
refusal receipts, exact modded-archive removal and final pristine-lab restoration,
unchanged production/historical inputs, and a deterministic text-only source
candidate with verified standalone extraction and accurate documentation.
The [execution record](../development/linux-candidate-finalization.md) owns
current results and their limits; planning itself ran no implementation checks.

Unexpected production-input changes, package mismatch or strict compiler failure
block finalization. Investigate within scope and report any correction requiring
runtime, recipe, dependency or artwork changes before implementing it. Do not
weaken checks or treat passing automation as visual acceptance.

## Resources and remaining decisions

The retained full installation took about 505 seconds. Its source candidate was
8.79 MB compressed and 155.73 MB unpacked; the retained package was 137.94 MB.
These are prior measurements, not completion-time or peak-space promises.
Avoid the separate complete generated tree, previously about 3.00 GB allocated.
Installer transactions still use several archive/generated copies; check several
GB of space plus build-cache headroom and measure actual usage. Planning observed
about 148 GB free; execution must recheck it.

No technical blocker was identified in read-only planning. The owner approved
this bounded implementation. Licensing remains an owner choice
before public distribution; a subsequent license-file change may require another
source-archive refresh. Commit, tagging and publication remain separate actions.
