# Reina, Juniper and March: Winter general actions, sleeping and kissing

The user approved the Winter blinking and seated-action batch, committed as
`e632b6b`. This slice adds five strips per character: general actions North,
South and East, plus sleeping and kissing East. Native West mirrors East.
Each character adds 26 source frames and twelve West views, making 114 review
cases. The user approved the artwork for commit on 2026-09-28. Material notes:
[Reina](reina-winter-standard.md), [Juniper](juniper-winter-standard.md) and
[March](march-winter-standard.md).

Reina now has 233 sources, Juniper 261 and March 347. The combined 36-character
trial has 3,325 sources and 13,300 variants. Home, Page Down and U retain the
five palette choices and portrait synchronization. Unsupported Winter special
actions remain original. No runtime Rust or GML code changed.

## Source and materials

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Pins remain strict; no mismatch override or earlier pin refresh was used.
All fifteen strips retain Default atlas, 80×80 frames and Middle/54 origin.
General actions have seven frames at `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`.
Kissing has four at `[0.15,0.15,0.8,0.15]`; sleeping uses single-frame defaults.
All native cycles are linear. General actions keep the native `[240,360]`
final-frame hold range and `idle` speaking fallback. Sleep and kiss use Single
East packs; general actions use directional Multi packs. Evidence:
`tmp/world-winter-standard-native-inputs.json`, the independent raw sidecar
exports and `tmp/world-winter-standard-timing-check.log`.

Each author inspected all actual frames across four target palettes. Their
notes record skin, Winter clothing and mouth boundaries, including shared
skin/material colors, and effective omission/spill controls.

The synthetic package test first rejected unregistered Winter action East,
then passed after all fifteen exact paths were registered. It checks complete
animation metadata, frame counts, generated PNGs and existing controls.
Evidence: `tmp/world-winter-standard-{red,package-checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-winter-standard-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-winter-standard-preview/index.html):
  every source frame and native West mirror, grouped by character.

Static images establish recoloring only. Natural schedules, speaking and pause
behavior, automatic outfit changes and full-engine rendering remain unverified.
Game-derived images, extracted files and packages stay ignored.

## Verification

Formatting, Clippy with warnings denied, all 176 active tests and the release
build passed. The normal suite leaves 195 opt-ins ignored; 68 local tests ran
separately and passed: three new material tests, 62 retained character corpus
tests and three GML tests. Omission/spill controls were effective; see each
material note. Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-winter-standard-{final-checks,gml}.log` and retained test logs.

All 13,300 combined variants pass exact recipe validation. The 3,364 standalone
variants match the combined bundle. All 8,260 earlier PNG/metadata files for these
three characters remain byte-identical, as do all 25,005 files for the other 33.
Previous runtime rows and hotkeys remain exact. Evidence:
`tmp/world-winter-standard-comparison.{json,log}`.

The definition audit preserves all 826 earlier regions, every source color,
color group and target mapping. Sets, portrait definitions and the collection
stay unchanged. Exactly fifteen profile/registry paths were added. Retained
assertions change only current corpus paths and total counts; all 36 export
reports retain prior source entries and the current archive hash. Evidence:
`tmp/world-winter-standard-input-audit.log`.

The probe's native animator and NPC object match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, native animation
packs pass 570 frame/palette observations and 120 linear completions, plus 120 final-frame hold checks, across
all five choices and four directions. Checks cover palette binding, native
frame/phase preservation, cycle counters, West mirroring, wrapper idempotence
and portrait synchronization. Natural schedules, pause policy, automatic outfit
changes and full-engine rendering remain unverified. Evidence:
`tmp/world-winter-standard-{native-source,runtime}.log`.

The combined summary verifies fifteen source/palette bindings and
136,800 exact pixels against complete artwork crops and raw metadata. Each
character's full gallery passes exact pixel, crop, source, metadata and browser
checks. Chromium also passes the combined landing page and its images/links.
All 114 cases are available in Vanilla and Debug Blue. The four preview folders
total about 1.78 MiB. Evidence: the material notes,
`tmp/world-winter-standard-summary-check.log`,
`tmp/world-winter-standard-preview-browser-check.json` and
`tmp/world-winter-standard-preview-sizes.json`.

MOMI installation passed in the fresh isolated `tmp/world-winter-standard-e632b6b-playtest`
copy, including compilation and installed-pixel/animation-metadata checks.
Installed archive SHA-256:
`3ea9e12a883ebf7d37d0ae00d3bd47bd93fb3bc4e7c00361d22bbb29e2777642`.
The mounted archive and the lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs stayed unchanged through generation
and installation. Evidence:
`tmp/world-winter-standard-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed; the desktop launcher remains unchanged. An
uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Add Winter seated reading for Reina, Juniper and March. Their Winter folders
currently cover 22 of 35, 33 and 44 strips respectively. The remaining inventory
is `tmp/world-winter-standard-remaining.json`.
