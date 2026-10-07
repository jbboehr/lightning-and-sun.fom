# Holt Summer standing writing

Following `f08c3a0`, this slice adds three Summer writing strips to Holt's
accepted 82 sources: standing start, loop and end South. The eight source frames
give eight review cases. The user approved the artwork on 2026-10-06. This native cycle has
no West mirrors. Completed outputs use `generated/review/` and `generated/build/`.

Holt has 85 sources; the 36-character collection has 4,435 sources and 17,740
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-093-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 82 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 150 component seeds select 353 skin pixels across
the eight frames: face, ears, neck, nose edge, bare forearms and moving hands.
Ginger hair, moustache, eye detail, pink shirt, green apron, footwear, board and
pen stay original. Every frame was inspected in five-choice contact sheets,
with labeled source grids around the hands, board, pen and face.

Skin remains above row 51 in these three pinned sources. The lowered hand
under the board reaches row 49 in start frame 1 and end frame 2 (one-based
frames); the previous general-action boundary would omit those pixels.
Footwear reuses skin shades farther down and stays original. The `#AD6945`
nose shade remains skin; `#762E21` hair/moustache details remain original.
The dark hand contours found in seated poses do not occur in these hands.
The board uses distinct browns such as `#C5895C`, `#7D4F3D` and `#57342B`;
the pen's yellow, purple, pink and pale colors also stay unchanged.
These observations describe the exact sources; tracked masks contain explicit
component seeds and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Start has two frames at 0.125 seconds each; the four-frame loop uses
0.1/0.125/0.1/0.3 seconds; end uses 0.125/0.1 seconds. The native `write`
cycle is complex, South-facing and standing. It has no seated flag or mirrored
directions. All stages and cycle metadata remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-094-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-094-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. The full
gallery includes every frame, with Vanilla left and Debug Blue right, full
character views and enlarged face, hand, footwear and writing-tool details.
Every opaque pixel fits the full-view crop. Game-derived images and packages
remain local and ignored by Git.
Its five pages and images total 200,650 bytes (about 196 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 306 regular tests and the
release build passed; 376 local opt-ins remain ignored in the normal run.
163 local tests passed separately: the new material test and all 162 checks
retained from slice 093, including Holt's portrait and earlier world tests and
three local GML tests.
The package test failed on the unregistered writing-end strip before registration
and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, the missing lowered hand under
the board, a seed spilling onto footwear and a seed spilling onto a moustache edge.

All 17,740 combined variants validate. All 340 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 820
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 40 frame/palette
observations and 20 writing-stage transitions with simulated engine services
and direct cycle selection. Checks cover palette binding, frame phase, wrapper
idempotence, standing status, cycle counters, completion and Summer portrait
synchronization. These strips have no linear-cycle completions or final-frame
holds. Natural scheduling, outfit/interaction dispatch and full-engine
rendering remain untested.

All five gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full character view, enlarged
detail and all 15 summary bindings: 1,879,072 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-094-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-094-`; author records use
`tmp/holt-summer-writing-author-`. Fresh exports are in
`extracted/holt-summer-writing-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-093-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring sprite folder is complete. Summer has four remaining strips:
whittling East and sweeping's start/loop/end South. Autumn and Winter each
have 27. The inventory is `tmp/slice-094-remainder.json`.
Continue Summer with whittling.
