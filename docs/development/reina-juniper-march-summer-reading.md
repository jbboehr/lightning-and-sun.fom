# Reina, Juniper and March: Summer seated reading

The user approved the Summer action/sleep/kiss batch, committed as `b54db8c`.
This slice adds three Summer seated-reading strips per character: start, loop
and end, all South-facing. The user approved this artwork for commit on
2026-09-26, including Juniper's two bracer-edge exceptions. Each character adds
ten source frames, giving thirty review cases. Material decisions are recorded
separately for [Reina](reina-summer-reading.md), [Juniper](juniper-summer-reading.md)
and [March](march-summer-reading.md).

Reina now has 166 sources, Juniper 198 and March 259. The combined 36-character
trial has 3,107 sources and 12,428 variants. Their existing Home, Page Down and U
controls keep portraits and supported sprites synchronized across all five
choices, starting with Vanilla. Other Summer actions and later outfits remain
original. No runtime Rust or GML code changed.

## Source and native metadata

The mounted source archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or pin refresh was used.
All nine strips retain Default atlas, 80×80 frames and Middle/54 offset.
Start/end each have three frames at 0.1 seconds; the loop has four frames at
`[3.0,0.1,3.0,0.1]`. The native `read_sit` cycle is complex, seated and
South-facing, with start/loop/end phases. The review contains every source
frame; these South-facing strips do not need additional West review images.
Evidence: `tmp/world-summer-reading-native-inputs.json` and the three
`tmp/world-summer-reading-*-raw.json` exports.

The synthetic package test first rejected an unregistered Summer reading path,
then passed after all nine exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and the existing controls.
Evidence: `tmp/world-summer-reading-package-{red,checks}.log`.
All nine author raw sidecars match the independent archive reads.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-summer-reading-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-summer-reading-preview/index.html):
  all thirty frames across the start, loop and end phases, grouped by character.

Juniper has two new bracer-edge pixel occurrences inseparable from nearby hand
skin under the current component-mask rules. They recolor to keep skin coverage
complete, following the earlier compromise. Separable borders remain original.
Her material notes and gallery identify the affected frames; literal assertions
cover both the exceptions and protected borders. Earlier approved masks remain
unchanged.

Static previews cover recoloring; they do not establish natural reading
interactions, seating, outfit changes, pauses or full-engine rendering.
Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 148 active tests and the release
build passed. The normal suite leaves 161 local opt-in tests ignored; 34 ran
separately and passed: three new material tests, 28 retained character corpus
tests and three GML tests. The new material tests also passed their missed-skin
and material-spill controls. Older opt-ins and live gameplay were not rerun.
Evidence: `tmp/world-summer-reading-final-checks.log`,
`tmp/world-summer-reading-*-retained-*.log`, `tmp/world-summer-reading-gml.log`
and the individual material notes.

Every combined variant passes exact recipe validation: 12,428 across 36 characters.
All 6,140 earlier original/variant PNG and metadata files for these three
characters remain byte-identical, as do all 25,005 files for the other 33.
The 2,492 standalone variants match the combined outputs. Previous runtime rows
and controls remain exact. Evidence: `tmp/world-summer-reading-comparison.{json,log}`.

The definition audit preserves all 614 earlier region objects, every source
color, color group and target mapping. Sets, portrait-only definitions and the
collection remain unchanged. Exactly nine profile/registry paths are added;
Reina's stylized description now mentions Summer reading. All 36 export reports
retain earlier source entries and the current archive hash. Retained tests
change only corpus paths and total counts; previous material assertions remain
intact. Evidence: `tmp/world-summer-reading-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, native
complex Single packs pass 150 frame/palette observations and 60 phase transitions
across all five choices. They check palette binding, wrapper idempotence,
animation phase, cycle counters, portrait synchronization and the complete
start-to-loop-to-end progression, including repeated loops and completion.
Natural reading interactions, seating, scheduling, pause policy, outfit changes
and full-engine rendering remain unverified.
Evidence: `tmp/world-summer-reading-runtime.log`.

The combined summary checks fifteen source/palette bindings and 142,560 exact
pixels, raw metadata and complete artwork crops. The three galleries verify
every source frame with exact rendered-pixel, metadata and browser checks;
their records are in the individual material notes. Chromium also passes the
combined landing page, summary and local links. The four preview directories
total about 0.70 MiB.
Evidence: `tmp/world-summer-reading-summary-check.log`,
`tmp/world-summer-reading-preview-browser-check.json`,
`tmp/world-summer-reading-preview-sizes.json` and
`tmp/world-summer-reading-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-summer-reading-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`a62e50f3a064b9ba6d0979fef197ebfd28030969a1d7a126041bb18d2d9f67c3`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-summer-reading-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue the Summer character-specific work: Reina's writing, Juniper's laugh
and March's smithing. The fresh inventory leaves ten Summer strips for Reina,
eight for Juniper and nineteen for March. Kitchen work, remaining magic and
March's injured poses follow in later slices.
