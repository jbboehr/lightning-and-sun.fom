# Plan to finish Lightning and Sun

2026-11-05. **Approved by the user; first integration batch validated, artwork acceptance pending.**
The [planning overview](../planning.md) owns current status and priorities.
The [assessment](../development/project-status.md) owns the starting evidence.

## Outcome and boundaries

Deliver two reviewable milestones:

1. **Content-complete alpha:** all recolorable portrait and world assets for the
   current 36-character roster on the preserved source archive are covered, with
   explicit no-change decisions for the rest; known art defects are resolved and
   new artwork is accepted. Existing test controls can remain for this milestone.
2. **Usable Linux release:** the accepted content has a small selection menu,
   opt-in remembered choices, reproducible local installation, verified removal,
   complete automated checks and a recorded natural-gameplay acceptance pass.

Keep the five existing choices per character and polish their existing mappings.
Debug Blue remains available for QA. Do not add new tone packs, new NPCs, shader
recoloring, runtime asset loading, a native extension bridge, a graphical installer,
Workshop publishing or other operating systems to this finish effort. Do not
promise compatibility with untested game builds or arbitrary mods.

The user approved this scope and release behavior on 2026-11-05. Its implementation
slices are authorized; public publishing and visual art acceptance remain separate
actions. Parallel character authors were explicitly authorized later in the same
session.

The current execution batch includes the validated Louis and Merri pilots,
Adeline Beach/Wedding and Juniper Beach Accident authoring, and P5 controls.
The user requested completion and integrated verification of this batch before
one consolidated art-review request. The checkpoint is complete; see the
[integration record](../development/finish-batch-2026-11-05.md). No new review requests precede that checkpoint.
The Louis pilot supplies an initial 12–24 combined author-hour allowance for the
remaining content; user review and P5/P6 acceptance are additional and unmeasured.
See the overview and each batch's development evidence for current counts and limits.

## P0 — remove friction and measure the work

Local GPU: **not required**. CPU tools only; no local model inference.

- Freeze the preserved archive fingerprint and selected configuration inputs.
  Produce one exhaustive omission inventory with a disposition for every path:
  recolor, intentionally unchanged with evidence, or unresolved. Include special
  outfits, root-level cameos and Priestess forms; avoid seasonal-prefix blind spots.
- Carry forward the Hayden shadow audit, verify Balor's shadow jump and inspect
  Caldarus/Seridia special forms. Zero selected skin can be correct, but must have
  an independent material/no-visible-skin rationale. Unresolved assets block closure.
- Repair the known Adeline archive-review test contract. Reproduce the failure,
  determine its historical 143-asset scope, preserve its mask/pixel assertions and
  historical membership where intended, and add current inventory agreement
  separately if needed. Do not simply raise a number or delete a valid assertion.
  Demonstrate that an omitted asset or changed mask still fails the intended check.
- Finish the existing Elsie six-strip tail as part of P1, then use Louis or Merri
  as the first new-character pilot after a quick source-material inspection.
  Author all four seasons together using existing review tools. This pilot counts
  toward P2 and determines whether whole-character batches are practical.
- Record total frames, exact unique frames, reused approved masks, unresolved
  material decisions, authoring/review time and verification time. Provide an
  observed effort range for the remaining waves, separating CPU elapsed time from
  human review time. Reorder the following waves if measured complexity warrants it.

Exit: complete classified inventory; the known test has a sound passing contract;
one accepted pilot and a measured throughput estimate. Inventory/test triage comes
first; Elsie and the pilot may run while other classifications are being resolved.

## P1–P4 — complete the content in larger batches

Local GPU: **not required** for masks, packages, CPU browser checks or interpreter
tests. Live rendering is checked separately in P6. No local model inference.

Counts below are raw omitted PNG strips, before no-change classification. They
are completeness accounting, not a promise that every strip needs a new mask.

| Wave | Characters / inventory | Raw omitted strips | Proposed integration batches |
| --- | --- | ---: | ---: |
| P1: finish existing characters | Elsie Winter seated writing 6; Adeline Beach 18 and Wedding 15; Juniper Beach Accident 6 | 45 | 1 |
| P2: smaller new inventories | Louis 44; Merri 44; Wheedle 56; Zorel 56; Taliferro 60; Vera 60; Stillwell 68 | 388 | 7, including the pilot |
| P2: cameo group | Darren 11; Linnet 12; Wiscar 11; Wynne 11 | 45 | 1 |
| P3: larger seasonal inventories | Errol 107; Hemlock 120; Luc 99; Maple 101; Terithia 131 | 558 | 5 |
| P4: special coverage | Caldarus 179; Seridia 206 | 385 | 2 |
| Audit dispositions | Hayden shadows 29; Balor shadow jump 1 | 30 | P0 audit |
| Total | All current raw world omissions | 1,451 | 16 main content batches plus audit |

