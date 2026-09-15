# Hayden, Ryis and Celine: remaining Spring special actions

The accepted [shocked and seated-reading batch](spring-world-reactions.md) was
committed as `691282d`. This parallel slice adds 23 special strips with 106 source
frames and 48 native West mirrors. The user accepted the offline artwork review.

| Character | New actions | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Hammer, harvest, pet, sigh, till, water, wipe brow | 7 | 42 | 32 | 168 |
| Ryis | Hammer, saw, seated eyes closed, wipe brow, writing | 8 | 27 | 12 | 145 |
| Celine | Standing book, herbs, sweep, water | 8 | 37 | 4 | 219 |

Writing, standing book and sweeping include every start/loop/end phase.
The archive inventory confirms complete coverage of Ryis's 36-strip Spring
folder. Hayden has 35 non-riding strips covered, with 18 riding strips remaining;
Celine has 36 normal strips covered, with 17 garden strips remaining. Other
outfits remain outside this batch. Inventory: `tmp/spring-special-remaining.json`.
F8, F10 and Insert retain their choices and Vanilla defaults. Rust and GML
runtime code is unchanged.

All accepted region objects, source colors, groups and target maps are preserved.
Character notes record masks, literal boundary tests and every-frame art checks:
[Hayden](hayden-special.md), [Ryis](ryis-special.md) and [Celine](celine-special.md).
The masks distinguish exposed fingers/forearms from matching clothing shades,
gloves, tools, book pages, the broom and dust effects.

## Offline review

- [Compact summary](../../generated/spring-special-preview/summary.png):
  wipe-brow/herb frame three in all five choices.
- [Complete review](../../generated/spring-special-preview/index.html):
  individual five-choice summaries and paginated Vanilla/Debug Blue images for
  every new frame and native West mirror.

Authors inspected all new frames in every target. Integration inspection also
covered Hayden's harvesting forearms, Ryis's hammer effects and tiny fingers,
and Celine's book, herbs, broom and watering poses. Exact pixel/source/metadata
checks and browser checks bind the summaries and full previews to the inspected
outputs; tool/effect extents are included. The combined summary verifies all
fifteen sample bindings and 402,120 display pixels. Evidence is recorded in the
character notes and `tmp/spring-special-summary-check.log` and
`tmp/spring-special-landing-browser.log`. Static images do not demonstrate timing,
natural schedules or separately drawn props.

## Verification

The portable package fixture first failed on an unregistered hammer path and
passed after the 23 exact registry additions. It checks every strip's PNG,
control and complete animation properties, including Ryis's numeric horizontal
origin `40.0`. Hayden/Celine retain `Middle`; all retain vertical `54.0`, 80×80
frames and Default atlas. The longest strip is Celine's fifteen-frame sweep.
Scalar durations, nonuniform arrays and omitted single-frame defaults all remain
intact. Evidence: `tmp/spring-special-package-{red,checks}.log` and
`tmp/spring-special-inventory.log`.

Formatting, Clippy, 96 active tests and the release build pass. The three new
local corpus tests, twelve retained character corpus tests and three GML tests
also pass. Old corpus assertions remain intact and use the expanded extractions.
Evidence: `tmp/spring-special-final-checks.log`,
`tmp/spring-special-retained-{hayden,ryis,celine}-checks.log`,
`tmp/spring-special-gml-checks.log` and the individual character notes.

The combined `generated/characters-spring-special-trial` contains 36 characters,
2,464 sources and 9,856 validated variants. All 5,090 earlier original/variant
PNG and metadata files for these three characters remain identical, as do
19,485 files for the other 33 characters. All 2,128 variants for the expanded
characters equal their inspected standalone outputs; previous table rows and
controls remain identical. Evidence: `tmp/spring-special-comparison.json`,
`tmp/spring-special-all-variants.log` and `tmp/spring-special-frozen-inputs.sha256`.

Native probes use actual 168/145/219-row tables alongside Adeline's 282 rows.
They exercise all 154 direction/frame cases in five choices, 115 linear
completions and 60 complex phase transitions at original last-frame boundaries.
Paused/fractional state, West reversal, portrait synchronization, independent
selection and sound-call preservation are checked. The original handler and
NPC `animate` method run with simulated engine services and relative frame
durations; pack fallbacks and sound fields follow the original factory source.
The full NPC factory/state machine, natural scheduling, speaking/background
interruption, outfit dispatch and audible sound are not exercised. Evidence:
`tmp/spring-special-{hayden,ryis,celine}-runtime.log` and corresponding
`-runtime-inputs.json` files.

A fresh isolated MOMI install passed pixel, metadata and table validation.
Installed archive SHA-256:
`8e89a168241b7edb425c8f8f026fa2b24317483b3f719e366b1589391b4b8f78`.
Evidence is recorded in
`tmp/spring-special-install-report.json`, `tmp/spring-special-install.log` and
`tmp/spring-special-source-after.sha256`. The fresh local copy is
`tmp/spring-special-playtest`; no preview helper or launcher change is included.
The original source and retained `previous.zip` match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Live gameplay and an uninstall roundtrip were not rerun. Game assets, previews
and isolated installs remain ignored.
