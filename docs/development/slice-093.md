# Holt Summer general actions

Following `10728e3`, this slice adds three Summer general-action strips to
Holt's accepted 79 sources: East, North and South. Each has seven frames;
21 source frames and seven native West mirrors give 28 review cases. The user approved
the artwork on 2026-10-06. Completed outputs use `generated/review/` and `generated/build/`.

Holt has 82 sources; the 36-character collection has 4,432 sources and 17,728
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-092-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 79 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 359 component seeds select 701 skin pixels across
21 frames: face, ears, neck, nose edge, bare forearms and moving hands. Ginger
hair, moustache, eye detail, pink shirt, green apron and footwear stay original.
Every frame was inspected in Vanilla/Blue contact sheets and five-choice views,
with labeled source grids around the nose, cuffs, hands and footwear.

In these exact strips, the four skin shades above row 49 belong to skin.
Footwear below that boundary reuses the first three shades. The North-facing
sixth frame has a raised finger reaching row 39 and a lowered hand reaching
row 47; both change. The East-facing nose uses `#AD6945`. The dark `#762E21`
hand contours found in seated poses do not occur in these hands; all occurrences
here belong to hair/moustache detail and stay original. Tracked masks contain
explicit component seeds and hashes, not a new runtime rule. No earlier artwork
or mappings change.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
All three strips use 0.1/0.25/0.25/0.25/0.25/0.1/0.4-second durations. The
native action cycle is linear, standing and has a 240–360-tick final-frame hold.
West mirrors East.

## Offline review

- [Five-choice sample summary](../../generated/slice-093-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-093-review/index.html).

The summary samples East frame 2, North frame 6 and South frame 3. The full
gallery includes every frame and native West mirror, with Vanilla left and
Debug Blue right, full character views and enlarged face, hand and footwear
details. Every opaque pixel fits the full-view crop. Game-derived images and
packages remain local and ignored by Git.
Its ten pages and images total 496,189 bytes (about 485 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 305 regular tests and the
release build passed; 375 local opt-ins remain ignored in the normal run.
162 local tests passed separately: the new material test and all 161 checks
retained from slice 092, including Holt's portrait and earlier world tests and
three local GML tests. The package test failed on the unregistered action strip
before registration and passed afterward.

Five negative controls generated successfully and then failed the intended
material assertion: omitted face selections, an omitted moving hand, a seed
spilling onto footwear, a seed spilling onto a moustache edge and an omitted
nose-edge component. Removing only the nose seed initially left it selected
through the adjacent shadow seed in the same color group; that was a no-op.
The corrected control removes both component seeds and fails as intended.

All 17,728 combined variants validate. All 328 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 790
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 140 frame/palette
observations, 20 linear completions and 40 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, frame phase, wrapper idempotence, standing status, cycle counters,
completion and Summer portrait synchronization. Natural scheduling,
outfit/interaction dispatch and full-engine rendering remain untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 5,220,384 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-093-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-093-`; author records use
`tmp/holt-summer-general-author-`. Fresh exports are in
`extracted/holt-summer-general-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-092-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring sprite folder is complete. Summer has seven remaining strips:
standing writing (start/loop/end), whittling, and sweeping (start/loop/end).
Autumn and Winter each have 27. The inventory is `tmp/slice-093-remainder.json`.
Continue Summer with standing writing.
