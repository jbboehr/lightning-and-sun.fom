# Holt Summer blinking and seated actions

Following `74b2b4f`, this slice adds eleven Summer strips to Holt's accepted
68 sources: blinking East/South and sitting, eating and drinking North/South/East.
The 31 source frames and 12 native West mirrors give 43 review cases. The user
approved the artwork on 2026-10-06. Completed outputs use `generated/review/` and `generated/build/`.

Holt has 79 sources; the 36-character collection has 4,429 sources and 17,716
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-091-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 68 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md), plus the existing portrait shadow for ten dark hand
contour pixels. Its 647 component seeds select 1,337 skin pixels across 31
frames: face, ears, neck, eyelids, forearms and moving hands. Ginger hair,
moustache, eye detail, pink shirt, green apron, footwear and mouth interiors
stay original. Every frame was inspected in Vanilla/Blue contact sheets, with
five-choice views and labeled source grids around the face, cuffs, hands,
mouth and footwear.

The first three world shades also occur in footwear. Seated East-facing shoes
start at row 47 and South-facing shoes at row 48. North-facing seated poses
show hands without visible footwear. Eating South's second frame lowers a
hand over the left shoe; its skin extends through row 49. The masks include
that hand while preserving the adjacent visible shoe pixels.

Ten `#762E21` pixels belong to hand contours: two in sitting North, one in each
North-facing eating/drinking frame, and one in each of eating South's first two
frames. They use the existing darkest target (`#445F83` in Debug Blue). Other
pixels of that shared source color retain their original appearance, including
hair and moustache edges. The `#AD6945` nose shade remains skin; red `#9E2626`
mouth interiors stay original. These observations describe the exact sources;
tracked masks contain explicit component seeds and hashes, not a new runtime
rule. No earlier artwork or mappings change.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Blinking has three frames with 0.075/0.125/0.075-second durations. Drinking and
North-facing eating have three one-second frames. East/South eating has five
frames with 0.125/0.15/0.175/0.125/0.6-second durations; sitting has one frame.
All cycles are linear. Seated status and eating/drinking's 240–360-tick
final-frame holds are preserved; West mirrors East.

## Offline review

- [Five-choice sample summary](../../generated/slice-092-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-092-review/index.html).

The summary samples blinking South, eating East and drinking North. The full
gallery includes every frame and native West mirror, with Vanilla left and
Debug Blue right, full character views and enlarged face, hand and footwear
details. Every opaque pixel fits the full-view crop. Its 20 pages and images
total 776,041 bytes (about 758 KiB). Game-derived images and packages remain
local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 304 regular tests and the
release build passed; 374 local opt-ins remain ignored in the normal run.
161 local tests passed separately: the new material test and all 160 checks
retained from slice 091, including Holt's portrait and earlier world tests and
three local GML tests. The package test failed on the unregistered blinking
strip before registration and passed afterward.

Five negative controls generated successfully and then failed the intended
material assertion: omitted face/eyelid selections, the missing lowered hand
in eating South's second frame, a seed spilling onto footwear, a seed spilling
onto the moustache edge and an omitted dark hand contour in sitting North.

All 17,716 combined variants validate. All 316 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 680
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 215 frame/palette
observations, 75 linear completions and 80 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, frame phase, wrapper idempotence, seated status, cycle counters,
completion and Summer portrait synchronization. Natural scheduling,
outfit/interaction dispatch and full-engine rendering remain untested.

All 20 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 7,766,304 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-092-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-092-`; author records use
`tmp/holt-summer-actions-author-`. Fresh exports are in
`extracted/holt-summer-actions-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-091-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring sprite folder is complete. Summer has ten remaining strips;
Autumn and Winter each have 27. The inventory is `tmp/slice-092-remainder.json`.
Continue Summer with general actions before special activities.
