# Merri and Terithia portrait batch

This batch covers all 32 Merri and 32 Terithia portrait strips in the supplied
archive: eight expressions in each of Spring, Summer, Autumn, and Winter.
Both have Vanilla, Debug Blue, Hayden, Ryis, and Seridia choices, with Vanilla
selected at launch. F1 cycles Merri; F3 cycles Terithia. The shared Rust and GML
runtime is unchanged.

The collection has 1,756 source strips and 7,024 generated variants across
twenty-five characters: 1,739 portraits and Adeline's 17 reviewed world strips.
Earlier characters retain their accepted recipes, including Luc's glasses-rim
blends, Maple's lip colors, and Hemlock's deferred Debug Blue hair contrast.

Merri's 32 original strips all use `PortraitsMisc`. Her Winter files sit
directly under `Merri/Portraits/`, while the other seasons use subfolders.
Terithia's eight strips per season use their corresponding seasonal atlas.
The registry retains the original paths and atlas assignments. All 64 source
names match the native NPC outfit/expression definitions.

Character-specific art notes are in [Merri portraits](merri-portraits.md) and
[Terithia portraits](terithia-portraits.md).

Merri uses eight source shades, including fine eye and lower-lip shading. The
pale eye-edge blend follows the target light color with its original RGB offset
of `(19,22,24)`; actual gray irises stay original. Terithia uses eleven shades,
including fine ear, hand, and neck contours. Her three pale scar colors retain
their offsets from the main skin light shade, preserving their brightness while
removing the fixed peach tint. Hair and shared-color clothing, cords, belts,
bags, and mouth interiors remain outside the selected skin regions.

## Local visual check

The compact comparison is `generated/merri-terithia-preview/summary.png`,
with `index.html` beside it. The combined bundle is
`generated/characters-merri-terithia-trial`. Images, packages, exports, copied
game files, and test helpers remain ignored.

The new local trial uses `tmp/merri-terithia-playtest`, with separate saves and
state and read-only mounts of the supplied game. F7 opens on Merri and advances
expressions; Page Up goes backward. F4 switches characters, F5 switches seasons,
and F1/F3 cycle Merri/Terithia palettes. F9 checks independent selections and
fractional portrait phase.

Optional world controls use Shift+F1 to spawn/reset Adeline's Spring actor,
Shift+F3 to change its action, Shift+F12 to change facing, and Shift+F11 to
check phase and movement. The installed pinned MMAPI supports these chords
and consumes their triggers before the plain-key callbacks. The helper retains
the scheduling fix that prevents manually placed actors from resuming town
pathfinding. These controls and scheduling changes are excluded from the player
package. The usual `./tmp/play-characters` launcher now opens this batch;
its executable, shell syntax, and match with the tested launcher were checked.

## Verification

The collection regression first failed with `Unsupported character: merri`.
After registry integration, all four character tests passed, covering standalone
and combined builds, twenty-five characters, F1/F3 controls, Vanilla-first
choices, generated pixels, metadata, and Merri's miscellaneous atlas.
The release build and three optional GML interpreter tests passed.

The authors and independent reviewer inspected all 128 unique full
Source/Blue/Ryis frame trios, all 128 enlarged face trios, and seasonal hand/body
closeups for both characters. Merri's five-way eye, mouth, and hand views and
Terithia's six distinct mouth shapes also received focused checks. Early source
findings were incorporated before the first actual candidate builds; no further
concrete omission or clothing/accessory spill remained after the full sweep.

All 256 new target strips passed the independent pixel audit, including 60
separately chosen source-art landmarks. Every Merri target selects and changes
88,720 pixels while protecting 2,320 other matching-color pixels; Terithia selects
and changes 197,904 and protects 31,088. The selected masks agree across all four
presets, with no identity mappings. Both galleries reuse all 64 character frames,
and reconstructed selections reproduce the actual Blue outputs.

Fresh formatting, Clippy, all 87 active tests, both local corpus tests, and the
release build passed through the pinned Fenix Nix shell. Merri's local test
checks 29 literal landmarks in all four palettes; Terithia's checks 148 skin/scar
points and 64 protected details. Their omission and spill controls failed for
the intended reasons before the corresponding corrections.

The source audit matched every registry path, PNG, metadata entry, source hash,
frame size, atlas, profile region, preset arity, and catalog or custom shade rule.
All 7,024 combined variants passed validation. Both new character outputs match
their art-reviewed standalone builds byte-for-byte. All twenty-three earlier
character trees match the accepted Luc/Maple detail bundle, including original
PNGs, recolors, metadata, and reports. The compact summary was inspected.

The normal CLI installer built and published a fresh isolated MOMI archive,
verified its pixels and animation metadata, and reported all twenty-five
characters with the expected controls. The installed archive's SHA-256 matches
its receipt; the supplied source archive remains byte-identical.

The installed game passed all 64 new portraits across all five choices: 320
source/preset pairs. The complete run checked 176 sources and 880 pairs,
including one representative per available outfit for each earlier character.
Independent selections, fractional portrait phase, seasonal wrapping, and F1/F3
controls passed. Shifted world spawn, action, facing, and phase/movement controls
also passed without changing Merri or Terithia's selected palette. The console
stayed hidden, no script errors were reported, and the game exited with status 0
after SIGTERM. Merri and Terithia's Spring Blue and Ryis game screenshots were
visually inspected. The user accepted the appearance and approved committing
this batch on 2026-09-12.

At commit time the original game mount was unavailable, so installation and
game checks were not rerun. The rebuilt local launcher passed executable and
shell-syntax checks, and the retained installed archive still matched its receipt.
Formatting, Clippy, all 87 active tests, both local portrait corpus tests, and
the release build passed again before the commit. Evidence is in
`tmp/merri-terithia-commit-checks.log`.

Local evidence remains ignored: `tmp/merri-terithia-art-review-final.md`,
`tmp/merri-terithia-final-checks.log`,
`tmp/merri-terithia-all-variants-validation.log`,
`tmp/merri-terithia-integration-audit.json`,
`tmp/merri-terithia-install-report.json`,
`tmp/merri-terithia-live-run.log`, and `tmp/merri-terithia-live-audit.json`.
The four game captures are under `tmp/merri-terithia-headless/state/`.
