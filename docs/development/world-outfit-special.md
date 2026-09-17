# Hayden, Ryis and Celine: riding runs/jumps and outfit specials

The user-approved [outfit action expansion](world-outfit-actions.md) is in
`9df327f`. This parallel slice adds 17 strips, 71 source frames and 38 native
West mirrors. The user accepted the offline artwork on 2026-09-16.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Spring riding run and jump | 6 | 24 | 8 | 186 |
| Ryis | Summer general action, sleep and kiss | 5 | 26 | 12 | 167 |
| Celine | Garden kiss, shocked, water and harvest | 6 | 21 | 18 | 236 |

Hayden and Celine now cover every strip in their 53-strip Spring sprite
folders, including all eighteen riding and seventeen garden strips respectively.
Ryis covers all 22 standard Summer strips, with eleven Summer special strips
remaining. This inventory compares the complete archive folders against the
registry: `tmp/world-outfit-special-remaining.json`.

F8, F10 and Insert retain their choices and Vanilla defaults. Runtime Rust/GML
code is unchanged. The existing regions, groups and target maps remain intact.
Character notes record every-frame review and material boundaries:
[Hayden](hayden-outfit-special.md), [Ryis](ryis-outfit-special.md) and
[Celine](celine-outfit-special.md). Horse/gear, clothing, hair, gloves, tools
and water effects stay original while exposed skin follows the chosen palette.

## Offline review

- [Compact five-choice summary](../../generated/world-outfit-special-preview/summary.png):
  Hayden jumping South frame two, Ryis's general action South frame three,
  and Celine watering East frame two.
- [Complete review](../../generated/world-outfit-special-preview/index.html):
  all 109 new direction/frame cases in Vanilla/Debug Blue, including every
  shocked phase, plus individual five-choice summaries.

Authors inspected every source frame in all four targets. Integration
inspection also covers Hayden's horse and jumping poses, Ryis's extended
hands and closed/kissing features, and Celine's tool and arm boundaries.
Review pages contain at most eight cases. Exact pixel, source, metadata and
West-reversal checks bind the images to the inspected package, with complete
visible extents. Chromium checks all images and links. The 1020×850 combined
summary verifies all fifteen bindings and 353,880 display pixels.
Evidence: the character notes, `tmp/world-outfit-special-summary-check.log`
and `tmp/world-outfit-special-landing-browser.log`.

## Verification

The portable package test failed on an unregistered riding-jump path, then
passed after the seventeen exact registry additions. It checks all PNGs,
controls and complete animation properties. Independent archive inventory
also verifies native cycle definitions, author metadata and local geometry.
All strips retain 80×80 frames, Default atlas and Middle/54 origin.
Scalar run timing, nonuniform duration arrays and omitted single-frame
defaults remain intact. Celine's harvest has ten frames and water has four;
her shocked start/loop/end each have one. Ryis's general actions have seven
frames each; sleep has one and kiss has four.
Evidence: `tmp/world-outfit-special-package-{red,checks}.log` and
`tmp/world-outfit-special-inventory.log`.

Formatting, Clippy, 99 active tests and the release build pass. The three new
corpus tests, twenty-one retained character corpus tests and three GML tests
also pass. Retained tests use the expanded extractions without weakening their
existing mask or color assertions. Evidence:
`tmp/world-outfit-special-final-checks.log`,
`tmp/world-outfit-special-retained-{hayden,ryis,celine}-checks.log`,
`tmp/world-outfit-special-gml-checks.log` and the character notes.

The combined `generated/characters-world-outfit-special-trial` contains
36 characters, 2,521 sources and 10,084 validated variants. All 5,720 previous
original/variant PNG and metadata files for these three characters remain
identical, as do 19,485 files for the other 33 characters. All 2,356 current
variants for the three expanded characters equal the inspected standalone
outputs; previous table rows and controls are unchanged. Evidence:
`tmp/world-outfit-special-comparison.json`,
`tmp/world-outfit-special-all-variants.log` and
`tmp/world-outfit-special-frozen-inputs.sha256`.

Native probes use the actual 186/167/236-row tables alongside Adeline's 282
rows. All 109 new direction/frame cases run in five choices, with 110 linear
completions and twenty Celine shocked phase transitions at original frame
boundaries. Ryis's twenty held-frame checks retain `[240,360]` and the
interpreter's original random function, checking persistence below the
minimum and completion beyond the maximum. Paused/fractional state, West
reversal, portrait synchronization, wrapper identity and independent selection
pass. Hayden's native gallop sound calls reach the simulated audio service;
palette changes preserve those calls and the native audio state.
Evidence: `tmp/world-outfit-special-{hayden,ryis,celine}-runtime.log` and
corresponding `-runtime-inputs.json` files.

The original handler and NPC `animate` method run with simulated engine
services; direction fallbacks and cycle fields reproduce the source factory.
Probes select packs and loop requests directly. They do not exercise the full
NPC factory/state machine, natural schedules, speaking/background interruptions,
automatic outfit changes, separate prop drawing, audible sound or live timing.

A fresh isolated MOMI install passed pixel, metadata and generated-table
validation in `tmp/world-outfit-special-playtest`. Installed archive SHA-256:
`79a82bc443f3c2da7f3855161a051180c62146ff81063656e8bdafd6ca534321`.
The source archive and retained `previous.zip` match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/world-outfit-special-install-report.json`,
`tmp/world-outfit-special-install.log` and
`tmp/world-outfit-special-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. The desktop launcher is unchanged. Game artwork,
previews, packages and isolated installs remain ignored.

## Next coverage

The queued Ryis Summer specials are seated reading and writing (all three
phases each), seated closed eyes South/East, hammer, saw and wipe brow:
eleven strips. Hayden and Celine can continue with small Summer idle/walk
pilots using the same per-character workflow before larger seasonal batches.
