# Landen Autumn standing and walking

Following `3703e41`, this slice adds six Autumn strips to Landen's accepted 77
sources: standing and walking North/South/East. The 15 source frames and five
native West mirrors give 20 review cases, approved by the user on 2026-10-03.
Completed outputs use `generated/review/` and `generated/build/`.

Landen has 83 sources; the 36-character collection has 4,227 sources and 16,908
variants. L retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Landen coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-054-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 77 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 048](slice-048.md). Its 319 seeds select 652 skin pixels across the 15
frames: face, ears, neck and moving hands. All four shades represent skin
throughout these six pinned Autumn strips. Gray hair, eye detail, purple shirt
and its pale trim, trousers and shoes stay original. Every frame was inspected
in Vanilla/Blue contact sheets, with source grids for material boundaries and
five-choice walking samples.

The tracked masks use connected-component seeds. The material test independently
checks reviewed source shades, literal landmarks, every output pixel, per-frame
selected counts, transparency and metadata. Selected pixels are counted
separately from changed pixels because two Ryis target shades match the source.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Standing strips have one frame; walking strips have four, with 0.15-second
frame durations. Both native cycles are linear; West mirrors East.

## Offline review

- [Five-choice sample summary](../../generated/slice-055-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-055-review/index.html).

The summary samples standing South, walking East and walking North. The full
gallery includes every frame and native West mirror, with complete character
views and enlarged details. Its 10 pages total 396,671 bytes (about 387 KiB).
Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 267 regular tests and the
release build passed; 337 local opt-ins remain ignored in the normal run.
121 local tests passed separately: the new material test and all 120 checks
retained from slice 054, including three local GML tests. The package test failed
on the unregistered standing strip before registration and passed afterward.
Missing face and hand seeds each generated successfully, then failed the
intended material assertion in two separate negative controls.

All 16,908 combined variants validate. All 332 standalone Landen variants pass
strict recipe validation and match the candidate and combined build. The 770
accepted Landen PNG/metadata files and 41,615 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Landen's expected total.

The original archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear completions with simulated engine services and direct
cycle selection. Checks cover native mirroring, palette binding, frame phase,
wrapper idempotence, cycle counters, completion and Autumn portrait
synchronization. Natural scheduling, outfit dispatch and full-engine rendering
remain untested.

All 10 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-055-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-055-`; author records use
`tmp/landen-autumn-world-author-`. Fresh exports are in
`extracted/landen-autumn-world-study`, reached through
`extracted/test-corpus/landen`. The fixed prior-art baseline is
`generated/slice-054-build/characters/landen`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Landen's Spring and Summer folders are complete. Autumn has 15 remaining strips;
Winter has 21. The inventory is `tmp/slice-055-remainder.json`.
After artwork review, continue Autumn with blinking and seated actions.
