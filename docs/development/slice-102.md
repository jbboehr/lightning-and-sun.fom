# Holt Autumn sweeping

Following `3ea06ad`, this slice adds Holt's three Autumn sweeping strips to his
accepted 113 sources, completing all 27 Autumn sprites. The twelve South-facing
source frames give twelve review cases. The user approved the artwork on 2026-10-07.
Completed outputs use `generated/review/` and `generated/build/`.

Holt has 116 sources; the 36-character collection has 4,466 sources and 17,864
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-101-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 113 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 271 component seeds select 599 skin pixels:
face, ears, neck and moving hands. Every frame was inspected in five-choice
contact sheets, with labeled source grids around the face, hands, green sleeves,
broom, dust and boots. Per-frame selections are 48 for end, 43 for start,
and 48/56/52/48/49/49/56/52/49/49 for the loop.

The four skin shades occur above row 51; footwear reuses them below and stays
original. Autumn's gripping hands use these same skin shades. The ten dark
`#762E21` hand-edge pixels from Summer are absent here: the matching Autumn
positions use `#96523A` or `#F0CC8E`, while the neighboring sleeves are green.
Autumn sleeves also cover more skin in several poses. Its source pixels determine
the mask; the Summer selections are not copied. The `#762E21` hair and moustache
details stay original, along with eyes, green shirt, brown apron, pink trim,
boots, broom and dust. The broom uses distinct brown and gold shades; gray dust
stays unchanged. These observations describe the pinned source; the tracked
profile contains component seeds and hashes, not new runtime rules.

The material test independently checks reviewed source shades, literal
landmarks (including gripping-hand edges and their neighboring sleeve pixels),
every output pixel, per-frame selected counts, transparency and metadata.
Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Start and end each have one frame with no explicit duration; the loop has ten
frames at 0.1 seconds each. The native `sweep` cycle is complex, standing and
South-facing, with start/loop/end stages and `on_pause_speaking = "idle"`.

## Offline review

- [Five-choice sample summary](../../generated/slice-102-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-102-review/index.html).

The summary samples the start and loop frames 2 and 7. The full gallery includes
all twelve source frames, with Vanilla left and Debug Blue right, full character
views and enlarged face, hand, clothing and broom details. The full views keep
all broom and dust pixels in frame. Game-derived images and packages remain
local and ignored by Git.
The seven pages and images total 279,909 bytes (about 273 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 314 regular tests and the
release build passed; 384 local opt-ins remain ignored in the normal run.
171 local tests passed separately: the new material test and all 170 checks
retained from slice 101, including Holt's portrait and earlier world tests and
three local GML tests. The package test failed on an unregistered sweeping
strip before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, an omitted hand, a seed spilling
onto footwear and a seed spilling onto a moustache edge.

All seven gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full character view, enlarged detail
and all 15 summary bindings: 2,742,624 rendered pixels.

All 17,864 combined variants validate. All 464 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 1,130
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 60 frame/palette
observations and 20 complex stage transitions with simulated engine services and
direct cycle selection. Checks cover palette binding, frame phase, wrapper
idempotence, standing status, cycle counters, completion and Autumn portrait
synchronization. This South-facing cycle has no native West mirror, linear
completion or final-frame hold. Natural scheduling, pause/interaction dispatch
and full-engine rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-102-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. The short review path also passes Chromium
loading and local link resolution. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-102-`; author records use
`tmp/holt-autumn-sweeping-author-`. Fresh exports are in
`extracted/holt-autumn-sweeping-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-101-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring, Summer and Autumn sprite folders are complete. Winter has 27
remaining strips, recorded in `tmp/slice-102-remainder.json`.
Next: Winter standing and walking.
