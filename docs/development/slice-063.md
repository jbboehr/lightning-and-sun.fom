# Nora Spring general actions and shocked reaction

Following `d9a1884`, this slice adds six Spring strips to Nora's accepted 49
sources: general actions North/South/East and shocked start/loop/end South.
The 24 source frames and seven native West mirrors give 31 review cases,
approved by the user on 2026-10-03. Completed outputs use `generated/review/` and
`generated/build/`.

Nora has 55 sources; the 36-character collection has 4,286 sources and 17,144
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-062-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 49 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 061](slice-061.md). Its 455 seeds select 839 skin pixels across the 24
frames: face, ears, neck, moving hands and exposed ankles. Blonde hair, eye
detail, pale blouse, blue overalls, shoes and mouth interiors stay original.
Every frame was inspected in Vanilla/Blue contact sheets, with a five-choice
shocked-loop sheet and source grids around raised hands, hair, cuffs and mouth.

The first three world shades are skin throughout these six pinned strips.
The deepest shade, `#762E21`, belongs to hair above row 41 and hands from row 41
onward. Two North-facing action frames place a hand-edge pixel at `[45,41]`;
those pixels are explicitly included. Hair tips at row 40 remain original,
including during the shocked start/end poses. This boundary describes these
exact sources; the tracked masks contain component seeds and hashes, not a new
runtime rule. Mouth-interior colors `#410808` and `#9E2626` remain unchanged.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
General actions have seven frames with 0.1/0.25/0.25/0.25/0.25/0.1/0.4-second
durations and native 240–360-tick final-frame holds. West mirrors East.
The shocked reaction uses the native South-only start/loop/end sequence, with
one frame in each strip. Its stages and cycle metadata remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-063-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-063-review/index.html).

The summary samples general actions East/North and the shocked loop. The full
gallery includes every frame, native West mirror and shocked stage, with
complete character views and enlarged face, hand and ankle details. Its 13
pages and images total 575,035 bytes (about 562 KiB). Game-derived images and packages
remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 275 regular tests and the
release build passed; 345 local opt-ins remain ignored in the normal run.
131 local tests passed separately: the new material test and all 130 checks
retained from slice 062, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered general-action strip before
registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted shocked-face selections, a missing raised-hand
contour, a seed spilling onto a hair tip and omitted ankle selections.

All 17,144 combined variants validate. All 220 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 490
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 155 frame/palette
observations, 20 linear completions, 40 final-frame hold checks and 20 shocked
stage transitions with simulated engine services and direct cycle selection.
Checks cover native mirroring, palette binding, frame phase, wrapper
idempotence, seated status, cycle counters, completion and Spring portrait
synchronization. Natural scheduling, outfit/interaction dispatch and full-engine
rendering remain untested.

All 13 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 5,729,568 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-063-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-063-`; author records use
`tmp/nora-spring-reactions-author-`. Fresh exports are in
`extracted/nora-spring-reactions-study`, reached through
`extracted/test-corpus/nora`. The fixed prior-art baseline is
`generated/slice-062-build/characters/nora`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Nora's Spring folder has nine remaining strips: seated writing, tapping and
sweeping, each with start/loop/end stages. Summer, Autumn and Winter each have
29. The inventory is `tmp/slice-063-remainder.json`.
After artwork review, start the Spring special activities with seated writing.
