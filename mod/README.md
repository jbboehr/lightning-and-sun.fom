# Preset scaffold

This directory contains MOMI manifest templates, the portrait toggle script, and a
draft selection model.
Build an installable directory with the [package command](../docs/TOOLS.md#export-recolor-and-compare).
Do not install this source directory into the game.

`config/character-palettes.json` records asset names and per-character palette
paths relative to the repository root. `config/presets.json` distinguishes the
vanilla default, the optional stylized proof, and future unimplemented palettes.
The current commands take explicit input directories and palette files. Editing
`selected_preset` alone does not change generated output or the running game.

`toggle/` is embedded by `package-toggle`. Rust generates a second portrait locally;
MOMI installs it alongside vanilla, and the GML script selects the displayed sprite
with F6. See the [toggle development notes](../docs/development/portrait-toggle.md).
The broader per-character selector is still unimplemented. Keep source asset
fingerprints with each build. Do not ship game images or generated packages from
this repository.
