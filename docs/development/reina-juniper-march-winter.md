# Reina, Juniper and March: Winter idle and walking

The user approved March's Autumn injured batch, committed as `7c48df6`.
This slice starts Winter coverage with six idle/walk strips per character:
North, South and East, with native West mirroring. The user approved the artwork
for commit on 2026-09-27.
Each character adds fifteen source frames and five West mirrors, making 60
review cases. Individual material notes: [Reina](reina-winter-world.md),
[Juniper](juniper-winter-world.md) and [March](march-winter-world.md).

Reina now has 217 sources, Juniper 245 and March 331. The combined 36-character
trial has 3,277 sources and 13,108 variants. Home, Page Down and U keep portraits
and supported sprites synchronized across all five choices, starting with
Vanilla. Other Winter actions and later outfits remain original. No runtime
Rust or GML code changed.

## Source and materials

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or pin refresh was used.
All eighteen strips retain Default atlas, 80×80 frames and Middle/54 origin.
Idle poses use single-frame defaults; walking has four frames at 0.15 seconds.
The native idle/walk cycles are linear and support Winter in all four directions.
West uses the East pack with native horizontal mirroring; walking retains its
idle pause fallbacks. Evidence: `tmp/world-winter-pilots-native-inputs.json`
and the three `tmp/world-winter-pilots-*-raw.json` exports.

Reina's scarf uses a portrait skin color, so those material pixels stay excluded.
March's gloves and Winter clothing remain original; exposed wrist strips change.
Juniper's hands are covered, so her North-facing strips stay unchanged. Her
circlet, cuffs and other jewelry also retain their original colors.
Each author's note records the actual frame inspections, source-material
boundaries and effective omission/spill controls.

The synthetic package test first rejected unregistered Winter idle East, then
passed after all eighteen exact paths were registered. It checks complete
animation metadata, frame counts, generated PNGs and existing controls.
Evidence: `tmp/world-winter-pilots-red.log` and
`tmp/world-winter-pilots-package-checks.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-winter-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-winter-preview/index.html):
  every source frame and native West mirror, grouped by character.

Static images establish recoloring only. Natural schedules, speaking and pause
behavior, automatic outfit changes and full-engine rendering remain unverified.
Game-derived images, extracted files and packages stay ignored.

## Verification

Formatting, Clippy with warnings denied, all 172 active tests and the release
build passed. The normal suite leaves 189 opt-ins ignored; 62 local tests ran
separately and passed: three new material tests, 56 retained character corpus
tests and three GML tests. Omission/spill controls were effective; see each
material note. Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-winter-pilots-{final-checks,gml}.log` and retained test logs.

All 13,108 combined variants pass exact recipe validation. The 3,172 standalone
variants match the combined bundle. All 7,750 earlier PNG/metadata files for these
three characters remain byte-identical, as do all 25,005 files for the other 33.
Previous runtime rows and hotkeys remain exact. Evidence:
`tmp/world-winter-pilots-comparison.{json,log}`.

The definition audit preserves all 775 earlier regions, every source color,
color group and target mapping. Sets, portrait definitions and the collection
stay unchanged. Exactly eighteen profile/registry paths were added. Retained
assertions change only current corpus paths and total counts; all 36 export
reports retain prior source entries and the current archive hash. Evidence:
`tmp/world-winter-pilots-input-audit.log`.

The probe's native animator and NPC object match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, native Multi
packs pass 300 frame/palette observations and 120 linear completions across
all five choices and four directions. Checks cover palette binding, native
frame/phase preservation, cycle counters, West mirroring, wrapper idempotence
and portrait synchronization. Natural schedules, pause policy, automatic outfit
changes and full-engine rendering remain unverified. Evidence:
`tmp/world-winter-pilots-{native-source,runtime}.log`.

The combined summary verifies fifteen source/palette bindings and
139,200 exact pixels against complete artwork crops and raw metadata. Each
character's full gallery passes exact pixel, crop, source, metadata and browser
checks. Chromium also passes the combined landing page and its images/links.
All 60 cases are available in Vanilla and Debug Blue. The four preview folders
total about 1.09 MiB. Evidence: the material notes,
`tmp/world-winter-pilots-summary-check.log`,
`tmp/world-winter-pilots-preview-browser-check.json` and
`tmp/world-winter-pilots-preview-sizes.json`.

MOMI installation passed in the fresh isolated `tmp/world-winter-pilots-playtest`
copy, including compilation and installed-pixel/animation-metadata checks.
Installed archive SHA-256:
`b6b5fc64fac713b6763dec3e5c021098ce9db8c8b679e4098fff194df7f4b77f`.
The mounted archive and the lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs stayed unchanged through generation
and installation. Evidence:
`tmp/world-winter-pilots-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed; the desktop launcher remains unchanged. An
uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Add Winter blinking, sitting, eating and drinking for Reina, Juniper and March.
Their Winter folders currently cover six of 35, 33 and 44 strips respectively.
The remaining inventory is `tmp/world-winter-pilots-remaining.json`.
