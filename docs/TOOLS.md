# Running the palette experiment

Run commands from the repository root. Use fresh output paths for each run.
The tool refuses existing or overlapping outputs, so it cannot silently replace
an earlier export or leave stale images from a previous preset.

## Build with Nix and Fenix

```sh
nix-shell --run 'cargo build --locked --release'
target/release/mistria-palette --help
```

[Fenix](https://github.com/nix-community/fenix) supplies Cargo, rustc, rustfmt,
Clippy, rust-analyzer, and matching standard-library sources. `flake.lock` pins
Rust 1.98.1 and the Nix inputs; `Cargo.lock` pins the Rust dependencies. No Python,
Pillow, system font, or rustup installation is required. Contact-sheet labels use
the embedded bitmap font from [font8x8](https://docs.rs/font8x8/0.3.1/font8x8/).

Use `nix-shell` for an interactive environment; leave with `exit`. Launch your
editor from that shell to use its rust-analyzer and `RUST_SRC_PATH`.
`nix develop` shares the same `shell.nix` and lock when the Nix files are tracked
by Git. `nix-shell` also works before staging. Avoid a path flake over the whole
working directory: it includes ignored game files. Both Linux shell definitions
were evaluated; execution was tested on x86_64 Linux only.

## CLI installation

After building in the Nix shell, close the game and other mod installers:

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria'
target/release/mistria-palette uninstall --game-dir '/path/to/Fields of Mistria'
```

By default the first command exports the Adeline spring neutral portrait, applies
the embedded blue palette, builds the toggle package, runs MOMI in isolation, and
verifies the actual atlas frames before publishing. F6 selects vanilla/blue during play.
This currently supports x86_64 Linux with Nix. No game images are downloaded.
Nix pins the MOMI binary, bubblewrap, dynamic loader, and runtime libraries.

Use `--palette path/to/palette.json` on `install` to use another exact-color recipe.
Use `--presets palettes/sets/adeline-trial.json` for the five-choice spring trial
described below. `--palette` and `--presets` are mutually exclusive.
When that recipe has regions, their exact asset paths select which supported
Adeline portraits and spring idle/walk sprites to install. A recipe without regions keeps the
neutral-only selection. Use
`--palette palettes/stylized/adeline-spring.json` for all 25 supported spring
expressions. Missing or changed source files stop installation. Lip refinement
remains deferred; other outfits and overworld sprites are outside this recipe.
Use `--presets palettes/sets/adeline-seasonal-trial.json` for 50 expressions across
spring and summer with Debug Blue, Hayden, Ryis, and Seridia colors. The blue-only
equivalent is `--palette palettes/stylized/adeline-spring-summer.json`.
Use `--presets palettes/sets/adeline-all-seasons-trial.json` for all 100 seasonal
strips with the same five choices, or `--palette palettes/stylized/adeline-all-seasons.json`
for Vanilla and Debug Blue only. Beach, wedding, and overworld sprites are not included.
Use `--presets palettes/sets/adeline-seasonal-beach-trial.json` to include the
19 beach strips as well, including the towel portrait, for 119 strips in total.
Its blue-only recipe is `palettes/stylized/adeline-seasonal-beach.json`.
Wedding portraits and overworld sprites remain original.
Use `--presets palettes/sets/adeline-portraits-trial.json` to add all seven wedding
strips, covering all 126 Adeline portrait strips in the reviewed archive. The
blue-only recipe is `palettes/stylized/adeline-portraits.json`. Overworld sprites
remain original; every launch starts with Vanilla.
Use `--presets palettes/sets/adeline-world-trial.json` to include those 126 portraits
and six spring overworld idle/walk strips. F6 switches both together. North, south,
east, and mirrored west are covered; other actions and overworld outfits remain
original. Its blue-only recipe is `palettes/stylized/adeline-world-trial.json`.
Use `--momi /absolute/path/to/installer` to override the Nix-provided MOMI binary.
Remove the installed study before rebuilding it with a changed recipe. The
`MISTRIA_MOMI_RUNNER` environment override is a developer/test integration point;
normal use needs no runner configuration.

Removal restores the exact pre-install archive, including previously installed
mods. Their source folders are never edited. Keep all existing mod sources in the
game's `mods/` or `Mods/` directory: MOMI rebuilds that selected set. ZIP/RAR mods,
symlinked mods, nested manifests, and duplicate mod names are unsupported by this
prototype. A skipped selected mod prevents publication.
If MOMI rebuilds existing mods in a different load order, publication also stops.
The pinned CLI chooses its own alphabetical order; use a MOMI workflow that
preserves your custom order if the palette installation reports this mismatch.

For an already MOMI-modified archive, also supply its current installed-mod list:

```sh
target/release/mistria-palette install \
  --game-dir '/path/to/Fields of Mistria' \
  --installed-mods '/path/to/FieldsOfMistria/config-or-branch/mods/manifest.json'
```

This JSON is MOMI's list in the game's config/save directory, not an individual
mod's manifest or the empty marker inside `assets.zip`. Its location depends on
the MOMI configuration and game branch. The mod names and versions in source manifests
must match the list before the palette is added. Use the list from the last MOMI
installation of this archive: the file has no archive checksum, so the CLI cannot
detect a stale or unrelated list. If that list is unavailable, restore a known
game/mod setup through MOMI first. Do not invent a replacement list from whichever
source folders remain.
Keep its entries in their original order: they also record mod precedence.

The palette package is built temporarily, not placed in the live `mods/` folder.
Use this CLI's `uninstall` before managing mods separately through MOMI. The wrapper
does not update the game's config-side Mods-tab list. Its saved `.mistria-palette/`
directory contains the previous archive and a receipt; keep both until removal.
If the game archive has since changed, the CLI refuses to restore the old one.
Keep the recovery directory and reconcile that change through your mod workflow;
do not force-copy an old backup over a game update.

Allow roughly 3 GB of temporary free space for this build; the retained previous
archive uses roughly 600 MB. A failed build leaves the live archive unchanged.
See [installer internals and verification](development/cli-installer.md).

## Export, recolor, and compare

The example below selects one portrait animation (two frames). The exporter reads exact
archive members and their `.meta.toml` files, preserving the `assets/…` tree. It
does not unpack the entire game or modify the ZIP. Its report pins the source ZIP
and exported bytes by SHA-256. Repeat `--asset` to select up to 132 distinct PNGs.
The older replacement `package` command still permits at most two changed assets.

```sh
target/release/mistria-palette export \
  --archive tmp/fields-of-mistria/assets.zip \
  --asset assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_neutral.png \
  --output extracted/adeline

target/release/mistria-palette apply \
  --input extracted/adeline \
  --palette palettes/stylized/adeline.json \
  --output generated/adeline-stylized

target/release/mistria-palette validate \
  --original extracted/adeline \
  --modified generated/adeline-stylized \
  --palette palettes/stylized/adeline.json

target/release/mistria-palette contact-sheet \
  --original extracted/adeline \
  --modified generated/adeline-stylized \
  --output generated/contact-sheet-adeline-stylized.png \
  --zoom 4 --changes

target/release/mistria-palette package \
  --original extracted/adeline \
  --modified generated/adeline-stylized \
  --output generated/momi-adeline-stylized \
  > generated/momi-build-report.json
```

Commands print deterministic JSON. Palette runs also write `palette-report.json`
inside their output tree. Contact sheets have a sibling `.json` containing asset
hashes, change counts, dimensions, and image positions. Keep reports outside the
MOMI package: MOMI can interpret root JSON files as installable game content.
`package` uses the manifest embedded when the binary was built; use
`--manifest path/to/manifest.toml` to supply a different one.

`validate --palette` checks every output pixel against the exact recipe, including
pixels outside selected regions and transparent RGB. Without `--palette`, it
retains the filename, dimension, alpha, and metadata checks. `contact-sheet
--changes` adds a third column: magenta marks changed pixels and the checkerboard
marks unchanged pixels. Large batches may exceed the preview's size limit; split
them into smaller input trees for enlarged comparisons.

Use `palettes/vanilla/adeline.json` with a fresh output to generate unchanged
copies. Building from those copies emits no replacement PNGs. Leave vanilla
uninstalled. To return an already modded game to vanilla, uninstall its replacement.

For the experimental in-game toggle, use this command instead of `package`:

```sh
target/release/mistria-palette package-toggle \
  --original extracted/adeline \
  --modified generated/adeline-stylized \
  --output generated/momi-adeline-toggle \
  > generated/momi-toggle-report.json
```

Install that generated folder as `mods/lns_palette` through MOMI v0.15.10. Remove
the earlier replacement study first so the base portrait is vanilla. This package
adds separate animations and an F6 hotkey: press F6 again to restore vanilla.
It accepts one through 132 supported Adeline strips: portraits from the four
seasons, beach, and wedding, plus spring overworld idle/walk north, south, and
east. It generates the matching runtime sprite table. Keep both generated GML
files in the package.
The choice lasts for the running game session and is not saved. Rebuild and
reinstall after changing its palette. See [the developer procedure](development/portrait-toggle.md)
and [complete portrait coverage](development/wedding-portraits.md) for verification details.
The [spring overworld study](development/overworld-spring.md) describes the new
sprite masks, runtime behavior, and playtest controls.

Palettes require an `rgba_map` object. Keys and values are `#RRGGBB` or
`#RRGGBBAA`; omitted alpha means `FF`. Replacements happen simultaneously against
decoded RGBA8 pixels. Duplicate source colors, alpha changes, and edits to fully
transparent RGB are rejected. The vanilla recipe has an empty map and no regions.

The included blue recipe also has `regions`, with one entry per input PNG. Each
entry names the exact relative `asset` path, original PNG `source_sha256`, `size`
as `[width, height]`, and `seeds` as `[x, y]` pixel coordinates. Coordinates start
at the strip's top-left; the second frame's coordinates include its horizontal
offset. A seed selects all nontransparent pixels reachable through up/down/left/
right neighbors whose original RGBA is a source key in `rgba_map`. Diagonal
contact does not connect regions. Repeated seeds in one region do not duplicate
work or counts. Empty seeds intentionally select no pixels.

Omitting `regions` retains unrestricted exact-color replacement. If present,
region entries must cover the exact input PNG set; an empty list, null, duplicate
assets, stale source hash, mismatched dimensions, or invalid seeds stop before
output is written. Recipe and region field names are checked for typos.

For custom target colors, edit the values in `rgba_map`. Adding or removing source
keys changes region connectivity and needs another visual review. After a source
PNG changes, review both the image and seeds before updating its checksum.
Masked reports include `selected_pixels` and `excluded_matching_pixels`; the latter
counts pixels outside the mask that unrestricted mapping would have changed.
Only recipes and seed coordinates belong in Git. All images stay local.
See [the region study](development/portrait-regions.md) for authoring and QA evidence.

The image tools support static PNGs, including indexed transparency, grayscale,
RGB, and RGBA. APNG is rejected. Changed images are encoded as RGBA8 PNGs. Sidecar
metadata is copied byte-for-byte. Unchanged PNGs retain their original bytes.
The tools do not preserve arbitrary PNG text/EXIF chunks after recoloring.
Contact sheets use 4x or 8x nearest-neighbor scaling over a checkerboard. Labels
outside the embedded font's basic character set display as `?`; the JSON report
retains exact UTF-8 paths.

## Palette catalog and preset sets

Inventory the palettes and portrait colors from your own game archive:

```sh
target/release/mistria-palette catalog \
  --archive tmp/fields-of-mistria/assets.zip \
  --output generated/palette-catalog
```

`catalog.json` records the archive hash, player lookup-table provenance, and each
portrait's original hash, dimensions, and opaque-color counts. `player-palettes.png`
shows the four skin colors of each creator option. The supplied build has 36
distinct creator ramps and 2,303 NPC/cameo portrait strips. Portrait histograms
are marked `unreviewed_colors`: they include hair, clothing, animals, and other
colors, and do not identify NPC skin ramps automatically. The numeric creator
catalog is also recorded in `palettes/catalog/player.json`.

`palettes/catalog/npc-spring.json` separately records manually sampled skin and
facial-detail ramps for 36 characters, with PNG hashes and source coordinates.
Its 34 distinct ramps come from the first frame of spring neutral portraits;
they are not complete source recoloring maps. Some entries have three or five
shades and need adaptation before use with Adeline's four-color profile.
See [the NPC palette study](development/npc-palettes.md) for scope and evidence.

`palettes/profiles/adeline-spring.json` holds the source colors and reviewed regions
for all 25 spring expressions. A single-color recipe can reference this profile
with `"profile": "../profiles/adeline-spring.json"` alongside its `rgba_map`.
The path is relative to the recipe file. Its source keys must exactly match the
profile's source colors. A recipe cannot specify both a profile and inline regions.
An omitted profile is supported; an explicit `null` profile is rejected.

`palettes/profiles/adeline-spring-summer.json` extends those same regions with 25
individually checked summer strips. Every variant keeps its outfit's atlas, so the
palette choice follows supported outfit changes without loading both seasons at
once. Use `palettes/sets/adeline-seasonal-trial.json` with this combined profile.

`palettes/profiles/adeline-all-seasons.json` retains those 50 regions and adds 25
autumn and 25 winter strips. Use `palettes/sets/adeline-all-seasons-trial.json` for
the complete seasonal set. The corresponding atlas families are `PortraitsSpring`,
`PortraitsSummer`, `PortraitsAutumn`, and `PortraitsWinter`.

A preset set references one profile and supplies target colors in the same order
as its `source_colors`:

```json
{
  "profile": "../profiles/adeline-spring.json",
  "presets": [
    {
      "id": "blue",
      "label": "Debug Blue",
      "colors": ["#9DB9D4", "#7F9FBD", "#6687AD", "#445F83"]
    }
  ]
}
```

Use `palettes/sets/adeline-trial.json` as the editable starting point. It includes
Debug Blue plus Player 01, 18, and 33. Copy other ramps from the numeric catalog;
keep the four-color order. IDs must be unique lowercase ASCII letters, digits,
or underscores, at most 32 characters. Labels must be nonblank, contain no control
characters, and fit in 64 UTF-8 bytes. A set accepts one through eight variants;
Vanilla is implicit and cannot be used as a variant ID. This limit keeps the
prebuilt portrait count bounded while the prototype is evaluated.

For NPC-derived colors, use `palettes/sets/adeline-npc-trial.json` instead. It
includes Debug Blue plus Hayden, Ryis, and Seridia palettes on the same Adeline
spring expressions. Pass this path to either `build-presets` or `install` in
place of `adeline-trial.json`. These choices recolor Adeline; they do not modify
the source NPCs.

Close the game and remove any previous study, then install the set:

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria' \
  --presets palettes/sets/adeline-trial.json
```

F6 cycles Vanilla, then each listed preset in order, and wraps. Switching creates
no HUD popups; the game log records the selected name. The choice lasts for the session. Other
installation, existing-mod, and removal requirements above still apply.

To generate a previewable tree and MOMI package without installing, first export
the profile's exact PNG set, then run:

```sh
target/release/mistria-palette build-presets \
  --original extracted/adeline-spring-study \
  --presets palettes/sets/adeline-trial.json \
  --output generated/adeline-presets-trial
```

The result contains `variants/<id>/` image trees, `package/`, and
`presets-report.json`. Pass an individual variant tree to `contact-sheet` as its
`--modified` directory. Install only `package/` through MOMI, never the bundle
root or reports. The package's generated `palette_assets.gml` defines both the
sprite table and preset labels; retain it alongside `palette_toggle.gml`.

These creator ramps are adaptations to Adeline's portrait shading. Lips, blush,
and outlines need further art review, especially at the extremes. Images remain
local. See [the catalog and preset study](development/palette-presets.md) for
provenance, atlas costs, and verification evidence.

## MOMI installation and removal

Download the Linux CLI from the pinned
[v0.15.10 release](https://github.com/Garethp/Mods-of-Mistria-Installer/releases/tag/v0.15.10).
The self-contained executable needs system libraries but does not require a .NET
SDK. For a reproducible NixOS experiment, follow
[the isolated installation procedure](development/momi-lab.md).

For a chosen game install, close the game and back up both the original archive and
saves. Put the generated mod folder directly inside that install's `mods/` folder,
so `mods/adeline-palette-study/manifest.toml` exists. Run MOMI and confirm the intended
game location and replacement appear in its log. It creates `assets.bak.zip` and
updates `assets.zip`. Do not install the source `mod/` folder, the extracted tree,
or the contact sheet.

To remove all MOMI mods, use its Uninstall action or run the CLI with `--uninstall`
against the intended installation. To remove only this study, move its folder out
of `mods/` and reinstall the remaining selection. Keep the matching pristine backup
until restoration has been checked. Do not just delete the mod folder and assume
the archive has changed.

MOMI checks Steam library locations before its current-directory fallback. Merely
starting it in a copied game folder does not isolate it. The documented lab makes
the copied archive the only visible installation.

For tests and local proof results, see [the verification record](development/verification.md).
