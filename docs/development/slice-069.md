# Nora Summer general actions

Following `7ec52f1`, this slice adds three Summer general-action strips to Nora's
accepted 81 sources: North/South/East. The 21 source frames and seven native
West mirrors give 28 review cases, approved by the user on 2026-10-04.
Completed outputs use `generated/review/` and `generated/build/`.

Nora has 84 sources; the 36-character collection has 4,315 sources and 17,260
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-068-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 81 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 061](slice-061.md). Its 413 seeds select 728 skin pixels across 21
frames: face, ears, neck, moving hands and exposed ankles. Blonde hair,
eye detail, blue shirt, purple apron, pale trousers and shoes stay original.
Every frame was inspected in Vanilla/Blue contact sheets, with five-choice
North-action views and source grids around the raised hands and hair.

The first three world shades are skin throughout these exact strips. The
deeper shared shade `#762E21` belongs to exposed skin contours from row 41.
North-facing frames 3 and 5 also have a hand-edge pixel at `[45,40]` in
frame-local coordinates. Both follow the skin palette; hair tips at row 40
in the East/South strips remain original. A material test reproduced the
two missed hand edges before those component seeds were added. This is a
distinction between these pinned source pixels, not a new runtime rule.
The pale trouser shades `#E8E2D1`, `#B8A68F` and `#807467` remain unchanged.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Each strip has seven frames at 0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds.
The native cycle is linear and standing, with North/South/East directions,
South as its default and West mirroring East. The 240–360-tick final-frame
hold range and pause/speaking settings remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-069-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-069-review/index.html).

The summary samples East frame 2, North frame 3 and South frame 4. The full
gallery includes every source frame and native West mirror, with complete
character views and enlarged face, hand and ankle details. Its ten pages
and images total 508,784 bytes (about 497 KiB). Game-derived images and packages
remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 281 regular tests and the
release build passed; 351 local opt-ins remain ignored in the normal run.
137 local tests passed separately: the new material test and all 136 checks
retained from slice 068, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered Summer general-action strip
before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, a missing raised-hand contour,
a seed spilling onto a hair tip and omitted ankle selections. The separate
raised-hand regression also failed before correcting the two row-40 pixels
and passed afterward.

All 17,260 combined variants validate. All 336 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 810
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 140 frame/palette
observations, 20 linear-cycle completions and 40 final-frame hold checks with
simulated engine services and direct cycle selection. Checks cover native
mirroring, palette binding, frame phase, wrapper idempotence, standing status,
cycle counters, completion and Summer portrait synchronization. These strips
have no complex transitions. Natural scheduling, speaking interruptions,
seasonal rollover, outfit/interaction dispatch and full-engine rendering remain
untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 5,220,384 rendered pixels. The short
review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-069-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-069-`; author records use
`tmp/nora-summer-general-actions-author-`. Fresh exports are in
`extracted/nora-summer-general-actions-study`, reached through
`extracted/test-corpus/nora`. The fixed prior-art baseline is
`generated/slice-068-build/characters/nora`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Nora's Spring sprite folder remains complete. Summer has nine remaining strips:
seated writing, tapping and sweeping, each with start/loop/end stages. Autumn
and Winter each have 29. The inventory is `tmp/slice-069-remainder.json`.
After artwork review, continue with Summer seated writing.
