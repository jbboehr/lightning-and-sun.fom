# Landen Summer general actions and hammer

Following `e0b03dd`, this slice adds four Summer strips to Landen's accepted 73
sources: general actions North/South/East and the East-facing hammer swing.
The 28 source frames and 14 native West mirrors give 42 review cases, completing
all 21 Summer strips. Artwork was approved by the user on 2026-10-03. Completed outputs use
`generated/review/` and `generated/build/`.

Landen has 77 sources; the 36-character collection has 4,221 sources and 16,884
variants. L retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only the new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Landen coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-053-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 73 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 048](slice-048.md). Its 609 seeds select 1,248 skin pixels across the 28
frames: face, ears, neck, moving hands and exposed knees. All four shades
represent skin throughout these four pinned Summer strips. Gray hair, eye
detail, blue patterned shirt, orange shorts, dark socks, orange shoes, hammer
and swing effects stay original. All frames were inspected in Vanilla/Blue
contact sheets, with source grids for material boundaries and five-choice
hammer sheets and summary.

The tracked masks use connected-component seeds. The material test independently
checks the reviewed source shades, literal landmarks, every output pixel,
per-frame selected counts, transparency and metadata. Selected pixels are
counted separately from changed pixels because two Ryis target shades already
match the source. The hammer uses the native `specialanimation` filename prefix.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Each strip has seven frames. The directional actions retain their
0.1/0.25/0.25/0.25/0.25/0.1/0.4-second timing and 240–360-tick final-frame hold.
The hammer retains its 0.1-second frame duration. Both native cycles are linear;
West mirrors the East sprite for each.

## Offline review

- [Five-choice sample summary](../../generated/slice-054-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-054-review/index.html).

The summary samples actions South and North and the hammer strike. The full
gallery includes every frame and native West mirror, with complete character
views including the hammer and swing effects, plus enlarged details. Its 14
pages total 725,306 bytes (about 708 KiB). Game-derived images and packages
remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 266 regular tests and the
release build passed; 336 local opt-ins remain ignored in the normal run.
120 local tests passed separately: the new material test and all 119 checks
retained from slice 053, including the three local GML tests. The package test
failed on the unregistered hammer strip before registration and passed afterward.
Missing face, knee and raised-hand seeds each generated successfully, then
failed the intended material assertion in three separate negative controls.

All 16,884 combined variants validate. All 308 standalone Landen variants pass
strict recipe validation and match the candidate and combined build. The 730
accepted Landen PNG/metadata files and 41,615 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Landen's expected total.

The original archive's native animator and NPC object pass 210 frame/palette
observations, 30 linear completions and 40 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, frame phase, wrapper idempotence, cycle counters, completion
and Summer portrait synchronization. Natural scheduling, outfit/interaction
dispatch and full-engine rendering remain untested.

All 14 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 6,710,784 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-054-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-054-`; author records use
`tmp/landen-summer-finish-author-`. Fresh exports are in
`extracted/landen-summer-finish-study`, reached through
`extracted/test-corpus/landen`. The fixed prior-art baseline is
`generated/slice-053-build/characters/landen`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Landen's Spring and Summer folders are complete. Autumn and Winter each have
21 remaining strips. The inventory is `tmp/slice-054-remainder.json`.
After artwork review, start Autumn with standing and walking.
