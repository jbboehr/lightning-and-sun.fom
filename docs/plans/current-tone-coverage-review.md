# Current twelve-choice tone coverage review

2026-11-08. **Prepared and technically verified; owner coverage assessment
pending.** The owner
replied "sure" to planning a fresh coverage review that
includes Player27 across all 36 characters and their twelve current choices.
Astra returned a read-only implementation plan; Sol checked the historical
builder, diagnostic supplement producer and recorded acceptance boundaries.
The owner then approved the returned implementation plan with "Go for it".
The [worksheet](../../tmp/current-tone-coverage-review/index.html) and
[portable download](../../tmp/current-tone-coverage-review.zip) are complete;
the [execution record](../development/current-tone-coverage-review.md) owns
the checks, preservation verdict, fingerprints and remaining assessment.
The ordered steps below retain the approved approach. The [overview](../planning.md)
owns priorities; the [native-tone plan](native-tone-coverage.md) owns the broader
outcome.

## Outcome and boundaries

Make the complete current range practical to assess in one compact worksheet.
Show sampled NPC/creator source ramps beside proposed representatives and
current portrait/world examples. Each source needs an assessed representative,
an explicit gap or an accepted limitation. Preparation ends with all new
coverage decisions pending; only actual owner feedback can change them.

The [historical review](native-tone-coverage-review.md) predates Player27.
The [rollout](../development/player27-rollout.md) provides the current artwork
and append-only diagnostics. Use these existing inputs without rebuilding
artwork, changing palettes/profiles/runtime/dependencies, installing the game or
refreshing the sealed Linux candidate. Preserve historical worksheets, decisions,
helpers, receipts and earlier approvals. Physical-controller acceptance,
unsampled actions/artwork, ordinary animation reachability, licensing and
publication retain their separate scope.

This work needs **no local GPU or local model inference**. CPU Chromium and the
existing authoring environment suffice. Implementation is suitable for one Sol
implementer; shared integration does not require parallel authors.

## Planning findings and contracts

Read-only planning at clean baseline `309b51f` found all 864 required PNGs and
72 metadata files available in the current build. The 792 PNGs corresponding
to the old worksheet remain byte-identical. Current recipe/catalog pins and
68 effective-profile pins match the retained rollout evidence. These findings
establish availability and provenance, not visual coverage; execution must
recheck its actual inputs.

| Input | Required contract |
| --- | --- |
| [Historical matrix](../development/native-tone-coverage-matrix.json) | Preserve 72 source identities, 70 distinct ramps, sampled colors, method and historical assignments. |
| [Diagnostic supplement](../../tmp/player27-rollout/reference-diagnostics.json) | Verify all 2,592 `before` assignments against the matrix; consume `assignments[].after` for current nearest references. |
| [Supplement producer](../../local/authoring/player27-rollout/diagnostics.py) | Reuse its recorded append-only result; running this producer would overwrite retained evidence. |
| [Historical catalog fixture](../../tests/fixtures/player27-six-family-catalog.json) | Resolve the matrix's six-family catalog pin separately from the supplement's current catalog pin; neither historical hash is rewritten. |
| [Current build report](../../generated/player27-rollout/characters-report.json) | Verify its hash against `artwork-checks.json`, itself pinned by `integration-checks.json` in `tmp/player27-rollout/`. |
| [Selected collection](../../palettes/sets/characters-trial.json) | Actual roster, recipe/profile bindings, choice IDs, labels and order must agree with the current report and retained pins. |
| [Historical decisions](../development/native-tone-coverage-review.json) | Preserve this record. Reuse coherent row IDs/schema in a new record with new provenance and pending feedback. |
| [Historical builder](../../local/authoring/native-tone-coverage-review/build.py) | Its fixed output paths and pending-record overwrite behavior make it unsuitable to rerun for this task. Adapt copies in a new namespace. |

Applying the existing common-representative rule to current diagnostics changes
only Player24, Player27 and Player30 from Player22 to Player27. The 164
character-specific alternatives remain. These are navigation proposals, with
no new metric, threshold, coverage percentage or visual acceptance claim.

## Ordered implementation

### 1. Establish fresh outputs and preservation fingerprints

Use these separate destinations; refuse existing final output paths and use
explicit fresh paths for revisions:

| Purpose | Destination |
| --- | --- |
| Owning execution/assessment record | `docs/development/current-tone-coverage-review.md` |
| New decisions | `docs/development/current-tone-coverage-review.json` |
| Adapted preparation/check helpers | `local/authoring/current-tone-coverage-review/` |
| Worksheet and portable download | `tmp/current-tone-coverage-review/`, `tmp/current-tone-coverage-review.zip` |
| Technical receipts and screenshots | `tmp/current-tone-coverage-review-checks/` |

Record the old packet's members/manifest/ZIP, pending decisions, matrix,
diagnostics, relevant helpers, prior review receipts, acceptance records and
sealed candidate fingerprints before preparation. Keep all old bytes intact.
Adapt the small historical helpers without generalizing or modifying them.

### 2. Bind inputs and prepare current proposals

Validate the report/evidence chain, catalogs, selected recipes, effective
profiles and the supplement's exact relationship to the historical matrix.
Load the approximately 116.5 MB report once per process; avoid duplicate
parallel parsing. Treat a stale or missing binding as a blocker rather than
falling back to an older report.

Retain all source identities, including duplicate Darren/Wynne and
Caldarus/Seridia provenance. Extend the shared references with Player27: nine
natural families plus special gray. Keep direct-choice or identical-Vanilla
resolution, actual menu labels/order and per-character nearest alternatives.
Debug Blue remains available as diagnostic context, not a natural representative.

