# Holt Winter whittling

Following `629162c`, this slice adds Holt's Winter whittling strip to his accepted
139 sources. Its nine source frames and nine native West mirrors give 18 review
cases. The user approved the artwork on 2026-10-07. Outputs use `generated/review/` and
`generated/build/`.

Holt has 140 sources; the 36-character collection has 4,490 sources and 17,960
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only one region and registry entry are added.
Prior regions, source colors, color groups, mappings, sets and runtime code
remain unchanged. Historical pixel tests retain their fixed expectations;
only the central Holt coverage total grows.

## Source and materials

The mount was empty. Source is the original archive backup at
`tmp/slice-106-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 139 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 143 component seeds select 303 skin pixels.
All nine source frames were inspected in five-choice contact sheets, with source
grids around hands, sleeves, knife, wood, chips and boots. Unlike Autumn's boots,
this Winter strip uses the four shades exclusively for skin. Removing the
provisional footwear boundary from the author helper produces the identical
profile. The material test checks all output pixels against an independent shade
oracle, 21 literal landmarks and fixed per-frame selected counts.

The hand's dark `#96523A` at local (40,45) in frame 1 changes, while the adjacent
shirt's mapped `#B45732` at (38,43) stays original. The moustache's `#762E21` at
(38,38) also stays original. Wood highlights use distinct colors: frame 7's
`#E7BA83` at (45,41) and flying-chip `#C09165` at (54,44) stay original, along
with chip shadows `#764B2D`. The knife retains `#DAE2E7` at (43,46) and
`#6C7781` at (43,47). Ginger hair, moustache, eyes, orange plaid shirt, teal
scarf, apron, trousers, boots, wood, chips and knife remain original. The profile
stores seeds and source hashes, without adding runtime rules.

Metadata retains nine 80×80 frames, Default atlas and numeric offsets 40/54.
Each frame lasts 0.1 seconds except frame 2 at 0.15 seconds. The native whittling
cycle is linear and uses the East sprite with West mirroring.

## Offline review

- [Five-choice sample summary](../../generated/slice-107-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-107-review/index.html).

The summary samples frames 1, 5 and 9. The full gallery includes every frame and
native West mirror, with Vanilla left and Debug Blue right, full character views
and enlarged face, hand, wood, knife and boot details. Game-derived images and
packages remain local and ignored by Git.
The eight pages and images total 360,203 bytes (about 352 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 319 regular tests and the
release build passed; 389 local opt-ins remain ignored in the normal run.
The new package test failed on an unregistered Winter whittling strip before
registration and passed afterward. Historical material tests are unchanged.
176 local tests passed separately: this material test plus all 175 checks
retained from slice 106, including three GML tests.

Four negative controls generated successfully, then failed the intended pixel
assertion: omitted face selections, omitted hand selections, a seed spilling
onto the orange shirt and a seed spilling onto the moustache.

All eight gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, native West mirror, metadata binding, full character
view, enlarged detail and all 15 summary bindings: 3,942,432 rendered pixels.

All 17,960 combined variants validate. All 560 standalone Holt variants pass
strict recipe validation and match both the inspected candidate and combined
build. The 1,390 prior Holt PNG/metadata files and 43,675 other-character files
remain byte-identical. Existing runtime rows, choices and hotkeys stay exact.

The original archive's native animator and NPC object pass 90 frame/palette
observations and ten linear completions using simulated engine services and
direct cycle selection. Checks include native mirroring, frame phase, wrapper
idempotence, cycle counters and Winter portrait synchronization. This cycle has
no final-frame hold or complex stage transitions. Natural scheduling,
pause/interaction dispatch and full-engine rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-107-playtest` lab, including
compilation and installed-pixel/metadata checks. Both original backups retain
the source hash above; five registry/recipe inputs remained unchanged through
generation and installation. The short review path passes Chromium loading and
local link resolution. No preview helper was installed or desktop launcher
changed. Live gameplay, an uninstall roundtrip and other local opt-ins were not
rerun.

Evidence uses `tmp/slice-107-`; author records use
`tmp/holt-winter-whittling-author-`. Fresh exports are in
`extracted/holt-winter-whittling-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-106-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring, Summer and Autumn sprite folders are complete. Winter now covers
24 of 27 strips; three remain, recorded in `tmp/slice-107-remainder.json`.
Next: Winter sweeping start/loop/end, completing his seasonal sprite folders.
