# Live Linux release verification

2026-11-06. **Live controls/persistence/basic-gameplay batch verified and accepted as displayed, with qualifications.**
The user's qualified "looks okay to me" assessment closes the remaining
displayed-world-art gate; the [CPU release record](linux-release-candidate-2026-11-06.md)
owns its scope, snapshots and alpha milestone. The approved finish plan now
requires a live matrix for the usable Linux release.

## Current result and limits

| Area | Exercised evidence |
| --- | --- |
| Hardware rendering | Original and final candidate on Radeon RX 9070 XT/Mesa 26.1.8 |
| Native controls | Title/journal, all 36 rows, five-choice cycle, mouse/wheel, keyboard and SDL virtual gamepad |
| Persistence | Default Off/no writes, opt-in, reset, opt-out, fresh restarts, read-only failures and visible warnings |
| Gameplay | Normal character setup, dialogue, movement, door round trip, sleep/day transition, actual save and fresh continuation |
| Art preservation | All 25,248 actual variant PNG hashes match accepted content |
| Displayed live review | Qualified user acceptance; inspection depth unspecified |
| Remaining acceptance | Physical controller and broader seasonal/action coverage; no exhaustive playthrough claimed |

## Boundaries

Use only supplied `tmp/fom` executables, disposable archives/config/saves and
the current packaged candidate. Keep real game installations, saves, Steam
libraries and user settings outside the game sandbox. Check `gpu-status`
immediately before rendering and hold `gpu-run` throughout each workload.
Do not alter production scripts merely to make a probe pass. Lab instrumentation
must remain local and identify actual input events separately from direct
callback/API dispatch.

## Initial evidence and route

The prior Xvfb/llvmpipe session produced black captures for both original and
modded archives, despite reaching the main loop and passing native API/storage
probes. It hid GPU render nodes and cannot establish hardware rendering. Several
old tool/driver store paths are no longer present; rebuild the existing graphics
tool closure with the pinned Nix inputs rather than reusing stale paths.

`gpu-status` confirms AMD Radeon RX 9070 XT visibility with normal desktop
workloads. The sandbox exposes render nodes and a host Wayland socket, but no
host X11 socket or input/uinput/uhid devices. A private GPU-backed compositor
and isolated game/state mounts are the preferred diagnostic; native Wayland
is an alternative if the runner supports it.

## Required live matrix

- Establish a visible hardware-rendered original title before attributing any
  modded rendering problem to palette code. Verify the renderer identity.
- Render the native Settings/Palettes page and final five-choice collection;
  check all 36 rows, bottom entries, scrolling, wrapping, notifications and
  page/category/world focus, close and reopen behavior.
- Drive actual keyboard/mouse and controller input paths. Default activation is
  E/south-face; Esc/east-face backs out. Separate virtual-controller coverage
  from physical hardware acceptance and direct callback invocation.
- Verify remembering/reset/restart through normal UI actions and actual
  mod-owned config bytes, including Off defaults and failure feedback.
- Exercise dialogue, movement, room transitions, portraits and world animation,
  native mirrors, representative outfits/actions and palette changes during play.
  Record precisely which cases run live rather than claiming an exhaustive playthrough.

The reproduction helpers and observations live under
`local/authoring/live-release/` and `tmp/live-release/`. Historical galleries,
accepted fixtures, CPU check packets and preserved source archives remain intact.

## Hardware baseline and first candidate run

The private Gamescope/Xwayland route renders both the supplied original archive
and the full installed candidate. The game reports AMD Radeon RX 9070 XT,
radeonsi/gfx1201, OpenGL 4.6 and Mesa 26.1.8; Gamescope uses RADV on that GPU.
Explicit Mesa driver discovery in the lab compositor resolved the initial GLX
failure. These are lab launcher changes, not game or palette changes.

Original title → Settings and Escape back work through actual OS mouse/keyboard
events. The full candidate title and a new normal world also render; OS keyboard
navigation opens native Settings. The new world uses the game's `--auto-start new`,
`--all-unlocks=true`, `--story-events=false`, `--debug-tools=true` setup arguments.
This establishes rendering and native input delivery, not an exhaustive natural
playthrough. Steam and audio hardware remain deliberately absent.

