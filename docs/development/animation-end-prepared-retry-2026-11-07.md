# Prepared native animation-end retry — 2026-11-07

Status: **instrumented native rendering check passes** for Terithia's Summer
east-facing fish collection with Vanilla and Debug Blue. The default control
keeps looping; both finite trials render the distinct stop stage and complete
with the native held first-start frame. The owner accepted the displayed
comparison with "looks good to me"; inspection depth is unspecified.
Ordinary gameplay reachability and the original live criterion remain separate.

The [local self-contained comparison](../../tmp/animation-end-prepared-retry/review.html)
contains paired videos and frames. Vanilla is on the left, Debug Blue on the
right. The [technical checks](../../tmp/animation-end-prepared-retry/checks.json)
and [media binding](../../tmp/animation-end-prepared-retry/media-binding.json)
own the detailed evidence. All game-derived media remains local and outside
release files or attachments under the
[distribution boundary](linux-candidate-support-evidence.md#distribution-boundary).

## Authorization and corrected setup

The owner replied "sure why not" to correcting the activity/facing pairing and
aborting captures when probe arming fails, followed by a bounded prepared-scene
GPU retry. The parent allocated up to 600 seconds for preparation and 300 for
the control, paired trials and restoration, with an overall 900-second cap.
The CPU corrections require no GPU or local inference. Both rendered sessions
used fresh `gpu-status` checks and foreground `gpu-run`; reservations are released.

The earlier plan paired a horizontal pose with `beach_wave_fishing`. Native
`assets/fiddle/activities.toml` restricts that activity to `beach/fishing_3`;
the native room object inherits the default trellis direction South. The
horizontal beach points belong to `beach_dock_fishing`. The native activity
handler's offscreen shortcut requires a target node; its trellis branch does
not assign one. Offscreen staging therefore does not bypass this travel path.

The [command wrapper](../../tmp/animation-end-prepared-retry/control.py)
now rejects native errors and requires an actual matching `armed` event for
each trial. Its [eight focused cases](../../tmp/animation-end-prepared-retry/control-checks.json)
cover rejection, missing/wrong/duplicate arming and missing acknowledgement.
Replaying the prior recorded native error reproduces acceptance by the old
acknowledgement-only condition and rejection by the corrected check.

The existing `matrix-summer` autosave was inspected read-only: its header has
calendar time 2,419,200, Summer 1 at 6 a.m. The source SHA256 is
`12856f5b3a268f3a63f7bd2a1d4f44712ce5e5ff1bcebe8d84f013bcff666b72`.
Both new sessions copied this save; neither edited its facts or advanced days.
The separate instrumented archive and unchanged probe were reused exactly from
the [first bounded attempt](animation-end-live-verification-2026-11-07.md).
The installer-managed lab stayed restored throughout; no new installation,
package build or archive copy was necessary.

## Attempts within the shared allocation

| Session | Result |
| --- | --- |
| `animation-end-prepared` | Native travel and the quest-board scene complicated preparation. The existing skip confirmation cleared that scene. Night dock fishing selected South; the West guard detached before any request. A later arm failed because no live Terithia was available at 2 a.m.; the corrected command wrapper stopped progression. No recording or finite request ran. The session was stopped after 357.472 seconds. |
| `animation-end-morning` | Reloaded the same unedited Summer 1 save. Terithia's existing Monday morning fishing routine settled at `eastern_road/fishing_2`, facing East, using native activity `fishing`. Native travel placed the player nearby and ordinary movement cleared the character and full rod. The clock was stopped at 7:30 a.m. after the activity was active. All three captures passed. Vanilla and the clock were restored through native input before controlled shutdown after 240.349 seconds. |

The correction from the beach proposal uses the same intended Summer horizontal
fish collection with a source-supported morning activity/facing pair. It changes
scene preparation, not artwork, native animation code or the probe's finite API.
No custom scene, forced sprite/phase/frame, NPC coordinate assignment or speed
change was used. Existing `npc play_animation terithia idle` and `fish` commands
selected fresh collections while the native activity remained active.

Preparation took 445.477 seconds before the capture allocation began. Capture
and restoration took 164.131 seconds. Total wall time was 609.609 seconds,
including the restart gap; both sessions shared one allocation. The retained
[deadline guard](../../tmp/animation-end-prepared-retry/guard.py) used the first
session's absolute start, rather than granting a fresh budget after restart.
Both native game exits `-15` are controlled termination, with compositor exit
zero; they are not crash findings.

## Native and rendered evidence

The two logs contain 27 preparation events and 2,031 morning events, respectively.
Every observed JSON event matches the original native log exactly.

| Trial | Native result | Recording |
| --- | --- | --- |
| Vanilla default control | 641 observed animation calls; default `I32_MAX` request; phase 1 reaches loop count 603 without entering stop or completing | 25 seconds, 750 decoded lossless frames |
| Vanilla finite | One `set_loop_x_times(600)` request after a fresh collection; 682 observed calls; phases `0 → 1 → 2 → 0`; all five stop indices; 15 completed native hold calls | 25 seconds, 750 decoded lossless frames |
| Debug Blue finite | Same finite request and native state sequence; original raw stop asset and selected generated rendered stop asset; all five stop indices and the same hold | 25 seconds, 750 decoded lossless frames |

The complete projected before/after native state sequences of both finite trials
are equal across phase, counts, request, completion, fractional frame, subimage,
speed, facing, position and original sprite identity. Both use native speed 1,
the same actor and Summer collection, and the captured one-frame loop has native
duration zero. The start contributes to the loop count; 600 is not a claim of
600 full loop iterations. Production palette substitution stays unchanged.

Four corruptions of the recorded packet are rejected: removing phase two,
labelling an earlier rendered sprite as stop, changing the palette and changing
the bound archive hash. Each original FFV1 clip passes hash, dimensions,
timestamp, duration and decode checks. Palette input receipts verify that only
Terithia changes, with the complete initial selection state restored afterward.

The comparison includes sixteen frames extracted from the lossless clips and two
14-second H.264 viewing copies. The
[CPU browser check](../../tmp/animation-end-prepared-retry/browser-checks.json)
loads all eighteen images and plays both videos at desktop and phone sizes,
without overflow or media errors. Frame PTS values are bound to the recorder's X11
Unix start and host-observed native events. The visible stop poses, including the
upright rod and catch/line motion, corroborate the distinct stop stage. Samples
labelled 0–4 are selected from the corresponding traced stop-index intervals;
timestamp alignment is not a claim of exact whole-screen RGB equality. The
existing exact source/atlas checks remain separate from screen lighting and
compositing. Both viewing copies preserve the native timing.

The console briefly covers the earliest start poses; later start, loop, distinct
stop and completed hold are visible. Exhaustive visibility of every start pose
is not claimed. Native completion holds the first start frame and does not
automatically return to idle. The owner's assessment below accepts the displayed
result; passing numerical checks alone does not supply art acceptance.

## Owner feedback

After being shown the paired comparison and asked whether the stop transition
looked right, the owner replied **"looks good to me"**. This accepts the displayed
Vanilla/Debug Blue Summer east-facing stop transition, including its presented
start/loop/stop/hold comparison. Inspection depth is unspecified; acceptance of
every video frame, unsampled artwork or ordinary gameplay reachability is not
inferred. The original live criterion and other release gates are not waived.

The accepted self-contained review retains SHA256
`c1cdfa672f1a7ffbf9d5176991e7a0e50a80cc6c17c8fd90e468998701fe0369`.
The media and technical receipts remain unchanged; their pending-assessment
fields describe the capture-time state before this later feedback. Recording
this decision uses CPU only, with no new GPU work or local inference.

## Preservation, review and retained artifacts

The [final preservation check](../../tmp/animation-end-prepared-retry/preservation-after.json)
passes all 1,182 production inputs, eight seed/config files, 31 historical
receipts, 651,800 local fixture files, three additional evidence files and
356 package files. Original source and lab archive SHA256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`,
with absent recovery state. Instrumented archive SHA256 remains
`fd710ea4c99255b0bec865183f2f2d2bbc0bafc25078af22a690e98acc70c337`.

The unchanged probe retains its prior 37 CPU controls, complete native-script
compile check and independent Breaker review within their recorded limits.
Those tests were not rerun into historical receipt paths. This follow-up adds
the command regression checks, actual native/recording verification, corrupted
packet controls and media checks. No production Rust/Nix behavior changed;
production inputs were freshly preserved. No second automatic review was run
for these follow-up orchestration changes.

All owned game, compositor, recorder, trace watcher and deadline-guard jobs have
ended. The GPU checks retain ordinary desktop/browser usage after shutdown.
Retain the failed preparation state, successful raw lossless clips, trace/input
logs, original recordings and compact review as local reproduction evidence.
The prior instrumented archive is reused without another large copy. New local
retention is approximately 233 MiB, dominated by the three original lossless
clips. No commit, release attachment or publication is part of this check.

The [final check receipt](../../tmp/animation-end-prepared-retry/final-checks.json)
owns the pre-feedback documentation snapshot, media/helper bindings, shutdown
verification and retained sizes. The
[acceptance receipt](../../tmp/animation-end-prepared-retry/acceptance-checks.json)
owns the subsequent documentation checks and binding to the unchanged review.