Each normal new-character batch includes all four seasons and relevant special
folders. Caldarus includes Beach/Wedding; Seridia also includes Beach/Wedding,
Flashback Priestess and Priestess. These require explicit form-by-form decisions.
The smaller waves are ordered by inventory size provisionally; same-color clothing
or unusual animation behavior can make a smaller character harder.

P1 also collects the pending Elsie slices 187–200 into one navigable review and
resolves the Hayden embarrassed-expression and Reina/Juniper mouth follow-ups.
These corrections and decisions are additional work beyond the raw strip counts.
If P1 proves too broad for one acceptance decision, keep separate per-character
review/acceptance records while retaining one combined integration checkpoint.

### Repeatable batch procedure

1. Export only needed sources plus the character's retained current corpus;
   refresh `extracted/test-corpus/<character>` without changing accepted baselines.
2. Use `review-batch` to group exact decoded frames. Reuse accepted masks only
   when full frame bytes/dimensions and source pins agree. Component suggestions
   remain suggestions; changed hands, eyelids, mouth edges and shared colors need
   source inspection. Add world colors/groups only as needed without changing
   existing palette-role semantics.
3. Author one character across outfits. Keep every-frame Vanilla/Debug Blue review
   pages small and separated by outfit, plus an explicitly sampled five-choice
   overview. Inspect the full set before asking for visual acceptance. Follow
   [batch review](../development/batch-review.md), including native mirrored views,
   current-bundle image bindings and crops containing every visible pixel.
4. Append reviewed regions and animation mappings. Preserve prior mappings and
   outputs unless an explicit art correction is part of the batch. The integrator
   updates `mod/config/characters.json` and `tests/profile_coverage.rs` once.
5. Validate every changed asset in all variants: exact recipe, dimensions, alpha,
   metadata, no-skin frames, selected skin coverage and material preservation.
   Keep meaningful independent pixel/material checks; seed-generation agreement
   alone is insufficient. Run affected new and retained tests in release mode,
   central inventory checks and GML tests for newly introduced cycle behavior.
6. Freeze a candidate gallery and record authored/validated/user-accepted states
   separately. Record exactly which images a user accepted. Pending art may be
   prepared ahead, but never silently accepted or made an accepted baseline.
7. After acceptance, preserve that character's baseline under `local/fixtures/`
   and reusable authoring inputs under `local/authoring/`. Reuse unchanged
   character snapshots as documented in local storage. Write one concise character
   completion record; update the overview with status and a link, not another
   long per-action narrative.

Exit per wave: every listed omission has accepted output or an evidenced no-change
decision; prior unrelated variants are unchanged; changed variants validate; full
collection generation, normal checks and an isolated MOMI install/verify/uninstall
round trip pass. Follow the established archive restoration and mod-order checks.

### Optional concurrency

Parallel character authors were explicitly authorized on 2026-11-05. Use two
character authors and one integrator with no
more than two unintegrated character candidates at once. Each author owns that
character's profile, palette mappings, review inputs, local sources and focused
tests. They are not alone in the checkout and must accommodate others' changes.
The integrator alone owns shared registry/count edits, runtime code, combined
builds and acceptance records. Shared prefix matches and collection files must
not be edited concurrently. UI work can replace an author slot when appropriate.

Do not parallelize approval or run multiple full builds against one output tree.
Parallelism improves throughput only if review and integration keep up; it does
not reduce the every-frame inspection requirement.

## P5 — make the completed collection usable

CPU development and interpreter tests need **no local GPU**. A **local GPU is
required for live UI/rendering verification**; no local model inference is planned.
This work may overlap content after P0 rather than waiting for every mask.

### P5a: selection menu

First inspect one existing game settings/menu pattern and the supported input and
mod-storage interfaces against the pinned build. Produce a tiny disposable menu
probe before committing to an integration. Time-box this to one investigation
slice; an unsupported interface is a recorded design question, not permission to
invent a Rust helper or runtime loader.

Implement one character list and the existing palette choices using the current
per-character selection path. Keep portrait and world state synchronized, preserve
animation phase and allow Vanilla restoration. Prefer existing game UI/input
patterns. Check keyboard/controller navigation, input focus, dialogue interaction
and hotkey conflicts. Test controls must not consume ordinary gameplay keys when
the released menu is closed; bind a conflict-checked entry point during the probe.
Choose its exact binding from observed controls rather than assume a spare key.

Exit: every character is selectable without memorizing 36 keys; cancel/close and
missing-asset paths work; unrelated input and normal game menus still work. The
same selection state drives portraits, world sprites and menu feedback.

### P5b: opt-in remembered choices

Use supported mod-owned storage established by the probe, with stable character
and preset IDs. Keep normal game saves untouched. Default to Vanilla on every
launch unless the user explicitly enables remembering. Add reset-to-Vanilla and
test restart, missing/invalid settings, unavailable presets and write failure.
Recover with a clear diagnostic and usable defaults; failed persistence must not
damage game saves or prevent play. Do not promise an engine API before verifying it.

