# Lightning and Sun palette study

An experimental Fields of Mistria palette toolkit. It exports selected assets from
your own game copy, applies exact color replacements, and makes comparison sheets.
Vanilla is the default. The optional Adeline study changes one portrait strip to
a stylized blue palette within reviewed skin regions, preserving robe and jewelry
details. It is unfinished test art.

Build the Rust command-line tool with the pinned Nix environment:

```sh
nix-shell --run 'cargo build --locked --release'
target/release/mistria-palette --help
```

The resulting binary runs without Python, Pillow, or a virtual environment.
`nix develop` provides the same Fenix environment in a tracked checkout. On x86_64
Linux, Nix also supplies MOMI v0.15.10 and its isolated runtime. The built binary
uses those Nix store dependencies on this machine.

Close the game, then install or remove the one-portrait toggle:

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria'
target/release/mistria-palette uninstall --game-dir '/path/to/Fields of Mistria'
```

Installation generates the blue study from your local assets and checks the
installed portrait frames before changing the selected game archive. It keeps the
exact previous archive in that game's `.mistria-palette/` directory. Keep that
directory until removal. If another installer or a game update changes the archive,
removal stops instead of overwriting those changes.

To try all 25 Adeline spring expressions, remove any previous palette installation,
then choose the larger study:

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria' \
  --palette palettes/stylized/adeline-spring.json
```

This covers both frames of each spring expression. Other outfits and overworld
sprites remain original. The colors are provisional, especially the lips. The
default installation continues to use only the neutral portrait.

For a trial with several skin palettes, uninstall the previous study, then use:

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria' \
  --presets palettes/sets/adeline-trial.json
```

Press **F6** to cycle **Vanilla → Debug Blue → Player 01 → Player 18 → Player 33**.
The three numbered palettes come from the game's player customization colors.
They cover all 25 Adeline spring expressions and share the reviewed skin regions.
Palette switching is silent so popups cannot obscure the portrait; the game log
records the selected name.
Every launch starts with Vanilla. Lips and blush remain provisional; these player
colors have not yet had a full portrait art pass. See
[palette options](docs/TOOLS.md#palette-catalog-and-preset-sets) to choose other ramps.

For all four seasonal outfits plus beach and wedding portraits, uninstall the previous study
and use this trial:

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria' \
  --presets palettes/sets/adeline-portraits-trial.json
```

F6 cycles **Vanilla → Debug Blue → Hayden → Ryis → Seridia** across all 25
expressions in each seasonal outfit, 19 beach strips (including the towel), and
seven wedding strips: all 126 Adeline portrait strips in the reviewed game build.
The selected palette carries across outfit changes for the current
session. These colors are adapted from the named NPCs; only
Adeline's portraits change. Overworld sprites remain original.

To also recolor Adeline while she stands and walks in her spring outfit,
uninstall the previous study and choose:

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria' \
  --presets palettes/sets/adeline-world-trial.json
```

F6 switches her portrait and supported overworld animations together, using the
same five choices. This includes all four walking directions. Other overworld
actions and outfits still show her original skin colors. All 126 portraits are
included. Each launch starts with Vanilla.

For spring sitting, eating, drinking, and blinking as well, use
`--presets palettes/sets/adeline-world-actions-trial.json` instead. F6 selects the
same palette across these actions and the included portraits. Other overworld
actions and outfits still use their original colors.

To include Hayden, Ryis, Reina, Juniper, Celine, March, Balor, Valen, Eiland,
Olric, Landen, Nora, Holt, Josephine, Darcy, Dell, Elsie, Errol, Hemlock, Louis,
Luc, Maple, Merri, Terithia, Stillwell, Taliferro, Vera, and Wheedle alongside
Adeline, uninstall the previous study, then install the combined trial:

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria' \
  --characters palettes/sets/characters-trial.json
```

**F6** cycles Adeline's palettes, **F8** Hayden's, and **F10** Ryis's.
**Home** cycles Reina's palettes; **Page Down** cycles Juniper's.
**Insert** cycles Celine's palettes; **U** cycles March's.
**I** cycles Balor's palettes; **O** cycles Valen's.
**J** cycles Eiland's palettes; **K** cycles Olric's.
**L** cycles Landen's palettes; **N** cycles Nora's.
**H** cycles Holt's palettes; **P** cycles Josephine's.
**B** cycles Darcy's palettes; **Y** cycles Dell's.
**Z** cycles Elsie's palettes; **X** cycles Errol's.
**F** cycles Hemlock's palettes; **V** cycles Louis's.
**F11** cycles Luc's palettes; **F12** cycles Maple's.
**F1** cycles Merri's palettes; **F3** cycles Terithia's.
**F4** cycles Stillwell's palettes; **F5** cycles Taliferro's.
**G** cycles Vera's palettes; **T** cycles Wheedle's.
Each character starts on **Vanilla** every launch, followed by **Debug Blue**
and three sampled NPC palettes. Selections are independent.

