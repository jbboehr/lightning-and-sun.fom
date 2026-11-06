# Linux release candidate: packaging and CPU verification

2026-11-06. **CPU packaging and integrated checks pass; this is a local
candidate, not a published release.**
The user accepted displayed Caldarus/Seridia artwork and requested a commit and
the next batch. Checkpoint `3751832` completes planned content authoring. The
approved finish plan advances through P6. The later qualified artwork assessment
below closes the earlier first/fourth art gate; live rendering/input/gameplay
remain separate gates.

## Scope and ownership

One Nix package for x86_64 Linux retains the CLI, existing MOMI runner and all
JSON recipes. The package worker owns Nix files; the parent owns documentation,
disposable compatibility fixtures, integrated checks and final evidence. No
runtime or palette behavior change is planned. Other operating systems,
portable non-Nix bundles and publishing remain outside this batch.

The source archive remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The reused lab is `tmp/finish-batch-install-lab`; no real game installation,
saves or user settings are modified. All derived game data stays local.

## Release-check contract

- Build the package in Nix isolation with locked Rust/Cargo inputs; preserve
  existing shell behavior and the pinned MOMI 0.16.4 runner. Check that the
  installed output retains its runtime closure and recipes outside the checkout.
- Inspect the source/output allowlists: code/configuration only, no game images,
  archives, extracted corpora, accepted fixtures or local state.
- Run focused installer regressions. Exercise the actual packaged binary and
  complete collection with independently authored overlapping folder mods.
  Verify their unique payloads, winning shared payload, recorded order,
  unchanged source folders/config, pristine backup and exact removal.
- Demonstrate mismatched source-pin, incompatible existing-order and external
  archive-change refusals in the disposable lab. Failed installation must
  publish nothing; failed removal must preserve external bytes and recovery.
- Compare every generated variant to the final accepted-content install report.
  Recheck unchanged spectral PNGs, complete metadata and native atlas pixels.
- Prepare a source candidate/checksums and usable installation/removal
  instructions. Consolidate pending first/fourth artwork from preserved galleries,
  retaining qualified Louis-glasses and Linnet-scar assessments.

Strict recipe compatibility pins each selected source PNG, rather than enforcing
a single global input-archive SHA. Whole-archive hashes protect publication and
recovery. Documentation must preserve that distinction. Existing mod-list
provenance, automatic updates, arbitrary mods and concurrent unrelated installers
retain their documented limits.

## Current evidence

### Nix package

`nix build` now exposes one x86_64 Linux package. A shared `nix/momi.nix` keeps
the shell and package on the exact original MOMI binary and isolated runner
store paths; `nix/package.nix` uses the locked Fenix toolchain and Cargo lock.
No Rust, GML, registry, palette or runtime behavior changes. The package contains
the binary and all 281 JSON recipes under `share/mistria-palette/palettes`,
preserving 176 relative references and the complete 36-character selection.

The isolated build passes all 451 ordinary tests (508 ignored).
`nix flake check --no-build --all-systems` and the pure developer shell pass.
Root/install/uninstall help work from outside the checkout with an empty command
path and no developer-shell variables. The output directly retains the original
MOMI binary/runner; its closure contains seventeen paths, including bubblewrap,
the dynamic loader and required libraries, totaling 335,093,488 NAR bytes.

Source/output audits find no symlinks, game images, archives or local fixtures.
The allowlisted build source contains 1,091 text files, including seven synthetic
GML test fixtures. The initial build exposed those missing fixtures; the source
allowlist was corrected and the entire isolated build/test phase passed. That
failure log is preserved, rather than presented as a runtime defect or hidden.
The installed package is 82,947,024 bytes; 78,388,088 are unchanged JSON recipes.

Package evidence and exact commands are `tmp/release-checks/nix-package-*.json`,
`nix-package-commands.txt` and `nix-package-build.log`. The package link is
`tmp/release-checks/nix-package`; keeping the out-link roots its dependencies.

### Installer regressions and probes

