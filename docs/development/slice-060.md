# Landen Winter general actions and hammer

Following `10405a9`, this slice adds the three directional Winter general-action
strips and the hammer strip to Landen's accepted 115 sources. The 28 source
frames and 14 native West mirrors give 42 review cases, approved by the user on 2026-10-03. Completed outputs use `generated/review/` and `generated/build/`.

Landen has 119 sources; the 36-character collection has 4,263 sources and 17,052
variants. L retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Landen coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-059-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 115 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 048](slice-048.md). Its 426 seeds select 1,005 skin pixels across the 28
frames: face, ears, exposed neck and moving hands. All four shades represent
skin throughout these four pinned Winter strips. Gray hair, eye detail, beige
coat and its pale trim, purple scarf, trousers, shoes, hammer and swing effects
stay original. Every frame was inspected in Vanilla/Blue contact sheets, with
source grids for the hammer hand and adjacent coat cuffs.

The tracked masks use connected-component seeds. The material test independently
checks reviewed source shades, literal landmarks, every output pixel, per-frame
selected counts, transparency and metadata. Selected pixels are counted
separately from changed pixels because two Ryis target shades match the source.

Metadata retains seven 80×80 frames, the Default atlas and numeric 40/54 offsets
for each strip. The action durations are 0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds;
hammer frames last 0.1 seconds each. Both cycles are linear. General actions'
240–360-tick final-frame holds are preserved; West mirrors East.

## Offline review

- [Five-choice sample summary](../../generated/slice-060-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-060-review/index.html).

The summary samples general actions South/North and the hammer swing. The full
gallery includes every frame and native West mirror, with complete character
views and enlarged details. Its 14 pages and images total 723,440 bytes
(about 706 KiB).
Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 272 regular tests and the
release build passed; 342 local opt-ins remain ignored in the normal run.
126 local tests passed separately: the new material test and all 125 checks
retained from slice 059, including three local GML tests. The package test failed
on the unregistered hammer strip before registration and passed afterward.
Missing face and raised-hand seeds each generated successfully, then failed
the intended material assertion in two separate negative controls.

All 17,052 combined variants validate. All 476 standalone Landen variants pass
strict recipe validation and match the candidate and combined build. The 1,150
accepted Landen PNG/metadata files and 41,615 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Landen's expected total.

The original archive's native animator and NPC object pass 210 frame/palette
observations, 30 linear completions and 40 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, frame phase, wrapper idempotence, seated status, cycle counters,
completion and Winter portrait synchronization. Natural scheduling,
outfit/interaction dispatch and full-engine rendering remain untested.

All 14 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 6,710,784 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-060-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-060-`; author records use
`tmp/landen-winter-finish-author-`. Fresh exports are in
`extracted/landen-winter-finish-study`, reached through
`extracted/test-corpus/landen`. The fixed prior-art baseline is
`generated/slice-059-build/characters/landen`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

The complete archive inventory matches the profile and registry: 32 portraits
(eight per season) and 87 world sprites (24 Spring and 21 each Summer, Autumn
and Winter). There are no remaining PNGs under Landen's NPC folder.
Four separate UI icons under `UI NEW/Generic/Icons/NPC` and `NPC Small` remain
original; icon recoloring is outside this portrait/world slice. The inventories
are `tmp/slice-060-complete.json` and `tmp/slice-060-remainder.json`.
After artwork review, another character's world-sprite pilot can follow.
