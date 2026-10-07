# Holt Summer standing and walking

Following `75dd912`, this slice adds six Summer standing and walking strips to
Holt's accepted 62 sources. The 15 source frames and five native West mirrors
give 20 review cases. The user approved the artwork on 2026-10-06. Completed outputs use
`generated/review/` and `generated/build/`.

Holt has 68 sources; the 36-character collection has 4,418 sources and 17,672
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only six regions and registry entries are added.
Source colors, color groups, target mappings, collection sets and prior regions
stay unchanged. No runtime code changes. Historical pixel tests retain their
fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-090-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 62 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 287 component seeds select 547 skin pixels across
15 frames: face, ears, neck, bare forearms and moving hands. Ginger hair,
moustache, eye detail, pink shirt, green apron and footwear stay original.
Every frame was inspected in Vanilla/Blue and five-choice contact sheets,
with labeled source grids around the face, nose, moustache, hands and footwear.

Skin remains above row 49 in these six pinned sources, including the lowered
hands at row 48 in North/South walking. Footwear reuses the first three world
shades from row 49 downward and stays original. The `#762E21` hair/moustache
shade remains original. The extra `#AD6945` nose shade appears once in each
of the five East-facing source frames and follows the palette. These
observations describe the exact sources; the tracked masks contain component
seeds and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Standing has one frame and no duration field; walking has four 0.15-second
frames. Both native cycles are linear, with West mirroring East. Cycle metadata
and all animation stages remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-091-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-091-review/index.html).

The summary samples standing South and walking East/North. The full gallery
includes every frame and native West mirror, with Vanilla left and Debug Blue
right, complete character views and enlarged face, hand and footwear details.
Every opaque pixel fits the full-view crop. Its ten pages and images total
394,408 bytes (about 385 KiB). Game-derived images and packages remain local
and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 303 regular tests and the
release build passed; 373 local opt-ins remain ignored in the normal run.
160 local tests passed separately: the new material test and all 159 checks
retained from slice 090, including Holt's portrait and Spring world tests and
three local GML tests. The package test failed on the unregistered standing-East
strip before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face, hand or nose-edge selections and a seed
spilling onto footwear.

All 17,672 combined variants validate. All 272 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 620
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear completions with simulated engine services and
direct cycle selection. Checks cover native mirroring, palette binding, frame
phase, wrapper idempotence, standing status, cycle counters, completion and
Summer portrait synchronization. These cycles have no final-frame hold or
complex-stage transitions. Natural scheduling, speaking interruptions,
outfit/interaction dispatch and full-engine rendering remain untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels. The short
review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-091-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-091-`; author records use
`tmp/holt-summer-movement-author-`. Fresh exports are in
`extracted/holt-summer-movement-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-090-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring sprite folder is complete. Summer has 21 remaining strips;
Autumn and Winter each have 27. The inventory is `tmp/slice-091-remainder.json`.
Continue Summer with blinking, sitting, eating and drinking.