All 25,248 candidate variant hashes match the prior CPU packet. Evidence is
`tmp/live-release/full-install-binding.json`; raw baseline captures and events are
in `baseline-gpu-driver/`, with candidate title/settings in `full-menu/` and the
initial world in `full-game/`. Each workload held a foreground GPU reservation
and was stopped before archive changes.

The first actual world reveals a release blocker: initialization logs
**Palette study disabled: a world object is missing.** Native `NpcId.gml` maps
32 regular characters but excludes Darren, Linnet, Wiscar and Wynne. Their cameo
actor system requires explicit integration; the earlier interpreter object stub
masked this difference. The failure currently prevents the palette page from
appearing. A source-backed fix and failing regression are in progress. Fresh
launch title initialization is separately being checked against MMAPI's lifecycle.
No palette-menu/persistence/gameplay pass is claimed from this failed run.

The title issue is independently reproduced: the new
`title_settings_initializes_palettes_before_game_begin_step` regression fails
before the change. MMAPI's `Game.gml` begin-step seam is the only installer drain;
its events module explicitly notes that Game never steps in the title room.
Native Setup loads/saves settings, constructs Anchor and initializes NPC registries
before spawning Title. Config helpers do not enforce the queue's IO-ready marker.
The existing Settings-open hook now lazily initializes only this palette mod;
it does not tick other mods or change framework flags. All three focused controls
interpreter cases pass afterward, including fresh title save/restore, later
installer idempotence, and the retained unavailable-runtime guard. Actual title
IO/render verification is still required after the cameo correction is integrated.

## Controller probe baseline

A lab-only SDL preload shim attaches a virtual gamepad to the game's existing
bundled SDL 3.4.8 runtime. Its descriptor uses the exact official release headers;
CPU tests cover all 15 buttons/30 DOWN-UP events, polling, stale/invalid commands,
detachment and dormant behavior without the explicit private input path. No host
input device, GML callback or palette state is manipulated by the shim.

In the original hardware-rendered game, D-pad/south opens native title Settings
and east backs to title; the game displays controller hints. Actual applied
button sequences and raw captures are retained in `tmp/live-release/baseline-pad/`.
This establishes the virtual-controller route before candidate tests; it does not
verify a physical controller, stick axes, rumble or every hardware mapping.

## Corrected candidate

The cameo correction uses native `try_string_to_cameo_id` and
`cameo_id_to_gm_obj_id` after the regular NPC lookup. All four cameo objects use
the same native `NpcAnimationHandler`; the existing sprite wrapper therefore
retains their frame/phase/position/facing and naturally attaches to new cutscene
instances. Strict unknown-object and missing-asset refusal remains. The faithful
cameo regression fails without the fix and passes afterward; four runtime GML,
three controls GML, 29 installer and two toggle-package cases pass. Formatting,
all-target Clippy with denied warnings, and scoped whitespace checks pass.

The rebuilt isolated Nix output is
`/nix/store/fl24syis972jjq7hfz4h1fzwyw6ky0fv-mistria-palette-0.1.0`,
rooted at `tmp/live-release/nix-package`. Its normal test phase passes 451 cases
with 510 opt-in cases ignored. This includes the two new opt-in regression cases;
they were separately executed above. Production changes are confined to the two
GML initialization fixes. Art, registries, recipes and accepted fixtures remain
unchanged. The corrected full collection passes strict compile/install/atlas/metadata/script
verification. All 25,248 actual variant hashes match the accepted CPU images.
Installed archive SHA256 is `676fb6c34aaf86fec03f9c91caa4ce2ef01513b0bdfa6bc9385d8ed045144449`;
`tmp/live-release/corrected-install-binding.json` binds the full install report.
Live checks now use this archive.

## Corrected title UI and first persistence phase

`title-fixed/` records actual keyboard/mouse and SDL virtual-gamepad events.
The fresh title exposes the eighth Palettes category. All 36 rows are reached and
cycled once using OS keys; all five Adeline labels are separately captured.
Mouse selection changes only Adeline, confirmed by persisted stable ID
`npc_hayden`, while the other 35 stay `blue`. Wheel scrolling, directional
bottom-to-top wrap, the bottom status row and native category focus are visible.
Keyboard and virtual gamepad both activate/back normally; point-control Escape
closes Settings directly, matching the native path, while directional east backs
from page to category before closing. Reopening retains session choices.

