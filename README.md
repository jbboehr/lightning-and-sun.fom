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
local MOMI package. See [INVESTIGATION.md](docs/INVESTIGATION.md) for compatibility and
the limits of the experiment. There is no in-game preset selector yet.

Game files, extracted PNGs, generated mods, and contact sheets stay local and are
excluded from Git. Keep an untouched game copy and back up saves before installing
any mod. Returning to vanilla means removing the replacement through MOMI.
