# Reina, Juniper and March: Winter seated reading

The user approved the Winter general-action/sleep/kiss batch, committed as
`c874344`. This slice adds three South-facing reading strips per character:
start, loop and end. Each adds ten source frames, making 30 review cases.
There are no native West views for this cycle. The user approved the artwork
for commit on 2026-09-28.
Material notes: [Reina](reina-winter-reading.md),
[Juniper](juniper-winter-reading.md) and [March](march-winter-reading.md).

Reina now has 236 sources, Juniper 264 and March 350. The combined 36-character
trial has 3,334 sources and 13,336 variants. Home, Page Down and U retain the
five palette choices and portrait synchronization. Unsupported Winter special
actions remain original. No runtime Rust or GML code changed.

## Source and materials

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Pins remain strict; no mismatch override or earlier pin refresh was used.
All nine strips retain Default atlas, 80×80 frames and Middle/54 origin.
Start and end each have three frames at `0.1`; the loop has four at
`[3.0,0.1,3.0,0.1]`. The native `read_sit` cycle is seated and complex, using
a Single South pack to sequence start, repeated loop and end. Evidence:
`tmp/world-winter-reading-native-inputs.json`, the independent raw sidecar
exports and `tmp/world-winter-reading-timing-check.log`.

Each author inspected all actual frames across four target palettes. Their
notes record skin, Winter clothing and book/page boundaries, including shared
skin/material colors, and effective omission/spill controls.

The synthetic package test first rejected unregistered Winter reading end,
then passed after all nine exact paths were registered. It checks complete
animation metadata, frame counts, generated PNGs and existing controls.
Evidence: `tmp/world-winter-reading-{red,package-checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-winter-reading-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-winter-reading-preview/index.html):
  every source frame in all three phases, grouped by character.

Static images establish recoloring only. Natural schedules, speaking and pause
behavior, automatic outfit changes and full-engine rendering remain unverified.
Game-derived images, extracted files and packages stay ignored.

## Verification

Formatting, Clippy with warnings denied, all 178 active tests and the release
build passed. The normal suite leaves 198 opt-ins ignored; 71 local tests ran
separately and passed: three new material tests, 65 retained character corpus
tests and three GML tests. Omission/spill controls were effective; see each
material note. Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-winter-reading-{final-checks,gml}.log` and retained test logs.

All 13,336 combined variants pass exact recipe validation. The 3,400 standalone
variants match the combined bundle. All 8,410 earlier PNG/metadata files for these
three characters remain byte-identical, as do all 25,005 files for the other 33.
Previous runtime rows and hotkeys remain exact. Evidence:
`tmp/world-winter-reading-comparison.{json,log}`.

The definition audit preserves all 841 earlier regions, every source color,
color group and target mapping. Sets, portrait definitions and the collection
stay unchanged. Exactly nine profile/registry paths were added. Retained
assertions change only current corpus paths and total counts; all 36 export
reports retain prior source entries and the current archive hash. Evidence:
`tmp/world-winter-reading-input-audit.log`.

The probe's native animator and NPC object match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, native animation
packs pass 150 frame/palette observations and 60 start/loop/end transitions across
all five choices in the native South direction. Checks cover palette binding, native
frame/phase preservation, cycle counters, wrapper idempotence
and portrait synchronization. Natural schedules, pause policy, automatic outfit
changes and full-engine rendering remain unverified. Evidence:
`tmp/world-winter-reading-{native-source,runtime}.log`.

The combined summary verifies fifteen source/palette bindings and
137,280 exact pixels against complete artwork crops and raw metadata. Each
character's full gallery passes exact pixel, crop, source, metadata and browser
checks. Chromium also passes the combined landing page and its images/links.
All 30 cases are available in Vanilla and Debug Blue. The four preview folders
total about 0.71 MiB. Evidence: the material notes,
`tmp/world-winter-reading-summary-check.log`,
`tmp/world-winter-reading-preview-browser-check.json` and
`tmp/world-winter-reading-preview-sizes.json`.

MOMI installation passed in the fresh isolated `tmp/world-winter-reading-c874344-playtest`
copy, including compilation and installed-pixel/animation-metadata checks.
Installed archive SHA-256:
`15c60abadad39adeb32d3fc93791039f186218a010b86deda0da230c1ba14231`.
The mounted archive and the lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs stayed unchanged through generation
and installation. Evidence:
`tmp/world-winter-reading-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed; the desktop launcher remains unchanged. An
uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue Winter specials with Reina's writing, Juniper's laugh and March's
smithing. Their Winter folders currently cover 25 of 35, 33 and 44 strips
respectively. The remaining inventory is `tmp/world-winter-reading-remaining.json`.
