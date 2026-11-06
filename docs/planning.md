# Project planning

Updated 2026-11-06. This is the current project overview. The original
[handoff](HANDOFF.md) preserves the investigation brief; it is historical.

The user approved the finish plan on 2026-11-05 with "let's do it".
Implementation is authorized; inventory and Louis pilot validation are complete.
Public publishing and art acceptance remain separate. The user explicitly authorized subagents later in the
same session; character authors completed the first batch in separate files. The user approved the next
Wheedle/Zorel and portrait follow-up batch with "go for it".
The [finish plan](plans/finish.md) defines batches, dependencies and acceptance.

## Current position

The original investigation/proof of concept is complete. The Rust pipeline,
per-character runtime and reversible CLI installation are implemented. The
remaining work is primarily world artwork, art acceptance and release verification. See the dated [assessment](development/project-status.md) for evidence
and limits; raw asset coverage is not a measure of remaining effort.

| Area | Status |
| --- | --- |
| Core toolkit and runtime | Implemented; current standard checks pass |
| Selected portrait inventory | All 2,201 strips for 36 characters included; some intentionally unchanged |
| Selected world inventory | 3,744 of 4,159 strips included; seventh batch integrated |
| Complete world coverage | 34 characters, including Terithia |
| Art acceptance | Elsie, displayed second-batch non-Reina art, corrected Reina, displayed Taliferro/Vera, Errol/Hemlock, Luc/Maple and Terithia art accepted; first/fourth-batch approval limits preserved |
| Release controls | Native Settings page and opt-in saved choices pass API/storage checks; rendered/input acceptance pending |
| Integration confidence | Current full-collection install/verify/remove passes with accepted Reina correction; gameplay/render acceptance remains |

## Approved finish line

Deliver a supported Linux release for the existing 36-character roster on the
preserved, fingerprinted game archive. Include portraits and every world asset
that needs recoloring, and explicitly account for intentional no-change assets.
Keep the current palette choices and correct their known art defects. Provide a
small character/palette menu, optional remembered choices, a reproducible CLI
installation path and verified removal. New users start on Vanilla; remembering
choices is opt-in. The first supported installation route uses the pinned Nix
environment. This scope was approved on 2026-11-05.

Use two milestones: **content-complete alpha**, then **usable Linux release**.
Content authoring and menu work proceed independently; their acceptance gates remain separate.
Neither milestone is complete until its specified checks and art acceptance pass.

Windows/macOS support, a graphical installer, Workshop publishing, arbitrary new
palette packs, new characters, and compatibility with every other mod are later
proposals. They do not expand this finish line. Existing supported-mod behavior
and known conflicts still need regression checks and clear documentation.

## Priority and dependencies

P0 pilot validation is complete. P1 Reina mouth correction and the P2 Wheedle/Zorel and Taliferro/Vera batches are validated and accepted as displayed. P2 authoring is complete; Stillwell/cameo art acceptance and older first-batch acceptance limits remain. P3 authoring is validated and its displayed artwork accepted. P4 implementation is approved. P5 implementation is complete; rendered/input acceptance remains.

| Order | Outcome | Dependency / state | Local GPU |
| --- | --- | --- | --- |
| P0 | Inventory/test repair and measured pilot validated | Louis measured and validated; artwork acceptance pending; see [pilot evidence](development/louis-world.md) and [inventory](development/finish-inventory.md) | No |
| P1 | Finish existing-character world gaps and named portrait defects | All 45 baseline strip gaps covered; Hayden/Juniper displayed details accepted; Reina mouth correction accepted as displayed | No for authoring/tests |
| P2 | Finish smaller inventories and four cameo characters | All strips authored and validated; Stillwell/cameo artwork awaits review, earlier acceptance limits preserved | No for authoring/tests |
| P3 | Finish Errol, Hemlock, Luc, Maple and Terithia | All strips authored, validated and accepted as displayed; live-game acceptance remains separate | No for authoring/tests |
| P4 | Finish Caldarus and Seridia, including an explicit special-form audit | 367 strips need authoring/review; 18 spectral strips audited unchanged | No for authoring/tests |
| P5 | Add one selection menu and opt-in remembered choices using the existing runtime | Native menu/API and real storage/restart/failure phases pass; rendered/input acceptance remains | CPU development; GPU for live verification |
| P6 | Validate complete artwork, package, installation/removal and natural gameplay; prepare supported Linux release | P1–P5 accepted | Yes for gameplay/rendering; CPU for other checks |

