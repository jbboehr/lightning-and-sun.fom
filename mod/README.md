# Preset scaffold

This directory contains a MOMI manifest template and a draft selection model.
Build an installable directory with the [package command](../docs/TOOLS.md#export-recolor-and-compare).
Do not install this source directory into the game.

`config/character-palettes.json` records asset names and per-character palette
paths relative to the repository root. `config/presets.json` distinguishes the
vanilla default, the optional stylized proof, and future unimplemented palettes.
The current commands take explicit input directories and palette files. Editing
`selected_preset` alone does not change generated output or the running game.

Future selection should resolve vanilla to no replacement and emit one chosen
palette per character, after export and QA. Keep source asset fingerprints with
each build. Do not ship game images or generated packages from this repository.
