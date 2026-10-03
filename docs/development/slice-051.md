# Landen Spring hammer animation

Following `44c8eac`, this slice adds Landen's hammer strip: seven source frames
and seven native West mirrors give 14 review cases. This completes all 24 strips
in his Spring folder. Artwork was approved by the user on 2026-10-03. Completed outputs use
`generated/review/` and `generated/build/`.

Landen has 56 sources; the 36-character collection has 4,200 sources and 16,800
variants. L retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only the new region and registry entry are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Landen coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-050-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 55 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 048](slice-048.md). Its 130 seeds select 313 skin pixels across the seven
frames, including face, ears, neck and moving hands. Shoes that share those
shades remain unselected. Gray hair, eye detail, scarf, purple shirt, trousers,
hammer and swing effects stay original. All seven frames were inspected in
source grids and five-choice contact sheets.

All exposed skin lies above row 49 in this pinned strip; shoe shades are below
that boundary. The tracked mask uses connected-component seeds. The material
test independently checks the boundary, literal source landmarks, every output
pixel, per-frame selected counts, transparency and metadata. Selected pixels
are counted separately from changed pixels because two Ryis target shades
already match the source.

Metadata retains 80×80 frames, the Default atlas, numeric 40/54 offsets and
0.1-second frame duration. The native hammer cycle is linear and East-facing,
with West mirrored from East. Its name includes the `specialanimation` prefix.

## Offline review

- [Five-choice sample summary](../../generated/slice-051-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-051-review/index.html).

The summary samples the ready, raised-hammer and strike poses. The full gallery
includes every frame and native West mirror, with complete character views
(including the hammer and swing effects) and enlarged details. Its six pages
total 294,457 bytes (about 288 KiB). Game-derived images and packages remain
local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 263 regular tests and the
release build passed; 333 local opt-ins remain ignored in the normal run.
117 local tests passed separately: the new material test and all 116 checks
retained from slice 050, including the three local GML tests. The package test
failed on the unregistered hammer strip before registration and passed afterward.
A missing skin seed and an extra shoe seed each generated successfully, then
failed the intended material assertion.

All 16,800 combined variants validate. All 224 standalone Landen variants pass
strict recipe validation and match the candidate and combined build. The 550
accepted Landen PNG/metadata files and 41,615 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Landen's expected total.

The original archive's native animator and NPC object pass 70 frame/palette
observations and ten linear completions with simulated engine services and
direct cycle selection. Checks cover native mirroring, palette binding, frame
phase, wrapper idempotence, cycle counters, completion and portrait
synchronization. Natural scheduling, outfit/interaction dispatch and full-engine
rendering remain untested.

All six gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 2,619,648 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-051-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-051-`; author records use
`tmp/landen-spring-hammer-author-`. Fresh exports are in
`extracted/landen-spring-hammer-study`, reached through
`extracted/test-corpus/landen`. The fixed prior-art baseline is
`generated/slice-050-build/characters/landen`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Landen's Spring folder is complete. Summer, Autumn and Winter each have 21
remaining strips. The inventory is `tmp/slice-051-remainder.json`.
After artwork review, start Summer with standing and walking.
