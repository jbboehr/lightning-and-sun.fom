# Holt Spring blinking and seated actions

Following `fd44ee9`, this slice adds eleven Spring strips to Holt's accepted
38 sources: blinking East/South and sitting, eating and drinking North/South/East.
The 31 source frames and 12 native West mirrors give 43 review cases, approved
by the user on 2026-10-05. Completed outputs use `generated/review/` and `generated/build/`.

Holt has 49 sources; the 36-character collection has 4,399 sources and 17,596
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-085-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 38 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 568 component seeds select 1,230 skin pixels
across the 31 frames: face, ears, neck, eyelids and moving hands. Ginger hair,
moustache, eye detail, orange scarf, green shirt, blue apron, trousers, boots
and mouth interiors stay original. Every frame was inspected in Vanilla/Blue
contact sheets, with five-choice samples and labeled source grids around the
face, cuffs, hands, mouth and boots.

The first three world shades also occur in boots. Unlike standing poses,
seated East-facing boots start at row 47 and South-facing boots at row 48.
North-facing seated poses show hands without visible boots. Eating South's
second frame lowers a hand over the left boot; its skin extends through row 49.
The masks include that hand while preserving the adjacent visible boot pixels.
These boundaries describe these exact sources; tracked masks contain explicit
component seeds and hashes, not a new runtime rule. The extra `#AD6945` nose
shade remains skin; deep `#762E21` hair/moustache/boot pixels and red `#9E2626`
mouth interiors remain original.

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

- [Five-choice sample summary](../../generated/slice-086-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-086-review/index.html).

The summary samples blinking South, eating East and drinking North. The full
gallery includes every frame and native West mirror, with Vanilla left and
Debug Blue right, full character views and enlarged face, hand and boot details.
Its 20 pages and images total 777,386 bytes (about 759 KiB). Game-derived images
and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 298 regular tests and the
release build passed; 368 local opt-ins remain ignored in the normal run.
155 local tests passed separately: the new material test and all 154 checks
retained from slice 085, including the Holt portrait and movement tests and
three local GML tests. The package test failed on the unregistered blinking
strip before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face/eyelid selections, the missing lowered hand
in eating South's second frame, a seed spilling onto a boot and a seed spilling
onto the moustache edge.

All 17,596 combined variants validate. All 196 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 380
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

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

MOMI installation passed in the isolated `tmp/slice-086-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-086-`; author records use
`tmp/holt-spring-actions-author-`. Fresh exports are in
`extracted/holt-spring-actions-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-085-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring folder has 13 remaining strips; Summer, Autumn and Winter each
have 27. The inventory is `tmp/slice-086-remainder.json`.
Continue Spring with general actions and shocked reactions
before special activities.
