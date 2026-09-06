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

The first command exports the Adeline spring neutral portrait, applies the embedded
blue palette, builds the toggle package, runs MOMI in isolation, and verifies the
actual atlas frames before publishing. F6 selects vanilla/blue during play.
This currently supports x86_64 Linux with Nix. No game images are downloaded.
Nix pins the MOMI binary, bubblewrap, dynamic loader, and runtime libraries.

Use `--palette path/to/palette.json` on `install` to use another exact-color recipe.
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

Only one portrait animation (two frames) is selected. The exporter reads exact
archive members and their `.meta.toml` files, preserving the `assets/…` tree. It
does not unpack the entire game or modify the ZIP. Its report pins the source ZIP
and exported bytes by SHA-256. Repeat `--asset` to select a second PNG; the proof
of concept limits exports and changed package replacements to two assets.

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
  --modified generated/adeline-stylized

target/release/mistria-palette contact-sheet \
  --original extracted/adeline \
  --modified generated/adeline-stylized \
  --output generated/contact-sheet-adeline-stylized.png \
  --zoom 4

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
adds a separate animation and an F6 hotkey: press F6 again to restore vanilla.
The choice lasts for the running game session, covers only Adeline's spring
neutral portrait, and is not saved. Rebuild and reinstall after changing its
palette. See [the developer procedure](development/portrait-toggle.md) for isolated
installation and verification.

Palettes require an `rgba_map` object. Keys and values are `#RRGGBB` or
`#RRGGBBAA`; omitted alpha means `FF`. Replacements happen simultaneously against
decoded RGBA8 pixels. Duplicate source colors, alpha changes, and edits to fully
transparent RGB are rejected. An empty map means vanilla.

The image tools support static PNGs, including indexed transparency, grayscale,
RGB, and RGBA. APNG is rejected. Changed images are encoded as RGBA8 PNGs. Sidecar
metadata is copied byte-for-byte. Unchanged PNGs retain their original bytes.
The tools do not preserve arbitrary PNG text/EXIF chunks after recoloring.
Contact sheets use 4x or 8x nearest-neighbor scaling over a checkerboard. Labels
outside the embedded font's basic character set display as `?`; the JSON report
retains exact UTF-8 paths.

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
