# March: injured Autumn sprites

The user approved March's Autumn standing poses, committed as `86fcd8c`.
This slice adds March's fourteen remaining Autumn strips: injured
idle and walk North/South/East, blink South/East, seated poses North/South/East,
seated blink South/East and a South-facing action. The user approved the artwork for commit on
2026-09-27. See the [material notes](march-autumn-injured-art.md) for the masks and
focused corpus test.

The new strips contain 37 source frames and twelve native West mirrors, making
49 review cases. March now has 325 sources and 1,300 variants; all 47 strips in
his Autumn sprite folder are covered. Reina and Juniper's Autumn folders remain
complete. The combined 36-character trial has 3,259 sources and 13,036 variants.
U keeps March's portraits and supported sprites on the same five-choice selector,
starting with Vanilla. Later overworld outfits retain their original sprites.
No runtime Rust or GML code changed.

## Source and native metadata

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or pin refresh was used.
All fourteen strips retain 80×80 frames, Default atlas and numeric origin
40.0/54.0. Walking has four frames at 0.15 seconds; standing/seated blinks have
three at `[0.075,0.125,0.075]`; idle and seated poses use single-frame defaults.
The seven-frame action retains `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` timing.

All six native cycles are linear. Idle, walk and seated poses support all four
directions; blinks support South and horizontal directions; the action faces
South. West uses the East pack with native horizontal mirroring. Seated cycles
retain their seated flag. The action retains its native `[240,360]` final-frame
hold interval and speaking fallback to injured idle. Evidence:
`tmp/world-autumn-injured-native-inputs.json` and
`tmp/march-autumn-injured-author-metadata.json`.

The synthetic package test first rejected unregistered March injured action,
then passed after all fourteen exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and March's existing U binding.
Evidence: `tmp/world-autumn-injured-red.log` and `tmp/world-autumn-injured-package-checks.log`.

## Offline review

- [Five-choice summary](../../generated/march-autumn-injured-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-autumn-injured-preview/blue-review/index.html):
  every source frame and native West mirror, including the action's final pose.

Static images cover recoloring; they do not establish natural injury-state
dispatch, schedules, interaction/outfit changes or full-engine rendering.
Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 170 active tests and the release
build passed. The standard suite leaves 186 opt-ins ignored; 23 local tests ran
separately and passed: the new material test, 19 retained March corpus tests
and three GML tests. Material omission/spill controls were effective; see the art
note. Other local opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-autumn-injured-final-checks.log`, the retained logs and
`tmp/world-autumn-injured-gml.log`.

All 13,036 combined variants pass exact recipe validation. The 1,300 standalone
March variants match the combined bundle. All 3,110 earlier March original/variant
PNG and metadata files remain byte-identical, as do all 29,515 files for the
other 35 characters. Previous runtime definitions and hotkeys remain exact.
Evidence: `tmp/world-autumn-injured-comparison.{json,log}`.

The definition audit preserves all 311 earlier region objects, source colors,
color groups and target mappings. Sets, portrait definitions and the collection
remain unchanged. Exactly fourteen profile/registry paths were added. Retained
assertions change only the current corpus paths and total counts. All 36 export
reports preserve prior entries and the current source archive hash. Evidence:
`tmp/world-autumn-injured-input-audit.log`.

The native animator and NPC object used by the probe match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 245 frame/palette observations, 95 linear completions and
ten final-frame hold checks across all five choices and supported directions.
The checks cover palette binding,
frame/phase preservation, cycle counters, West mirroring, wrapper idempotence
and portrait synchronization. The action remains on its final frame below the
native minimum and completes above its maximum hold time. Natural injury-state
dispatch, scheduling, speaking/background pause and live rendering are not
exercised. Evidence:
`tmp/world-autumn-injured-{native-source,runtime}.log`.

The previews retain exact source/palette identity and include every visible
pixel of each displayed frame. The full gallery covers all 49 cases.
Pixel, metadata and Chromium checks pass; details
are in the material note. The complete preview directory is 834,083 bytes
(about 815 KiB). All documentation links resolve.

MOMI installation passed in a fresh isolated `tmp/world-autumn-injured-playtest`
copy, including compilation and installed-pixel/animation-metadata checks.
Installed archive SHA-256:
`7ba79938740f5c58de0111debce0cab4ec19c8fbedca0dfd82de9930558a5c2e`.
The mounted archive and the lab's `previous.zip` retain the original source
hash. All five frozen registry/recipe inputs stayed unchanged through generation
and installation. Evidence:
`tmp/world-autumn-injured-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed; the desktop launcher remains unchanged. An
uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Reina, Juniper and March now have complete Spring, Summer and Autumn sprite
folders. The next proposed slice is a Winter idle/walk pilot for these three,
using the same source inspection and complete offline review workflow.
