# Landen Summer blinking and seated actions

Following `664cb14`, this slice adds eleven Summer strips to Landen's accepted
62 sources: blinking East/South and sitting, eating and drinking North/South/East.
The 31 source frames and 12 native West mirrors give 43 review cases. Artwork
was approved by the user on 2026-10-03. Completed outputs use `generated/review/` and `generated/build/`.

Landen has 73 sources; the 36-character collection has 4,217 sources and 16,868
variants. L retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only the new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Landen coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-052-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 62 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 048](slice-048.md). Its 651 seeds select 1,475 skin pixels across the 31
frames: face, ears, eyelids, neck, hands and visible knees. All four shades
represent skin throughout these eleven pinned Summer strips. Gray hair, eye
detail, blue patterned shirt, orange shorts, dark socks, orange shoes and the
apple stay original. All frames were inspected in the Vanilla/Blue contact
sheets, with source grids for the material boundaries and a five-choice summary.

The tracked masks use connected-component seeds. The material test independently
checks the reviewed source shades, literal landmarks, every output pixel,
per-frame selected counts, transparency and metadata. The lowered hand in the
second South eating frame has an explicit landmark at local (36,49).
Selected pixels are counted separately from changed pixels because two Ryis
target shades already match the source.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Blinking has three frames with 0.075/0.125/0.075-second timing. Sitting uses
single frames. Drinking and North eating have three frames at one second each;
East/South eating have five frames at 0.125/0.15/0.175/0.125/0.6 seconds.
The native sitting, eating and drinking cycles retain their seated flag;
eating and drinking retain the 240–360-tick final-frame hold. All four cycles
are linear and West mirrors East.

## Offline review

- [Five-choice sample summary](../../generated/slice-053-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-053-review/index.html).

The summary samples blinking South, eating East and drinking North. The full
gallery includes every frame and native West mirror, with complete character
views and enlarged details. Its 20 pages total 763,279 bytes (about 745 KiB).
Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 265 regular tests and the
release build passed; 335 local opt-ins remain ignored in the normal run.
119 local tests passed separately: the new material test and all 118 checks
retained from slice 052, including the three local GML tests. The package test
failed on the unregistered blinking strip before registration and passed
afterward. Missing face, knee and lowered-hand seeds each generated successfully,
then failed the intended material assertion in three separate negative controls.

All 16,868 combined variants validate. All 292 standalone Landen variants pass
strict recipe validation and match the candidate and combined build. The 620
accepted Landen PNG/metadata files and 41,615 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Landen's expected total.

The original archive's native animator and NPC object pass 215 frame/palette
observations, 75 linear completions and 80 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, frame phase, wrapper idempotence, cycle counters, seated flags,
completion and Summer portrait synchronization. Natural scheduling,
outfit/interaction dispatch and full-engine rendering remain untested.

All 20 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 7,766,304 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-053-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-053-`; author records use
`tmp/landen-summer-actions-author-`. Fresh exports are in
`extracted/landen-summer-actions-study`, reached through
`extracted/test-corpus/landen`. The fixed prior-art baseline is
`generated/slice-052-build/characters/landen`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Landen's Spring folder is complete. Summer has four remaining strips: three
directional general actions and the hammer animation. Autumn and Winter each
have 21. The inventory is `tmp/slice-053-remainder.json`.
After artwork review, finish Summer with those four strips.
