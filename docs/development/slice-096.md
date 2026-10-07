# Holt Summer sweeping

Following `9296134`, this slice adds Holt's three Summer sweeping strips to his
accepted 86 sources, completing all 27 Summer sprites. The twelve South-facing
source frames give twelve review cases. The user approved the artwork on 2026-10-07.
Completed outputs use `generated/review/` and `generated/build/`.

Holt has 89 sources; the 36-character collection has 4,439 sources and 17,756
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-095-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 86 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md), plus the existing `#762E21` shadow mapping used for
specific hand contours in [slice 092](slice-092.md). Its 273 component seeds select
625 skin pixels: face, ears, neck and moving hands. Every frame was inspected in
five-choice contact sheets, with labeled source grids around the face, hands,
shirt, broom, dust and boots. Per-frame selections are 52 for end, 47 for start,
and 49/58/54/49/51/50/59/55/50/51 for the loop.

The four primary skin shades occur above row 51; footwear reuses them below and
stays original. Ten pixels on the gripping-hand edge use `#762E21`. The initial
four-shade candidate missed these; precise selections now include them. Eight
nearby pixels in shirt creases share that dark brown and remain original, as do
the hair and moustache. Matching Spring poses help distinguish hand positions
from clothing, but Summer's own pixels and materials determine the mask. Pink
shirt highlights are not pale skin. The broom uses distinct brown and gold
shades; gray dust stays unchanged. These observations describe the pinned source;
the tracked profile contains component seeds and hashes, not new runtime rules.

The material test independently checks reviewed source shades, literal
landmarks (including all ten hand contours and eight shared-color shirt pixels),
every output pixel, per-frame selected counts, transparency and metadata.
Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Start and end each have one frame with no explicit duration; the loop has ten
frames at 0.1 seconds each. The native `sweep` cycle is complex, standing and
South-facing, with start/loop/end stages and `on_pause_speaking = "idle"`.

## Offline review

- [Five-choice sample summary](../../generated/slice-096-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-096-review/index.html).

The summary samples the start and loop frames 2 and 7. The full gallery includes
all twelve source frames, with Vanilla left and Debug Blue right, full character
views and enlarged face, hand, clothing and broom details. The full views keep
all broom and dust pixels in frame. Game-derived images and packages remain
local and ignored by Git.
The seven pages and images total 277,806 bytes (about 271 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 308 regular tests and the
release build passed; 378 local opt-ins remain ignored in the normal run.
165 local tests passed separately: the new material test and all 164 checks
retained from slice 095, including Holt's portrait and earlier world tests and
three local GML tests. The package test failed on an unregistered sweeping
strip before registration and passed afterward.

Five negative controls generated successfully and then failed the intended
material assertion: omitted face selections, an omitted hand, a seed spilling
onto footwear, a seed spilling onto a moustache edge and an omitted dark hand
contour. This last control specifically catches the initial candidate's gap.

All seven gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full character view, enlarged detail
and all 15 summary bindings: 2,742,624 rendered pixels.

All 17,756 combined variants validate. All 356 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 860
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 60 frame/palette
observations and 20 complex stage transitions with simulated engine services and
direct cycle selection. Checks cover palette binding, frame phase, wrapper
idempotence, standing status, cycle counters, completion and Summer portrait
synchronization. This South-facing cycle has no native West mirror, linear
completion or final-frame hold. Natural scheduling, pause/interaction dispatch
and full-engine rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-096-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. The short review path also passes Chromium
loading and local link resolution. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-096-`; author records use
`tmp/holt-summer-sweeping-author-`. Fresh exports are in
`extracted/holt-summer-sweeping-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-095-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring and Summer sprite folders are complete. Autumn and Winter each
have 27 remaining strips, recorded in `tmp/slice-096-remainder.json`.
Next: Autumn standing and walking.
