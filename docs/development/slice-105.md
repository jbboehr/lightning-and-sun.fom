# Holt Winter general actions

Following `eb3dc71`, this slice adds Holt's three Winter general-action strips
to his accepted 133 sources. The 21 source frames and seven native West mirrors
give 28 review cases. The user approved the artwork on 2026-10-07. Outputs use
`generated/review/` and `generated/build/`.

Holt has 136 sources; the 36-character collection has 4,486 sources and 17,944
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Prior regions, source colors, color groups, mappings, sets and runtime
code remain unchanged. Historical pixel tests retain their fixed expectations;
only the central Holt coverage total grows.

## Source and materials

The mount was empty. Source is the original archive backup at
`tmp/slice-104-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 133 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 298 component seeds select 582 skin pixels.
All 21 source frames were inspected in five-choice contact sheets, with source
grids around face, ears, moving hands, sleeves and boots. In these pinned strips
the four shades belong exclusively to skin. The material test uses an independent
whole-image oracle, 26 literal material landmarks and fixed per-frame counts.

Winter's North raised arm is an orange sleeve. Its ears and lowered hand change;
the shirt's mapped `#B45732` stays original. East frame 2's hand includes
`#CD9067` at local (46,42), `#96523A` at (48,42) and `#F0CC8E` at (46,43).
The adjacent sleeve at (45,42) uses `#B45732` and stays original. Unlike Autumn,
there is no extra dark hand contour; East frame 2's `#762E21` at (40,37) belongs
to the moustache. Ginger hair, moustache, eyes, orange plaid shirt, teal scarf,
apron, trousers and boots remain original. The profile stores component seeds
and source hashes, without adding runtime rules.

Metadata retains 80×80 frames, Default atlas, numeric offsets 40/54 and durations
0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds. Native cycles retain East-to-West
mirroring and final-frame holds of 240–360 ticks.

## Offline review

- [Five-choice sample summary](../../generated/slice-105-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-105-review/index.html).

The summary samples East frame 2, North frame 6 and South frame 3. The full
gallery includes all 21 frames and seven native West mirrors, with Vanilla left
and Debug Blue right, full character views and enlarged face, hand, clothing and
boot details. Game-derived images and packages remain local and ignored by Git.
The ten pages and images total 497,366 bytes (about 486 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 317 regular tests and the
release build passed; 387 local opt-ins remain ignored in the normal run.
174 local tests passed separately: this material test plus all 173 checks
retained from slice 104, including three GML tests. The new package test failed
on an unregistered Winter general-action strip before registration and passed
afterward. Historical material tests are unchanged.

Four negative controls generated successfully, then failed the intended pixel
assertion: omitted face selections, omitted hand selections, a seed spilling
onto the orange shirt and a seed spilling onto the moustache.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, native West mirror, metadata binding, full character
view, enlarged detail and all 15 summary bindings: 5,220,384 rendered pixels.

All 17,944 combined variants validate. All 544 standalone Holt variants pass
strict recipe validation and match both the inspected candidate and combined
build. The 1,330 prior Holt PNG/metadata files and 43,675 other-character files
remain byte-identical. Existing runtime rows, choices and hotkeys stay exact.

The original archive's native animator and NPC object pass 140 frame/palette
observations, 20 linear completions and 40 final-frame hold checks using
simulated engine services and direct cycle selection. Checks include mirroring,
frame phase, wrapper idempotence, cycle counters and Winter portrait
synchronization. These cycles have no complex stage transitions. Natural
scheduling, pause/interaction dispatch and full-engine rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-105-playtest` lab, including
compilation and installed-pixel/metadata checks. Both original backups retain
the source hash above; five registry/recipe inputs remained unchanged through
generation and installation. The short review path passes Chromium loading and
local link resolution. No preview helper was installed or desktop launcher
changed. Live gameplay, an uninstall roundtrip and other local opt-ins were not
rerun.

Evidence uses `tmp/slice-105-`; author records use
`tmp/holt-winter-general-author-`. Fresh exports are in
`extracted/holt-winter-general-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-104-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring, Summer and Autumn sprite folders are complete. Winter now covers
20 of 27 strips; seven remain, recorded in `tmp/slice-105-remainder.json`.
Next: Winter standing writing in three start/loop/end strips, then whittling
and sweeping.