The remaining 30 strips outside P1–P4 are Hayden shadows (29, already audited)
and Balor's shadow jump (1), both audited in P0. Other listed inventories may contain
intentional no-change art; classification can reduce new-mask work without
silently dropping assets. Rows reconcile the complete raw omission inventory.

## Execution policy for speed

- Target 16 main content batches: one existing-character finish batch, one cameo
  batch, and one batch for each of the other 14 remaining characters. Start with a
  small new-character pilot from P2; it counts toward that wave. Split a character
  only for a demonstrated material, engine or review risk.
- Reuse the existing exact-frame and component authoring tools. Author all seasons
  together, while keeping complete review pages small and separated by outfit.
- Validate every changed asset and palette per batch. Run full collection and
  isolated installation checks at wave boundaries and after integration changes;
  do not repeat them for each individual action. Preserve historical assertions.
- Finish each active batch and its integrated checks before requesting one consolidated review, as requested by the user.
- Track authored, validated and user-accepted art separately. Batch reviews may
  be consolidated; silence and requests to continue do not count as art approval.
- Keep one owner for shared registry/count/runtime integration. Two character
  authors are authorized; their file ownership is separate and integration stays
  with the parent. The menu/storage source probe and runtime review are complete.

First batch evidence: [integration record](development/finish-batch-2026-11-05.md). Combined checks and full installation/removal pass; broader art acceptance remains pending.

Second batch: combined checks and full-collection installation/removal pass. The user reported no issues outside Reina's mouth; record displayed non-Reina art acceptance, inspection depth unspecified. Reina's skin-spike correction is validated and accepted as displayed; see the [correction evidence](development/reina-mouth-correction.md) and [batch record](development/second-finish-batch-2026-11-05.md). No local GPU was required.

Third batch complete: Taliferro and Vera world art is authored and validated.
Combined checks and full-collection installation/removal pass, preserving accepted
Reina and prior art. Displayed Taliferro/Vera art is accepted; see the
[third-batch evidence](development/third-finish-batch-2026-11-05.md). No local GPU was required.

Fourth batch complete: Stillwell and the four cameo characters cover the final
113 P2 strips. Combined checks and full-collection installation/removal pass;
the user considers Linnet's preserved scar expected and authorized a commit.
Broader art acceptance remains pending. See the [batch evidence](development/fourth-finish-batch-2026-11-05.md).

Fifth batch complete: Errol/Hemlock's 227 P3 world strips are authored,
integrated and validated. Combined checks and full-collection installation/removal
pass; displayed artwork is accepted, with inspection depth unspecified and no
live-game acceptance inferred. See the [batch record](development/fifth-finish-batch-2026-11-06.md).

Sixth batch complete: Luc/Maple's 200 P3 world strips are authored, integrated
and validated. Combined checks and full-collection installation/removal pass;
displayed artwork is accepted, with inspection depth unspecified and no live-game
acceptance inferred. See the
[batch record](development/sixth-finish-batch-2026-11-06.md). Terithia follows,
then Caldarus/Seridia. No local GPU is needed for authoring/tests.

Seventh batch complete: Terithia's 131 world strips are authored, integrated and
validated. Combined checks and full-collection installation/removal pass;
displayed artwork is accepted, with inspection depth unspecified and no live-game
acceptance inferred. See the
[batch record](development/seventh-finish-batch-2026-11-06.md). Caldarus/Seridia's
367 authoring/review strips are next. No local GPU or local model inference was
required for this batch or is needed for P4 authoring/tests.

## Open decisions and limits

Release scope, menu/remembering behavior and parallel authoring are approved.
Source-backed menu/storage feasibility and actual engine IO/restart/failure checks passed; rendered UI and physical input remain unverified.
Controller navigation and current gameplay behavior need live checks. No new runtime image loader or Rust/GML bridge
is planned.

The provisional allowance for remaining content is 12–24 combined author-hours
(roughly 6–12 hours with two sustained authors), before user acceptance and release
verification. Confidence is low: complex special forms and large action inventories
are not represented by the repeated-pose pilot. This is a planning range rather
than a delivery date; see [pilot measurements](development/louis-world.md).

## Completed planning work

2026-11-05: assessed the current checkout, recorded verification and omissions,
and proposed the two completion milestones. See the
[assessment](development/project-status.md) and [finish plan](plans/finish.md).
