# Holt Spring sweeping

Following `387b4b5`, this slice adds three Spring sweeping strips to Holt's
accepted 59 sources: start, loop and end South. The 12 source frames give 12
review cases. The user approved the artwork on 2026-10-06. There are no native West mirrors. This
completes all 30 strips in Holt's Spring sprite folder. Completed outputs use
`generated/review/` and `generated/build/`.

Holt has 62 sources; the 36-character collection has 4,412 sources and 17,648
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-089-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 59 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 267 component seeds select 633 skin pixels across
the 12 frames: face, ears, neck and moving hands. Ginger hair, moustache, eye
detail, orange scarf, green shirt, blue apron, trousers, boots, broom and dust
stay original. Every frame was inspected in Vanilla/Blue contact sheets, with
five-choice views and source grids around the face, hands, broom and boots.

Skin remains above row 51 in these pinned strips; the lowered hand in start
and end reaches row 48. Boots reuse skin shades from row 51 downward and stay
original. The `#762E21` hair/moustache/boot shade remains original. The broom's
`#653D25`, `#936244`, `#BB8151`, `#E9B03A` and `#FFE36F` shades are distinct
from skin; dust uses `#A0A0A0`. These observations describe the exact sources;
the tracked masks contain component seeds and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Start and end each have one frame and no duration field. The ten-frame loop
uses 0.1 seconds per frame. The native `sweep` cycle is complex, South-only
and standing; it switches to `idle` when paused for speaking. All stages and
cycle metadata remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-090-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-090-review/index.html).

The summary samples the start and loop frames 2 and 7. The full gallery
includes all 12 frames, with Vanilla left and Debug Blue right, full character
views and enlarged face, hand and boot details. The full-view crop is wider to
retain dust on both sides; every opaque pixel fits. Its seven pages and images
total 277,845 bytes (about 271 KiB). Game-derived images and packages remain
local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 302 regular tests and the
release build passed; 372 local opt-ins remain ignored in the normal run.
159 local tests passed separately: the new material test and all 158 checks
retained from slice 089, including Holt's portrait and earlier world tests and
three local GML tests. The package test failed on the unregistered sweeping-end
strip before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, omitted hand selections, a seed
spilling onto a boot and a seed spilling onto a moustache edge.

All 17,648 combined variants validate. All 248 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 590
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 60 frame/palette
observations and 20 sweeping-stage transitions with simulated engine services
and direct cycle selection. Checks cover palette binding, frame phase, wrapper
idempotence, standing status, cycle counters, completion and Spring portrait
synchronization. These strips have no linear-cycle completions or final-frame
holds. Natural scheduling, speaking interruptions, outfit/interaction dispatch
and full-engine rendering remain untested.

All seven gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full character view, enlarged detail
and all 15 summary bindings: 2,742,624 rendered pixels. The short review path
also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-090-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-090-`; author records use
`tmp/holt-spring-sweeping-author-`. Fresh exports are in
`extracted/holt-spring-sweeping-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-089-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring sprite folder is complete. Summer, Autumn and Winter each have
27 remaining strips. The inventory is `tmp/slice-090-remainder.json`. Start Summer with standing and walking.
