# Lightning and Sun

Skin palettes for the 36 supported Fields of Mistria characters, covering their
portraits and recolorable world sprites. Each character offers twelve choices:
Vanilla, Debug Blue, three previous donor palettes and seven additional tones
adapted from the game's NPC and player-creator colors. Clothing,
hair, equipment, markings and intentional spectral forms keep their original
colors.

This is an unpublished Linux candidate. Installation/removal, native menus,
keyboard/mouse, virtual-gamepad, remembered choices and representative gameplay
checks pass. Displayed artwork and live samples have qualified acceptance;
physical-controller, distinct complex-animation end-stage and broader artwork/tone
coverage checks remain open. See the [candidate notes](docs/releases/linux-candidate.md)
for included behavior and tested limits.

## Build and install

Requires **x86_64 Linux**, Nix with flakes enabled, and your own game installation
containing `assets.zip` and `Maybe.toml`. Nix supplies pinned MOMI 0.16.4 and its
runtime dependencies; the host must allow its Bubblewrap sandbox to run.
Run from this repository:

```sh
nix build
result/bin/mistria-palette --help
```

Close the game and other mod installers, then install the complete collection:

```sh
result/bin/mistria-palette install \
  --game-dir '/path/to/Fields of Mistria' \
  --characters result/share/mistria-palette/palettes/sets/characters-trial.json
```

Keep the `result` link while using this build; it retains the packaged tool,
recipes and runtime dependencies. Release contents are the tool, text recipes
and masks, and mod scripts. Neither original game assets nor recolored,
game-derived assets are distributed. Replacement images are generated locally
from your own copy; image galleries, screenshots and clips are also excluded
from the release. Installation verifies generated atlas pixels, metadata and
scripts before publishing the local game archive.

The `--characters` option selects the complete collection. Omitting it runs the
older single-portrait Adeline study. To use a single character instead, supply
its set, for example:

```sh
result/bin/mistria-palette install \
  --game-dir '/path/to/Fields of Mistria' \
  --presets result/share/mistria-palette/palettes/sets/caldarus-native-tones.json
```

Uninstall the current selection before installing another recipe.

## Palette controls

Open **Settings → Palettes**, select a character and cycle their palette.
Portraits and world sprites use the same choice. Each character starts on
**Vanilla** unless you enable **Remember choices**, which starts Off.
**Reset all to Vanilla** resets the included characters. Use the normal menu
back/close controls to leave the page. Default controls use **E** or the
controller south button to select, and **Esc** or the east button to go back.

Remembered choices use mod-owned settings. If saving fails, session choices
remain usable and a warning indicates that persistence was not confirmed.
Disable remembering to start future sessions on Vanilla. Resetting palettes
does not change game saves.

The previous donor choices remain: most characters offer Hayden, Ryis and Seridia
alongside Vanilla and Debug Blue.
Hayden offers Adeline/Ryis/Seridia; Ryis offers Adeline/Hayden/Seridia;
Caldarus and Seridia offer Adeline/Hayden/Ryis. Every character also offers:

- Light peach (Player 04)
- Light golden (March)
- Warm peach (Wiscar)
- Warm brown (Player 16)
- Deep brown (Player 22)
- Very deep (Player 35)
- Deep umber (Player 27)

These tones approximate a
broader native range; they do not reproduce every original skin color exactly.
Natural skin ramps, glow effects,
scars and body markings can make results differ between characters.

## Remove or change the mod

Close the game, then run:

```sh
result/bin/mistria-palette uninstall --game-dir '/path/to/Fields of Mistria'
```

Removal restores the exact archive from before palette installation, including
any earlier compatible mods. Selecting Vanilla restores appearance during play;
uninstalling removes the installed palette package.

Keep the `.mistria-palette/` recovery directory until removal succeeds. If the
live archive changed after installation, removal refuses to overwrite it and
retains recovery files. Restore the matching installed archive or investigate
the other update before retrying; do not delete recovery state to bypass this
check. Remove this palette installation before updating the game or using MOMI
separately.

## Compatibility

This candidate was verified using the supplied Linux executable and the preserved
`assets.zip` with SHA256:

```text
8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4
```

The fingerprint identifies the tested source; installation does not require the
entire archive to match it. Recipes reject changed reviewed source PNGs by
default and retain inventory, dimension, mask and installed-output checks.
Passing these checks on another archive does not establish support for another
game build. No supported game-version range has been established.

The advanced `--allow-source-hash-mismatch` option bypasses only source PNG hash
equality. Other checks remain, but changed artwork can still recolor incorrectly;
inspect the output before relying on it. The override does not establish support
for another game build.

Existing MOMI mods require complete unpacked source folders under `mods/` or
`Mods/`, the matching pristine `assets.bak.zip`, and the installed-mod list MOMI
wrote for that archive. For example, add:

```sh
--installed-mods '/path/to/MOMI-config/mods/manifest.json'
```

Each source mod needs its own manifest directly inside its folder.
The `--installed-mods` file is MOMI's installed-mod list from the game
configuration directory, separate from those per-mod manifests.
Use the list from the last MOMI installation of that archive: it has no archive
checksum, so matching names and versions cannot prove its provenance. If the
correct list is unavailable, restore a known setup through MOMI first.
A modded live archive normally differs from the pristine backup's fingerprint.

Installation rejects missing/mismatched selections, skipped mods or a rebuild
that changes existing mod precedence. Custom order that MOMI's CLI cannot preserve
is refused.
Conflicting sprite/script replacements and arbitrary mod combinations are not
guaranteed compatible. Windows, macOS, ARM installation, portable non-Nix bundles,
archive-packed mod inputs and automatic game-update recovery are outside the
supported route. Verification on one Linux configuration does not establish
support for every distribution or hardware setup.

For exporting artwork, custom palettes and other toolkit commands, see
[the tool guide](docs/TOOLS.md). Earlier individual study recipes remain
available; their coverage is narrower than the complete collection.
