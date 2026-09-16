# Hayden, Ryis and Celine: riding, Summer and garden actions

The user-approved [outfit idle/walk pilots](world-outfit-pilots.md) were
committed as `4da6359`. This parallel expansion adds 22 strips, 75 source
frames and 36 native West mirrors. The user accepted the offline artwork review.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Riding idle 2, idle 3 and blink | 6 | 35 | 20 | 180 |
| Ryis | Summer blink, sit, eat and drink | 11 | 31 | 12 | 162 |
| Celine | Spring garden blink and sit | 5 | 9 | 4 | 230 |

Native directions vary: idle 3 is East only, blinks are South/East, and other
actions include North/South/East. West mirrors East. F8, F10 and Insert retain
their existing choices and Vanilla defaults. Rust and GML runtime code is
unchanged. Prior regions, groups and target mappings remain unchanged.

Character notes record every-frame inspection and literal skin/material
boundaries: [Hayden](hayden-outfit-actions.md), [Ryis](ryis-outfit-actions.md)
and [Celine](celine-outfit-actions.md). Hayden's moving fingers are separated
from shirt shadows and the horse's animated muzzle, eyes and gear. Ryis's
raised fingers and exposed legs recolor while gloves, cups, footwear, hair
and red mouth details stay original. Celine's exposed arms and hand outlines
recolor while clothing, hair and boot outlines stay original.

## Offline review

- [Compact five-choice summary](../../generated/world-outfit-actions-preview/summary.png):
  Hayden's riding idle 2 South frame two, Ryis's drink South frame two, and
  Celine's seated South frame.
- [Complete review](../../generated/world-outfit-actions-preview/index.html):
  all 111 direction/frame cases in Vanilla/Debug Blue, with individual
  five-choice summaries.

Authors inspected every source frame in all four targets. Integration
inspection also covered Hayden's alternate horse poses, Ryis's raised eating
hand and red mouth, and Celine's seated outlines. Full reviews are paginated
at no more than eight cases per page. Exact pixel, source, metadata and
West-mirror checks bind every review case to the inspected package; Chromium
checks all images and links. The 1020×850 combined summary verifies all
fifteen bindings and 283,800 display pixels, with full visible extents.
Evidence: the character notes, `tmp/world-outfit-actions-summary-check.log`
and `tmp/world-outfit-actions-landing-browser.log`.

## Verification

The portable package test first failed on an unregistered riding-blink path,
then passed after the 22 exact registry additions. It checks every packaged
PNG, hotkey and complete animation properties. Independent source inventory
confirms all new metadata, local geometry and native cycle definitions.
All strips retain 80×80 frames, Default atlas and Middle/54 origin. Seated
single-frame defaults, scalar durations and nonuniform arrays remain intact.
Hayden's idle 2 East has nine frames ending in a `0.8` duration; idle 3 East
has eight. Ryis's eating East/South have five frames while North has three.
Evidence: `tmp/world-outfit-actions-package-{red,checks}.log` and
`tmp/world-outfit-actions-inventory.log`.

Formatting, Clippy, 98 active tests and the release build pass. The three
new corpus tests, eighteen retained character corpus tests and three GML tests
also pass. Existing mask/color assertions are preserved; retained tests use
the expanded local extractions. The initial full run hit `Text file busy`
while an existing installer recovery test started its generated test runner.
That test passed alone and the complete suite then passed, without changing
installer code or tests. Evidence: `tmp/world-outfit-actions-initial-checks.log`,
`tmp/world-outfit-actions-installer-retry.log`,
`tmp/world-outfit-actions-final-checks.log`,
`tmp/world-outfit-actions-retained-{hayden,ryis,celine}-checks.log`,
`tmp/world-outfit-actions-gml-checks.log` and the character notes.

The combined `generated/characters-world-outfit-actions-trial` contains
36 characters, 2,504 sources and 10,016 validated variants. All 5,500 earlier
original/variant PNG and metadata files for these three characters remain
identical, as do 19,485 files for the other 33 characters. All 2,288 current
variants for the three expanded characters equal their inspected standalone
outputs. Previous table rows and controls remain identical. Evidence:
`tmp/world-outfit-actions-comparison.json`,
`tmp/world-outfit-actions-all-variants.log` and
`tmp/world-outfit-actions-frozen-inputs.sha256`.

Native probes use the actual 180/162/230-row tables alongside Adeline's 282
rows. They exercise all 111 new direction/frame cases in five choices, 155
linear completions and 45 explicitly configured blink-to-idle resets.
Ryis's 40 eating/drinking held-frame checks retain the native `[240,360]`
random hold: they check persistence below the minimum and completion beyond
the maximum, leaving the interpreter's random function unchanged.
Paused/fractional state, seated flags, West reversal, portrait synchronization,
wrapper identity and independent selections also pass. The original handler
and NPC `animate` method run with simulated engine services. Factory direction
fallbacks and cycle fields follow the source; a small NPC-method adapter
performs the configured blink reset. Evidence:
`tmp/world-outfit-actions-{hayden,ryis,celine}-runtime.log` and corresponding
`-runtime-inputs.json` files.

These probes select packs and reset requests directly. Automatic blink
triggering, natural schedules, full NPC factory/state machines, separately
drawn props, outfit dispatch, audible sound and live timing are not exercised.

A fresh isolated MOMI install passed pixel, metadata and generated-table
validation in `tmp/world-outfit-actions-playtest`. Installed archive SHA-256:
`73b22cf5e769b04286f87deda7d268bb63ea0221cfa1aff4207e101ced20f3f9`.
The source archive and retained `previous.zip` match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/world-outfit-actions-install-report.json`,
`tmp/world-outfit-actions-install.log` and
`tmp/world-outfit-actions-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. The desktop launcher is unchanged. Game artwork,
previews, packages and isolated installs remain ignored.

## Next coverage

The existing outfit inventory now leaves six Hayden riding strips (run/jump),
six Celine garden strips (kiss, shocked start/loop/end, water and harvest),
and sixteen Ryis Summer strips. The next batch can finish those Hayden/Celine
outfits and add Ryis's five general-action, sleep and kiss strips. His remaining
eleven Summer special strips can follow separately. Inventory:
`tmp/world-outfit-actions-remaining.json`.