The fresh focused installer suite passes all 29 regular tests. Its log is
`tmp/release-installer-tests.log`. The two historical ignored MOMI tests target
absent, older Adeline-only labs; they are not enabled or relabeled as coverage of
the final collection.

The explicitly selected archive-review case
`batch_reuse_reproduces_every_approved_adeline_mask` also passes freshly in
release mode. It reconstructs and validates all 282 masks in its current
source-backed contract while retaining the original 143-asset membership.
Its omission/duplicate/historical-loss controls pass in the ordinary suite;
the local archive-run log is `tmp/release-checks/adeline-archive-review.log`.

The actual pinned runner installed two original, unused `.mist` probe payloads
per folder mod. Both unique payloads are present and Zulu wins the shared payload
in the normal Alpha/Zulu order. The pristine backup and all six source files
match their pins. Probe preparation, hashes and logs are under
`tmp/release-checks/`; reproduction helpers live in
`local/authoring/linux-release/`. No game process is launched for these checks.

The packaged binary ran from `/tmp` with `PATH=/nonexistent`, `LANG=C.UTF-8`
and `TZ=UTC`, without a developer shell or MOMI overrides. Full installation
generated all 25,248 variants for 36 characters and passed production strict
MOMI compilation, atlas-pixel, metadata and three-script verification. All
25,248 PNG hashes match the accepted-content checkpoint's actual install report;
7,972 PNGs also match twelve frozen accepted snapshots, including Caldarus and
Seridia. Both probe mods' unique payloads remain present and Zulu retains the
shared payload. All six original source files, the existing config list and
pristine MOMI backup remain unchanged.

The separate source-bound spectral verifier also passes against that installed
archive: eighteen byte-identical loose PNGs, complete native TOML and 48 native
atlas frames. Its result binds the same archive hash as the full install report.

Three negative paths pass against the actual packaged binary:

| Check | Fixture and observed result |
| --- | --- |
| Source compatibility | A copied one-region recipe has a deliberately wrong source PNG pin. Installation refuses it before MOMI and leaves the live archive, backup, source folders and config unchanged; no recovery is published. |
| External archive change | A known trailer is appended to the installed archive. Removal refuses, preserving every external byte and recovery files. The probe then verifies the unchanged original prefix and reverses only its own appended bytes. |
| Existing precedence | A custom Zulu/Alpha starting fixture has Alpha's shared payload and a consistent reversed selection list. MOMI would rebuild Alpha/Zulu; installation refuses without changing live bytes, backup, source folders or config. The fixture is constructed offline, not through a GUI session. |

Normal removal restores the exact pre-palette modded archive and removes recovery
state. After the custom-order check, the lab is returned to the exact pristine
source using its verified backup. The temporary backup is consumed by that
restoration; tiny probe source/config files are retained outside the lab for
reproduction. The restored lab contains only the original archive, `Maybe.toml`
and the expected persistent lock. No additional game archive copy is retained.

Results are `tmp/release-checks/packaged-install-checks.json`, per-case stdout/
stderr logs and `spectral-installed-check.json`. The reproduction helper is
`local/authoring/linux-release/check-packaged-install.py`. Accepted snapshots,
historical reviews and all source/runtime inputs remain unchanged.

### Standalone source candidate and documentation

The candidate is `tmp/release-checks/lightning-and-sun-linux-candidate.tar.gz`.
Its explicit text-only source allowlist includes code, locked build definitions,
configuration, synthetic tests and documentation. It contains no game images,
archives, local state or symlinks. A per-file source manifest is included. The
candidate is extracted into an otherwise clean tree with no Git checkout, game
data or local fixtures; locked `nix build` resolves and builds the exact same
tested package output. This is a standalone-source/cache-reuse check on the
current Nix installation, not a test of a fresh operating system or empty store.

The root README now documents the complete collection, rooted Nix package,
controls, opt-in remembering, reset, exact removal and compatibility/refusal
behavior. The tool guide points to that route while preserving older study
scopes. Historical installer evidence remains intact and links to this record.
No project license is assigned by this work; public distribution remains a
separate owner decision.

