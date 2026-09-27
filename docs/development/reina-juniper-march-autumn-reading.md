# Reina, Juniper and March: Autumn seated reading

The user approved the Autumn action/sleep/kiss batch, committed as `d0b1e1a`.
This slice adds three seated-reading strips per character: start, loop and end,
all facing South. Each character adds ten source frames, giving 30 review cases.
The user approved this artwork on 2026-09-27. Material notes:
[Reina](reina-autumn-reading.md), [Juniper](juniper-autumn-reading.md) and
[March](march-autumn-reading.md).

Reina now has 201 sources, Juniper 231 and March 303. The combined 36-character
trial has 3,219 sources and 12,876 variants. Home, Page Down and U keep their
portraits and supported sprites synchronized across all five choices, starting
with Vanilla. Other Autumn actions and later outfits retain original sprites.
No runtime Rust or GML code changed.

## Source and native metadata

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or source-pin refresh was used.
All nine strips retain Default atlas, 80×80 frames and Middle/54 origin.
Start/end have three frames at duration `0.1`; the four-frame loop retains
durations `[3.0,0.1,3.0,0.1]`.

The native `read_sit` cycle uses a seated complex Single pack with South as its
declared and default direction. The review includes all source frames without
invented mirrored views. Evidence: `tmp/world-autumn-reading-native-inputs.json`
and the three `tmp/world-autumn-reading-*-raw.json` sidecar exports.
All nine author sidecars match independent archive reads; native timing and
cycle checks passed. Evidence: `tmp/world-autumn-reading-metadata-check.log`
and `tmp/world-autumn-reading-timing-check.log`.

The synthetic package test first rejected an unregistered Autumn reading path,
then passed after all nine exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and the existing controls.
Evidence: `tmp/world-autumn-reading-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-autumn-reading-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-autumn-reading-preview/index.html):
  every source frame, grouped by character.

Static previews cover recoloring. They do not establish natural reading
interactions, seating, outfit changes, pauses or full-engine rendering.
Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 162 active tests and the release
build passed. The normal suite leaves 179 local opt-in tests ignored; 52 ran
separately and passed: three new material tests, 46 retained character corpus
tests and three GML tests. The new material tests also passed their missed-skin
and material-spill controls. Older opt-ins and live gameplay were not rerun.
Evidence: `tmp/world-autumn-reading-final-checks.log`,
`tmp/world-autumn-reading-*-retained-*.log`, `tmp/world-autumn-reading-gml.log`
and the individual material notes.

Every combined variant passes exact recipe validation: 12,876 across 36 characters.
All 7,260 earlier original/variant PNG and metadata files for these three
characters remain byte-identical, as do all 25,005 files for the other 33.
The 2,940 standalone variants match the combined outputs. Previous runtime rows
and controls remain exact. Evidence: `tmp/world-autumn-reading-comparison.{json,log}`.

The definition audit preserves all 726 earlier region objects, every source
color, color group and target mapping. Sets, portrait-only definitions and the
collection remain unchanged. Exactly nine profile/registry paths are added;
Reina's stylized description now mentions Autumn reading. All 36 export reports
retain earlier source entries and the current archive hash. Retained tests
change only corpus paths and total counts; previous material assertions remain
intact. Evidence: `tmp/world-autumn-reading-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, native
complex Single packs pass 150 frame/palette observations and 60 phase transitions
across all five choices. They check palette binding, wrapper idempotence,
animation phase, cycle counters, portrait synchronization and the complete
start-to-loop-to-end progression, including repeated loops and completion.
Natural reading interactions, seating, scheduling, pause policy, outfit changes
and full-engine rendering remain unverified.
Evidence: `tmp/world-autumn-reading-runtime.log`.

The combined summary checks fifteen source/palette bindings and 137,280 exact
pixels, raw metadata and complete artwork crops. The three galleries verify
every source frame with exact rendered-pixel, metadata and browser checks;
their records are in the individual material notes. Chromium also passes the
combined landing page, summary and local links. The four preview directories
total about 0.71 MiB.
Evidence: `tmp/world-autumn-reading-summary-check.log`,
`tmp/world-autumn-reading-preview-browser-check.json`,
`tmp/world-autumn-reading-preview-sizes.json` and
`tmp/world-autumn-reading-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-autumn-reading-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`60128a65b900f52dd86e58c73b2d6f801cc462320dbb5ca9da648070016d0d35`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-autumn-reading-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue the seasonal character-specific work with Reina's writing, Juniper's
laugh and March's smithing. The fresh inventory leaves ten Autumn strips for
Reina, eight for Juniper and 22 for March. Kitchen work, remaining magic and
March's injured poses follow in later slices.
