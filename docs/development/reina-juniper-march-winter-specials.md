# Reina, Juniper and March: Winter writing, laugh and smithing

The user approved the Winter seated-reading batch, committed as `ee38899`.
This slice adds fourteen strips: Reina's standing and seated writing, Juniper's
laugh, and March's seated work, hammering and brow-wiping. The user approved
the artwork for commit on 2026-09-28.

| Character | New strips | Source frames | West mirrors | Total sources | Winter coverage | Art notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Reina | 6 | 16 | 0 | 242 | 31/35 | [Reina](reina-winter-specials.md) |
| Juniper | 3 | 5 | 0 | 267 | 28/33 | [Juniper](juniper-winter-specials.md) |
| March | 5 | 23 | 7 | 355 | 30/44 | [March](march-winter-specials.md) |

The combined 36-character trial has 3,348 sources and 13,392 variants. Home,
Page Down and U retain the five palette choices and portrait synchronization.
Unsupported Winter actions remain original. No runtime Rust or GML code changed.

## Sources and materials

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Pins remain strict; no mismatch override or earlier pin refresh was used.
All fourteen strips retain Default atlas, 80×80 frames and vertical offset 54.
Standing writing, laugh, hammer and brow-wiping use numeric horizontal offset
40.0; seated writing and work use Middle. Raw sidecars preserve mixed duration
arrays and absent timing fields on single-frame strips.

Writing and laugh face South. March's seated work faces North, hammer faces
East with native West mirroring, and brow-wiping faces South. The four complex
cycles retain start/loop/end phases and seated flags. Hammer retains its native
sound configuration. Fresh Winter metadata matches the previously inspected
seasonal configuration. All fourteen author sidecars match independent archive
reads. Evidence: `tmp/world-winter-specials-native-inputs.json`, the three
raw sidecar exports and `tmp/world-winter-specials-{metadata,timing}-check.log`.

Each author inspected every actual frame across four target palettes and
checked shared skin/material colors. Winter clothing, books, stationery,
gloves and tools retain their original colors. Unlike the Autumn outfit,
March's Winter seated-work loop exposes two rear-neck pixels per frame; those
are included in the recoloring. See the material notes for literal landmarks
and effective omission/spill controls.

The synthetic package test first rejected unregistered Reina writing end,
then passed after all fourteen exact paths were registered. It checks complete
animation metadata, frame totals, generated PNGs and existing controls.
Evidence: `tmp/world-winter-specials-{red,package-checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-winter-specials-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-winter-specials-preview/index.html):
  all 44 source frames and seven native West hammer mirrors.

Static previews cover recoloring; they do not establish natural interactions,
seating, schedules, automatic outfit changes, pauses, sound or full-engine
rendering. Game-derived images, extracted files and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 180 active tests and the release
build passed. The normal suite leaves 201 opt-ins ignored; 74 local tests ran
separately and passed: three new material tests, 68 retained character corpus
tests and three GML tests. Omission/spill controls were effective; see each
material note. Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-winter-specials-{final-checks,gml}.log` and retained test logs.

All 13,392 combined variants pass exact recipe validation. The 3,456 standalone
variants match the combined bundle. All 8,500 earlier PNG/metadata files for these
three characters remain byte-identical, as do all 25,005 files for the other 33.
Previous runtime rows and hotkeys remain exact. Evidence:
`tmp/world-winter-specials-comparison.{json,log}`.

The definition audit preserves all 850 earlier regions, every source color,
color group and target mapping. Sets, portrait definitions and the collection
stay unchanged. Exactly fourteen profile/registry paths were added. Retained
assertions change only current corpus paths and total counts; all 36 export
reports retain prior source entries and the current archive hash. Evidence:
`tmp/world-winter-specials-input-audit.log`.

The probe's native animator and NPC object match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, native animation
packs pass 255 frame/palette observations and 80 start/loop/end transitions plus 15 linear completions across
all five choices and native directions, including West hammer mirroring. Checks cover palette binding, native
frame/phase preservation, cycle counters, wrapper idempotence
and portrait synchronization. Natural schedules, pause policy, automatic outfit
changes and full-engine rendering remain unverified. Evidence:
`tmp/world-winter-specials-{native-source,runtime}.log`.

The combined summary verifies fifteen source/palette bindings and
140,080 exact pixels against complete artwork crops and raw metadata. Each
character's full gallery passes exact pixel, crop, source, metadata and browser
checks. Chromium also passes the combined landing page and its images/links.
All 51 frame/direction cases are available in Vanilla and Debug Blue. The four preview folders
total about 1.06 MiB. Evidence: the material notes,
`tmp/world-winter-specials-summary-check.log`,
`tmp/world-winter-specials-preview-browser-check.json` and
`tmp/world-winter-specials-preview-sizes.json`.

MOMI installation passed in the fresh isolated `tmp/world-winter-specials-ee38899-playtest`
copy, including compilation and installed-pixel/animation-metadata checks.
Installed archive SHA-256:
`cec19b71d02fdefe6a6eb69484b32d2a09da1a00028faa2e8d20633796def4eb`.
The mounted archive and the lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs stayed unchanged through generation
and installation. Evidence:
`tmp/world-winter-specials-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed; the desktop launcher remains unchanged. An
uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Finish Reina's four Winter kitchen-work strips and Juniper's five magic/gesture
strips. March has fourteen Winter injured strips left; keep those as a separate
slice. Inventory: `tmp/world-winter-specials-remaining.json`.
