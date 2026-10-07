# Holt Winter standing writing

Following `6bf266e`, this slice adds Holt's three Winter writing start/loop/end
strips to his accepted 136 sources. All eight frames face South; there are no
native West mirrors. The user approved the artwork on 2026-10-07. Outputs use
`generated/review/` and `generated/build/`.

Holt has 139 sources; the 36-character collection has 4,489 sources and 17,956
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Prior regions, source colors, color groups, mappings, sets and runtime
code remain unchanged. Historical pixel tests retain their fixed expectations;
only the central Holt coverage total grows.

## Source and materials

The mount was empty. Source is the original archive backup at
`tmp/slice-105-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 136 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 147 component seeds select 348 skin pixels.
All eight source frames were inspected in five-choice contact sheets, with
source grids around the hands, board, pen, sleeves and boots. In these Winter
strips, the four shades belong exclusively to skin. Removing the provisional
Autumn footwear boundary from the author helper produces the identical profile.
The material test checks all output pixels against an independent shade oracle,
23 literal landmarks and fixed per-frame selected counts.

The lowered hand under the board reaches row 49: start frame 1 uses `#CD9067`
at local (44,49) and `#F0CC8E` at (45,49). The adjacent board at (42,48) uses
`#C5895C` and stays original. The writing hand changes beside the pink pen
detail: loop frame 3 uses `#F0CC8E` at (34,45), while the pen at (37,44) uses
`#E3779D` and stays original. The sleeve's mapped `#B45732` at (36,41) in start
frame 1 stays original, as does the moustache's `#762E21` at (36,38).
Ginger hair, moustache, eyes, orange plaid shirt, teal scarf, apron, trousers,
boots, board and pen remain original. The profile stores seeds and source hashes,
without adding runtime rules.

Metadata retains 80×80 frames, Default atlas and numeric offsets 40/54.
Start has two frames at 0.125 seconds each; loop has four at
0.1/0.125/0.1/0.3 seconds; end has two at 0.125/0.1 seconds. The native writing
cycle is a South-facing complex start/loop/end sequence.

## Offline review

- [Five-choice sample summary](../../generated/slice-106-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-106-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. The full gallery
includes all eight frames, with Vanilla left and Debug Blue right, full character
views and enlarged face, hand, board, pen and boot details. Game-derived images
and packages remain local and ignored by Git.
The five pages and images total 201,616 bytes (about 197 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 318 regular tests and the
release build passed; 388 local opt-ins remain ignored in the normal run.
175 local tests passed separately: this material test plus all 174 checks
retained from slice 105, including three GML tests. The new package test failed
on an unregistered Winter writing strip before registration and passed
afterward. Historical material tests are unchanged.

Four negative controls generated successfully, then failed the intended pixel
assertion: omitted face selections, the omitted hand below the board, a seed
spilling onto the orange shirt and a seed spilling onto the moustache.

All five gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full character view, enlarged detail
and all 15 summary bindings: 1,879,072 rendered pixels.

All 17,956 combined variants validate. All 556 standalone Holt variants pass
strict recipe validation and match both the inspected candidate and combined
build. The 1,360 prior Holt PNG/metadata files and 43,675 other-character files
remain byte-identical. Existing runtime rows, choices and hotkeys stay exact.

The original archive's native animator and NPC object pass 40 frame/palette
observations and 20 complex stage transitions using simulated engine services
and direct cycle selection. Checks include start/loop/end progression, frame
phase, wrapper idempotence, cycle counters and Winter portrait synchronization.
There are no linear completions or final-frame holds in this batch. Natural
scheduling, pause/interaction dispatch and full-engine rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-106-playtest` lab, including
compilation and installed-pixel/metadata checks. Both original backups retain
the source hash above; five registry/recipe inputs remained unchanged through
generation and installation. The short review path passes Chromium loading and
local link resolution. No preview helper was installed or desktop launcher
changed. Live gameplay, an uninstall roundtrip and other local opt-ins were not
rerun.

Evidence uses `tmp/slice-106-`; author records use
`tmp/holt-winter-writing-author-`. Fresh exports are in
`extracted/holt-winter-writing-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-105-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring, Summer and Autumn sprite folders are complete. Winter now covers
23 of 27 strips; four remain, recorded in `tmp/slice-106-remainder.json`.
Next: Winter whittling, then sweeping.