Exit: opt-in choices survive restart, opting out restores session-default behavior,
old/invalid input is handled explicitly, and reset affects only this mod's settings.
If storage/menu feasibility fails, revise the proposal openly; the alpha remains
a separate deliverable and does not silently become the completed usable release.

## P6 — one integrated release candidate

Local GPU: **required for natural gameplay and rendered UI checks**; automated
asset/interpreter/build checks use CPU. Check availability before live runs and
use the required GPU reservation for any high-demand workload. No local inference.

- Reconcile the final inventory with the original path set: every source is
  covered or has an explicit no-change disposition. Do not use a raw target of
  6,360 selected strips because known shadows do not require generated variants.
- Run formatting, Clippy, all regular tests, the explicitly selected relevant
  local artwork suite, all GML tests and browser reviews. The archive-review check
  must pass under its repaired contract. Run all chosen variants through recipe
  validation, and check complete review pages against current bundle bytes.
- Generate one full candidate and verify installed pixels/metadata, fresh install,
  removal, exact restoration, existing supported folder-mod handling and refusal
  of mismatched source or externally modified archives in disposable labs.
- Run a natural-gameplay matrix: dialogue and movement, seasonal/outfit changes,
  seated and complex action transitions, native mirrors, new menu input, selected
  palettes during both portraits and world animation, restart/remember/reset and
  controller navigation. Exercise new animation families and known risk cases;
  record what was tested live versus only inspected/offline simulated. Every frame
  still receives offline coverage; do not claim an exhaustive live playthrough.
- Perform the applicable reliability review once for the completed runtime change
  task under repository/global instructions, after normal tests; avoid separate
  reviews for each data-only character. Fix concrete in-scope defects and rerun
  affected checks.
- Prepare a reproducible Linux binary/install artifact with its required runtime
  dependencies accounted for. Use the pinned Nix environment as the first supported
  installation route; a portable non-Nix bundle is outside this fast finish scope.
  Verify the documented route in a clean environment with no accidental dependence
  on unretained Nix paths. Ship code/configuration/tooling only; users
  generate assets from their own game copy. Verify package contents contain no
  game images, archives, generated replacement assets or local fixtures.
- Update end-user installation, palette controls, reset/removal and compatibility
  documentation. Keep build/release evidence in maintainer documents. A graphical
  installer and CI expansion are not prerequisites if reproducible release checks
  are recorded. Prepare the tag/release notes; publish only when authorized.

Exit: content and UI are accepted, required automated checks pass, the live matrix
has no unresolved release blockers, install/remove is reproducible, and the
supported game fingerprint/platform and residual limitations are explicit.

## Throughput and stopping rules

The speed improvement comes from exact reuse, whole-character authoring, bounded
integration and consolidated reviews. Run normal full checks and combined MOMI
round trips at wave boundaries and after runtime/installer changes; run focused
checks while editing. Do not turn a single failed frame into a new project-wide
rebuild. Never skip required checks or relabel failed assertions as completion.

Do not build a new authoring framework. Use the existing release CLI and retained
helpers. Add a small reproducible helper only if the pilot identifies a repeated
manual step that is actually limiting throughput. Do not recreate complete
collection snapshots for every action or add a new test target for every strip.

After the pilot, forecast remaining time using measured unique-frame authoring
and review rates plus verification time, with a separate allowance for new
materials and P5/P6. Update the range after each wave. If complex forms or UI
integration materially change scope, record that fact before widening the plan.
Finishing a batch does not erase pending acceptance or authorize deferred features.

## Second batch checkpoint — 2026-11-05

Wheedle/Zorel authoring and combined checks are complete; paired/full-collection installation and
exact removal pass. Named portrait details were audited without a concrete mask
defect; the prior treatment is retained and subjective judgment remains pending.
See [second-batch evidence](../development/second-finish-batch-2026-11-05.md).
The user-approved batch used two character authors plus portrait follow-up work,
required no local GPU and does not close the release rendering/input gate.

The user subsequently reported Reina mouth issues and no other issues in the
displayed second batch. Preserve displayed non-Reina acceptance without inferring
inspection depth or gameplay acceptance. A focused Reina-only correction is
authorized; retain the original review, prior art lineage and unrelated artwork.
Authoring, material checks and CPU browser review require no local GPU.

Reina's reported lower-mouth skin spikes are corrected in a new preserved
lineage. Standard/material checks and targeted installation/removal pass;
the user accepted the displayed correction, inspection depth unspecified. See the
[correction evidence](../development/reina-mouth-correction.md).

## Third batch — Taliferro and Vera

All 120 world strips are authored and validated. Combined checks and full
36-character install/verify/remove pass, including accepted Reina output.
The user accepted the displayed Taliferro/Vera artwork with "looks good to me"
and requested a commit and the next batch. Inspection depth is unspecified;
live-game acceptance remains separate. See the
[batch evidence](../development/third-finish-batch-2026-11-05.md). This approved
P2 batch used two independent character authors and required no local GPU.
