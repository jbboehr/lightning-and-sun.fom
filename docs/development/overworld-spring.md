# Adeline spring overworld trial

This extends the [complete portrait trial](wedding-portraits.md) with spring idle
and walk animations. The combined set contains 132 strips: the existing 126
portraits and six overworld strips. Vanilla, Debug Blue, Hayden, Ryis, and Seridia
remain the five choices. F6 selects one palette for both renderers, with Vanilla
at launch. Other overworld actions, outfits, and characters remain original.

## Sources and masks

The reviewed archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The six sources are under `assets/animations/NPCs/Adeline/Sprites/Spring/`, named
`spr_npc_adeline_spring_{idle,walk}_{north,south,east}`. The game mirrors east to
face west. Each frame is 80×80; idle has one frame and walk has four, for 15 source
frames. Walk duration is 0.15. All six belong to the **Default** atlas, with
horizontal offset Middle and vertical offset 54. The idle metadata omits
`frame_len`; packaging and installed verification now honor the engine's default
of one while continuing to reject explicit invalid values.

The new definitions are:

- `palettes/profiles/adeline-world-trial.json`
- `palettes/sets/adeline-world-trial.json`
- `palettes/stylized/adeline-world-trial.json` (blue only)

Overworld skin uses E9A980, DE8F5D, BA6A4C, and 7D3B14, distinct from the portrait
ramp E3A17B, D48363, C47054, and 9F5544. Each preset maps both ramps to the same four
target shades, in the same order. The first 126 region definitions equal the
preceding profile, and every preceding portrait variant PNG remains byte-for-byte
identical across all four recolors.

Each new region is bound to its source PNG hash and dimensions. Component seeds
select the face, neck, and exposed arms. Gold belt and cape trim reuse some of
these colors and remain excluded, along with boots and trouser details. The east
idle pose and walk frames zero and two include a disconnected one-pixel far arm.
Hair, eyes, hood, clothing, and other colors remain unchanged. The six strips
change **557 pixels per recolor**; the combined profile changes 422,385 per recolor,
or 1,689,540 across all four.

All 15 source frames were inspected across all five choices, including mirrored
west previews. Contact sheets are ignored local files at
`tmp/world-{idle,walk}-{north,south,east,west}-presets.png`. Originals are in
`extracted/adeline-world-trial`; the bundle is `generated/adeline-world-trial`.
Only code, recipes, seeds, hashes, tests, and documentation belong in Git.

## Runtime behavior

The game's `NpcAnimationHandler` keeps its original sprite and animation data in
`current`; `par_NPC.animate()` advances it, then obtains the render sprite through
`animator.sprite()`. The palette mod wraps that getter on Adeline instances. It
maps a supported original to the selected variant without replacing animation
packs or changing frame count, phase, direction, or movement. Unsupported sprites
pass through the original getter. F6 also refreshes `sprite_index` directly so a
paused actor can change palette without calling `animate()`.

The installed MMAPI repeatedly runs `mmapi_register` callbacks at begin-step. The
initializer uses this to attach once to each new/replaced Adeline animator. An
actor created after that callback attaches on the next begin-step. The mod uses
no NPC creation hook; the pinned installed MMAPI does not provide the creation
hook found in the separate newer source checkout. A missing asset disables
palette switching before the world getter is attached or the hotkey registered;
the original render assets remain in use.

MOMI verification exposed an existing source detail: three fully transparent
pixels in `idle_east` have different hidden RGB in its loose PNG and pristine
atlas reconstruction. The installed verifier now permits RGB differences only
when both alpha values are zero. Alpha and all visible RGBA values remain exact;
loose original PNG bytes, metadata, geometry, atlas membership, and variant
properties still receive their existing checks. The regression fixture first
failed on invisible RGB, then passed with the adjustment; changing alpha still
fails without publishing the archive.

## Verification

Focused tests cover six-strip packaging, absent idle frame counts, walk timing,
origin preservation, mixed portrait/world installation, wrong atlas/folder/pixel
rejection, and transparent RGB handling. Fabricator executes the shipped GML with
new/replaced actors, palette selection before spawning, idempotent attachment,
fractional phase, mirrored facing, unsupported actions, and missing assets.
The local corpus test checks all 15 source frames, reviewed head/arm landmarks,
clothing exclusions, alpha, metadata, and unchanged non-source colors.
A deliberate mutation removing the arm seeds failed at the east idle hand
`[35,44]`, confirming that a face-only mask cannot pass this test. Its log is
`tmp/world-missing-hands.log`.

All **528 generated variants** passed exact recipe validation. Reports are
`tmp/world-<id>-validation.json`. A real MOMI install/uninstall roundtrip in
`tmp/cli-installer-lab` verified the 132 originals and 528 variants in their source
atlases and restored the existing probe-mod archive exactly:
`7a6f08bf68c931efbcc374aba941ae0b55d9d2f627cb1d7e50efeb85432010ca`.
Its pristine backup still matches the source archive. Reports are
`tmp/world-roundtrip-{install,uninstall}.json`.

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test overworld -- --ignored --nocapture'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored --nocapture'
```

Final verification passed: formatting, Clippy, **68 active tests**, the release
build, the local overworld corpus test, and both GML tests. The log is
`tmp/world-final-checks.log`. The remaining opt-in corpus and legacy MOMI fixtures
were not rerun.

Reliability verdict: **PASS_WITH_RESIDUAL_RISK**. Independent Breaker and Test
Attacker passes found no demonstrated defects or accepted static findings. Test
hardening checks the exact six-strip extension without altering prior portrait
regions, rejects malformed explicit frame counts, verifies omitted counts through
installation, and exercises replacement of an animator on an existing actor.
The concrete unverified areas are listed at the end of this document.

## Isolated playtest

Close any older trial, then run **`./tmp/play-world`** from the normal NixOS Wayland
desktop terminal. It uses the desktop GPU, a separate archive and save/config
state at `tmp/world-playtest`, and a read-only mount of the original game.

- F1 spawns or replaces Adeline beside the player in her spring outfit.
- F2 cycles south, east, north, and west.
- F3 switches idle/walk. The helper moves her a short distance while walking.
- F7 opens the portrait gallery; F6 cycles both renderers through the five choices.
- F11 cycles all choices, asserts unchanged world position, facing and animation
  phase, and returns to the starting palette.
- F5 cycles portrait outfits; F4 switches wedding/spring. Those controls can
  switch her overworld outfit too; use F1 to restore the supported spring trial.

These extra controls are an ignored development helper; the palette mod adds
only F6. The helper creates a real Adeline actor through `spawn_npc`, uses the
game's facing and animation setters, and drives short deterministic movement.
It does not exercise natural itinerary/pathfinding behavior.

The engine run observed all **100 direction/preset/frame combinations** and
passed **45** full palette cycles that checked world position, facing, and phase,
including five actor replacements. Independent expected/observed lists and an
empty missing list are in `tmp/world-{expected,observed,missing}-frames.txt`.
The log and screenshots are under `tmp/world-engine/state/`. The run used Xvfb
and software Mesa. The engine reported unavailable Steam initialization and the
auto-start room's missing entry transition; no palette assertions or texture
errors occurred.

Normal schedules, natural room transitions, event scenes, and interactions with
other mods that wrap the same animator getter remain unverified. Unsupported
world actions/outfits deliberately retain their original colors. The previous
portrait expression matrix was not repeated; its generated outputs were compared
byte for byte. The user approved the appearance in the isolated trial on
2026-09-07.
