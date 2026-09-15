# Hayden, Ryis and Celine: outfit idle/walk pilots

The user-approved [remaining Spring specials](spring-world-special.md) were
committed as `c7c5c1f`. This parallel batch adds six strips per character, with
45 source frames and 15 native West mirrors. The user accepted the offline artwork review.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Spring riding idle 1 and walk | 6 | 15 | 5 | 174 |
| Ryis | Summer idle and walk | 6 | 15 | 5 | 151 |
| Celine | Spring garden idle and walk | 6 | 15 | 5 | 225 |

All include North, South and East source strips. F8, F10 and Insert retain
their existing choices and Vanilla defaults. No Rust or GML runtime changes
are required. The previous regions, color groups and target mappings remain
unchanged; only the six reviewed regions and registry paths per character are
added. Character notes document the masks and literal material boundaries:
[Hayden](hayden-outfit-pilot.md), [Ryis](ryis-outfit-pilot.md) and
[Celine](celine-outfit-pilot.md).

Hayden's horse, mane, tail, saddle and gear are baked into these strips and
stay original. Ryis's exposed lower legs recolor while his dark gloves, short
rear hair and pink footwear stay original. Celine's bare arms and small leg
edges recolor while her hair, scarf, belt and boots stay original.

## Offline review

- [Compact five-choice summary](../../generated/world-outfit-pilots-preview/summary.png):
  walking South, frame two, for all three characters.
- [Complete review](../../generated/world-outfit-pilots-preview/index.html):
  individual summaries and paginated Vanilla/Debug Blue views for every new
  frame and native West mirror, 60 cases in total.

Authors inspected every new frame in all four targets. Integration inspection
also covered the horse/gear, Summer glove and leg edges, and garden clothing
boundaries. The compact summary is 1020×850 and checks all fifteen bindings,
343,720 display pixels, native metadata, source hashes and full visible extents.
Each character's review has three pages, at most eight cases per page.
Exact pixel checks and Chromium checks bind the full reviews to the inspected
outputs. Evidence: the character notes,
`tmp/world-outfit-pilots-summary-check.log` and
`tmp/world-outfit-pilots-landing-browser.log`.

## Verification

The portable package test first failed on an unregistered Hayden riding path
and passed after the eighteen registry additions. It checks each PNG, control
and complete animation properties. All new strips retain 80×80 geometry,
Default atlas and Middle/54 origin. Idle retains omitted single-frame defaults;
walk retains four frames, with scalar duration `0.125` for Hayden and `0.15`
for Ryis and Celine. Independent archive inventory also verifies the original
Spring idle/walk metadata and native cycle definitions.
Evidence: `tmp/world-outfit-pilots-package-{red,checks}.log` and
`tmp/world-outfit-pilots-inventory.log`.

Formatting, Clippy, 97 active tests and the release build pass. The three new
local corpus tests, fifteen retained character corpus tests and three GML tests
also pass. Retained corpus assertions use the expanded extractions without
changing their existing mask or color expectations. Evidence:
`tmp/world-outfit-pilots-final-checks.log`,
`tmp/world-outfit-pilots-retained-{hayden,ryis,celine}-checks.log`,
`tmp/world-outfit-pilots-gml-checks.log` and the character notes.

The combined `generated/characters-world-outfit-pilots-trial` contains
36 characters, 2,482 sources and 9,928 validated variants. All 5,320 earlier
original/variant PNG and metadata files for these three characters remain
identical, as do 19,485 files for the other 33 characters. All 2,200 current
variants for the three expanded characters equal their inspected standalone
outputs. Previous table rows and controls remain identical. Evidence:
`tmp/world-outfit-pilots-comparison.json`,
`tmp/world-outfit-pilots-all-variants.log` and
`tmp/world-outfit-pilots-frozen-inputs.sha256`.

Native probes use the actual 174/151/225-row tables alongside Adeline's
282 rows. Across the three characters they exercise 120 direction/frame cases
in five choices and 240 linear completions at original last-frame boundaries.
Each probe includes both the new pilot and original Spring idle/walk packs.
Explicit Spring-to-pilot-to-Spring walking sequences retain each selected
palette and synchronized portrait. Paused/fractional state, West reversal,
independent selection, wrapper identity and sound-call preservation also pass.
The original handler and NPC `animate` method run with simulated engine
services; pack direction fallbacks and sound fields reproduce the source
factory. Hayden's walking horse sound is accepted by the simulated audio service.
Evidence: `tmp/world-outfit-pilots-{hayden,ryis,celine}-runtime.log` and
corresponding `-runtime-inputs.json` files.

These probes select packs directly. They do not exercise automatic outfit
dispatch, natural schedules, the full NPC factory/state machine, speaking or
background interruptions, audible sound, or live rendering.

A fresh isolated MOMI install passed pixel, metadata and generated-table
validation in `tmp/world-outfit-pilots-playtest`. Installed archive SHA-256:
`8fa4039c14b021a3254dc884b27ae77335e1b9bcff43215a018b6d63ce0b639d`.
The source archive and retained `previous.zip` match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/world-outfit-pilots-install-report.json`,
`tmp/world-outfit-pilots-install.log` and
`tmp/world-outfit-pilots-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. The desktop launcher is unchanged. Game artwork,
previews, packages and isolated installs remain ignored.

## Next coverage

The archive inventory leaves twelve Hayden riding strips, eleven Celine garden
strips and twenty-seven Ryis Summer strips outside this pilot. These include
Hayden's alternate riding idles, blink, run and jump; Celine's garden blink,
sit, kiss, shocked, watering and harvest; and Ryis's remaining Summer actions.
Inventory: `tmp/world-outfit-pilots-remaining.json`.

With this pilot accepted, continue with the same per-character authors and
small review batches: Hayden's alternate idles/blinks, Celine's garden
blinks/sitting, and Ryis's Summer blinks/sitting/eating/drinking. Leave the longer
movement and special-action sequences for subsequent batches.
