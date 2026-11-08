# Project planning

Updated 2026-11-07. This is the current project overview. The original
[handoff](HANDOFF.md) preserves the investigation brief; it is historical.

The user approved the finish plan on 2026-11-05 with "let's do it".
Implementation is authorized; inventory and Louis pilot validation are complete.
Public publishing and art acceptance remain separate. The user explicitly authorized subagents later in the
same session; character authors completed the first batch in separate files. The user approved the next
Wheedle/Zorel and portrait follow-up batch with "go for it".
The [finish plan](plans/finish.md) defines batches, dependencies and acceptance.

## Current position

The original investigation and finish-plan authoring are complete. Original
artwork and displayed live controls/basic gameplay have recorded acceptance,
including the qualifications in their owning records. Release verification and
final release gates remain.

The approved native-tone expansion adds six source families across all 36
characters. Expansion and the complete portrait corrections pass integrated
checks. The latest [Holt edge comparison](development/holt-moustache-edge-integration.md)
received "much better" feedback with no further defect reported. Record this as
qualified positive feedback on the displayed revision, with inspection depth
unspecified. The user authorized this checkpoint commit; no reported
portrait defect remains open. Broader approximate-coverage assessment and the
remaining release gates stay separate. The
[native-tone plan](plans/native-tone-coverage.md) owns scope and next steps;
original approvals and their limits remain recorded. The subsequent delegated
Player27 addition is implemented and passes full image, package, native control,
persistence and removal checks. Its [rollout record](development/player27-rollout.md)
owns the complete comparison; the [representative sheet record](development/player27-review-sheet.md#owner-feedback)
records qualified acceptance of the displayed 36-character sample. Broader artwork
and approximate-coverage assessment remain separate.

| Area | Status |
| --- | --- |
| Core toolkit and runtime | Implemented; current standard checks pass |
| Selected portrait inventory | All 2,201 strips for 36 characters included; some intentionally unchanged |
| Selected world inventory | All 4,111 recolorable strips included; remaining 30 shadows and 18 spectral strips audited unchanged |
| Complete world coverage | All 36 recolorable inventories authored and validated |
| Art acceptance | Original approvals, qualified Celine/roster/Adeline/Holt feedback and qualified Player27 representative-portrait acceptance recorded; no current reported portrait defect; broader coverage assessment remains |
| Content-complete alpha | Reached on 2026-11-06; live-game acceptance remains separate |
| Release controls | Live title/journal keyboard, mouse, virtual-gamepad and persistence checks pass; displayed review has qualified user acceptance; physical-controller acceptance remains |
| Integration confidence | Final package passes install/remove and basic gameplay checks; basic live review and seven further seasonal/action/scene samples accepted as displayed with qualifications; untested coverage remains open |
| Linux packaging | Installable pinned-Nix candidate built and checked from standalone source; not published |
| Palette breadth | Six sourced additions committed; one delegated Player27 addition fully verified across all 36, retaining earlier choices; displayed representative portraits accepted with qualifications; broader artwork and coverage assessment remain |

## Approved finish line

Deliver a supported Linux release for the existing 36-character roster on the
preserved, fingerprinted game archive. Provide recipes covering portraits and
every world asset that needs recoloring, and account for intentional no-change
assets. Distribute tooling, text recipes/masks and mod scripts only. Exclude
original game assets and recolored, game-derived assets, including image review
media. Users generate replacements locally from their own game copy.
Keep the current palette choices and correct their known art defects. Provide a
small character/palette menu, optional remembered choices, a reproducible CLI
installation path and verified removal. New users start on Vanilla; remembering
choices is opt-in. The first supported installation route uses the pinned Nix
environment. This scope was approved on 2026-11-05.

On 2026-11-06 the user requested planning broader palette coverage next. The
[native-tone expansion plan](plans/native-tone-coverage.md) proposes representative
NPC and creator tones across the existing roster, using the accepted masks.
The user approved implementation with "let's do it" after reviewing that plan.
This extends the original five-choice scope; new artwork still needs acceptance.

Use two milestones: **content-complete alpha**, then **usable Linux release**.
Content authoring and menu work proceed independently; their acceptance gates remain separate.
The content-complete alpha is reached. The usable Linux release still requires
its live checks and remaining release gates.

Windows/macOS support, a graphical installer, Workshop publishing, arbitrary new
palette packs, new characters, and compatibility with every other mod are later
proposals. The bounded native-tone expansion above is approved; other proposals do not
expand this finish line. Existing supported-mod behavior
and known conflicts still need regression checks and clear documentation.

## Priority and dependencies

P0 pilot validation and all P1–P4 authoring are complete. All planned artwork is
accepted as displayed, including qualified first/fourth-batch approval. Earlier
glasses/scar qualifications and unspecified inspection depth remain recorded.
P5 title/journal menu, keyboard/mouse/virtual-gamepad input, saving/reset/restart and failure feedback pass live after three bounded initialization/notification fixes. Basic natural gameplay and actual save/load also pass; the displayed live review has qualified user acceptance. Physical-controller and broader seasonal/action acceptance remain. See the [live verification record](development/live-release-verification-2026-11-06.md).

The six-family native-tone implementation and reported portrait corrections
are committed. The subsequent Player27 addition is verified and committed locally;
its representative portrait review has qualified acceptance. The
[coverage assessment](development/palette-tone-coverage.md) owns the evidence;
the [native-tone plan](plans/native-tone-coverage.md) owns the remaining coverage
work. Existing approvals and the original content-complete alpha remain valid.

| Order | Outcome | Dependency / state | Local GPU |
| --- | --- | --- | --- |
| P0 | Inventory/test repair and measured pilot validated | Complete; Louis artwork accepted with recorded qualifications; see [pilot evidence](development/louis-world.md) and [inventory](development/finish-inventory.md) | No |
| P1 | Finish existing-character world gaps and named portrait defects | All 45 baseline strip gaps covered; Hayden/Juniper displayed details accepted; Reina mouth correction accepted as displayed | No for authoring/tests |
| P2 | Finish smaller inventories and four cameo characters | Complete and accepted as displayed, with recorded qualifications | No for authoring/tests |
| P3 | Finish Errol, Hemlock, Luc, Maple and Terithia | All strips authored, validated and accepted as displayed; live-game acceptance remains separate | No for authoring/tests |
| P4 | Finish Caldarus and Seridia, including an explicit special-form audit | Complete and accepted as displayed; live-game acceptance remains separate | No for authoring/tests |
| P5 | Add one selection menu and opt-in remembered choices using the existing runtime | Native menu/storage/input checks pass and displayed live review accepted with qualifications; physical-controller acceptance remains | CPU development; GPU for live verification |
| Native-tone checkpoint | Broaden selectable tones across the existing roster | Six-family checkpoint and Player27 follow-up committed; qualified representative-portrait acceptance recorded; broader coverage assessment remains in the [native-tone plan](plans/native-tone-coverage.md) | No for commit; yes for further live rendering |
| P6 | Validate complete artwork, package, installation/removal and natural gameplay; prepare supported Linux release | Earlier artwork/basic live acceptance retained; seven-case follow-up accepted as displayed with qualifications; untested cases and final release gates remain | Yes for gameplay/rendering; CPU for other checks |

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

The next planned batch and remaining release work are:

1. Assess broader approximate native-tone coverage separately from the qualified
   displayed feedback recorded with this checkpoint and its frozen review packet.
   The approved [compact coverage review](plans/native-tone-coverage-review.md)
   is prepared and technically verified using existing diagnostics and current
   artwork. Owner coverage assessment remains pending; no local GPU was required.
   The user approved a bounded [deep-umber pilot](development/deep-umber-pilot.md)
   comparing Player 27 and Player 30; it is prepared and technically verified.
   After the one-addition proposal, the owner delegated the decision with
   "up to you". Sol selects Player 27; the [single-family rollout](plans/player27-rollout.md)
   is implemented and technically verified, retaining every existing choice.
   Full image preservation, pinned package/install/remove and twelve-choice
   native controls/persistence pass. The complete comparison remains available.
   The subsequently requested [representative review sheet](development/player27-review-sheet.md)
   has qualified owner acceptance of the displayed neutral portraits for all 36
   characters. Its record owns the exact feedback and inspection limits; broader
   artwork and coverage acceptance are not inferred.
2. Complete remaining live acceptance and record any accepted limits. The approved
   [bounded live matrix](plans/remaining-live-verification.md) has exercised all
   seven representative cases on the current candidate and restored the exact
   original lab archive. Its [result record](development/remaining-live-verification-2026-11-07.md)
   owns the review, setup failures, qualified displayed-sample acceptance and
   verification limits. Physical-controller acceptance,
   distinct complex-animation end-stage dispatch and unsampled forms/actions
   remain open; debug scene access does not establish normal story progression.
   Further live work requires checked GPU availability and a reservation.
   Prepare release notes from the exercised matrix and resolve concrete blockers.
   The approved [animation-end route investigation](development/animation-end-route-investigation-2026-11-07.md)
   found no suitable bounded native caller; its CPU controls pass. The end-stage
   live criterion remains open. The owner approved planning for the
   [isolated instrumented check](plans/animation-end-live-check.md), then approved
   implementation/execution and a corrected prepared-scene retry. The
   [instrumented result](development/animation-end-prepared-retry-2026-11-07.md)
   passes paired Summer horizontal stop-stage rendering and preservation checks.
   The owner accepted the displayed paired comparison; inspection depth is
   unspecified. Ordinary gameplay reachability and the original live criterion
   remain separate; no acceptance limit is waived.
3. Candidate release notes and support guidance prepared, 2026-11-07; see the
   [documentation record](development/linux-candidate-support-evidence.md) and
   [completed plan](plans/linux-release-documentation.md). Finalize the tested
   candidate and resolve the owner's licensing choice before public distribution.
   Remaining acceptance gates and separate publication authorization still apply.
   The approved [private candidate finalization](plans/linux-candidate-finalization.md)
   has completed current twelve-choice CPU compatibility and standard checks,
   restored the original lab and prepared the updated text-only source snapshot.
   The [owning record](development/linux-candidate-finalization.md) links the
   external sealed-artifact and standalone-build verdict. No local GPU or
   inference was required; remaining release gates stay open.

The content-complete alpha is reached following qualified acceptance of the final
consolidated artwork review. The usable Linux release still requires the remaining
P6 acceptance and final release gates.

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

Remaining live sample batch accepted as displayed with qualifications, 2026-11-07:
all seven seasonal/action and special-scene cases have representative evidence;
the exact original lab archive is restored. The
[result record](development/remaining-live-verification-2026-11-07.md) owns the
feedback, packet validation and remaining gaps. Physical hardware, distinct
complex-animation end dispatch and unsampled forms/actions remain open.

## Open decisions and limits

Release scope, menu/remembering behavior and parallel authoring are approved.
Broader native-tone implementation is approved. Six additions are integrated across all 36 characters; full strict package checks
and expanded native checks pass. Qualified positive displayed-art feedback is
recorded through the final Holt correction; full inspection/approximate-coverage
approval is not inferred. The six-family checkpoint raised the bounded ceiling
to ten generated variants; the authorized Player27 addition raises it to eleven
plus Vanilla. Its complete verification and comparison packet are recorded in
the [Player27 rollout](development/player27-rollout.md). Qualified acceptance of
the displayed representative portraits is recorded separately from broader
artwork and coverage assessment. The existing preset
limit is a starting budget, not evidence of approximate tone coverage.
Rendered title/journal UI, actual OS keyboard/mouse and SDL virtual-gamepad events,
persistence/failures and basic game save/load pass; the displayed review has
qualified user acceptance. Physical controller and broader
seasonal/action playthrough acceptance remain. No new runtime image loader or Rust/GML bridge
is planned.

The provisional 12–24 combined author-hour content allowance is now historical;
all original-scope authoring is finished. Character records retain measured work and
verification evidence. The active tone expansion has measured generation/package costs in its
[coverage record](development/palette-tone-coverage.md).
Remaining release effort also depends on live GPU checks and final
release preparation; the CPU Linux candidate is prepared. No delivery date is established.
See [pilot measurements](development/louis-world.md) for the original estimate.

## Completed planning work

2026-11-05: assessed the current checkout, recorded verification and omissions,
and proposed the two completion milestones. See the
[assessment](development/project-status.md) and [finish plan](plans/finish.md).
