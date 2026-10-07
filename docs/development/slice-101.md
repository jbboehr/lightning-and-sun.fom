# Holt Autumn whittling

Following `ac8a652`, this slice adds Holt's Autumn whittling strip to his accepted
112 sources. The nine source frames and nine native West mirrors give 18 review
cases. The user approved the artwork on 2026-10-07. Completed outputs use `generated/review/`
and `generated/build/`.

Holt has 113 sources; the 36-character collection has 4,463 sources and 17,852
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only one region and registry entry are added.
Source colors, color groups, target mappings, collection sets and prior regions
stay unchanged. No runtime code changes. Historical pixel tests retain their
fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-100-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 112 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 147 component seeds select 304 skin pixels across
the nine frames: face, ears, neck, nose edge and moving hands.
Ginger hair, moustache, eye detail, green shirt, brown apron, pink trim, boots, wood,
flying chips and knife stay original. Every frame was inspected in five-choice
contact sheets, with labeled source grids around the hands, face, knife, wood
highlights, flying chips and footwear.

Skin remains above row 51 in this pinned strip. Footwear reuses skin shades
from row 51 downward and stays original. The `#AD6945` nose shade remains skin;
`#762E21` hair/moustache detail remains original. The dark hand contours found
in other Autumn poses do not occur in these hands. The wood's light `#E7BA83` and
brown `#BB8151`/`#936244` shades are distinct from skin; the flying chips use
`#C09165`/`#764B2D`. The knife's pale `#DAE2E7` and grey `#6C7781` shades also
stay unchanged. These observations describe the exact source; the tracked mask
contains component seeds and a hash, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
The nine-frame strip uses 0.1 seconds per frame except its second frame at
0.15 seconds. The native `whittle` cycle is linear and standing, defaults to
East and mirrors East for West. Its metadata remains unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-101-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-101-review/index.html).

The summary samples frames 1, 5 and 9. The full gallery includes every source
frame and native West mirror, with Vanilla left and Debug Blue right, full
character views and enlarged face, hand, footwear, wood, chip and knife details.
Game-derived images and packages remain local and ignored by Git.
The flying chips remain inside both the full-frame and enlarged crops. Its
eight pages and images total 363,321 bytes (about 355 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 313 regular tests and the
release build passed; 383 local opt-ins remain ignored in the normal run.
170 local tests passed separately: the new material test and all 169 checks
retained from slice 100, including Holt's portrait and earlier world tests and
three local GML tests.
The package test failed on the unregistered whittling strip before registration
and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, an omitted hand, a seed spilling
onto footwear and a seed spilling onto a moustache edge.

All eight gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 3,942,432 rendered pixels.

All 17,852 combined variants validate. All 452 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 1,120
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 90 frame/palette
observations and ten linear completions with simulated engine services and
direct cycle selection. Checks cover native mirroring, palette binding, frame
phase, wrapper idempotence, standing status, cycle counters, completion and
Autumn portrait synchronization. This cycle has no final-frame hold or complex
stage transitions. Natural scheduling, pause/interaction dispatch and full-engine
rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-101-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. The short review path also passes Chromium
loading and local link resolution. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-101-`; author records use
`tmp/holt-autumn-whittling-author-`. Fresh exports are in
`extracted/holt-autumn-whittling-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-100-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring and Summer sprite folders are complete. Autumn now covers 24 of
27 strips; only sweeping's start/loop/end South remain. Winter has 27 strips.
The inventory is `tmp/slice-101-remainder.json`. Complete Autumn with sweeping.
