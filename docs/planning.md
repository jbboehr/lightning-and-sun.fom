# Project planning

Updated 2026-11-08. Current status, remaining work and standing decisions only.
Each linked record owns its evidence, exact owner feedback and acceptance limits.
The original [handoff](HANDOFF.md) preserves the investigation brief; it is historical.

## Finish line

Approved 2026-11-05 ("let's do it"); the [finish plan](plans/finish.md) defines
batches, dependencies and acceptance.

Deliver a supported Linux release for the existing 36-character roster on the
preserved, fingerprinted game archive. Provide recipes covering portraits and
every world asset that needs recoloring, and account for intentional no-change
assets. Distribute tooling, text recipes/masks and mod scripts only. Exclude
original game assets and recolored, game-derived assets, including image review
media. Users generate replacements locally from their own game copy.

Provide a small character/palette menu, optional remembered choices, a
reproducible CLI installation path and verified removal. New users start on
Vanilla; remembering choices is opt-in. The first supported installation route
uses the pinned Nix environment.

Two milestones: **content-complete alpha** (reached 2026-11-06), then **usable
Linux release** (open).

On 2026-11-06 the owner approved a bounded native-tone expansion of the palette
choices across the existing roster; the [native-tone plan](plans/native-tone-coverage.md)
owns its scope. Windows/macOS, a graphical installer, Workshop publishing,
arbitrary new palette packs, new characters and compatibility with every other
mod are later proposals, not part of this finish line. Existing supported-mod
behavior and known conflicts still need regression checks and documentation.

2026-11-08 owner decisions: the first release ships the current thirteen
generated choices plus Vanilla ("fine for now"; broader tone coverage becomes a
later proposal). The project is licensed AGPL-3.0-only. Ordinary-gameplay
coverage comes from an owner playthrough of the whole game with the mod active,
started once the agent judges the project mostly done.

## Status

"Accepted" below means accepted as displayed; several acceptances carry
qualifications or unspecified inspection depth, recorded in the linked record.

| Area | Status |
| --- | --- |
| Core toolkit and runtime | Implemented; standard checks pass |
| Portrait inventory | All 2,201 strips for 36 characters covered, some intentionally unchanged |
| World inventory | All 4,111 recolorable strips covered; 30 shadow and 18 spectral strips audited as unchanged |
| Original artwork | All batches accepted; see [Completed](#completed) |
| Palette choices | Thirteen generated choices plus Vanilla for all 36 characters, technically verified; frozen for the first release; Stillwell/Terithia accepted provisionally ("for now") |
| Menu, input, persistence | Title/journal menu, keyboard, mouse, virtual gamepad, saving/reset and failure feedback pass live and are accepted; physical controller untested |
| Live gameplay | Basic gameplay, save/load and seven seasonal/action/scene samples accepted; gaps listed below |
| Linux packaging | Private AGPL-3.0-only pinned-Nix candidate built from standalone source and verified; not published |

## Remaining work

| Item | State | Who acts | Record | Local GPU |
| --- | --- | --- | --- | --- |
| Physical-controller acceptance | Not exercised; covered if the playthrough uses a controller | Owner playthrough | [live verification](development/live-release-verification-2026-11-06.md) | Yes |
| Animation end stage in ordinary gameplay | Instrumented prepared-scene check passes and is accepted; no bounded native caller found; original live criterion open | Owner playthrough | [prepared retry](development/animation-end-prepared-retry-2026-11-07.md), [route investigation](development/animation-end-route-investigation-2026-11-07.md) | Yes |
| Unsampled forms and actions | Seven representative cases sampled; others untested; debug scenes do not establish normal story progression | Owner playthrough | [live samples](development/remaining-live-verification-2026-11-07.md) | Yes |
| Unclassified Adeline pathfinding error | Seen once after a debug warp during EndDay; not reproduced on reload; watch for it in the playthrough | Owner playthrough | [live samples](development/remaining-live-verification-2026-11-07.md#setup-failures-and-controls) | Yes |
| Owner playthrough | Ready to start; follow the [playthrough guide](development/owner-playthrough.md) and report problems | Owner | [playthrough guide](development/owner-playthrough.md) | Owner's own play |
| Publication | Separate authorization; candidate notes and support guidance prepared | Owner | [candidate finalization](development/linux-candidate-finalization.md), [support evidence](development/linux-candidate-support-evidence.md) | No |

Live work requires checked GPU availability and a reservation. No delivery date
is established.

## Standing decisions and constraints

- Track authored, validated and owner-accepted art separately. Silence and
  requests to continue are not art approval. Passing checks are not visual
  acceptance.
- Finish a batch and its integrated checks before requesting one consolidated
  review. Validate every changed asset and palette per batch; run full collection
  and isolated install/remove checks at batch boundaries and after integration
  changes. Preserve historical assertions and accepted snapshots.
- Subagents and two parallel character authors are authorized with separate file
  ownership; the parent owns shared registry/count/runtime integration.
- New palette choices append; existing IDs, defaults, version-1 saves and
  previously generated images stay exact.
- The preset limit is a budget, not evidence of tone coverage.
- No new runtime image loader or Rust/GML bridge is planned.

## Completed

- 2026-11-05: Investigation, [status assessment](development/project-status.md)
  and [finish plan](plans/finish.md); P0 inventory and
  [Louis pilot](development/louis-world.md) with [inventory](development/finish-inventory.md).
- 2026-11-05: Batches 1–4: [first](development/finish-batch-2026-11-05.md),
  [second](development/second-finish-batch-2026-11-05.md) with
  [Reina mouth correction](development/reina-mouth-correction.md),
  [third](development/third-finish-batch-2026-11-05.md),
  [fourth](development/fourth-finish-batch-2026-11-05.md).
- 2026-11-06: Batches 5–8: [fifth](development/fifth-finish-batch-2026-11-06.md),
  [sixth](development/sixth-finish-batch-2026-11-06.md),
  [seventh](development/seventh-finish-batch-2026-11-06.md),
  [eighth](development/eighth-finish-batch-2026-11-06.md). All planned world
  authoring complete; content-complete alpha reached.
- 2026-11-06: [Linux release candidate](development/linux-release-candidate-2026-11-06.md)
  and [live verification](development/live-release-verification-2026-11-06.md).
- 2026-11-06: Six-family native-tone expansion and portrait corrections, through
  the [Holt edge comparison](development/holt-moustache-edge-integration.md);
  [coverage evidence](development/palette-tone-coverage.md).
- 2026-11-07: [Deep-umber pilot](development/deep-umber-pilot.md),
  [Player 27 rollout](development/player27-rollout.md) and
  [review sheet](development/player27-review-sheet.md#owner-feedback).
- 2026-11-07: [Remaining live samples](development/remaining-live-verification-2026-11-07.md)
  ([plan](plans/remaining-live-verification.md)) and
  [animation-end check](plans/animation-end-live-check.md).
- 2026-11-07: [Release documentation](plans/linux-release-documentation.md) and
  [private candidate finalization](plans/linux-candidate-finalization.md).
- 2026-11-08: [Twelve-choice coverage review](plans/current-tone-coverage-review.md)
  prepared; [muted-tone pilot](development/muted-tone-pilot.md) and
  [Stillwell/Terithia rollout](development/muted-tone-rollout.md#owner-feedback).
- 2026-11-08: [AGPL-3.0-only applied and candidate resealed](development/licensed-candidate-2026-11-08.md);
  [playthrough guide](development/owner-playthrough.md) prepared.
