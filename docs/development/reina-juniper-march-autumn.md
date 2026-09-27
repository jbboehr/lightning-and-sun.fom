# Reina, Juniper and March: Autumn idle and walking

The user approved March's Summer injured batch, committed as `4f48c35`.
This slice starts Autumn coverage with six idle/walk strips per character:
North, South and East, with native West mirroring. The user approved this artwork
for commit on 2026-09-27.
Each character adds fifteen source frames and five West mirrors, making 60
review cases. Individual material notes: [Reina](reina-autumn-world.md),
[Juniper](juniper-autumn-world.md) and [March](march-autumn-world.md).

Reina now has 182 sources, Juniper 212 and March 284. The combined 36-character
trial has 3,162 sources and 12,648 variants. Existing Home, Page Down and U controls
keep portraits and supported sprites synchronized across all five choices,
starting with Vanilla. Other Autumn actions and later outfits remain original.
No runtime Rust or GML code changed.

## Source and material boundaries

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins are strict; no mismatch override or pin refresh was used.
All eighteen strips retain Default atlas, 80×80 frames and Middle/54 origin.
Idle poses have single-frame defaults; walking uses four frames at 0.15 seconds.
The native idle/walk cycles are linear, support Autumn and all four directions,
and use the East pack with horizontal mirroring for West. Walking retains its
native idle pause fallbacks. Evidence: `tmp/world-autumn-pilots-native-inputs.json`
and the three `tmp/world-autumn-pilots-*-raw.json` exports.

Reina's eight boot-trim pixels share a skin shade and are explicitly excluded.
Juniper's exposed midriff adds a `#F1E791` highlight alias, appended to the world
profile, presets and stylized mapping using the existing highlight target. All
previous indices and entries remain intact. March uses his existing skin ramp.
The individual notes record the actual frame inspections and material controls.

The synthetic package test first rejected an unregistered Autumn idle path,
then passed after all eighteen exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and the three existing controls.
Evidence: `tmp/world-autumn-pilots-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-autumn-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-autumn-preview/index.html):
  every source frame and native West mirror, grouped by character.

Static previews cover recoloring. They do not establish natural movement,
outfit changes, pause handling or full-engine rendering. Game-derived images,
variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 156 active tests and the release
build passed. The normal suite leaves 170 local opt-in tests ignored; 43 ran
separately and passed: three new material tests, 37 retained corpus tests
and three GML tests. The material tests passed their missed-skin and material-spill
controls. Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-autumn-pilots-{final-checks,gml}.log`,
`tmp/world-autumn-pilots-*-retained-*.log` and the individual material notes.

All eighteen author raw sidecars match independent archive reads. Every combined
variant passes exact recipe validation: 12,648 across 36 characters.
All 6,600 earlier original/variant PNG and metadata files for the three
characters remain byte-identical, as do all 25,005 files for the other 33.
The 2,712 standalone variants match the combined outputs. Previous runtime
rows and controls remain exact. Evidence:
`tmp/world-autumn-pilots-metadata-check.log` and
`tmp/world-autumn-pilots-comparison.{json,log}`.

The definition audit preserves all 660 earlier region objects and every prior
source color, group and mapping. Juniper's world profile, presets and stylized
recipe append only the verified highlight alias. Portrait-only definitions and
tests and the collection remain unchanged. Exactly eighteen profile/registry
paths are added. All 36 export reports retain earlier source entries and the
current archive hash. Retained tests change corpus paths and total counts;
Juniper's existing mapping test also expects the appended alias without changing
its prior entries or material assertions. Evidence:
`tmp/world-autumn-pilots-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, probes
pass 300 frame/palette observations and 120 linear completions across all five
choices. They check palette binding, wrapper idempotence, animation phase,
cycle counters, completion state, West mirroring and portrait synchronization.
Natural movement, scheduling, pause policy, outfit changes and full-engine
rendering remain unverified. Evidence: `tmp/world-autumn-pilots-runtime.log`.

The combined summary checks fifteen source/palette bindings and 139,200 exact
pixels, raw metadata and complete artwork crops. The three complete galleries
verify every source frame and West mirror with exact rendered-pixel, metadata
and browser checks, recorded in the individual notes. Chromium also passes the
combined landing page, summary and links. The preview directories total about
1.08 MiB. All new documentation links resolve. Evidence:
`tmp/world-autumn-pilots-summary-check.log`,
`tmp/world-autumn-pilots-preview-browser-check.json`,
`tmp/world-autumn-pilots-preview-sizes.json` and
`tmp/world-autumn-pilots-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-autumn-pilots-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`0948a0eee6a7de03b19631eb60b5aaa39c30128d63280793113f21a2e885dddc`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-autumn-pilots-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Add Autumn blinking, sitting, eating and drinking for these three characters,
following the existing seasonal coverage plan. The fresh inventory shows 29
remaining Autumn strips for Reina, 27 for Juniper and 41 for March after this
slice. General actions, reading and character-specific work follow later.