Reuse the existing representative-selection rule with current assignments.
Keep all 17 historical priority comparisons and add Player24/27, yielding
19 priority rows because Player30 is already included. Identify the three
changed proposals explicitly. No replacement matrix or new color metric is
needed.

Create a separate decision record with supplement/historical provenance,
pending statuses, empty owner feedback and no claim of a reviewed artifact.
Keep character exceptions and accepted-limit fields available in the existing
coherent shape. Later feedback must identify the exact new delivered artifact.

### 3. Assemble the compact worksheet and portable packet

Reuse the old worksheet's exact 72 selected portrait/world strips, portrait
crops and native-frame metadata. Copy samples from the unified current tree:

```text
generated/player27-rollout/characters/<character>/original/…
generated/player27-rollout/characters/<character>/variants/<choice>/…
```

Bind each of 864 PNGs and 72 metadata files to its current report entry.
Validate the full character/choice/asset association as well as the hash; use
the [current gallery validator](../../local/authoring/player27-review/browser-check.py)
as the existing pattern. Require equality for the 792 historical sample images.

Keep source swatches beside the selected reference and Vanilla artwork beside
the chosen palette. Expose all 36 characters, twelve choices, 70 source ramps,
nearest alternatives, face/full-portrait controls and native-frame access.
Keep special gray clearly identified. Preserve `data.json`/`data.js` direct-file
loading and responsive framing.

Describe the current Player27 complete gallery accurately as a four-choice
comparison. Earlier ten-choice galleries are historical optional context;
their checkout links are not portable dependencies. Bundle all worksheet
samples, swatches, metadata, pending decision snapshot and fingerprint manifests
in the ZIP, without bundling the complete generated collection or build report.

This worksheet uses a different presentation and frame selection from the
[accepted representative sheet](../development/player27-review-sheet.md#owner-feedback).
Its qualified feedback does not transfer into new coverage decisions.

### 4. Verify the complete packet

- **Data and provenance:** require 36 characters, twelve ordered choices each,
  70 ramps/72 identities and 2,592 unique source-character assignments. Resolve
  every common proposal and nearest alternative to an available non-debug
  choice. Validate duplicate diagnostics, source provenance, all image/metadata
  bindings, dimensions/crops/frame bounds and the report/catalog/profile chain.
- **Historical preservation:** compare all recorded frozen fingerprints after
  preparation. Do not run the old checker's current-input assertions: its
  historical recipe/catalog pins intentionally predate Player27.
- **Browser behavior:** CPU Chromium at 1440×1000 and 390×844 exercises all
  432 character-choice selections per viewport, displaying 864 portrait/world
  samples. Exercise all 70 source selections and Inspect controls, nearest
  alternatives, filters, crop toggle and last-frame access. Explicit cases
  include Player24/27/30, Vanilla alternatives, Caldarus gray and Debug Blue.
- **Displayed content:** assert reference swatches and labels update with
  source/character/palette changes, retaining the stale-Vanilla-swatches
  regression. Compare selected canvas output against independently decoded
  source crops/frames, especially Player27 and crop/frame changes. Reuse the
  [premultiplied-RGBA method](../../local/authoring/player27-review/build-review.py).
- **Negative controls:** reject missing Player27, a stale PNG binding, a wrongly
  routed choice/asset and a stale displayed reference. These must fail for their
  intended reason before accepting the completed checker.
- **Portability:** verify ZIP integrity, member hashes, manifests and required
  links. Open an extracted packet by direct file outside the checkout; identify
  optional complete-gallery links separately from required bundled resources.
- **Final review:** inspect representative desktop/mobile screenshots, check
  changed documentation links and `git diff --check`, and review the final diff
  for scope and unnecessary complexity.

Use the established Pillow authoring environment from the
[pilot](../development/deep-umber-pilot.md#verification-and-limits) if independent
decoding needs it; default Python lacks Pillow. No project dependency or lockfile
change is necessary. Full Cargo/Nix builds, generation, installation/removal and
game sessions add no relevant verification for this report-only scope.

### 5. Record preparation and request one assessment

The new owning development record holds actual verification, artifact
fingerprints, remaining limits and review instructions. Update this plan, the
native-tone plan and overview when preparation completes. Ask one consolidated
coverage question: whether the displayed range adequately covers the sampled
sources with its approximations, or which specific source/family remains missing.

After actual feedback, record the exact words, artifact hashes, affected
groups/character exceptions and inspection limits in the new decision/evidence
record. Keep the delivered packet's pending snapshot and ZIP frozen. A named gap
is a valid assessment result; coverage remains open until it is addressed or
explicitly accepted as a limitation. Selected samples do not establish exhaustive
expression, seasonal/outfit or live-game acceptance.

## Alternatives, resources and remaining decisions

A Player27-only addendum is smaller but leaves the owner reconciling stale
proposals and controls across two reviews. A fresh compact current worksheet
better serves the complete comparison. Parameterizing historical helpers adds
coupling to frozen evidence, and rebuilding complete galleries duplicates
already verified artwork without answering the coverage question.

Planning measured selected PNGs at 14,134,038 bytes. Expect tens of MiB for the
worksheet, archive and checks; execution measures actual use. Retain the final
packet, manifests, check receipts and a few review screenshots. Remove only
verified disposable task-created scratch, preserving previous evidence.

No technical blocker remains. The approved implementation and focused CPU
checks are complete; subjective coverage assessment is pending. The sealed
candidate and all remaining release gates keep their existing status.
The execution record distinguishes completed technical checks from the earlier
read-only planning pass and from owner acceptance.
