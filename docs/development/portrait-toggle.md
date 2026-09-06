# One-portrait runtime toggle

This document records the original one-portrait slice. The later
[spring coverage slice](spring-portraits.md) extends packaging, installation,
verification, and runtime selection to a recipe-defined set of up to 25 expressions.

Rust prepares a blue variant from the player's local Adeline spring neutral
portrait. MOMI installs it as a new animation in `PortraitsSpring`; it does not
replace the original animation. GML selects the installed sprite during dialogue.
No Rust process or image processing runs inside the game.

The package contains a manifest, one original GML script, and a locally generated
PNG/metadata pair. Only code, recipes, and synthetic fixtures belong in Git or a
public source release. The generated package contains game-derived art and stays
local. This packaging choice is not a general copyright determination.

## Behavior and boundary

F6 toggles vanilla/blue for the current session. Vanilla is the initial state.
Only `spr_portrait_adeline_spring_neutral` and the added
`spr_lns_adeline_spring_neutral_blue` are eligible. Other expressions, outfits,
characters, overworld sprites, and secondary cutscene portraits are outside this
slice. There is no saved selection, preset UI, graphical installer, or Workshop
integration yet.

The `ui.menu_opened` MMAPI hook wraps each textbox's `set_speaker` method once,
calls the original method first, and applies the selected sprite immediately.
Swapping preserves the node's fractional animation index because Anchor's
`set_sprite` resets it. F6 also updates an already open textbox. Missing portrait
assets disable the toggle with one warning.

The Rust packager validates the one-portrait input and horizontal frame strip,
preserves all asset properties, and omits the original ID. MOMI assigns a fresh
ID. Reusing the original ID would overwrite vanilla. The script resolves names
at runtime, so it does not depend on MOMI's generated ID.

## Generate and install locally

Build the binary and follow [TOOLS.md](../TOOLS.md) to export and recolor the
portrait. Use `package-toggle` with a fresh output:

```sh
target/release/mistria-palette package-toggle \
  --original extracted/adeline-rust \
  --modified generated/adeline-rust-stylized \
  --output generated/momi-adeline-toggle \
  > generated/momi-toggle-report.json
```

Use the [isolated MOMI procedure](momi-lab.md), replacing every `tmp/momi-lab`
reference with a fresh `tmp/toggle-lab`. Copy `generated/momi-adeline-toggle` to
`tmp/toggle-lab/mods/lns_palette` instead of the replacement study. Keep the folder
name `lns_palette` so MOMI's namespace linter recognizes the GML functions. Run
`run_momi_lab --strict-lints --fail-on-skip` with the adjusted sandbox. Do not
install both studies: the older replacement changes the base portrait itself.

MOMI must report one installed mod, a packed new animation, and passing compile
checks. The compile checker accepts late-bound names; its success alone does not
establish that the script can run in Mistria.

Before uninstalling, check the installed atlas:

```sh
nix-shell --pure --run 'cargo test --locked --test momi_install installed_toggle -- --ignored --nocapture'
```

This local test expects the `*-rust` inputs and package path above. It reconstructs
both vanilla and both blue frames from the installed atlas, compares exact RGBA,
checks source bytes and animation properties, and checks existing placements.
The older replacement test requires its own separately installed lab.

To restore the lab, use the same adjusted sandbox with `--uninstall` and compare
both `assets.zip` and `assets.bak.zip` with the untouched source archive. Merely
removing a mod directory does not update the installed archive.

## Executable script tests

Normal Cargo tests use synthetic assets. The optional GML lifecycle test runs the
production script on Fabricator with small engine/MMAPI boundary stubs. Build the
interpreter at the revision identified in this game's embedded paths and used by
the inspected MOMI compiler:

```sh
git clone https://github.com/kyren/fabricator tmp/runtime-capabilities/fabricator
git -C tmp/runtime-capabilities/fabricator checkout f2483a61d53c730020f2357c790603dd6362476b
nix develop --command cargo build --locked \
  --manifest-path tmp/runtime-capabilities/fabricator/Cargo.toml \
  -p fabricator-cli --bin interpreter
FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" \
  nix develop --command cargo test --locked --test gml_runtime -- --ignored
```

