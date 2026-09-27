# Reina, Juniper and March: Autumn everyday actions

The user approved the Autumn idle/walk pilots, committed as `a97386b`.
This slice adds eleven Autumn strips per character: blink South/East and
sit/eat/drink North/South/East. Each contains 31 source frames and twelve native
West mirrors, giving 129 review cases across the three characters. The user
approved this artwork for commit on 2026-09-27. Material notes: [Reina](reina-autumn-actions.md),
[Juniper](juniper-autumn-actions.md) and [March](march-autumn-actions.md).

Reina now has 193 sources, Juniper 223 and March 295. The combined 36-character
trial has 3,195 sources and 12,780 variants. Their Home, Page Down and U controls
keep portraits and supported sprites synchronized across all five choices,
starting with Vanilla. Other Autumn actions and later outfits retain their
original sprites. No runtime Rust or GML code changed.

## Source and native metadata

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or source-pin refresh was used.
All 33 strips retain Default atlas, 80×80 frames and Middle/54 origin.
Blinks have three frames at `[0.075,0.125,0.075]`; seated poses retain single-frame
defaults. Drinks and North-facing eating have three frames at duration 1.
Eating South/East has five frames at `[0.125,0.15,0.175,0.125,0.6]`.

All four native cycles are linear. Sit, eat and drink retain their seated flag;
eat and drink retain the `[240,360]` final-frame hold interval. Blinks support
South and horizontal directions; the other three cycles support all four.
West uses native horizontal mirroring of the East pack. Evidence:
`tmp/world-autumn-actions-native-inputs.json` and the three
`tmp/world-autumn-actions-*-raw.json` sidecar exports.

The synthetic package test first rejected an unregistered Autumn blink path,
then passed after all 33 exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and the three existing controls.
Evidence: `tmp/world-autumn-actions-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-autumn-actions-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-autumn-actions-preview/index.html):
  every source frame and native West mirror, grouped by character.

Static images cover recoloring. They do not establish natural state dispatch,
outfit changes, pause handling or full-engine rendering. Game-derived images,
variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 158 active tests and the release
build passed. The normal suite leaves 173 local opt-in tests ignored; 46 ran
separately and passed: three new material tests, 40 retained corpus tests
and three GML tests. The material tests passed their missed-skin and material-spill
controls. Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-autumn-actions-{final-checks,gml}.log`,
`tmp/world-autumn-actions-*-retained-*.log` and the individual material notes.

All thirty-three author raw sidecars match independent archive reads. Every combined
variant passes exact recipe validation: 12,780 across 36 characters.
All 6,780 earlier original/variant PNG and metadata files for the three
characters remain byte-identical, as do all 25,005 files for the other 33.
The 2,844 standalone variants match the combined outputs. Previous runtime
rows and controls remain exact. Evidence:
`tmp/world-autumn-actions-metadata-check.log` and
`tmp/world-autumn-actions-comparison.{json,log}`.

The definition audit preserves all 678 earlier region objects and every source
color, group and mapping, including Juniper's existing Autumn highlight alias.
All sets, portrait-only definitions and tests, and the collection remain
unchanged. Exactly thirty-three profile/registry paths are added. All 36 export
reports retain earlier source entries and the current archive hash. Retained
tests change only corpus paths and total counts; previous material assertions
remain intact. Evidence:
`tmp/world-autumn-actions-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, probes
pass 645 frame/palette observations, 225 linear completions and 240 final-frame
hold boundary checks across all five choices. They check palette binding,
wrapper idempotence, animation phase,
cycle counters, completion state, West mirroring and portrait synchronization.
Eating and drinking stay on their final frame below the native minimum hold
and complete above its maximum.
Natural movement, scheduling, pause policy, outfit changes and full-engine
rendering remain unverified. Evidence: `tmp/world-autumn-actions-runtime.log`.

The combined summary checks fifteen source/palette bindings and 118,560 exact
pixels, raw metadata and complete artwork crops. The three complete galleries
verify every source frame and West mirror with exact rendered-pixel, metadata
and browser checks, recorded in the individual notes. Chromium also passes the
combined landing page, summary and links. The preview directories total about
1.96 MiB. All new documentation links resolve. Evidence:
`tmp/world-autumn-actions-summary-check.log`,
`tmp/world-autumn-actions-preview-browser-check.json`,
`tmp/world-autumn-actions-preview-sizes.json` and
`tmp/world-autumn-actions-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-autumn-actions-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`48c29752b77535f63982f80141a0995fb42fa01c935b24afb3464fc2021f73e6`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-autumn-actions-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue the existing seasonal plan with Autumn general actions, sleeping and
kissing for these three characters. The fresh inventory leaves 18 Autumn strips
for Reina, 16 for Juniper and 30 for March. Reading and character-specific work
follow in later batches.
