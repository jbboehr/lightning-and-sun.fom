# Nora Autumn standing and walking

Following `13378b2`, this slice adds six Autumn strips to Nora's accepted 93
sources: standing and walking North/South/East. The 15 source frames and five
native West mirrors give 20 review cases, approved by the user on 2026-10-04.
Completed outputs use `generated/review/` and `generated/build/`.

Nora has 99 sources; the 36-character collection has 4,330 sources and 17,320
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-072-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 93 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 061](slice-061.md). Its 297 seeds select 493 skin pixels across 15 frames:
face, ears, neck, moving hands and exposed ankles. Blonde hair, eye detail,
blue shirt, pale apron, purple trousers and shoes stay original. Every frame
was inspected in Vanilla/Blue contact sheets, with a five-choice North-walking
sheet and source grids around the hands, hair, trousers and ankles.

The first three world shades are skin throughout these exact strips. The
deeper shared shade `#762E21` belongs to hair through row 41 and to exposed
hand contours from row 47. Eight hair-tip pixels reach row 41 in walking
East/South frames 2 and 4. The initial mask incorrectly recolored those tips;
the material regression failed at `walk_east [119,41]` before the mask was
corrected, then passed. The pale apron and purple trouser shades are distinct
from skin and remain unchanged.
Tracked masks contain component seeds and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Standing strips each have one frame and no duration field. Walking strips
each have four frames at 0.15 seconds per frame. Both native cycles are linear,
with North/South/East directions and West mirroring East. All metadata and
pause/speaking settings remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-073-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-073-review/index.html).

The summary samples standing South and walking East/North. The full gallery
includes every source frame and native West mirror, with complete character
views and enlarged face, hand and ankle details. Its ten pages and images total
404,514 bytes (about 395 KiB). Game-derived images and packages remain local
and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 285 regular tests and the
release build passed; 355 local opt-ins remain ignored in the normal run.
141 local tests passed separately: the new material test and all 140 checks
retained from slice 072, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered Autumn standing strip before
registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, a missing dark hand contour,
a seed spilling onto a moving hair tip and omitted ankle selections.

All 17,320 combined variants validate. All 396 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 930
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear-cycle completions with simulated engine services
and direct cycle selection. Checks cover native mirroring, palette binding,
frame phase, wrapper idempotence, standing status, cycle counters, completion
and Autumn portrait synchronization. These strips have no final-frame holds or
complex transitions. Natural scheduling, speaking interruptions, seasonal
rollover, outfit/interaction dispatch and full-engine rendering remain untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels. The short
review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-073-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-073-`; author records use
`tmp/nora-autumn-world-author-`. Fresh exports are in
`extracted/nora-autumn-world-study`, reached through `extracted/test-corpus/nora`.
The fixed prior-art baseline is `generated/slice-072-build/characters/nora`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Nora's Spring and Summer sprite folders remain complete. Autumn has 23 remaining
strips; Winter has 29. The inventory is `tmp/slice-073-remainder.json`.
After artwork review, continue Autumn with blinking, sitting, eating and drinking.
