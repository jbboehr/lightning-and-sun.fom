# Nora Winter standing and walking

Following `3e3c562`, this slice adds six Winter strips to Nora's accepted 122
sources: standing and walking North/South/East. The 15 source frames and five
native West mirrors give 20 review cases, approved by the user on 2026-10-05.
Completed outputs use `generated/review/` and `generated/build/`.

Nora has 128 sources; the 36-character collection has 4,359 sources and 17,436
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-078-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 122 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 061](slice-061.md). Its 187 seeds select 400 skin pixels across 15 frames:
face, ears, neck and moving hands. Blonde hair, eye detail, blue scarf, pale
coat, purple skirt, brown boots and pink earrings stay original. There are no
exposed ankles in these strips. Every frame was inspected in Vanilla/Blue
contact sheets and five-choice sheets, with source grids around the hands,
neck, hair and boots.

The first three world shades are skin throughout these exact strips. The
deeper shared shade `#762E21` belongs to hair through row 40 and to exposed
hand contours from row 47. Clothing and boot shades are distinct from the
selected skin colors. Tracked masks contain component seeds and hashes,
not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Standing strips each have one frame and no duration field. Walking strips
each have four frames at 0.15 seconds per frame. Both native cycles are linear,
with North/South/East directions and West mirroring East. All metadata and
pause/speaking settings remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-079-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-079-review/index.html).

The summary samples standing South and walking East/North. The full gallery
includes every source frame and native West mirror, with complete character
views and enlarged face, hand and boot details. Its ten pages and images total
411,390 bytes (about 402 KiB). Game-derived images and packages remain local
and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 291 regular tests and the
release build passed; 361 local opt-ins remain ignored in the normal run.
147 local tests passed separately: the new material test and all 146 checks
retained from slice 078, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered Winter standing strip before
registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, a missing dark hand contour,
a seed spilling onto moving hair and omitted North-facing neck selections.
An initial East-facing neck control removed a redundant seed without changing
the mask, so it was replaced with the North-facing component omission.

All 17,436 combined variants validate. All 512 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 1,220
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear-cycle completions with simulated engine services
and direct cycle selection. Checks cover native mirroring, palette binding,
frame phase, wrapper idempotence, standing status, cycle counters, completion
and Winter portrait synchronization. These strips have no final-frame holds or
complex transitions. Natural scheduling, speaking interruptions, seasonal
rollover, outfit/interaction dispatch and full-engine rendering remain untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels. The short
review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-079-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-079-`; author records use
`tmp/nora-winter-world-author-`. Fresh exports are in
`extracted/nora-winter-world-study`, reached through `extracted/test-corpus/nora`.
The fixed prior-art baseline is `generated/slice-078-build/characters/nora`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Nora's Spring, Summer and Autumn sprite folders remain complete. Winter has
23 remaining strips. The inventory is `tmp/slice-079-remainder.json`.
After artwork review, continue Winter with blinking, sitting, eating and drinking.
