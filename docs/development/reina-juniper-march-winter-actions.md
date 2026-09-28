# Reina, Juniper and March: Winter blinking and seated actions

The user approved the Winter idle/walk pilots, committed as `22e3d44`.
This slice adds eleven strips per character: blinking South/East and sitting,
eating and drinking North/South/East. Native West mirrors East. Each character
adds 31 source frames and twelve West views, making 129 review cases. The user
approved the artwork for commit on 2026-09-27. Material notes: [Reina](reina-winter-actions.md),
[Juniper](juniper-winter-actions.md) and [March](march-winter-actions.md).

Reina now has 228 sources, Juniper 256 and March 342. The combined 36-character
trial has 3,310 sources and 13,240 variants. Home, Page Down and U retain the
five palette choices and portrait synchronization. Other unsupported Winter
actions remain original. No runtime Rust or GML code changed.

## Source and materials

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Pins remain strict; no mismatch override or earlier pin refresh was used.
All 33 strips retain Default atlas, 80×80 frames and Middle/54 origin.
Sitting uses single-frame defaults. Blinking has three frames at
`[0.075,0.125,0.075]`; drinking and North eating have three at `1.0`.
South/East eating has five at `[0.125,0.15,0.175,0.125,0.6]`.
The native cycles are linear; sitting, eating and drinking are seated, and
eating/drinking retain the native `[240,360]` final-frame hold range. Evidence:
`tmp/world-winter-actions-native-inputs.json`, the three independent raw sidecar
exports and `tmp/world-winter-actions-timing-check.log`.

Reina's scarf shares a portrait skin color, so its shading stays excluded.
Juniper's gloves and jewelry share several skin shades, including bright cuff
highlights in the moving poses. March's exposed wrist and neck shading changes,
while his gloves, apron and red mouth interiors stay original. Each author's
note records the inspected material boundaries and effective omission/spill
controls.

The synthetic package test first rejected unregistered Winter blink East,
then passed after all 33 exact paths were registered. It checks complete
animation metadata, frame counts, generated PNGs and existing controls.
Evidence: `tmp/world-winter-actions-{red,package-checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-winter-actions-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-winter-actions-preview/index.html):
  every source frame and native West mirror, grouped by character.

Static images establish recoloring only. Natural schedules, speaking and pause
behavior, automatic outfit changes and full-engine rendering remain unverified.
Game-derived images, extracted files and packages stay ignored.

## Verification

Formatting, Clippy with warnings denied, all 174 active tests and the release
build passed. The normal suite leaves 192 opt-ins ignored; 65 local tests ran
separately and passed: three new material tests, 59 retained character corpus
tests and three GML tests. Omission/spill controls were effective; see each
material note. Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-winter-actions-{final-checks,gml}.log` and retained test logs.

All 13,240 combined variants pass exact recipe validation. The 3,304 standalone
variants match the combined bundle. All 7,930 earlier PNG/metadata files for these
three characters remain byte-identical, as do all 25,005 files for the other 33.
Previous runtime rows and hotkeys remain exact. Evidence:
`tmp/world-winter-actions-comparison.{json,log}`.

The definition audit preserves all 793 earlier regions, every source color,
color group and target mapping. Sets, portrait definitions and the collection
stay unchanged. Exactly 33 profile/registry paths were added. Retained
assertions change only current corpus paths and total counts; all 36 export
reports retain prior source entries and the current archive hash. Evidence:
`tmp/world-winter-actions-input-audit.log`.

The probe's native animator and NPC object match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, native Multi
packs pass 645 frame/palette observations and 225 linear completions, plus 240 final-frame hold checks, across
all five choices and four directions. Checks cover palette binding, native
frame/phase preservation, cycle counters, West mirroring, wrapper idempotence
and portrait synchronization. Natural schedules, pause policy, automatic outfit
changes and full-engine rendering remain unverified. Evidence:
`tmp/world-winter-actions-{native-source,runtime}.log`.

The combined summary verifies fifteen source/palette bindings and
118,560 exact pixels against complete artwork crops and raw metadata. Each
character's full gallery passes exact pixel, crop, source, metadata and browser
checks. Chromium also passes the combined landing page and its images/links.
All 129 cases are available in Vanilla and Debug Blue. The four preview folders
total about 1.97 MiB. Evidence: the material notes,
`tmp/world-winter-actions-summary-check.log`,
`tmp/world-winter-actions-preview-browser-check.json` and
`tmp/world-winter-actions-preview-sizes.json`.

MOMI installation passed in the fresh isolated `tmp/world-winter-actions-22e3d44-playtest`
copy, including compilation and installed-pixel/animation-metadata checks.
Installed archive SHA-256:
`726c740340031dfd6d5562eb36b944fc9fb441381fbb3107e3697e0666465c50`.
The mounted archive and the lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs stayed unchanged through generation
and installation. Evidence:
`tmp/world-winter-actions-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed; the desktop launcher remains unchanged. An
uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Add Winter general actions, sleeping and kissing for Reina, Juniper and March.
Their Winter folders currently cover 17 of 35, 33 and 44 strips respectively.
The remaining inventory is `tmp/world-winter-actions-remaining.json`.