| Character | Additional palettes |
| --- | --- |
| Adeline | Hayden, Ryis, Seridia |
| Hayden | Adeline, Ryis, Seridia |
| Ryis | Adeline, Hayden, Seridia |
| Reina | Hayden, Ryis, Seridia |
| Juniper | Hayden, Ryis, Seridia |
| Celine | Hayden, Ryis, Seridia |
| March | Hayden, Ryis, Seridia |
| Balor | Hayden, Ryis, Seridia |
| Valen | Hayden, Ryis, Seridia |
| Eiland | Hayden, Ryis, Seridia |
| Olric | Hayden, Ryis, Seridia |
| Landen | Hayden, Ryis, Seridia |
| Nora | Hayden, Ryis, Seridia |
| Holt | Hayden, Ryis, Seridia |
| Josephine | Hayden, Ryis, Seridia |
| Darcy | Hayden, Ryis, Seridia |
| Dell | Hayden, Ryis, Seridia |
| Elsie | Hayden, Ryis, Seridia |
| Errol | Hayden, Ryis, Seridia |
| Hemlock | Hayden, Ryis, Seridia |
| Louis | Hayden, Ryis, Seridia |
| Luc | Hayden, Ryis, Seridia |
| Maple | Hayden, Ryis, Seridia |
| Merri | Hayden, Ryis, Seridia |
| Terithia | Hayden, Ryis, Seridia |
| Stillwell | Hayden, Ryis, Seridia |
| Taliferro | Hayden, Ryis, Seridia |
| Vera | Hayden, Ryis, Seridia |
| Wheedle | Hayden, Ryis, Seridia |

These adapt the sampled colors to each character's reviewed shading and masks.
The trial includes 133 Hayden, 109 Ryis, 103 Reina, 132 Juniper, 183 Celine,
181 March, 110 Balor, 92 Valen, 78 Eiland, 36 Olric, 32 Landen, 32 Nora,
32 Holt, 32 Josephine, 32 Darcy, 32 Dell, 36 Elsie, 36 Errol, 32 Hemlock,
32 Louis, 32 Luc, 32 Maple, 32 Merri, 32 Terithia, 36 Stillwell, 36 Taliferro,
32 Vera, and 32 Wheedle portrait strips.
These cover seasonal portraits plus beach, bath, and wedding artwork where
available. Celine's gardening portraits, March's special expressions, and Olric's
bunny-ears portraits are included.
Their overworld sprites remain original. Hayden's embarrassed expression and
possible Reina/Juniper mouth details await a later art pass. To try one character
alone, use its set instead; for example,
`--presets palettes/sets/landen-portraits-trial.json` or
`--presets palettes/sets/nora-portraits-trial.json`.

The included palette is tied to the reviewed portrait's original PNG bytes. If
those change, installation stops until the region definition is reviewed. See
[palette and preview options](docs/TOOLS.md#export-recolor-and-compare) for custom
colors, exact recipe validation, and changed-pixel previews.

Existing MOMI mods need their complete source folders in `mods/` or `Mods/`, plus
the matching `assets.bak.zip`. Add `--installed-mods /path/to/mods/manifest.json`
using the installed-mod list MOMI wrote in the game's config directory for that
archive. Installation refuses a missing list or mismatched source selection.
It also stops if MOMI would change the existing mod load order.
This prototype supports unpacked folder mods with a manifest directly inside each
folder. Uninstall this palette study before using MOMI separately or installing a
different palette. See [CLI details](docs/TOOLS.md#cli-installation) for restrictions
and custom palette input.

Follow [TOOLS.md](docs/TOOLS.md) to export the portrait, generate a preview, and build a
local MOMI package. The experimental `package-toggle` mode keeps vanilla installed
and adds blue variants. Press **F6** during play to switch the included Adeline
seasonal, beach, and wedding portraits, plus spring idle/walk sprites when selected
by the recipe; each game launch starts with vanilla. Portraits
outside the chosen recipe and other characters are unchanged. There is no preset menu or
graphical installer yet.
See [INVESTIGATION.md](docs/INVESTIGATION.md) for compatibility and experiment limits.

Game files, extracted PNGs, generated mods, and contact sheets stay local and are
excluded from Git. Keep an untouched game copy and back up saves before installing
any mod. Cycle the character's palette control back to Vanilla to restore their
original colors; removing the mod entirely still
uses the same installation method: CLI-installed studies use `uninstall` above;
manually installed packages use MOMI. The older replacement package must be removed
through MOMI too.