Skip cloning if that pinned checkout already exists. Tests exercise initialization,
vanilla defaults, F6 in both directions, fractional frame preservation, later
speaker changes, unrelated portraits, closing/reopening menus, and missing assets.
A temporary mutation removing the index restoration made the frame-preservation
assertion fail. Independent review also identified that Anchor's `get_index()`
returns only the integer frame; tests must model that distinction from the raw
animation phase stored in `index`.

## Real-game smoke procedure

Use a disposable session and the isolated installed archive. The test-only
[`game_smoke.gml`](../../tests/gml/game_smoke.gml) can be copied into the lab mod's
`gml/` directory before a MOMI reinstall. It adds F7 to open a real Adeline textbox
and F8 to play test text; it records observed portrait frames in the game log.
F9 sets the real node to phase 1.25, switches both ways, and asserts that its raw
index remains 1.25 before the next animation tick.
It is not embedded in `package-toggle` and must not be included in a player package.

The local Linux run used `--auto-start new --all-unlocks=true --story-events=false
--debug-tools=true`. These are shipped game options, with saves/config directed to
the lab's new XDG directories. The original game directory was mounted read-only,
with only the lab's installed `assets.zip` overlaid. No Steam libraries or user
saves were visible.

On a private Xvfb display, Mesa software rendering needed
[`SDL_VIDEO_FORCE_EGL=1`](https://wiki.libsdl.org/SDL3/SDL_HINT_VIDEO_FORCE_EGL).
FMOD also needed an ALSA config containing `pcm.!default { type null }`, supplied
through `ALSA_CONFIG_PATH`. The sandbox provided `/bin/sh`, a writable
`/tmp/.X11-unix`, and the required Nix graphics/audio libraries. Xdotool focused
the game and delivered actual F7/F6/F8 key events to that private display.

Check vanilla after opening with F7, blue after F6, and vanilla after another F6.
Use F8 and toggle during the talking animation, then F9 to check fractional phase.
Restart the game and confirm the
initial portrait is vanilla again. Remove the test driver and reinstall before
checking the final deployable archive.

## Evidence and limits

On 2026-09-06, MOMI v0.15.10 accepted the package with strict lints and compile
checks. The installed atlas test passed for all four frames. The game title
reported `v1.0.4 (modified)`. The real textbox displayed vanilla, blue, and restored
vanilla in response to actual F6 events. Screenshots and logs remain under ignored
`tmp/toggle-lab/state/`; generated images remain under ignored `generated/`.
During the talking-text check, the real portrait node visited frames 0 and 1 with
both vanilla and blue selected. A second new-game launch again started vanilla.
After the phase fix, the real-game F9 assertions passed in both directions at
index 1.25, and the game exited normally.
MOMI uninstall restored both the lab archive and its backup byte-for-byte to the
source fingerprint recorded in the investigation.

This is a test-driver conversation, not a playthrough of naturally encountered
dialogue. Other mods, game updates, other platforms, controller bindings, a full
character art pass, and installation UX remain unverified. The blue palette also
changes some shared robe trim colors, as documented in the earlier asset study.

## Review and automated verification

Independent Breaker and Test Attacker passes reproduced one defect: reading the
floored getter reset phase 1.25 to 1. The engine stub now matches that getter, and
the script preserves the raw `node.index` before restoring it through `set_index`.
The same lifecycle regression failed before the fix and passed afterward.
A package test also checks that a second PNG is rejected even when unchanged.

The final normal checks passed with 19 synthetic tests, Clippy, formatting, and
a release build:

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
```

The opt-in Fabricator lifecycle test also passed after the fix. The older
replacement-install test was not rerun in this slice; the new toggle atlas check
exercises the shared frame-reconstruction helper against all four frames.
After removing the smoke driver, the final package was reinstalled with strict
checks and the atlas test passed again. The installed archive contains the
production toggle script and no smoke driver. `tmp/toggle-lab` is left installed;
the source archive and the lab's pristine backup still match the original hash.

Reliability verdict: **PASS_WITH_RESIDUAL_RISK** for this one-portrait prototype.
The demonstrated phase defect is fixed; no additional static defects were
accepted. The unverified areas are listed above.
