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
planned content authoring is complete. Displayed artwork is accepted; release verification remains. See the dated [assessment](development/project-status.md) for evidence
and limits; raw asset coverage is not a measure of remaining effort.

| Area | Status |
| --- | --- |
| Core toolkit and runtime | Implemented; current standard checks pass |
| Selected portrait inventory | All 2,201 strips for 36 characters included; some intentionally unchanged |
| Selected world inventory | All 4,111 recolorable strips included; remaining 30 shadows and 18 spectral strips audited unchanged |
| Complete world coverage | All 36 recolorable inventories authored and validated |
| Art acceptance | All planned displayed artwork accepted, including qualified first/fourth-batch acceptance; recorded inspection and glasses/scar qualifications preserved |
| Content-complete alpha | Reached on 2026-11-06; live-game acceptance remains separate |
| Release controls | Live title/journal keyboard, mouse, virtual-gamepad and persistence checks pass; displayed review has qualified user acceptance; physical-controller acceptance remains |
| Integration confidence | Final package passes install/remove and basic rendered dialogue/movement/room/day/save/load checks; displayed review accepted with qualifications; broader live acceptance remains |
| Linux packaging | Installable pinned-Nix candidate built and checked from standalone source; not published |

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
The content-complete alpha is reached. The usable Linux release still requires
its live checks and remaining release gates.

Windows/macOS support, a graphical installer, Workshop publishing, arbitrary new
palette packs, new characters, and compatibility with every other mod are later
proposals. They do not expand this finish line. Existing supported-mod behavior
and known conflicts still need regression checks and clear documentation.

## Priority and dependencies

P0 pilot validation and all P1–P4 authoring are complete. All planned artwork is
accepted as displayed, including qualified first/fourth-batch approval. Earlier
glasses/scar qualifications and unspecified inspection depth remain recorded.
P5 title/journal menu, keyboard/mouse/virtual-gamepad input, saving/reset/restart and failure feedback pass live after three bounded initialization/notification fixes. Basic natural gameplay and actual save/load also pass; the displayed live review has qualified user acceptance. Physical-controller and broader seasonal/action acceptance remain. See the [live verification record](development/live-release-verification-2026-11-06.md).

| Order | Outcome | Dependency / state | Local GPU |
| --- | --- | --- | --- |
| P0 | Inventory/test repair and measured pilot validated | Complete; Louis artwork accepted with recorded qualifications; see [pilot evidence](development/louis-world.md) and [inventory](development/finish-inventory.md) | No |
| P1 | Finish existing-character world gaps and named portrait defects | All 45 baseline strip gaps covered; Hayden/Juniper displayed details accepted; Reina mouth correction accepted as displayed | No for authoring/tests |
| P2 | Finish smaller inventories and four cameo characters | Complete and accepted as displayed, with recorded qualifications | No for authoring/tests |
| P3 | Finish Errol, Hemlock, Luc, Maple and Terithia | All strips authored, validated and accepted as displayed; live-game acceptance remains separate | No for authoring/tests |
| P4 | Finish Caldarus and Seridia, including an explicit special-form audit | Complete and accepted as displayed; live-game acceptance remains separate | No for authoring/tests |
| P5 | Add one selection menu and opt-in remembered choices using the existing runtime | Native menu/storage/input checks pass and displayed live review accepted with qualifications; physical-controller acceptance remains | CPU development; GPU for live verification |
| P6 | Validate complete artwork, package, installation/removal and natural gameplay; prepare supported Linux release | Artwork and displayed basic live review accepted; broader live matrix and final release gates remain | Yes for gameplay/rendering; CPU for other checks |

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

First batch complete: combined checks and full installation/removal pass;
displayed artwork has qualified acceptance, with inspection depth unspecified
and no live-game approval inferred. See the
[integration record](development/finish-batch-2026-11-05.md).

Second batch: combined checks and full-collection installation/removal pass. The user reported no issues outside Reina's mouth; record displayed non-Reina art acceptance, inspection depth unspecified. Reina's skin-spike correction is validated and accepted as displayed; see the [correction evidence](development/reina-mouth-correction.md) and [batch record](development/second-finish-batch-2026-11-05.md). No local GPU was required.

