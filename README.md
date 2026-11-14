# Lightning and Sun

A skin-tone mod for [Fields of Mistria](https://www.fieldsofmistria.com/). It
adds a **Settings → Palettes** page where you choose a skin tone for each of 36
characters. The choice applies to that character's portraits and world sprites
together, and you can change it at any time while playing.

Only skin changes. Clothing, hair, equipment, scars and markings keep their
original colors, and every character starts on the game's own look until you
pick something else.

The mod ships no images. The installer recolors the artwork in your own copy of
the game on your machine, and uninstalling puts your game files back exactly as
they were.

## Status

This is an early release for **Linux only**. Installing, uninstalling and the
palette menu are tested. The artwork has been reviewed on a sample of portraits,
sprites and scenes, but nobody has yet played the whole game with the mod on,
so you may find a frame that was missed. Physical controllers are untested. The
[release notes](docs/releases/linux-candidate.md) list what was tested and the
known limits.

## Requirements

- x86_64 Linux.
- Either [Nix](https://nixos.org/download/) with flakes enabled, which provides
  everything else the installer needs, or a Rust toolchain and a downloaded copy
  of the [MOMI](https://github.com/Garethp/Mods-of-Mistria-Installer) mod
  installer; see [Without Nix](#without-nix).
- Your own installation of the game: the folder containing `assets.zip` and
  `Maybe.toml`.
- About 6 GB of free space in the game folder while installing. Afterwards the
  mod uses roughly 1 GB there, most of it the backup of your original archive.

Back up your save files before you start. The mod does not edit saves, but this
is an early release.

## Build and install

From a checkout of this repository:

```sh
nix build
```

Close the game and any other mod installer, then run:

```sh
result/bin/mistria-palette install --game-dir '/path/to/Fields of Mistria'
```

This takes ten to twenty minutes. The installer generates the recolored artwork,
checks it, and only then replaces the game's archive. If anything fails, your
game is left untouched.

If the install fails or you stop it, run the same command again. It reuses the
recolored artwork it already made, which it keeps in a
`.mistria-palette.resume/` folder in the game directory until an install
succeeds. Delete that folder if you would rather start over.

Keep the `result` link and the `.mistria-palette/` folder that appears in the
game directory. You need both to uninstall.

To install one character instead of all 36, name that character's set:

```sh
result/bin/mistria-palette install \
  --game-dir '/path/to/Fields of Mistria' \
  --presets result/share/mistria-palette/palettes/sets/caldarus-native-tones.json
```

Uninstall before installing a different selection.

### Without Nix

The Nix build is the tested route. This one works on the development machine
but has not been tried on other distributions.

You need a recent stable Rust toolchain (the project is built with Rust 1.99)
and the MOMI 0.16.4 command-line installer, `ModsOfMistriaInstaller-cli-linux`,
from its
[release page](https://github.com/Garethp/Mods-of-Mistria-Installer/releases/tag/v0.16.4).
Make it executable with `chmod +x`. MOMI needs the system's C++ runtime, zlib,
OpenSSL and ICU libraries, which most desktop distributions already have.

```sh
cargo build --locked --release
target/release/mistria-palette install \
  --game-dir '/path/to/Fields of Mistria' \
  --momi /path/to/ModsOfMistriaInstaller-cli-linux
```

Use `target/release/mistria-palette` in place of `result/bin/mistria-palette` in
the other commands on this page, and keep the checkout until you uninstall.

One case is refused. MOMI cannot be told which game folder to use, and it
prefers any Steam copy of the game it can find over the working copy this
installer prepares. The installer hides the Steam libraries in your home
directory from it. It cannot hide a library at the top of a mounted drive, such
as `/mnt/games/SteamLibrary`, so if the game is installed in one the installer
stops before changing anything. The Nix build runs MOMI in a sandbox and has no
such limit.

## Palette controls

Open **Settings → Palettes** from the title screen or the journal, select a
character and cycle through their choices. By default **E** or the controller's
south button selects, and **Esc** or the east button goes back.

Every character offers fourteen choices:

- **Vanilla**, the game's original colors.
- Nine tones adapted from the game's own characters and character creator:
  Light peach, Light golden, Warm peach, Warm brown, Deep brown, Very deep,
  Deep umber, Muted light and Muted medium.
- Three tones borrowed from other characters, usually Hayden, Ryis and Seridia.
- **Debug Blue**, a bright blue test palette that makes any missed pixels
  obvious.

The tones are approximations of the game's range, and a character's natural
shading, glow effects and markings can make the same tone look a little
different from one character to the next.

Six rows after **Reset all to Vanilla** change every character at once:

- **Everyone** gives all characters the same tone and steps to the next tone
  each time you select it.
- **Tanning Bed** shows each character one step darker than their original
  look.
- **Mirror** swaps light and dark: the lightest characters become the darkest
  and the reverse.
- **Milk & Honey** puts everyone on one of two light tones, and **Grayscale
  Agenda** puts everyone on either the lightest or the darkest tone.
- **Randomize all** gives each character a random tone.

A character whose original look already matches what a preset asks for goes
back to Vanilla.

Choices last for the current session. Turn on **Remember choices** to keep them
between sessions; if saving fails you get a warning and your choices still work
until you quit. **Reset all to Vanilla** returns every character to the original
look. None of this changes your game saves.

## Remove or change the mod

Close the game, then run:

```sh
result/bin/mistria-palette uninstall --game-dir '/path/to/Fields of Mistria'
```

This restores the exact archive you had before installing, including any mods
that were already in it.

Uninstall before you update the game or manage other mods. If the game's archive
has changed since you installed, for example after a game update, the uninstaller
refuses to overwrite it and keeps its recovery files. Do not delete
`.mistria-palette/` to get past this; it holds the only copy of your earlier
archive.

## Compatibility

The mod was tested against one build of the game, on one Linux machine. That
build's `assets.zip` has this SHA-256:

```text
8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4
```

You can compare yours with `sha256sum assets.zip` before installing any mods.
On a different build the installer still runs. If artwork it recolors has
changed, it prints a warning naming each changed image and continues; those
images may be recolored incorrectly, so check the affected characters in game.
Add `--strict-source-hashes` to make the installer stop instead. No range of
supported game versions has been established.

Windows, macOS and ARM are not supported.

### Other mods

If the game already has mods installed through MOMI, the installer rebuilds the
archive with them included. It needs each mod's unpacked folder under `mods/` or `Mods/`,
MOMI's `assets.bak.zip` backup, and the list of installed mods that MOMI keeps
in the game's configuration directory:

```sh
result/bin/mistria-palette install \
  --game-dir '/path/to/Fields of Mistria' \
  --installed-mods '/path/to/MOMI-config/mods/manifest.json'
```

The installer stops, without changing anything, if a mod is missing or would
load in a different order than before. This was tested only with simple mods;
mods that replace the same character artwork or scripts may conflict. The
[tool guide](docs/TOOLS.md#cli-installation) has the full rules.

## More documentation

- [Release notes](docs/releases/linux-candidate.md): what is included, what was
  tested and known limits.
- [Tool guide](docs/TOOLS.md): exporting artwork, custom palettes, advanced
  installer options and the other commands.
- [Development documentation](docs/planning.md): project status and the records
  behind each piece of artwork.

## License

The tool, recipes, masks and mod scripts in this repository are licensed under
the GNU Affero General Public License, version 3 only (AGPL-3.0-only); see
[LICENSE](LICENSE). Fields of Mistria and its assets, including the images this
tool generates from your copy of the game, are not covered by this license and
remain the property of their owners.
