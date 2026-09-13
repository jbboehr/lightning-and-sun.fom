# Adeline spring everyday actions

This records the original everyday-action slice. The same profile and preset
set now include [all standard Spring actions](overworld-standard-actions.md),
bringing current coverage to 151 strips. The measurements and playtest below
describe the earlier 143-strip package.

This extends the [spring idle/walk trial](overworld-spring.md) with sitting,
eating, drinking, and blinking. The combined profile has **143 strips**: all 126
portraits and 17 spring overworld strips. F6 keeps Vanilla, Debug Blue, Hayden,
Ryis, and Seridia in sync across the supported animations and portraits.

## Source coverage

The source archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The new files use the prefix
`assets/animations/NPCs/Adeline/Sprites/Spring/spr_npc_adeline_spring_`.

| Action | Source directions | Frames per strip | Duration metadata |
| --- | --- | --- | --- |
| sit | north, south, east | 1 | omitted |
| drink | north, south, east | 3 | 1.0 |
| eat | south, east | 5 | 0.125, 0.15, 0.175, 0.125, 0.6 |
| eat | north | 3 | 1.0 |
| blink | south, east | 3 | 0.075, 0.125, 0.075 |

These **11 strips / 31 frames** are 80×80 per frame on the Default atlas, with
Middle/54 origin. West uses the east strip mirrored; there is no north blink
strip. The game marks sit/eat/drink as seated and gives eat/drink a randomized
last-frame hold of 240–360. Packaging preserves each source's metadata, including
per-frame duration arrays. The runtime retains the game's original animation
pack, so the long holds and loop counters remain owned by the game.

No production GML changed. This is a data and allowlist extension of the existing
renderer hook. The packager rejects unreviewed actions and invented directional
strips, including `sleep_east`, `sit_west`, and `blink_north`.

## Masks and palette definitions

- Profile: `palettes/profiles/adeline-world-actions.json`
- Five-choice set: `palettes/sets/adeline-world-actions-trial.json`
- Blue-only recipe: `palettes/stylized/adeline-world-actions.json`

The source and target ramps match the preceding world trial. Its 132 regions are
preserved exactly. The extension adds 127 component seeds bound to source hashes
and dimensions. It recolors 1,163 new skin pixels per preset. The combined set
changes 423,548 pixels per preset, or **1,694,192** across the four recolors.

The masks include raised hands, seated arms, the small hand patches seen from
behind, and the disconnected far-hand pixel in east blinking. South eating frame
1 lowers a hand to y=49; that patch is skin, while the neighboring trouser and
cape details remain original. Gold edging and belt details reuse skin shades and
remain excluded. Hair, eyes, hood, clothes, and the open mouth's red colors are
unchanged. The game renders food, plates, and drinks separately; these assets are
not included in the recolor recipe.

All 31 source frames were inspected across all five choices. Local contact
sheets are `tmp/actions-{blink,sit,eat,drink}-{south,east,north,west}-presets.png`
(except north blink). The source set is `extracted/adeline-world-actions-study`;
the generated bundle is `generated/adeline-world-actions-trial`. All 1,056 prior
variant PNG/metadata files match the preceding bundle byte for byte; the comparison
is recorded in `tmp/actions-previous-comparison.txt`. Game files, previews,
packages, launchers, logs, and saves remain ignored by Git.

## Verification

The expanded export-bound test failed at 143 inputs against the previous limit;
the action package test failed on unsupported `blink_east`. Both passed after the
allowlist/bound extension. The package fixture checks every added direction,
per-frame timing, idle defaults, origin, fresh IDs, unchanged PNG bytes, and the
complete generated table. It also rejects unreviewed actions before output.

The local corpus test checks all 31 frames, head and raised-hand colors, reviewed
hand landmarks, alpha, metadata, preserved clothing, and other source colors.
Removing rear-facing right-hand seeds caused it to fail on `drink_north [45,45]`;
the mutation log is `tmp/actions-missing-hand.log`.

