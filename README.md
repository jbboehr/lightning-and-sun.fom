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
and adds the blue variant. Press **F6** during play to switch Adeline's spring
neutral portrait; each game launch starts with vanilla. Other expressions and
characters are unchanged. There is no preset menu or graphical installer yet.
See [INVESTIGATION.md](docs/INVESTIGATION.md) for compatibility and experiment limits.

Game files, extracted PNGs, generated mods, and contact sheets stay local and are
excluded from Git. Keep an untouched game copy and back up saves before installing
any mod. F6 returns the toggle study to vanilla; removing the mod entirely still
uses the same installation method: CLI-installed studies use `uninstall` above;
manually installed packages use MOMI. The older replacement package must be removed
through MOMI too.
