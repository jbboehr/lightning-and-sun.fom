# Holt Autumn standing and walking

Following `1ebe5aa`, this slice adds Holt's six Autumn standing and walking strips
to his accepted 89 sources. The fifteen source frames and five native West
mirrors give twenty review cases. The user approved the artwork on 2026-10-07.
Completed outputs use `generated/review/` and `generated/build/`.

Holt has 95 sources; the 36-character collection has 4,445 sources and 17,780
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only six regions and registry entries are added.
Source colors, color groups, target mappings, collection sets and prior regions
stay unchanged. No runtime code changes. Historical pixel tests retain their
fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-096-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 89 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md), plus the existing `#762E21` shadow mapping used for
specific hand contours in [slice 092](slice-092.md). Its 282 component seeds select
506 skin pixels: face, ears, neck, nose edge and moving hands. Every source frame
was inspected in five-choice contact sheets, with labeled source grids around
the face, hands, hair, clothing and footwear.

The four primary skin shades occur above row 49; footwear reuses them below and
stays original. Three pixels at local (45,45), in East standing and East walking
frames 1 and 3, use `#762E21` on the hand edge. The initial four-shade candidate
missed these; precise selections now include them. Ginger hair, moustache, eyes,
green shirt, brown apron, pink trim and boots remain original. The dark hair
reaching down to rows 39/40 in North-facing poses stays original. Apron browns
and pink trim use different shades from skin. These observations describe the
pinned source; the tracked profile contains component seeds and hashes, not new
runtime rules.

Per-frame selections are 38/22/45 for East/North/South standing, 38/42/38/35 for
East walking, 22/17/22/17 for North walking and 45/40/45/40 for South walking.
The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Standing has one frame per direction with no explicit duration; walking has
four per direction at 0.15 seconds each. The native `idle` and `walk` cycles are
linear and standing, default to South and mirror East for West. Their pause
configuration remains unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-097-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-097-review/index.html).

The summary samples South standing, East walking frame 2 and North walking
frame 2. The full gallery includes all fifteen source frames and five native
West mirrors, with Vanilla left and Debug Blue right, full character views and
enlarged face, hand, clothing and boot details. Game-derived images and packages
remain local and ignored by Git.
The ten pages and images total 402,958 bytes (about 394 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 309 regular tests and the
release build passed; 379 local opt-ins remain ignored in the normal run.
166 local tests passed separately: the new material test and all 165 checks
retained from slice 096, including Holt's portrait and earlier world tests and
three local GML tests. The package test failed on an unregistered Autumn standing
strip before registration and passed afterward.

Five negative controls generated successfully and then failed the intended
material assertion: omitted face selections, an omitted hand, a seed spilling
onto footwear, an omitted nose edge and an omitted dark hand contour. The last
control specifically catches the initial candidate's gap.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, native West mirror, metadata binding, full character
view, enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels.

All 17,780 combined variants validate. All 380 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 890
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear completions with simulated engine services and direct
cycle selection. Checks cover native mirroring, palette binding, frame phase,
wrapper idempotence, standing status, cycle counters, completion and Autumn
portrait synchronization. These cycles have no final-frame hold or complex stage
transitions. Natural scheduling, pause/interaction dispatch and full-engine
rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-097-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. The short review path also passes Chromium
loading and local link resolution. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-097-`; author records use
`tmp/holt-autumn-movement-author-`. Fresh exports are in
`extracted/holt-autumn-movement-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-096-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring and Summer sprite folders are complete. Autumn now covers six of
27 strips; 21 Autumn and 27 Winter strips remain, recorded in
`tmp/slice-097-remainder.json`. Next: Autumn blinking, sitting, eating and drinking.
