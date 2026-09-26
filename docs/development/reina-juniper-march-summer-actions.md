# Reina, Juniper and March: Summer everyday actions

The user approved the Summer idle/walk batch, committed as `7a94055`.
This slice adds eleven Summer strips per character: blink South/East and
sit/eat/drink North/South/East. The user approved this artwork for commit on 2026-09-26.
Each character adds 31 source frames and twelve native West mirrors; the full
review has 129 cases. Material decisions are recorded separately for
[Reina](reina-summer-actions.md), [Juniper](juniper-summer-actions.md) and
[March](march-summer-actions.md).

Reina now has 158 sources, Juniper 190 and March 251. The combined 36-character
trial has 3,083 sources and 12,332 variants. Their existing Home, Page Down and U
controls keep portraits and supported sprites synchronized across all five
choices, starting with Vanilla. Other Summer actions and later outfits remain
original. No runtime Rust or GML code changed.

## Source and native metadata

The mounted source archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or pin refresh was used.
All 33 strips retain Default atlas, 80×80 frames and Middle/54 offset.
Blinks have three frames at `[0.075,0.125,0.075]`; seated poses have single-frame
defaults. Drinking and North-facing eating have three frames at 1.0 seconds.
South/East eating has five frames at `[0.125,0.15,0.175,0.125,0.6]`.

All four native cycles are linear. Sitting, eating and drinking retain their
seated flag. Eating and drinking retain the native `[240,360]` final-frame hold.
West shares the East pack with horizontal mirroring; blinking supports South
and horizontal directions. Evidence: `tmp/world-summer-actions-native-inputs.json`
and the three `tmp/world-summer-actions-*-raw.json` exports.

The synthetic package test first rejected an unregistered Summer blink path,
then passed after all 33 exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and the existing controls.
Evidence: `tmp/world-summer-actions-package-{red,checks}.log`.
All 33 author raw sidecars match the independent archive reads.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-summer-actions-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-summer-actions-preview/index.html):
  all frames and native West mirrors, grouped by character.

Juniper has fifteen bracer-edge pixel occurrences inseparable from nearby skin
under the current component-mask rules. They recolor to keep skin coverage
complete, following the earlier bracer-edge compromise. Separable gold borders
remain original. Her material notes and gallery identify the affected frames;
literal assertions cover both the exceptions and protected borders.

Static previews cover recoloring; they do not establish natural cycle dispatch,
outfit changes, seating, pauses or full-engine rendering. Game-derived images,
variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 144 active tests and the release
build passed. The normal suite leaves 155 local opt-in tests ignored; 28 ran
separately and passed: three new material tests, 22 retained character corpus
tests and three GML tests. The new material tests also passed their missed-skin
and material-spill controls. Older opt-ins and live gameplay were not rerun.
Evidence: `tmp/world-summer-actions-final-checks.log`,
`tmp/world-summer-actions-*-retained-*.log`, `tmp/world-summer-actions-gml.log`
and the individual material notes.

Every combined variant passes exact recipe validation: 12,332 across 36 characters.
All 5,660 earlier original/variant PNG and metadata files for these three
characters remain byte-identical, as do all 25,005 files for the other 33.
The 2,396 standalone variants match the combined outputs. Previous runtime rows
and controls remain exact. Evidence: `tmp/world-summer-actions-comparison.{json,log}`.

The definition audit preserves all 566 earlier region objects, every source
color, color group and target mapping. Sets, portrait-only definitions and the
collection remain unchanged. Exactly 33 profile/registry paths are added;
Reina's stylized description now includes Summer actions. All 36 export reports
retain earlier source entries and the current archive hash. Retained tests
change only corpus paths and total counts; previous material assertions remain
intact. Evidence: `tmp/world-summer-actions-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, probes
pass 645 frame/palette observations, 225 linear completions and 240 final-frame
hold boundary checks across all five choices. They check palette binding,
wrapper idempotence, animation phase, cycle counters, completion state, West
mirroring and portrait synchronization. Eating and drinking hold below the
native minimum and complete above the maximum. Natural cycle dispatch,
seating, scheduling, pause policy, outfit changes and full-engine rendering
remain unverified. Evidence: `tmp/world-summer-actions-runtime.log`.

The combined summary checks fifteen source/palette bindings and 116,480 exact
pixels, raw metadata and complete artwork crops. The three galleries verify
every source frame and native West mirror with exact rendered-pixel, metadata
and browser checks; their records are in the individual material notes.
Chromium also passes the combined landing page, summary and local links.
The four preview directories total about 1.96 MiB.
Evidence: `tmp/world-summer-actions-summary-check.log`,
`tmp/world-summer-actions-preview-browser-check.json`,
`tmp/world-summer-actions-preview-sizes.json` and
`tmp/world-summer-actions-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-summer-actions-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`6ab8a63d900a402ae91a6d5715ee08642127aaa623159d3ae42342e960b0b90c`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-summer-actions-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Add Summer general actions, sleeping and kissing: five strips per character.
The fresh inventory leaves eighteen Summer strips for Reina, sixteen for
Juniper and 27 for March after this batch. Reading and character-specific work
follow in later slices.