### Remaining artwork review

The [consolidated review](../../tmp/pending-art-review/index.html) and portable
`tmp/pending-art-review.zip` combine the broader first/fourth-batch artwork
submitted for acceptance: Louis, Merri, Adeline, Juniper, Stillwell, Darren, Linnet,
Wiscar and Wynne. No artwork was changed. All 884 source gallery files match the
original manifests. Shared collection/registry/count inputs that later batches
intentionally extended are verified separately. The review covers
240 world strips, 812 native frames and 295 West mirrors: 1,107 complete cases.

Samples use Vanilla, Debug Blue, Hayden, Ryis and Seridia; complete comparisons
put Vanilla left and Debug Blue right. The root and all nine character indexes
pass twenty CPU desktop/mobile loads with every image/link available and no
overflow. The portable manifest binds every packaged file and its validation
page is bound to this owning record. Earlier qualified Louis-glasses and
Linnet-scar assessments remain in force; the subsequent broader assessment is
recorded below.
`generated/review` points to this consolidated pending-art review; prior reviews
remain preserved at their original paths.

### Qualified user acceptance — 2026-11-06

The user replied "looks okay to me" to the explicit question about all nine
characters in the consolidated remaining-art review. This is qualified
acceptance of the displayed world artwork; inspection depth is unspecified.
Earlier Louis-glasses and Linnet-scar qualifications remain in force. It does
not establish live rendering, physical input, controller or gameplay acceptance.

The nine accepted snapshots are frozen under `local/fixtures/finish-<character>/`,
with 7,726 verified file hashes and 3,836 PNG bindings to the final packaged
install report. The review ZIP remains unchanged at
`9253bdee4b2b9a7176b9a32a77011d113609605e31822c69f36598353a7925fd`.
Retaining prior portraits for regression does not widen world-art approval.
The original review/check/source-candidate metadata preserve their earlier
pending status as historical evidence; this section records the later assessment.

All planned recolorable content is now authored, validated and accepted as
displayed, with intentional no-change assets accounted for. The approved
**content-complete alpha is reached**. The usable Linux release still requires
the remaining live checks and release gates. Their evidence belongs to the
[live release-verification record](live-release-verification-2026-11-06.md).

## Retention and verification limits

Nix retains the rooted package and reusable build inputs. Local reports/helpers
and the source candidate retain reproducible CPU evidence. Its approximately
88 MiB extracted standalone-source check fixture is retained for reproduction;
the pending review
retains its original 22 MiB gallery and approximately 5.5 MiB portable ZIP.
No archive from a real installation or personal settings is copied or changed.
The package's ordinary test suite does not claim the 508 optional source/game/
browser cases; the affected artwork and interpreter results remain in the
accepted eighth-batch record. No substantial runtime behavior changed, so this
packaging/data-preservation batch does not restart the completed runtime
reliability gate.

## Remaining gates

Qualified displayed-art acceptance closes the remaining content approval gate.
At this CPU checkpoint, visible menu/input, controller, persistence during natural
play and the gameplay matrix remained. The subsequent live record below owns
their later results and qualified displayed-review acceptance. No tag,
push or public release is authorized by this batch.

`gpu-status` confirms host GPU visibility; its snapshot is retained under the
release checks. No GPU job or game process was started. Live verification must
recheck availability and hold `gpu-run` for the complete rendering workload.

## Subsequent live candidate

The [live verification record](live-release-verification-2026-11-06.md) owns the
three bounded runtime fixes discovered after this CPU packet, current package
parity, native UI/persistence/gameplay/save/load results, qualified acceptance of
the displayed live review and remaining limits.
The CPU archives/checksums above remain preserved historical evidence. The current
source-only candidate and consolidated live review are separate artifacts under
`tmp/live-release/`; earlier accepted artwork and its review ZIP remain unchanged.
