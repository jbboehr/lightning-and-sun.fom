# Josephine Winter blinking and seated actions

Following `a42a8fb`, this slice adds Josephine's eleven Winter blinking, sitting,
eating and drinking strips. There are 31 source frames plus twelve native West
mirrors: 43 review cases. The user approved the artwork on 2026-10-09. Outputs use
`generated/review/` and `generated/build/`.

Josephine now has 151 sources: 32 portraits, all 36 Spring world strips, all 33
Summer world strips, all 33 Autumn world strips and 17 Winter world strips.
The 36-character collection has 4,612 sources and 18,448 variants. P retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. Only eleven regions and registry entries are added. Source
colors, groups, target mappings, sets and
runtime code remain unchanged. Historical pixel tests retain their expectations;
only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-133-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed game
version. All 140 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 538 component seeds select
1,216 skin pixels across 31 source frames, including eyelids and moving hands.
These shades belong exclusively to skin here. Brown hair, lashes, eye details,
gold earrings, pink clothing, dark leggings and boots stay original.
The open mouths retain red `#9E2626` and dark `#410808` interiors. Pink clothing
shades `#A15663`, `#E18A8A` and `#FAC0B5`, and boot highlight
`#EE678F`, remain unchanged. Winter clothing covers the ankles. The portrait
skin shade `#3A1D24` is dark clothing here and stays outside the skin mask.

Every source frame was inspected in five-choice contact sheets, with source
grids around the closed eyelids, mouth interiors, hands and clothing. The material
test independently checks every output pixel against a literal shade oracle,
118 fixed landmarks and per-frame selected counts, plus alpha and metadata.
Landmarks span all eleven strips and include closed eyelids, lashes, moving
hands, dark clothing, mouth interiors and neighboring materials. The earlier
140 sources are compared with the fixed slice 133 baseline. The profile contains
component seeds and pinned source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Each blink has three frames at 0.075, 0.125 and 0.075 seconds. Each drink has
three frames at one second per frame; North eating does too. East and South
eating retain five frames at 0.125, 0.15, 0.175, 0.125 and 0.6 seconds. Sitting
has one frame per direction with no explicit duration or frame count. Native
cycles remain linear; sitting, eating and drinking retain their seated flag.
Eating and drinking retain the native `[240, 360]` final-frame hold range.

## Offline review

- [Five-choice sample summary](../../../generated/slice-134-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-134-review/index.html).

The summary samples South blinking frame 2, East eating frame 2 and North
drinking frame 2. The full gallery includes all 31 source frames and twelve West
mirrors, with Vanilla left and Debug Blue right, full views and enlarged face,
hand and clothing details. Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy, all 346 regular tests and the release build passed through
the Nix shell. The regular run left 416 opt-in tests ignored; 201 selected local
asset checks passed, including the new material test and retained character checks. Three GML
runtime tests passed after rebuilding the pinned interpreter: 204 local checks
in total. Historical pixel tests are unchanged.

The first three GML test launches failed because the existing Fabricator binary's
Nix dynamic loader was gone. Its clean checkout at
`f2483a61d53c730020f2357c790603dd6362476b` was rebuilt with the project Nix shell;
the three tests and native-animation harness then passed. The first browser
launch also referenced a removed Nix store path; the complete gallery audit was
rerun using the current Chromium 154 installation. Original failure logs remain
local alongside `gml-tests.log`, `fabricator-build.log`, `browser-check.log`
and the successful standalone recheck.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations omitted the face, moving hand or the connected skin component containing
both reviewed closed-eyelid pixels; the fourth leaked recoloring into dark
leggings through a `#3A1D24` seed. Each generated successfully and failed the
intended material assertion.
All eleven raw metadata sidecars matched independent original-archive reads.

All 604 standalone Josephine variants validated, and their 1,208 PNG/metadata
files matched the author candidate. All 18,448 combined variants validated.
The 1,400 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 133; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The twenty Chromium pages loaded all local links and images without overflow.
The 43 cases have 86 full-view bindings plus enlarged details and fifteen
summary bindings; 7,766,304 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. The complete review is 784,874 bytes.
The short review/build aliases point to this slice, and the review loads through
its short alias.

The isolated MOMI installation passed in `tmp/slice-134-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `671393425cbb55b415da9b5189b211b95f0af5883e54a1c1257646e70c334ae7`.
The archive's native animator and NPC animate function passed 215 frame/palette
observations, 75 linear completions and 80 final-frame hold checks under
Fabricator. Checks preserve frame phase, wrapper idempotence, cycle counters,
the seated flag, West mirroring and Winter portrait synchronization. There are
no complex transitions here. Engine services were simulated and cycles selected
directly. This does not exercise natural gameplay dispatch, eating/drinking
context, actual seated behavior or live rendering. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-134-`; author records use
`tmp/josephine-winter-actions-author-`. Fresh exports are in
`extracted/josephine-winter-actions-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-133-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring, Summer and Autumn world folders are complete. Winter has sixteen strips
left. Next: Winter general actions. Inventory is recorded in
`tmp/slice-134-remainder.json`. Natural gameplay dispatch, eating/drinking
context, seated behavior and live rendering remain pending.
