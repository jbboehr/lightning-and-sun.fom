# Nora Winter sweeping

Following `dcdd045`, this slice adds three Winter sweeping strips to Nora's
accepted 148 sources: start, loop and end South. Seventeen source frames give
17 review cases, approved by the user on 2026-10-05. This native sequence has no West mirrors.
Completed outputs use `generated/review/` and `generated/build/`.

Nora has 151 sources; the 36-character collection has 4,382 sources and 17,528
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-083-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 148 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the reviewed world skin shades from [slice 061](slice-061.md).
Its 330 seeds select 755 skin pixels across 17 frames: face, ears, neck and
moving hands. Blonde hair, eye detail, blue scarf, pale coat, purple skirt,
brown boots, pink earrings, broom and dust stay original. Boots cover the
ankles. Every frame was inspected in Vanilla/Blue and five-choice contact
sheets, with source grids around the arms, hands, broom and hair.

The three shades `#FCD9B3`, `#F0B988` and `#D37A57` are skin throughout these
exact strips. The deeper shared shade `#762E21` is hair through row 40 and
hand contours from row 47. Eight such skin pixels are selected, including
`[33,48]` and `[46,48]` in the starting and ending poses. The four other pixels
follow the moving hand in loop frames 1, 13, 14 and 15 (one-based frames).
Broom browns/golds and gray dust are distinct from the selected skin.
Tracked masks contain component seeds and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Start and end each have one frame and no duration field. The fifteen-frame
loop uses 0.1 seconds except frames 6, 12 and 15 (one-based), which use
0.2/0.4/0.8 seconds. The native `sweep` cycle is complex, with South as its
only direction, and is not seated. All stages and cycle metadata remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-084-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-084-review/index.html).

The summary samples start and two loop frames. The full gallery includes every
frame, with Vanilla left and Debug Blue right, complete character views and
enlarged face, hand and boot details. Its eight pages and images total
358,670 bytes (about 350 KiB). Game-derived images and packages remain local
and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 296 regular tests and the
release build passed; 366 local opt-ins remain ignored in the normal run.
152 local tests passed separately: the new material test and all 151 checks
retained from slice 083, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered Winter sweeping-end strip
before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, a missing dark hand contour,
a seed spilling onto hair and omitted raised-hand selections around the broom.

All 17,528 combined variants validate. All 604 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 1,480
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 85 frame/palette
observations and 20 sweeping-stage transitions with simulated engine services
and direct cycle selection. Checks cover palette binding, frame phase, wrapper
idempotence, standing status, cycle counters, completion and Winter portrait
synchronization. These strips have no linear-cycle completions or final-frame
holds. Natural scheduling, speaking interruptions, seasonal rollover,
outfit/interaction dispatch and full-engine rendering remain untested.

All eight gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full character view, enlarged detail
and all 15 summary bindings: 3,558,048 rendered pixels. The short review path
also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-084-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-084-`; author records use
`tmp/nora-winter-sweeping-author-`. Fresh exports are in
`extracted/nora-winter-sweeping-study`, reached through
`extracted/test-corpus/nora`. The fixed prior-art baseline is
`generated/slice-083-build/characters/nora`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Nora's 32-strip Spring and 29-strip Summer, Autumn and Winter sprite folders
are complete. With 32 portraits, all 151 animation sources under
`assets/animations/NPCs/Nora/` in this archive are included. The inventory at
`tmp/slice-084-remainder.json` has no remaining Nora PNGs. Continue the
overworld coverage with another character.
