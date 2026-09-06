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
