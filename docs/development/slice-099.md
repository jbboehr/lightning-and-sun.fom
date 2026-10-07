# Holt Autumn general actions

Following `085f187`, this slice adds Holt's three Autumn general-action strips
to his accepted 106 sources. The 21 source frames and seven native West mirrors
give 28 review cases. The user approved the artwork on 2026-10-07. Completed outputs use
`generated/review/` and `generated/build/`.

Holt has 109 sources; the 36-character collection has 4,459 sources and 17,836
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-098-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 106 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md), plus the existing `#762E21` shadow mapping used in
[slice 097](slice-097.md). Its 336 component seeds select 649 skin pixels:
face, ears, neck, nose edge and moving hands. Every source frame was inspected
in five-choice contact sheets, with source grids around hands, sleeves, boots
and shared dark shades.

Boots reuse three of the primary skin shades from row 49 downward; separate
selections preserve them. One pixel at local (45,45) in East frame 7 uses
`#762E21` on the hand edge. The initial four-shade candidate missed it; a precise
selection now includes it. Other uses of that brown in hair and moustache remain
original. These selections follow the actual Autumn source, including the green
sleeves and hands below the apron.

Ginger hair, moustache, eyes, green shirt, brown apron, pink trim and boots remain
original. These observations describe the pinned source; the tracked profile
contains component seeds and hashes, not new runtime rules. The material test
independently checks reviewed source shades, literal landmarks, every output
pixel, per-frame selected counts, transparency and metadata. Selected pixels
are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
All three strips have seven frames at 0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds.
The native standing action cycle retains North/South/East directions, East-to-West
mirroring, final-frame holds of 240–360 ticks and idle on speaking pause.

## Offline review

- [Five-choice sample summary](../../generated/slice-099-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-099-review/index.html).

The summary samples East frame 2, North frame 6 and South frame 3.
The full gallery includes all 21 source frames and seven native West mirrors,
with Vanilla left and Debug Blue right, full character views and enlarged face,
hand, clothing and boot details. Game-derived images and packages remain local
and ignored by Git.
The ten pages and images total 509,048 bytes (about 497 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 311 regular tests and the
release build passed; 381 local opt-ins remain ignored in the normal run.
168 local tests passed separately: the new material test and all 167 checks
retained from slice 098, including Holt's portrait and earlier world tests and
three local GML tests. The package test failed on an unregistered Autumn action
strip before registration and passed afterward.

Five negative controls generated successfully and then failed the intended
material assertion: omitted face selections, omitted hand selections, a seed
spilling onto footwear, a seed spilling onto a moustache edge and an omitted
dark hand contour. The last control catches the initial candidate's gap without
changing the shared dark hair color.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, native West mirror, metadata binding, full character
view, enlarged detail and all 15 summary bindings: 5,220,384 rendered pixels.

All 17,836 combined variants validate. All 436 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 1,060
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 140 frame/palette
observations, 20 linear completions and 40 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, frame phase, wrapper idempotence, standing status, cycle
counters, completion and Autumn portrait synchronization. This cycle has no
complex stage transitions. Natural scheduling, pause/interaction dispatch and
full-engine rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-099-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. The short review path also passes Chromium
loading and local link resolution. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-099-`; author records use
`tmp/holt-autumn-general-author-`. Fresh exports are in
`extracted/holt-autumn-general-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-098-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring and Summer sprite folders are complete. Autumn now covers 20 of
27 strips; seven Autumn and 27 Winter strips remain, recorded in
`tmp/slice-099-remainder.json`. Next: Autumn standing writing.
