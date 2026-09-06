# Lightning and Sun palette study

An experimental Fields of Mistria palette toolkit. It exports selected assets from
your own game copy, applies exact color replacements, and makes comparison sheets.
Vanilla is the default. The optional Adeline study changes one portrait strip to
a stylized blue palette. It is unfinished test art.

Build the Rust command-line tool with the pinned Nix environment:

```sh
nix-shell --run 'cargo build --locked --release'
target/release/mistria-palette --help
```

The resulting binary runs without Python, Pillow, or a virtual environment.
`nix develop` provides the same Fenix environment in a tracked checkout.

Follow [TOOLS.md](docs/TOOLS.md) to export the portrait, generate a preview, and build a
local MOMI package. The experimental `package-toggle` mode keeps vanilla installed
and adds the blue variant. Press **F6** during play to switch Adeline's spring
neutral portrait; each game launch starts with vanilla. Other expressions and
characters are unchanged. There is no preset menu or graphical installer yet.
See [INVESTIGATION.md](docs/INVESTIGATION.md) for compatibility and experiment limits.

Game files, extracted PNGs, generated mods, and contact sheets stay local and are
excluded from Git. Keep an untouched game copy and back up saves before installing
any mod. F6 returns the toggle study to vanilla; removing the mod entirely still
requires MOMI. The older replacement package must be removed through MOMI too.
