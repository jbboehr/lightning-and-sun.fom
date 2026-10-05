# Nora Winter general actions

Following `fe21cca`, this slice adds three Winter general-action strips to Nora's
accepted 139 sources: North/South/East. The 21 source frames and seven native
West mirrors give 28 review cases, approved by the user on 2026-10-05.
Completed outputs use `generated/review/` and `generated/build/`.

Nora has 142 sources; the 36-character collection has 4,373 sources and 17,492
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-080-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 139 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 061](slice-061.md). Its 251 seeds select 521 skin pixels across 21
frames: face, ears, neck and moving hands. Blonde hair, eye detail, blue scarf,
pale coat, purple skirt, brown boots and pink earrings stay original. The boots
cover the ankles in these strips.
Every frame was inspected in Vanilla/Blue contact sheets and five-choice
views, with source grids around the raised hands and hair.

The first three world shades are skin throughout these exact strips. The
deeper shared shade `#762E21` belongs to exposed skin contours from row 42.
North-facing hands also use this shade at row 41, including `[45,41]` in
frames 1 and 6 (one-based frames, frame-local coordinates). Hair uses this
shade through row 40 and stays original. Clothing, boots and earrings use
other shades. These are distinctions between pinned source pixels, not new
runtime rules.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Each strip has seven frames at 0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds.
The native cycle is linear and standing, with North/South/East directions,
South as its default and West mirroring East. The 240–360-tick final-frame
hold range and pause/speaking settings remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-081-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-081-review/index.html).

The summary samples East frame 2, North frame 3 and South frame 4. The full
gallery includes every source frame and native West mirror, with Vanilla
on the left and Debug Blue on the right. It includes complete character
views and enlarged face, hand and boot details. Its ten pages and images
total 517,510 bytes (about 505 KiB). Game-derived images and packages remain
local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 293 regular tests and the
release build passed; 363 local opt-ins remain ignored in the normal run.
149 local tests passed separately: the new material test and all 148 checks
retained from slice 080, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered Winter general-action strip
before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, a missing raised-hand contour,
a seed spilling onto hair and omitted North-facing neck selections.

All 17,492 combined variants validate. All 568 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 1,390
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 140 frame/palette
observations, 20 linear-cycle completions and 40 final-frame hold checks with
simulated engine services and direct cycle selection. Checks cover native
mirroring, palette binding, frame phase, wrapper idempotence, standing status,
cycle counters, completion and Winter portrait synchronization. These strips
have no complex transitions. Natural scheduling, speaking interruptions,
seasonal rollover, outfit/interaction dispatch and full-engine rendering remain
untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 5,220,384 rendered pixels. The short
review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-081-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-081-`; author records use
`tmp/nora-winter-general-actions-author-`. Fresh exports are in
`extracted/nora-winter-general-actions-study`, reached through
`extracted/test-corpus/nora`. The fixed prior-art baseline is
`generated/slice-080-build/characters/nora`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Nora's Spring, Summer and Autumn sprite folders remain complete. Winter has
nine remaining strips: seated writing, tapping and sweeping, each with
start/loop/end stages. The inventory is `tmp/slice-081-remainder.json`.
After artwork review, continue with Winter seated writing.
