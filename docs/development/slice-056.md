# Landen Autumn blinking and seated actions

Following `d6280cd`, this slice adds eleven Autumn strips to Landen's accepted
83 sources: blinking East/South and sitting, eating and drinking North/South/East.
The 31 source frames and 12 native West mirrors give 43 review cases, approved
by the user on 2026-10-03. Completed outputs use `generated/review/` and
`generated/build/`.

Landen has 94 sources; the 36-character collection has 4,238 sources and 16,952
variants. L retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Landen coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-055-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 83 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 048](slice-048.md). Its 558 seeds select 1,359 skin pixels across the 31
frames: face, ears, neck, eyelids and moving hands. All four shades represent
skin throughout these eleven pinned Autumn strips. Gray hair, eye detail,
purple shirt and its pale trim, trousers, shoes and apple stay original.
Every frame was inspected in Vanilla/Blue contact sheets, with source grids
for blinking and the lowered hand beside the shoe in the second eating frame.

The tracked masks use connected-component seeds. The material test independently
checks reviewed source shades, literal landmarks, every output pixel, per-frame
selected counts, transparency and metadata. Selected pixels are counted
separately from changed pixels because two Ryis target shades match the source.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Blinking has three frames with 0.075/0.125/0.075-second durations. Drinking and
North-facing eating have three one-second frames. East/South eating has five
frames with 0.125/0.15/0.175/0.125/0.6-second durations; sitting has one frame.
All cycles are linear. Seated status and eating/drinking's 240–360-tick
final-frame holds are preserved; West mirrors East.

## Offline review

- [Five-choice sample summary](../../generated/slice-056-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-056-review/index.html).

The summary samples blinking South, eating East and drinking North. The full
gallery includes every frame and native West mirror, with complete character
views and enlarged details. Its 20 pages total 761,874 bytes (about 744 KiB).
Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 268 regular tests and the
release build passed; 338 local opt-ins remain ignored in the normal run.
122 local tests passed separately: the new material test and all 121 checks
retained from slice 055, including three local GML tests. The package test failed
on the unregistered blinking strip before registration and passed afterward.
Missing face and lowered-hand seeds each generated successfully, then failed
the intended material assertion in two separate negative controls.

All 16,952 combined variants validate. All 376 standalone Landen variants pass
strict recipe validation and match the candidate and combined build. The 830
accepted Landen PNG/metadata files and 41,615 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Landen's expected total.

The original archive's native animator and NPC object pass 215 frame/palette
observations, 75 linear completions and 80 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, frame phase, wrapper idempotence, seated status, cycle counters,
completion and Autumn portrait synchronization. Natural scheduling,
outfit/interaction dispatch and full-engine rendering remain untested.

All 20 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 7,766,304 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-056-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-056-`; author records use
`tmp/landen-autumn-actions-author-`. Fresh exports are in
`extracted/landen-autumn-actions-study`, reached through
`extracted/test-corpus/landen`. The fixed prior-art baseline is
`generated/slice-055-build/characters/landen`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Landen's Spring and Summer folders are complete. Autumn has four remaining
strips: general actions North/South/East and the hammer animation. Winter has
21. The inventory is `tmp/slice-056-remainder.json`.
After artwork review, finish Autumn with general actions and the hammer.
