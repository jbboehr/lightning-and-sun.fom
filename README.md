# Lightning and Sun

Skin palettes for the 36 supported Fields of Mistria characters, covering their
portraits and recolorable world sprites. Each character offers twelve choices:
Vanilla, Debug Blue, three previous donor palettes and seven additional tones
adapted from the game's NPC and player-creator colors. Clothing,
hair, equipment, markings and intentional spectral forms keep their original
colors.

This is a Linux release candidate. The complete collection is authored and
passes installation/removal checks. Representative Player 27 portraits have
qualified visual acceptance; broader artwork review remains. Prior displayed
artwork is accepted. Native menus,
keyboard/mouse, virtual gamepad, remembered choices and basic gameplay/save/load
checks pass; physical controller and broader seasonal gameplay acceptance remain.

## Build and install

Requires **x86_64 Linux**, Nix with flakes enabled, and your own game installation
containing `assets.zip` and `Maybe.toml`. Run from this repository:

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
recipes and runtime dependencies. No game files are downloaded or distributed.
Replacement images are generated locally from your copy. Installation verifies
the generated atlas pixels, metadata and scripts before publishing the archive.

The `--characters` option selects the complete collection. Omitting it runs the
older single-portrait Adeline study. To use a single character instead, supply
its set, for example:

```sh
result/bin/mistria-palette install \
  --game-dir '/path/to/Fields of Mistria' \
  --presets result/share/mistria-palette/palettes/sets/caldarus-world-trial.json
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

Recipes check the reviewed source PNG hashes and reject changed source artwork
by default. This does not promise compatibility with every game update. An
advanced `--allow-source-hash-mismatch` option permits changed sources while
retaining structural checks; inspect the output before relying on it.

Existing MOMI mods require complete unpacked source folders under `mods/` or
`Mods/`, the matching pristine `assets.bak.zip`, and the installed-mod list MOMI
wrote for that archive. For example, add:

```sh
--installed-mods '/path/to/MOMI-config/mods/manifest.json'
```

The manifest must sit directly inside each mod folder. Installation rejects
missing/mismatched selections, skipped mods or a rebuild that changes existing
mod precedence. Custom order that MOMI's CLI cannot preserve is refused.
Conflicting sprite/script replacements and arbitrary mod combinations are not
guaranteed compatible. Windows, macOS, archive-packed mod inputs and automatic
game-update recovery are outside the supported route.

For exporting artwork, custom palettes and other toolkit commands, see
[the tool guide](docs/TOOLS.md). Earlier individual study recipes remain
available; their coverage is narrower than the complete collection.