Third batch complete: Taliferro and Vera world art is authored and validated.
Combined checks and full-collection installation/removal pass, preserving accepted
Reina and prior art. Displayed Taliferro/Vera art is accepted; see the
[third-batch evidence](development/third-finish-batch-2026-11-05.md). No local GPU was required.

Fourth batch complete: Stillwell and the four cameo characters cover the final
113 P2 strips. Combined checks and full-collection installation/removal pass;
displayed artwork has qualified acceptance, retaining the earlier Linnet-scar
assessment without inferring live-game approval. See the
[batch evidence](development/fourth-finish-batch-2026-11-05.md).

Fifth batch complete: Errol/Hemlock's 227 P3 world strips are authored,
integrated and validated. Combined checks and full-collection installation/removal
pass; displayed artwork is accepted, with inspection depth unspecified and no
live-game acceptance inferred. See the [batch record](development/fifth-finish-batch-2026-11-06.md).

Sixth batch complete: Luc/Maple's 200 P3 world strips are authored, integrated
and validated. Combined checks and full-collection installation/removal pass;
displayed artwork is accepted, with inspection depth unspecified and no live-game
acceptance inferred. See the
[batch record](development/sixth-finish-batch-2026-11-06.md). No local GPU was
needed for authoring/tests.

Seventh batch complete: Terithia's 131 world strips are authored, integrated and
validated. Combined checks and full-collection installation/removal pass;
displayed artwork is accepted, with inspection depth unspecified and no live-game
acceptance inferred. See the
[batch record](development/seventh-finish-batch-2026-11-06.md). No local GPU or
local model inference was required.

Eighth batch complete, 2026-11-06: Caldarus/Seridia world artwork is authored,
integrated, validated and accepted as displayed. Earlier variants and accepted
snapshots remain exact; audited spectral forms are preserved through installation.
See the [batch record](development/eighth-finish-batch-2026-11-06.md). No local GPU
was required. All planned world authoring is complete; intentional no-change
assets and earlier approval limits remain recorded separately.

The remaining work is live verification and final release preparation:

1. Complete remaining physical-controller and broader seasonal/outfit/action
   verification, recording any accepted limits. Further live work requires
   checked GPU availability and a reservation. Prepare release notes from the
   exercised matrix and resolve concrete blockers.
2. Finalize the tested candidate and supported-platform/compatibility notes;
   resolve the owner's licensing choice before any public distribution.
   Publishing remains separate.

The content-complete alpha is reached following qualified acceptance of the final
consolidated artwork review. The usable Linux release still requires the live
P6 matrix and final release gates.

Linux CPU release batch complete, 2026-11-06: the installable Nix candidate,
standalone-source build, folder-mod/refusal and exact removal checks pass with
the final collection. End-user instructions are current; the subsequent
consolidated artwork review is accepted with qualifications. See the
[release-candidate record](development/linux-release-candidate-2026-11-06.md).
The subsequent live record owns rendered/input/gameplay results and limits;
publishing remains separate.

Live verification batch accepted as displayed with qualifications, 2026-11-06: hardware-rendered native
menus, input, persistence/failures and basic dialogue/movement/room/day/save/load
pass in an isolated lab. Three concrete integration defects are repaired; broader
seasonal/action and physical-controller acceptance remain.
See the [live verification record](development/live-release-verification-2026-11-06.md).

## Open decisions and limits

Release scope, menu/remembering behavior and parallel authoring are approved.
Rendered title/journal UI, actual OS keyboard/mouse and SDL virtual-gamepad events,
persistence/failures and basic game save/load pass; the displayed review has
qualified user acceptance. Physical controller and broader
seasonal/action playthrough acceptance remain. No new runtime image loader or Rust/GML bridge
is planned.

The provisional 12–24 combined author-hour content allowance is now historical;
all planned authoring is finished. Character records retain measured work and
verification evidence. Remaining effort depends on live GPU checks and final
release preparation; the CPU Linux candidate is prepared. No delivery date is established.
See [pilot measurements](development/louis-world.md) for the original estimate.

## Completed planning work

2026-11-05: assessed the current checkout, recorded verification and omissions,
and proposed the two completion milestones. See the
[assessment](development/project-status.md) and [finish plan](plans/finish.md).
