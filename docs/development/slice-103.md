# Holt Winter standing and walking

Following `e4c4d9b`, this slice adds Holt's six Winter standing and walking strips
to his accepted 116 sources. The fifteen source frames and five native West
mirrors give twenty review cases. The user approved the artwork on 2026-10-07.
Completed outputs use `generated/review/` and `generated/build/`.

Holt has 122 sources; the 36-character collection has 4,472 sources and 17,888
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only six regions and registry entries are added.
Source colors, color groups, target mappings, collection sets and prior regions
stay unchanged. No runtime code changes. Historical pixel tests retain their
fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-102-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 116 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 244 component seeds select 447 skin pixels:
face, ears, neck, nose edge and moving hands. Every source frame was inspected in
five-choice contact sheets, with labeled source grids around face, hands,
clothing and footwear.

Skin uses the four shades above row 49. Two East walking boot pixels reuse
`#96523A` below that boundary: local (37,50) in frame 2 and (42,52) in frame 4.
They stay original. Winter sleeves cover more skin than Autumn, including its
three dark East-view hand contours. Winter's `#762E21` belongs to hair and
moustache, including hair reaching down to rows 39/40 in North-facing poses.
The brown sleeve edge at local (45,45) in East standing uses `#622A10`, which
also appears in clothing and footwear; it stays original. Ginger hair, moustache,
eyes, orange plaid shirt, teal scarf, brown apron, green trousers and boots retain
their original colors. These observations describe the pinned source; the tracked
profile contains component seeds and hashes, not new runtime rules.

Per-frame selections are 34/16/39 for East/North/South standing, 34/38/34/32 for
East walking, 16/16/16/16 for North walking and 39/39/39/39 for South walking.
The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Standing has one frame per direction with no explicit duration; walking has
four per direction at 0.15 seconds each. The native `idle` and `walk` cycles are
linear and standing, default to South and mirror East for West. Their pause
configuration remains unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-103-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-103-review/index.html).

The summary samples South standing, East walking frame 2 and North walking
frame 2. The full gallery includes all fifteen source frames and five native
West mirrors, with Vanilla left and Debug Blue right, full character views and
enlarged face, hand, clothing and boot details. Game-derived images and packages
remain local and ignored by Git.
The ten pages and images total 398,108 bytes (about 389 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 315 regular tests and the
release build passed; 385 local opt-ins remain ignored in the normal run.
172 local tests passed separately: the new material test and all 171 checks
retained from slice 102, including Holt's portrait and earlier world tests and
three local GML tests. The package test failed on an unregistered Winter standing
strip before registration and passed afterward.

Five negative controls generated successfully and then failed the intended
material assertion: omitted face selections, an omitted hand, a seed spilling
onto footwear, an omitted nose edge and a seed spilling onto a moustache edge.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, native West mirror, metadata binding, full character
view, enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels.

All 17,888 combined variants validate. All 488 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 1,160
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear completions with simulated engine services and direct
cycle selection. Checks cover native mirroring, palette binding, frame phase,
wrapper idempotence, standing status, cycle counters, completion and Winter
portrait synchronization. These cycles have no final-frame hold or complex stage
transitions. Natural scheduling, pause/interaction dispatch and full-engine
rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-103-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. The short review path also passes Chromium
loading and local link resolution. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-103-`; author records use
`tmp/holt-winter-movement-author-`. Fresh exports are in
`extracted/holt-winter-movement-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-102-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring, Summer and Autumn sprite folders are complete. Winter now covers
six of 27 strips; 21 remain, recorded in `tmp/slice-103-remainder.json`.
Next: Winter blinking, sitting, eating and drinking.
