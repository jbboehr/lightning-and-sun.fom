# Remaining live gameplay verification

2026-11-07. **Live execution complete with limits; displayed samples accepted with qualifications.**
The owner approved Astra planning with "go for it" after Sol proposed the
remaining seasonal/outfit/action checks. Astra completed a read-only planning
pass; Sol checked the consequential source and lab assumptions below. The
owner separately approved this concrete execution proposal with "go for it".
Existing P6 scope and recorded artwork/live approvals retain their limits.

The [execution record](../development/remaining-live-verification-2026-11-07.md)
owns all seven observed samples, raw/portable evidence, restoration and packet
validation. Physical hardware and the distinct complex-animation end-stage
criterion below remain open. Native scene feasibility was established, with
explicit debug shortcuts and a bounded Priestess setup recovery. The
[owner feedback](../development/remaining-live-verification-2026-11-07.md#owner-feedback)
accepts the displayed samples with inspection depth unspecified; it does not waive
the remaining acceptance criteria.

The owner subsequently approved a CPU investigation of the missing end-stage
route, with live confirmation conditional on finding a suitable native caller.
The [follow-up record](../development/animation-end-route-investigation-2026-11-07.md)
finds no such bounded route and records passing native-handler CPU controls.
No further rendering occurred; the criterion remains open. An instrumented live
sample would require separate authorization.

## Outcome and boundaries

Exercise representative seasonal outfits, seated and complex actions, native
West rendering, actor recreation and portrait/world palette agreement using
the current twelve-choice Linux candidate. Produce reviewable transition
footage and one compact result matrix. This is representative integration
evidence; exhaustive frames and exact pixels retain their existing offline
proofs. The [finish plan](finish.md) owns the release contract, the
[earlier live record](../development/live-release-verification-2026-11-06.md)
owns the accepted basic checks, and the
[Player27 record](../development/player27-rollout.md) owns current image,
package, twelve-choice input and persistence evidence.

Use supplied executables and disposable copies of lab saves/configuration.
Keep real installations, saves, Steam libraries and user settings outside the
game sandbox. Preserve historical packets, accepted snapshots and source
archives. New palettes, production probe hooks, runtime redesign, a new scene
framework and publishing are outside this proposal. A discovered production
defect requires a bounded reproduction and fix decision; it does not justify
unrelated changes.

Planning, setup and evidence processing use CPU. Live rendering requires a
fresh `gpu-status` check immediately before each launch and foreground
`gpu-run` for the game/compositor workload. No local model inference is needed.

## Evidence classes and palette selection

Every result must identify its setup and evidence class:

| Class | Supported claim |
| --- | --- |
| Normal play after recorded preparation | Native controls, scheduling, movement, interaction, outfit/room/day transitions and save/load from that prepared state |
| Built-in debug-driven scene/activity | Actual engine rendering and animation dispatch, with debug-triggered access; ordinary story progression or scheduling is not established |
| Direct callback/API instrumentation, only if separately proposed | Only the stated observed invariant; normal input and physical-controller acceptance are not established |

Use Vanilla, Debug Blue and `player_27` across the selected cases. Blue makes
missed mappings visible; Player27 exercises the newest family. Add one existing
natural donor and Player35 to the seasonal/dialogue anchor. Existing twelve-choice
navigation and persistence checks do not need repetition for every scene.

Use the released journal controls for every palette change. For dialogue, use
**converse → close → journal palette change → close → converse again**, checking
the world actor and portrait. The native player refuses journal opening while
paused, and a textbox closes an existing journal. Instantaneous palette changes
inside an open textbox would require separate instrumented evidence.

## Proposed representative matrix

The table preserves the source-supported setup proposed during planning. Access
from copied early-game saves was unproven then; the execution record above owns
the actual routes and results. Confirm the actual schedule, weather, outfit and
animation before counting a result.

| Case | Setup and observations | Integration risk |
| --- | --- | --- |
| Four seasons and a real season boundary | Adeline dialogue/world anchor. Prepare Spring 28, then use native bed/sleep to reach Summer 1. Prepare separate Autumn and Winter samples. Change palettes through the journal, converse, leave/re-enter and converse again. | Seasonal portrait/world assets, fresh actors and choice retention over room/day changes |
| Beach → Summer outfit change | Pleasant Summer Wednesday, Juniper at the beach. Observe her departure from the Lighthouse entry toward Hayden's house; that departure restores `summer`. Set the palette before departure and retain the before/after outfit plus subsequent dialogue. | A schedule-driven outfit change |
| Winter seated writing | Pleasant Winter Tuesday/Sunday morning, Elsie writing in her bedroom/manor entry, with departure at 7:11. Retain arrival/start, loop, dialogue interruption/resumption and departure where available. Change palettes while the journal pauses the scene. | Final Winter writing strips, seated offsets and complex phases |
| Fishing/baiting and native West | Pleasant Summer Tuesday, Terithia's `beach_wave_fishing` at 13:32 and later `beach_dock_fishing`. Observe activity transitions, props and horizontal facings. Also retain an unmistakable native East→West walk/turn through interaction or pathfinding. | Complex horizontal packs, moving hands/tools and engine-dispatched mirroring |
| Special outfit chain | Replay `heart_events/juniper_eight_hearts` on its own branch. Compare selected palette and Vanilla around Beach/swimming → Beach Accident → Beach → seasonal, including room/actor recreation. | Special outfit identifiers, cutscene actors and choice retention |
| Priestess spell | Attempt `story_events/mines/priestess_quarters_pt_2` on its own branch. Retain Seridia's fade, Priestess `spell_cast`, return to idle and removal. | Priestess/glow mapping, alpha and unchanged materials |
| Wedding sample | Attempt native `heart_events/wedding` through customization into the ceremony. Record the debug fallback spouse if used. Check Juniper's wedding portrait/world and native seated/facing behavior. | Disjoint formal outfit and ceremony actor construction |

Wedding and Priestess access are material feasibility questions. Give each
unproven special route at most one existing 900-second lab session for initial
setup/diagnosis. If inaccessible, preserve the failure, mark the case open and
report the remaining gate. Do not substitute a loose sprite or silently omit
the case. Propose any custom scene separately if the native console route fails.

## Execution tasks and dependencies

1. **Freeze inputs and install the current candidate once.** Verify the package,
   executable, recipes and production GML against the Player27 evidence. The
   retained `tmp/player27-rollout/nix-package` currently resolves to
   `/nix/store/il1nhcfkdfsvlg2dp9d7a8acxxnhl151-mistria-palette-0.1.0`.
   Verify that path before execution; do not search and execute store paths in
   bulk. Use `tmp/finish-batch-install-lab` and write fresh install/verification
   receipts under `tmp/remaining-live-verification/`. The old Player27
   `install.py` hard-codes historical receipt paths and must not be run unchanged.
   Require all 69,432 generated bindings and production installation checks.
   Pin this installation's archive hash rather than requiring generated asset
   IDs to reproduce an earlier installed archive hash.
2. **Prepare independent disposable branches and establish feasibility.** Reuse
   `local/authoring/live-release/game-child.py --resume-lab=final-continued`,
   which copies that lab's `data` and `config`. Pin the seed save/config before
   copying. Launch with `--auto-start continue --debug-tools=true`; retain normal
   unlock/story behavior on the ordinary-play branch. Reset inherited Blue
   choices through the native menu, then select the matrix subjects. Keep
   special-scene experiments on separate branches. Confirm actual console
   access and the intended schedule before recording a pass.
3. **Add bounded transition recording.** Existing `gpu-session.py`,
   `ui-control.py` and `pad-control.py` provide isolation, inputs, still captures
   and shutdown. Add FFmpeg to the existing pinned
   `local/authoring/live-release/tools.nix` and a small local recording helper
   targeting only the private session display/window. Record short lossless
   clips around transitions, with raw PNG comparison stills. Validate decoded
   frames, duration and timestamps, and stop owned recorder processes reliably.
   A logged console text-entry action is optional; existing key events suffice.
   This does not require a production package change or a new controller shim.
4. **Run the matrix in several short sessions.** Reuse the 900-second game limit
   rather than extending it by default. Group seasonal anchor, Summer schedule,
   Winter writing and special-scene sessions. Prepare close to an activity
   boundary, then let native time/pathfinding perform the transition. Log setup
   commands and actual keyboard/mouse or existing virtual-gamepad inputs.
   Clip recording depends on task 3; rendering depends on checked/reserved GPU
   availability and the installed candidate. Stop and report inaccessible cases.
5. **Bind evidence, restore the lab and present one review.** Write a compact
   matrix receipt linking each claim to selected stable IDs, package/archive
   hashes, save origin, scene setup, evidence class, input logs, stills and clips.
   Include failures and unobserved phases. Stop owned jobs, release reservations,
   uninstall with the current package, require exact original restoration and
   absent recovery state, and recheck seed-save hashes. Record results in a new
   owning development record; update concise status links in this plan, the
   overview and finish plan. Preserve the earlier live record and review ZIPs.

Native setup facilities include `manana <season> <date>`,
`clock jump <hour> <minute>`, `goto <npc/location/trellis point>`, `t2 status <npc>`,
`npc request_activity`, `npc end_activity` and `mist run <scene>`.
`manana` runs `new_day_non_grid`, so it is preparation rather than ordinary
day progression. Native sleep across Spring 28→Summer 1 supplies that transition.
Jumping past a schedule boundary does not establish its start/end behavior.
Avoid forced `t2 schedule` changes and arbitrary fact/save edits by default.

## Acceptance and validation

For every passing case, identify the character, actual outfit/animation,
selected stable palette IDs, input path and any unobserved behavior. Across the
completed matrix, require:

- The expected portrait/world assets visibly use the selected palette;
  unrelated subjects retain their choices and Vanilla restoration works.
- Observed animation resumes without restart, freezing, missing stages,
  facing/position jumps or broken props. Retain at least one complete complex
  start→loop→end transition and actual native West dispatch in footage.
- Room/day actor recreation retains the choice, and logs have no unexpected
  runtime error or palette-disabled warning.
- Setup shortcuts, debug triggers, skipped segments and unobserved phases are
  explicit. A blocked or unobserved case remains open.
- The portable review's local links, media decoding, manifests and fingerprints
  validate, and original archive/seed hashes match after cleanup.

`mod/toggle/gml/palette_toggle.gml` owns the invariants: render substitution
leaves animator packs/time/phase alone, selection refreshes paused actors,
new actors attach on later begin steps, one selection drives portrait/world,
and portrait replacement preserves fractional index. Footage establishes
observed continuity; existing interpreter tests retain numerical phase proof.

Keep existing regular/image/persistence results applicable by checking that
production inputs remain pinned. Test the local capture/input helper narrowly.
If a production fix is proposed, reproduce it, add a meaningful failing
regression, run affected and standard checks, and apply the required combined
runtime review once for the authorized change. Planning validation alone does
not establish live success or owner visual acceptance.

## Planning evidence, uncertainties and remaining gates

Sol confirmed the package link, helper isolation/copy behavior, 900-second
limit, absent clip recorder, dialogue pause guard, console date/travel/scene
commands, Juniper's schedule outfit reset and native special-scene script steps.
The lab archive freshly hashes to
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`,
with no `.mistria-palette` recovery directory at the planning checkpoint.
The following members of supplied `tmp/fom/assets.zip` support the routes:

- `assets/gml/scripts/Debug/BuggerInitialize.gml` (console commands);
  `assets/gml/objects/characters/obj_ari.gml` and
  `assets/gml/scripts/UI/Anchor/Menus/TextboxMenu.gml` (journal/dialogue limits).
- `assets/t2/Schedules/Summer Schedules/summer_wednesday.s.toml` (Juniper);
  `assets/t2/Schedules/Winter Schedules/winter_tuesday.s.toml` (Elsie);
  `assets/t2/Schedules/Summer Schedules/summer_tuesday/summer_tuesday_0.s.toml`
  and `_1.s.toml` (Terithia).
- `assets/mist_scripts/heart_events/juniper_eight_hearts.mist`,
  `assets/mist_scripts/story_events/mines/priestess_quarters_pt_2.mist` and
  `assets/mist_scripts/heart_events/wedding.mist` (special scenes).

Astra confirmed physical input device paths remain absent; Sol rechecked
`/dev/input`, `/dev/uinput` and `/dev/uhid`. The inner launcher also creates a
private `/dev` and binds graphics devices only. Existing virtual buttons do
not establish physical mappings, sticks, disconnect recovery or rumble.
A separate physical test needs an available controller and an explicitly
permitted isolated device route. Its minimum matrix is title/journal navigation,
all rows and bottom controls, twelve-choice wrap, south/east activation/back,
shoulder tabs, D-pad/left stick, returning to gameplay and unplug/reconnect.
Record model, connection and mapping; test rumble only if claiming it.

Scene access and GPU availability were the main planning uncertainties. No
reliable elapsed-time estimate follows from the earlier menu tests. Ordinary
play plus native debug scene access is the recommended approach: pure normal
progression would require extensive story access, while direct sprite rendering
would leave scheduling, actor construction and transitions untested. Add only
the local recording capability needed to preserve those transitions.

Owner assessment of the displayed samples is complete with qualifications.
Unresolved cases, physical-controller acceptance or an explicitly accepted limit,
broader tone/unsampled-art assessment, final support/compatibility notes, licensing
and publication decisions remain separate. This batch cannot alone declare the
usable release done.

Retain raw clips/stills/logs, source-member pins and only the new save branches
needed for reproduction. Do not duplicate the full artwork corpus. Inventory
new storage at handoff; remove only verified disposable files created by this
batch. Existing labs, archives, caches and snapshots retain their prior rules.

Execution is authorized, including the local clip recorder and isolated live
runs. No game sessions or helper implementation occurred during planning;
execution evidence is recorded in the linked development record.