Before opting in, all changes leave `lns_palette.json` absent. Actual gamepad
Down/south enables Remember choices; the resulting version-1 file has precisely
36 known stable character IDs, all `blue`, and `remember=true`. The default and
wrapped multi-line status are legible. The game exits through its native Exit
confirmation with code 0. A new isolated process copies only this lab's own
config/data to test restart; no real user configuration or saves are introduced.

## Restart and title failure follow-up

`title-restored/` launches a new process with only the preceding disposable
config/data. Actual virtual-gamepad input opens Settings/Palettes; all 36 restored
Blue choices are logged and the rendered top rows agree. The primary config is
byte-identical to the preceding native exit.

Making only this copied mod directory/files read-only then selecting the first
row reproduces a further release defect: title has no `Menu.InfoToasts` instance,
so the notification call raises **bad object undefined** and leaves the UI frozen.
The error is in `palette_controls.gml` notification dispatch; this is not a
successful failure-feedback test. The primary file remained byte-identical.
The lab was stopped and its permissions restored. A bounded native title-toast
correction and faithful missing-menu regression are in progress; no gameplay or
release acceptance is inferred from the frozen frame.

## Title notification correction and live failure/reset results

The missing native toast menu regression reproduces the exact bad-object error
before the fix and passes after it. A first title notice lazily spawns native
InfoToasts and ties that instance to the current Settings' preserved `on_close`;
existing gameplay toasts are reused without taking ownership. Native Setup
already drives Anchor/CHAINS and this constructor needs no Game/player. Four
controls and four runtime interpreter cases, 29 installer and two toggle-package
cases pass. Source/evidence pins live in
`local/authoring/live-release/title-notice-source/evidence.json`.

To overlap GPU verification with the final Nix rebuild/install, a separate
lab-only archive replaces exactly `palette_controls.gml` with its production
bytes. All other 148,602 members are byte-identical to the strict-installed
candidate, verified by SHA256; installer lab and receipts stay untouched.
`tmp/live-release/toast-source/binding.json` owns this derivation. This is a
bounded test archive, not a substitute for final packaged verification.

`title-notices/` now shows immediate wrapped native warnings for a failed top-row
choice and failed opt-out, keeps focus/scroll/input and session choices usable,
and leaves the previous primary config byte-identical. After restoring only the
lab permissions, remembering and Reset all successfully save all 36 Vanilla IDs.
Selecting Adeline Blue then opting out writes `remember=false` while retaining
this session's Blue. French language refresh retains raw palette labels and the
selection; native English is restored afterward. The game exits normally with
code 0, exercising the Settings-owned toast close callback without errors.
The next process tests the saved Off state in a debug-created normal world.

## Final packaged gameplay and save continuation

The final package is
`/nix/store/klgh5dg7f22bfvr2wn181wcg626gjmk7-mistria-palette-0.1.0`,
rooted at `tmp/live-release/final-nix-package`. Its isolated test phase passes
451 regular cases with 511 opt-in cases ignored; four controls/four runtime
interpreter cases and focused installer/package cases execute separately above.
All-target Clippy with denied warnings passes again. Full strict install passes;
all 25,248 accepted PNG hashes agree and both production GML modules are exact.
Installed archive SHA256 is
`d7c9468ad61d60e82dd8f13026395ac86523f5f1e9d3b04d663c1b9788c998c3`.
Compared with the overlapped test archive, 25,276 metadata members differ only in
generated asset IDs and their references; every parsed metadata structure agrees
after the complete old-to-new ID mapping. Other member CRC/size headers agree.
`tmp/live-release/final-install-binding.json` owns that comparison.

`final-package/` uses this final installed archive. The normal story flow runs
with `--all-unlocks=false --debug-tools=false`; actual input advances dialogue,
uses native cutscene skip, accepts default character setup, walks indoors and
outdoors, exits to the farm and returns through the door. Live Vanilla portraits
include Eiland, Adeline, Balor and Celine. The earlier no-story/full-unlocks probe
is not counted as a successful player-movement check; the normal flow supplies
the decisive evidence. Virtual D-pad movement/jump and keyboard movement work.