All **572 generated variants** passed exact recipe validation; reports are
`tmp/actions-<id>-validation.json`. A real MOMI install/uninstall roundtrip in
`tmp/cli-installer-lab` verified all 143 originals and 572 variants and restored
the prior probe-mod archive exactly:
`7a6f08bf68c931efbcc374aba941ae0b55d9d2f627cb1d7e50efeb85432010ca`.
The pristine backup still matches the source archive. Reports are
`tmp/actions-roundtrip-{install,uninstall}.json`.

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test world_actions --test overworld -- --ignored --nocapture'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored --nocapture'
```

Formatting, Clippy, **69 active tests**, the release build, both local overworld
corpus tests, and both GML tests passed. The final log is
`tmp/actions-final-checks.log`. The earlier portrait corpus and legacy MOMI opt-in
tests were not repeated; prior generated outputs were compared instead.

## Isolated playtest

Close the preceding trial and run **`./tmp/play-world-actions`** from a normal
NixOS Wayland desktop terminal. It uses the desktop GPU and separate assets and
save/config state in `tmp/world-actions-playtest`; the original game stays mounted
read-only. If Nix has collected the launcher, rebuild it with:

```sh
nix build --impure --expr 'import ./tmp/world-actions-playtest-launch.nix' --out-link tmp/play-world-actions
```

- F1 spawns or replaces Adeline in her spring outfit beside the player.
- F2 cycles facing, skipping north while blinking.
- F3 cycles idle → walk → sit → eat → drink → blink. A long pause during eating or
  drinking is the game's normal last-frame hold.
- F7 opens the portrait gallery; F6 switches both renderers through five choices.
- F11 cycles all palettes and checks unchanged position, facing, animation state,
  loop counter, animation pack, and food/drink renderer references.

These extra controls belong to the ignored helper; the palette mod adds only F6.
The helper calls the real NPC animation/facing setters, including the game's
food/drink setup. It resets the frame when changing actions to make the preview
repeatable. Walking uses a small deterministic movement trace.

The engine run observed all **315 frame/direction/palette combinations** across
the six supported actions, including mirrored west, and passed **120** full
palette cycles that preserved animation and renderer state. Five actor
replacements were included. The independently enumerated expected and observed
sets matched; lists are `tmp/actions-{expected,observed,missing}-frames.txt`.
The game exited gracefully. Apart from unavailable Steam initialization and the
auto-start room's missing entry transition, there were no engine warnings,
palette assertions, or texture errors. Logs and screenshots are in
`tmp/world-actions-engine/state/`. This automated run used Xvfb and software Mesa;
the desktop launcher uses the host GPU. The launcher built through Nix and passed
a shell syntax check.

## Preview scheduler crash

The initial engine matrix opened the portrait dialog, which paused the NPC
scheduler. Testing with that dialog closed exposed a preview-helper bug: the
manually relocated Adeline could resume her town schedule. The game then failed
to construct a path from `player_home::172x145` to `town::1432x1740`.
`brain_dead = true` was insufficient because `npcs_on_step()` can clear it when
an activity or schedule needs the brain. The crash was reproduced by pressing
only F1 and leaving the dialog closed; F3 was not required. The failing trace is
`tmp/actions-unpaused-red-game.log`.

The local helper now sets `NPC_WHITELIST[NpcId.Adeline] = false` before relocating
and spawning her. This debug control excludes that preview NPC from autonomous
step, room-start, day-start, and time-jump scheduling; manual spawning and the
actor's animation still work. The desktop and automated action launchers enable
`DEBUG_TOOLS`, which is required for this control. The change is in the ignored
action playtest helper and its rebuilt local archive. The shipped palette hook
still follows normal game-owned NPCs without changing their scheduling.

The same F1-only regression then passed for 45 seconds with the dialog closed.
A second unpaused engine run passed 23 action changes, 23 complete palette cycles
with state assertions, facing changes, and five actor replacements. Both runs
exited gracefully without script errors. Evidence is in
`tmp/actions-unpaused-green-game.log` and `tmp/actions-unpaused-controls/state/`;
the corresponding drivers are `tmp/actions-unpaused-session.sh` and
`tmp/actions-unpaused-controls-session.sh`. Formatting, Clippy, all 69 active
tests, and the release build passed again in `tmp/actions-scheduler-fix-checks.log`.
The user retested the rebuilt desktop trial and reported that it looked okay.

## Remaining work

Natural schedules, seating at real furniture, room transitions, and story scenes
remain unverified. Other overworld outfits and unreviewed spring actions retain
the original colors. The user accepted the added actions in the isolated preview.

The subsequent [standard-action slice](overworld-standard-actions.md) adds the
eight remaining standard Spring strips: three general-action directions,
shocked start/loop/end, sleep, and kiss. Seventeen special-animation strips remain
(writing, reading, working, thinking, finger snapping, and fainting).
Other outfits follow a complete Spring review.
