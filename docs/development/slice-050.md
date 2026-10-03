# Landen Spring actions and shocked reaction

Following `9a1c6bd`, this slice adds six Spring strips to Landen's accepted 49
sources: three directional general actions and the shocked start/loop/end
sequence. The 24 source frames and seven native West mirrors give 31 review
cases. Artwork was approved by the user on 2026-10-03. Completed outputs use `generated/review/`
and `generated/build/`.

Landen has 55 sources; the 36-character collection has 4,199 sources and 16,796
variants. L retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only the new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Landen coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-049-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 49 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 048](slice-048.md). The 472 new seeds select 1,056 skin pixels across
these frames, including face, ears, neck, eyelids and moving hands. The 190 shoe
pixels that share those shades remain unselected. Gray hair, eye detail, scarf,
purple shirt, trousers and the shocked mouth interior stay original.

All exposed skin lies above row 49 in these six pinned strips, including the
jumping shocked pose; shoe shades are below that boundary. The tracked masks
use connected-component seeds. The material test independently checks the
boundary, literal source landmarks, every output pixel, per-frame selected
counts, transparency and metadata. Ryis changes 785 of the 1,056 selected pixels
because two target shades already match the source; selected pixels are counted
separately from changed pixels. All 24 frames were inspected in source grids
and five-choice contact sheets.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Each directional action has seven frames with durations
0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds. Its native cycle holds the final frame
for 240–360 ticks. West mirrors East. The Spring-only shocked cycle retains its
three one-frame start, loop and end strips and the native complex-cycle order.

## Offline review

- [Five-choice sample summary](../../generated/slice-050-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-050-review/index.html).

The summary samples actions East and North and the shocked loop pose. The full
gallery includes every frame and native West mirror, with complete character
crops and enlarged details. Its 13 pages total 570,688 bytes (about 557 KiB).
Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 262 regular tests and the
release build passed; 332 local opt-ins remain ignored in the normal run.
116 local tests passed separately: the new material test and all 115 checks
retained from slice 049, including the three local GML tests. The package test
failed on the unregistered action strip before registration and passed afterward.
A missing skin seed and an extra shoe seed each generated successfully, then
failed the intended material assertion.

All 16,796 combined variants validate. All 220 standalone Landen variants pass
strict recipe validation and match the candidate and combined build. The 490
accepted Landen PNG/metadata files and 41,615 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Landen's expected total.

The original archive's native animator and NPC object pass 155 frame/palette
observations, 20 linear completions, 40 final-frame hold checks and 20 complex
transitions with simulated engine services and direct cycle selection. Checks
cover native mirroring, palette binding, frame phase, wrapper idempotence,
cycle counters, completion and portrait synchronization. Natural scheduling,
outfit/interaction dispatch and full-engine rendering remain untested.

All 13 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character crop,
enlarged detail and all 15 summary bindings: 5,729,568 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-050-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-050-`; author records use
`tmp/landen-spring-reactions-author-`. Fresh exports are in
`extracted/landen-spring-reactions-study`, reached through
`extracted/test-corpus/landen`. The fixed prior-art baseline is
`generated/slice-049-build/characters/landen`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

One Landen Spring strip remains: the hammer animation. Summer, Autumn and Winter
each have 21 remaining strips. The inventory is `tmp/slice-050-remainder.json`.
After artwork review, finish Spring with the hammer, then start seasonal movement.