The final journal page cycles all 36 Blue selections and saves remembering On.
A read-only top-row save failure uses the existing gameplay toast, preserves
prior config bytes, keeps selection usable and allows normal play after closing
the journal. Permissions are restored. Native bed/sleep/next-day actions advance
to Spring 2 and create the actual 110,336-byte
`game-1453366268-autosave.sav`, SHA256
`156c17f349ea5a8ae04e130507186dbc3ef7864fc0b211ec5b9b8dd28bd02b95`.
Native Exit to Desktop returns code 0. `final-continued/` copies only this lab's
data, starts with `--auto-start continue`, loads that exact save on Spring 2 and
restores all 36 Blue palette choices before UI actions. Its native desktop exit
also returns code 0. `--auto-start continue` re-enters that save after returning
to title, so it is deliberately omitted from the final normal-title story replay.

## Scope of the live evidence

These are keyboard/mouse events and SDL virtual-gamepad events through native
menus and gameplay, rather than direct callback assertions. Physical controller,
stick/rumble/device mappings, Steam/Cloud integration and an exhaustive seasonal,
outfit, action or story playthrough remain unverified. Full native frame/mirror
coverage and unchanged spectral assets retain their existing offline evidence;
only explicitly listed scenes are claimed live. The displayed live review has
qualified user acceptance below. The usable Linux release still needs the
remaining P6 cases/limits to be resolved or explicitly accepted. The owner's licensing choice precedes public
distribution; no publishing is authorized.

## Remembered portraits, packaging parity and handoff

The final normal-title replay (`final-story/`, no auto-start/debug setup flags)
uses the private saved config and selects New Game through the native title UI.
Eiland and Adeline render with Blue skin in both dialogue portraits and world
actors, preserving hair/clothing. Their corresponding Vanilla prologue captures
are in `final-package/`; this binds the same story path to actual final atlas
rendering. The replay was intentionally stopped after these captures.

The repository's documented `nix build .#` route also builds successfully at
`tmp/live-release/flake-package`, output
`/nix/store/a31xh6rh78aab7ypx65vwv1rvpdcvczp-mistria-palette-0.1.0`.
All 282 output files, including the executable and every recipe, are byte-identical
to the live-tested package. The derivation differs only because a test-fixture
comment changed while the first build ran. Both source sets have 1,092 files;
`flake-package-binding.json` records exact parity. New Nix files and the cameo
fixture had intent-to-add entries for local flake discovery at the verification
handoff; the index then contained no staged content. That checkpoint preceded
the user's subsequent approval and commit request below.

Final packaged removal succeeds and independently restores archive SHA256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`;
the recovery directory is absent. All private GPU sessions are stopped. Raw
captures/events/config/save pins remain under `tmp/live-release/`. The current
portable review is `tmp/live-release/review.zip`; its HTML preserves the dated
verification record as it stood before user acceptance. The source-only candidate, per-file manifest/checksums and
standalone build evidence are also under `tmp/live-release/`, distinct from the
preserved earlier CPU packet under `tmp/release-checks/`.

The private derived replay archive is retained at `tmp/live-release/toast-source/`
(about 687 MiB) for reproducibility until review. Other live captures/logs/private
state total about 181 MiB before the portable packet/source fixture. The prior
92 MiB standalone CPU fixture is retained as already recorded. These are local
research inputs/evidence and are excluded from the source-only deliverable.

## Qualified displayed-live acceptance, 2026-11-06

The user reviewed the consolidated live packet and replied **"looks pretty good.
commit please"**. Record qualified acceptance of the displayed native controls,
persistence/failure feedback and basic gameplay captures. Inspection depth is
unspecified; this does not extend acceptance to physical controllers or the
broader seasonal/outfit/action matrix. Earlier art qualifications remain in force.

The reviewed artifact is [the portable live review](../../tmp/live-release/review.zip),
containing 44 raw screenshots and 47 ZIP entries, SHA256
`60c76538bbeefd74307039e5bb91a5af86a2d46a854cb27b99539e236c88f05d`.
Its HTML record, manifests, check packet and source-only candidate remain frozen
at the pre-approval verification checkpoint. This section records the later
acceptance without regenerating those artifacts. The commit request authorizes
the local packaging/runtime/documentation commit; public distribution remains
separate.
