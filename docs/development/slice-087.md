# Holt Spring general actions and shocked reaction

Following `fd15339`, this slice adds six Spring strips to Holt's accepted 49
sources: general actions North/South/East and shocked start/loop/end South.
The 24 source frames and seven native West mirrors give 31 review cases,
approved by the user on 2026-10-05. Completed outputs use `generated/review/` and
`generated/build/`.

Holt has 55 sources; the 36-character collection has 4,405 sources and 17,620
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-086-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 49 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 423 component seeds select 863 skin pixels across
the 24 frames: face, ears, neck and moving hands, including the raised fingers
in the shocked loop. Ginger hair, moustache, eye detail, orange scarf, green
shirt, blue apron, trousers, boots and mouth interiors stay original.
Every frame was inspected in Vanilla/Blue contact sheets, with five-choice
samples and labeled source grids around raised hands, cuffs, face and boots.

Skin remains above row 49 in these six pinned sources. Boots reuse three skin
shades below that boundary, including the outstretched boots in the shocked
loop. Both raised hands include their small shadow-colored fingertips. The
extra `#AD6945` nose shade remains skin; `#762E21` hair/moustache/boot details
and mouth-interior colors `#410808` and `#9E2626` remain original.
This boundary describes these exact sources; tracked masks contain explicit
component seeds and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
General actions have seven frames with 0.1/0.25/0.25/0.25/0.25/0.1/0.4-second
durations and native 240–360-tick final-frame holds. West mirrors East.
The shocked reaction uses the native South-only start/loop/end sequence, with
one frame in each strip. Its stages and cycle metadata remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-087-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-087-review/index.html).

The summary samples general actions East/North and the shocked loop. The full
gallery includes every frame, native West mirror and shocked stage, with
Vanilla left and Debug Blue right, full character views and enlarged face,
hand and boot details. Its 13 pages and images total 576,698 bytes (about
563 KiB). Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 299 regular tests and the
release build passed; 369 local opt-ins remain ignored in the normal run.
156 local tests passed separately: the new material test and all 155 checks
retained from slice 086, including Holt's portrait, movement and seated-action
tests and three local GML tests. The package test failed on the unregistered
general-action strip before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted shocked-face selections, an omitted raised hand,
a seed spilling onto a shocked-pose boot and a seed spilling onto a moustache
edge during an East-facing action.

All 17,620 combined variants validate. All 220 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 490
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 155 frame/palette
observations, 20 linear completions, 40 final-frame hold checks and 20 shocked
stage transitions with simulated engine services and direct cycle selection.
Checks cover native mirroring, palette binding, frame phase, wrapper
idempotence, seated status, cycle counters, completion and Spring portrait
synchronization. Natural scheduling, pause/interaction dispatch and full-engine
rendering remain untested.

All 13 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 5,729,568 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-087-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-087-`; author records use
`tmp/holt-spring-reactions-author-`. Fresh exports are in
`extracted/holt-spring-reactions-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-086-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring folder has seven remaining strips: South-facing writing and sweeping,
each with start/loop/end stages, plus whittling East. Summer, Autumn and Winter
each have 27. The inventory is `tmp/slice-087-remainder.json`.
Start the Spring special activities with South-facing writing.
