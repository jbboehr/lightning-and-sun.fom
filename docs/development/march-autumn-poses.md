# March Autumn standing poses

The user approved Reina and Juniper's Autumn completion batch, committed as
`7dec3dd`. This slice adds March's standing poses facing North, South and East,
plus the native mirrored West view. All three strips contain one frame. March
now has 311 sources and 1,244 variants; the combined 36-character trial has
3,245 sources and 12,980 variants. The user approved this artwork for commit on 2026-09-27.

The Autumn inventory now covers 33 of 47 strips. The remaining fourteen are
injured animations. U still cycles Vanilla, Debug Blue, Hayden, Ryis and Seridia
for March's supported portraits and world sprites. No runtime Rust or GML code
changed. See the [material review](march-autumn-poses-art.md) for mask details.

## Source and native metadata

The read-only mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Pins remain strict; no source hash override or refresh was used.
The sources are under `assets/animations/NPCs/March/Sprites/Autumn/`, named
`spr_npc_march_specialanimation_autumn_pose_{north,south,east}.png`.
All retain the Default atlas, 80×80 frames and numeric origin `[40.0,54.0]`.
The raw metadata omits frame count and duration, retaining single-frame defaults.
The author's sidecars match the independent archive reads exactly.

Native NPC data declares a linear `pose` cycle for Spring and Autumn, defaulting
to South with North/South/East packs. Speaking and background pause fall back
to idle. The native animator selects the East pack for West and the NPC renderer
flips it. Evidence: `tmp/world-autumn-poses-native-inputs.json`,
`tmp/world-autumn-poses-march-raw.json` and `tmp/world-autumn-poses-remaining.json`.
The synthetic package test first rejected unregistered pose East, then passed
after the three exact paths were registered. It checks full animation metadata,
frame totals, PNGs and the existing U control. Evidence:
`tmp/world-autumn-poses-red.log` and `tmp/world-autumn-poses-package-checks.log`.

## Offline review

- [Five-choice summary](../../generated/march-autumn-poses-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/march-autumn-poses-preview/blue-review/index.html):
  all three source frames and native West.

Static images establish recoloring only. Natural interactions, scheduling,
automatic outfits, pause behavior and full-engine rendering remain unverified.
Game-derived artwork, extracted files and packages stay ignored.

## Verification

Formatting, Clippy with warnings denied, all 168 active tests and the release
build passed. The standard suite leaves 185 opt-ins ignored; 22 local tests ran
separately and passed: the new material test, 18 retained March corpus tests
and three GML tests. Material omission/spill controls were effective; see the art
note. Other local opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-autumn-poses-final-checks.log`, the retained logs and
`tmp/world-autumn-poses-gml.log`.

All 12,980 combined variants pass exact recipe validation. The 1,244 standalone
March variants match the combined bundle. All 3,080 earlier March original/variant
PNG and metadata files remain byte-identical, as do all 29,515 files for the
other 35 characters. Previous runtime definitions and hotkeys remain exact.
Evidence: `tmp/world-autumn-poses-comparison.{json,log}`.

The definition audit preserves all 308 earlier region objects, source colors,
color groups and target mappings. Sets, portrait definitions and the collection
remain unchanged. Exactly three profile/registry paths were added. Retained
assertions change only the current corpus paths and total counts. All 36 export
reports preserve prior entries and the current source archive hash. Evidence:
`tmp/world-autumn-poses-input-audit.log`.

The native animator and NPC object used by the probe match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native Multi pack passes 20 frame/palette observations and 20 linear completions
across all five choices and four directions. The checks cover palette binding,
frame/phase preservation, cycle counters, West mirroring, wrapper idempotence
and portrait synchronization. Natural scheduling, speaking/background pause
and live rendering are not exercised. Evidence:
`tmp/world-autumn-poses-{native-source,runtime}.log`.

The summary and full gallery retain exact source/palette identity and include
every visible source pixel. Pixel, metadata and Chromium checks pass; details
are in the material note. The complete preview directory is 140,271 bytes
(about 137 KiB). All documentation links resolve.

MOMI installation passed in a fresh isolated `tmp/world-autumn-poses-playtest`
copy, including compilation and installed-pixel/animation-metadata checks.
Installed archive SHA-256:
`6419fcaa8cfb0758d3965d89589c97bf41396ca033e4b46d5b9377af0747f808`.
The mounted archive and the lab's `previous.zip` retain the original source
hash. All five frozen registry/recipe inputs stayed unchanged through generation
and installation. Evidence:
`tmp/world-autumn-poses-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed; the desktop launcher remains unchanged. An
uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Add March's fourteen injured Autumn strips, completing his Autumn folder.
The inventory is `tmp/world-autumn-poses-remaining.json`.
