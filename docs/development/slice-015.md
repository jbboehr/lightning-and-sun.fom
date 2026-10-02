# Eiland Wedding completion

Following the artwork accepted in `86a2661`, this slice adds the remaining nine
Wedding strips: three actions, two blinks, three sitting directions and one kiss.
Their 34 source frames and 15 native West mirrors give 49 review cases. The user
approved this slice for commit on 2026-10-01. Completed outputs use `generated/review/` and
`generated/build/`.

Eiland now has 277 sources, covering all 15 Wedding strips and all adult assets
in his archive folder. The 36-character collection has 3,981 sources and 15,924
variants. J retains its five choices, Vanilla default and portrait/world
synchronization. Only registry/profile data, tests and documentation change.
Historical pixel tests remain unchanged; the growing total changes only in
`tests/profile_coverage.rs`. Eiland's stable corpus link advances to this export.

## Source and material decisions

The game mount was absent during this slice. Work used the original backup at
`tmp/slice-014-playtest/.mistria-palette/previous.zip`, whose SHA-256 matches the
last approved source archive:
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This establishes coverage for that game version; it does not check for a newer
installation. All 268 prior source pins match fresh exports. No pin refresh or
hash override was used. Earlier masks, source roles, groups and ramps stay exact.

Exposed faces and necks use the existing five world skin shades. Gold trim and
clothing share four of those shades in 189 reviewed pixels: 167 `#9C5241`,
14 `#BA6A4C`, two `#DE8F5D` and six `#7D3B14`. Those coordinates stay original,
as do gloves, boots, hair and eyes. The new local test checks every pixel across
all four palettes, with literal material coordinates and per-frame counts.
North-facing action and sitting have no exposed skin; their generated PNGs
must remain byte-identical to the originals.

Every sidecar matches an independent archive read, retaining 80×80 frames,
the Default atlas, native offsets, durations and cycle properties. West uses
the native East mirror. No runtime code changes.

## Offline review

- [Five-choice sample summary](../../generated/slice-015-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-015-review/index.html).

The summary samples acting South, kissing East and sitting East. The full review
includes all 49 cases, including unchanged north-facing views, with complete
character crops and enlarged details. The 19-page gallery totals 901,592 bytes
(about 880 KiB). All images and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 227 regular tests and the
release build passed. The normal suite leaves 297 local opt-ins ignored.
Seventy-nine local tests passed separately: one new material test, 75 retained
corpus tests and three GML runtime tests. Release artifacts were used to avoid
recreating the debug cache removed during cleanup. Both omission and clothing
spill controls generated before failing the intended pixel assertions. The
synthetic package test rejected the unregistered action strip before the data
additions and passed afterward.

All 15,924 combined variants validate. The 1,108 standalone Eiland variants
pass strict recipe validation and match the combined build. All 2,680 prior
Eiland original/variant PNG and metadata files, and all 37,215 files for the
other 35 characters, remain byte-identical. Previous runtime rows, choices and
hotkeys remain exact. The new north-facing variants retain their original PNG
bytes.

The source archive's native animator and NPC object pass 245 frame/palette
observations, 65 linear completions and 40 last-frame hold checks with simulated
engine services and direct cycle selection. Checks cover seated state, native
mirroring, palette binding, wrapper idempotence, frame phase, cycle counters,
completion state and portrait synchronization. Natural scheduling, interaction
and outfit dispatch, and full-engine rendering remain untested.

All 19 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every case, mirror, complete character crop, enlarged detail and
all 15 summary bindings: 8,784,672 rendered pixels. The `review` shortcut also
passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-015-playtest` lab, including
required compilation and installed-pixel/metadata verification. Both original
backups retain the source hash above; five registry/recipe inputs stayed exact
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-015-`; author records use
`tmp/eiland-wedding-finish-author-`. Fresh exports live in
`extracted/eiland-wedding-finish-study`, reached through
`extracted/test-corpus/eiland`. `evidence.json` summarizes checks and `scope.log`
confirms the nine expected paths and unchanged historical pixel tests.

## Cleanup and remaining work

Before this slice, cleanup removed superseded installation labs, duplicate
packages and candidates, temporary compiled helpers, and `target/debug`, freeing
61.47 GiB. It preserved 187 referenced test-baseline directories, all review
galleries, the latest complete build/lab, release tools and extracted corpora.
All 512,871 retained files or links matched their recorded contents; all 1,915
review pages and 16,269 local links survived. The original archive backup and Git
state were unchanged. Local audit records are in `tmp/cleanup-86a2661/`.
Subsequent builds add some new files; these figures describe the cleanup itself.

Eiland's separate Children folders remain future work: four portraits and 46
sprites. No adult assets remain unregistered. `tmp/slice-015-remainder.json`
records the inventory. A proposed next slice is Olric's Spring standing and
walking, continuing coverage of characters whose overworld sprites are original.
