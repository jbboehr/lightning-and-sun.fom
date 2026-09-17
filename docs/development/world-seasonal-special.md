# Summer specials and Ryis's Autumn standard animations

The user accepted the [previous seasonal standard batch](world-seasonal-standard.md),
committed as `1c227f1`. This slice adds 23 strips, 108 source frames and 41 native
West mirrors. The user accepted this offline artwork on 2026-09-16.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Summer seated reading, water, wipe brow, harvest, till and hammer | 8 | 41 | 25 | 216 |
| Ryis | Autumn general actions, sleep and kiss | 5 | 26 | 12 | 200 |
| Celine | Regular Summer standing/seated reading, sweep and water | 10 | 41 | 4 | 268 |

Hayden now covers his complete thirty-strip Summer sprite folder. Celine covers
all thirty-two regular Summer strips; her fourteen Summer garden strips remain
separate. Ryis covers twenty-two of thirty-three Autumn strips. Prior coverage
remains intact. The archive-folder inventory is recorded in
`tmp/world-seasonal-special-remaining.json`.

F8, F10 and Insert retain the same five choices and Vanilla defaults. Runtime
Rust/GML code is unchanged. Character notes describe material boundaries and
every-frame review: [Hayden](hayden-seasonal-special.md),
[Ryis](ryis-seasonal-special.md) and [Celine](celine-seasonal-special.md).

## Offline review

- [Compact five-choice summary](../../generated/world-seasonal-special-preview/summary.png):
  Hayden harvesting East frame three, Ryis's general action South frame three,
  and Celine reading while standing South loop frame one.
- [Complete Vanilla/Debug Blue review](../../generated/world-seasonal-special-preview/index.html):
  all 149 direction/frame cases, with paginated character reviews and individual
  five-choice summaries.

Static previews show recoloring and material boundaries. They do not establish
natural scheduling, automatic outfit changes, separately drawn props or live
rendering. Extracted artwork, previews, packages and isolated installs remain
ignored.

Every new source frame was inspected in all four target palettes. Reviews
preserve Hayden's shirt seams, beard, book and tools; Ryis's gloves, coat and
mouth outline; and Celine's hair, belt, sandals, books, broom and watering can.
Detached fingers and bare Summer arm shadows are included. Exact checks bind
each review case to the source, palette, metadata and West reversal, with full
visible extents. Chromium checks the pages, images and navigation links.

The 1020×850 combined summary is about 60 KB. Its fifteen sample bindings and
431,280 display pixels match the inspected package. Evidence is in the character
notes, `tmp/world-seasonal-special-summary-check.log` and
`tmp/world-seasonal-special-landing-browser.log`.

## Metadata and packaging

The portable package regression failed on an unregistered Summer hammer path,
then passed with the twenty-three exact registry additions. It checks PNGs,
controls and complete animation properties. An independent inventory matches
pristine archive sidecars, local geometry, author metadata and native cycle
definitions. All frames retain 80×80 geometry, Default atlas and Middle/54 origin.
Evidence: `tmp/world-seasonal-special-package-{red,checks}.log` and
`tmp/world-seasonal-special-inventory.log`.

Reading start/loop/end phases retain 3/4/3 frames, with the native loop durations
`[3.0, 0.1, 3.0, 0.1]`. Celine's sweep retains 1/15/1 phases. Hayden's hammer,
harvest, till, water and wipe brow retain their nonuniform durations. Ryis's
general actions retain seven frames and native final-frame holds of `[240, 360]`;
kiss retains four frames and sleep retains its single-frame defaults. Seated
reading and Hayden's hammer sound retain their native configuration.

## Verification

Formatting, Clippy, 103 active tests and the release build pass. The three new
character corpus tests, thirty-three retained corpus tests and three GML tests
also pass. Retained tests use expanded extractions; only corpus paths,
ignore-count descriptions and three overall source-count assertions changed.
Existing mask and color assertions remain intact. Evidence:
`tmp/world-seasonal-special-final-checks.log`,
`tmp/world-seasonal-special-gml-checks.log`,
`tmp/world-seasonal-special-retained-{hayden,ryis,celine}-checks.log` and the
character notes.

The combined `generated/characters-world-seasonal-special-trial` contains
36 characters, 2,616 sources and 10,464 validated variants. All 6,610 previous
original/variant PNG and metadata files for these three characters remain
identical, as do 19,485 files for the other 33 characters. All 2,736 current
variants for the expanded characters match the inspected standalone outputs.
Previous runtime table rows and controls are unchanged. Prior profile fields,
complete palette sets and existing region objects remain identical. Evidence:
`tmp/world-seasonal-special-comparison.json`,
`tmp/world-seasonal-special-all-variants.log`,
`tmp/world-seasonal-special-input-audit.log` and
`tmp/world-seasonal-special-frozen-inputs.sha256`.

Native probes use the actual 216/200/268-row tables alongside Adeline's 282
rows. All 149 direction/frame cases run in five choices: 745 palette
observations, 95 linear completions, eighty reading/sweeping phase transitions
and twenty final-frame hold checks. The original random function remains in
use; holds persist below the minimum and complete beyond the maximum.
Paused/fractional state, seating, West reversal, portrait synchronization,
wrapper identity and independent selection pass. Hayden's hammer sound reaches
the simulated audio service; palette changes preserve sound calls and state.
Evidence: `tmp/world-seasonal-special-{hayden,ryis,celine}-runtime.log` and
corresponding `-runtime-inputs.json` files.

These probes run the original handler and NPC `animate` method with simulated
engine services. Factory direction fallbacks, seating, hold and sound fields
are reproduced from source; packs and loop requests are selected directly.
They do not exercise the full NPC factory/state machine, natural schedules,
speaking/background interruptions, automatic outfit changes, separately drawn
props, audible sound or live timing.

A fresh isolated MOMI install passes pixel, metadata and generated-table
validation in `tmp/world-seasonal-special-playtest`. Installed archive SHA-256:
`a2f48ab7f9a0442b6c95bdfb8e8b8308a54ed31c9ce980ac0a10300fc6f7dc0d`.
The source archive and retained `previous.zip` match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/world-seasonal-special-install-report.json`,
`tmp/world-seasonal-special-install.log` and
`tmp/world-seasonal-special-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. The desktop launcher is unchanged.

## Next coverage

Continue Celine's fourteen Summer garden strips, Ryis's eleven Autumn specials,
and a small Hayden Autumn idle/walk pilot. All three retain their complete Spring
coverage, and Hayden and Ryis retain complete Summer coverage.
