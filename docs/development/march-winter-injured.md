# March: injured Winter sprites

Following Reina and Juniper's Winter completion (`a83dea9`), this slice adds
March's fourteen remaining Winter strips: injured idle and walk North/South/East,
blink South/East, seated poses North/South/East, seated blink South/East and a
South-facing action. The user approved the offline artwork on 2026-09-28. See the
[material notes](march-winter-injured-art.md) for masks and the focused corpus test.

The new strips contain 37 source frames and twelve native West mirrors, making
49 review cases. March now has 369 sources and 1,476 variants; all 44 strips in
his Winter sprite folder are covered. Reina, Juniper and March now have complete
Spring, Summer, Autumn and Winter sprite folders. The combined 36-character
trial has 3,371 sources and 13,484 variants. U keeps March's portraits and
supported sprites on the same five-choice selector, starting with Vanilla.
Other overworld outfits retain their original sprites. No runtime Rust or
GML code changed.

## Sources and native metadata

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Pins remain strict; no mismatch override or earlier pin refresh was used.
All fourteen strips retain 80×80 frames, Default atlas and numeric origin
40.0/54.0. Walking has four frames at 0.15 seconds; standing/seated blinks have
three at `[0.075,0.125,0.075]`; idle and seated poses use single-frame defaults.
The seven-frame action retains `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` timing.

All six native cycles are linear. Idle, walk and seated poses support all four
directions; blinks support South and horizontal directions; the action faces
South. West uses the East pack with native horizontal mirroring. Seated cycles
retain their seated flag. The action retains its native `[240,360]` final-frame
hold interval and speaking fallback to injured idle. All fourteen author raw
sidecars match independent archive reads. Evidence:
`tmp/world-winter-injured-native-inputs.json`,
`tmp/march-winter-injured-author-metadata.json` and
`tmp/world-winter-injured-{metadata,timing}-check.log`.

The synthetic package test first rejected unregistered March injured action,
then passed after all fourteen exact paths were registered. It checks complete
animation metadata, frame totals, PNG outputs and March's existing U binding.
Evidence: `tmp/world-winter-injured-{red,package-checks}.log`.

## Offline review

- [Five-choice summary](../../generated/march-winter-injured-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-winter-injured-preview/blue-review/index.html):
  every source frame and native West mirror, including the action's final pose.

Static images cover recoloring; they do not establish natural injury-state
dispatch, schedules, interaction/outfit changes or full-engine rendering.
Game-derived images, extracted files and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 183 active tests and the release
build passed. The standard suite leaves 204 opt-ins ignored; 29 local tests ran
separately and passed: the new material test, 25 retained March corpus tests
and three GML tests. Material omission/spill controls were effective; see the art
note. Other local opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-winter-injured-final-checks.log`, the retained logs and
`tmp/world-winter-injured-gml.log`.

All 13,484 combined variants pass exact recipe validation. The 1,476 standalone
March variants match the combined bundle. All 3,550 earlier March original/variant
PNG and metadata files remain byte-identical, as do all 30,195 files for the
other 35 characters. Previous runtime definitions and hotkeys remain exact.
Evidence: `tmp/world-winter-injured-comparison.{json,log}`.

The definition audit preserves all 355 earlier region objects, source colors,
color groups and target mappings. Sets, portrait definitions and the collection
remain unchanged. Exactly fourteen profile/registry paths were added. Retained
assertions change only the current corpus paths and total counts. All 36 export
reports preserve prior entries and the current source archive hash. Evidence:
`tmp/world-winter-injured-input-audit.log`.

The native animator and NPC object used by the probe match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 245 frame/palette observations, 95 linear completions and
ten final-frame hold checks
across all five choices and supported directions. The checks cover palette binding,
frame/phase preservation, cycle counters, West mirroring, wrapper idempotence
and portrait synchronization. The action remains on its final frame below the
native minimum and completes above its maximum hold time. Natural injury-state
dispatch, scheduling, speaking/background pause
and live rendering are not exercised. Evidence:
`tmp/world-winter-injured-{native-source,runtime}.log`.

The summary and full gallery retain exact source/palette identity and include
every visible source pixel. Pixel, metadata and Chromium checks pass; details
are in the material note. The complete preview directory is 830,507 bytes
(about 811 KiB). All documentation links resolve.

MOMI installation passed in a fresh isolated `tmp/world-winter-injured-a83dea9-playtest`
copy, including compilation and installed-pixel/animation-metadata checks.
Installed archive SHA-256:
`fbb19137985fc0b3020f169a19cf44915883ce71b94a408662c9275c1533fde2`.
The mounted archive and the lab's `previous.zip` retain the original source
hash. All five frozen registry/recipe inputs stayed unchanged through generation
and installation. Evidence:
`tmp/world-winter-injured-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed; the desktop launcher remains unchanged. An
uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Proposed next slice: Beach idle/walk pilots for Reina, Juniper and March, followed
by their remaining Beach animations. Their four seasonal folders are now
covered; the complete March Winter inventory is
`tmp/world-winter-injured-remaining.json`.
