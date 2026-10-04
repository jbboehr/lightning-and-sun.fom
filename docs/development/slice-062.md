# Nora Spring blinking and seated actions

Following `7ccaa93`, this slice adds eleven Spring strips to Nora's accepted
38 sources: blinking East/South and sitting, eating and drinking North/South/East.
The 31 source frames and 12 native West mirrors give 43 review cases, approved by the user on 2026-10-03. Completed outputs use `generated/review/` and
`generated/build/`.

Nora has 49 sources; the 36-character collection has 4,280 sources and 17,120
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-061-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 38 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 061](slice-061.md). Its 590 seeds select 1,065 skin pixels across the 31
frames: face, ears, neck, eyelids, moving hands and exposed ankles. Blonde hair,
eye detail, pale blouse, blue overalls, shoes and mouth interiors stay original.
Every frame was inspected in Vanilla/Blue contact sheets, with five-choice
eating frames and source grids around hair, cuffs, hands, ankles and mouths.

The first three world shades are skin throughout these eleven pinned strips.
The deepest shade, `#762E21`, belongs to hair above row 42 and hands from row 42
onward. The standing/walking authoring boundary from slice 061 would miss six
dark hand-edge pixels across eating and drinking frames. These are explicitly
included here, while bobbing hair tips at row 40 remain original. This boundary
describes these exact sources; the tracked masks contain component seeds and
hashes, not a new runtime rule. Mouth-interior colors `#410808` and `#9E2626`
remain unchanged.

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

- [Five-choice sample summary](../../generated/slice-062-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-062-review/index.html).

The summary samples blinking South, eating East and drinking North. The full
gallery includes every frame and native West mirror, with complete character
views and enlarged face, hand and ankle details. Its 20 pages and images total
774,552 bytes (about 756 KiB). Game-derived images and packages remain local and ignored
by Git.

## Verification

Formatting, release Clippy with warnings denied, all 274 regular tests and the
release build passed; 344 local opt-ins remain ignored in the normal run.
130 local tests passed separately: the new material test and all 129 checks
retained from slice 061, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered blinking strip before
registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face/eyelid selections, a missing raised-hand
contour, a seed spilling onto a lowered hair tip and omitted ankle selections.

All 17,120 combined variants validate. All 196 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 380
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 215 frame/palette
observations, 75 linear completions and 80 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, frame phase, wrapper idempotence, seated status, cycle counters,
completion and Spring portrait synchronization. Natural scheduling,
outfit/interaction dispatch and full-engine rendering remain untested.

All 20 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 7,766,304 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-062-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-062-`; author records use
`tmp/nora-spring-actions-author-`. Fresh exports are in
`extracted/nora-spring-actions-study`, reached through
`extracted/test-corpus/nora`. The fixed prior-art baseline is
`generated/slice-061-build/characters/nora`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Nora's Spring folder has 15 remaining strips: general actions, shocked reactions,
seated writing, tapping and sweeping. Summer, Autumn and Winter each have 29.
The inventory is `tmp/slice-062-remainder.json`.
After artwork review, continue Spring with general actions and shocked reactions.
