# Summer garden and Autumn expansion

The user accepted the [previous seasonal special batch](world-seasonal-special.md),
committed as `6f08984`. This slice adds 31 strips, 94 source frames and 44 native
West mirrors. The user accepted this offline artwork on 2026-09-16.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Autumn idle and walk North/South/East | 6 | 15 | 5 | 222 |
| Ryis | Autumn reading, writing, closed-eye sitting, hammer, saw and wipe brow | 11 | 37 | 12 | 211 |
| Celine | Complete Summer garden outfit | 14 | 42 | 27 | 282 |

All three characters now cover their complete Spring and Summer sprite folders.
Ryis also covers all thirty-three Autumn strips. Hayden covers six of thirty
Autumn strips. Celine's fourteen garden strips finish her forty-six-strip Summer
folder. The archive-folder inventory is recorded in
`tmp/world-seasonal-expansion-remaining.json`.

F8, F10 and Insert retain the same five choices and Vanilla defaults. Runtime
Rust/GML code is unchanged. Character notes describe material boundaries and
every-frame review: [Hayden](hayden-seasonal-expansion.md),
[Ryis](ryis-seasonal-expansion.md) and [Celine](celine-seasonal-expansion.md).

## Offline review

- [Compact five-choice summary](../../generated/world-seasonal-expansion-preview/summary.png):
  Hayden walking South frame two, Ryis reading South loop frame one, and Celine
  watering East frame two in her garden outfit.
- [Complete Vanilla/Debug Blue review](../../generated/world-seasonal-expansion-preview/index.html):
  all 138 direction/frame cases, with paginated character reviews and individual
  five-choice summaries.

Static previews show recoloring and material boundaries. They do not establish
natural scheduling, automatic outfit changes, separately drawn props or live
rendering. Extracted artwork, previews, packages and isolated installs remain
ignored.

Every new source frame was inspected in all four target palettes. Reviews
preserve Hayden's purple shirt, beard and lower clothing; Ryis's gloves, coat,
books, clipboard and tools; and Celine's braids, belt, sandals and watering can.
Moving-hand shadows, detached fingers and bare garden wrists are included.
Exact checks bind each case to source pixels, metadata, palette and West reversal,
with full visible extents. Chromium checks the pages, images and navigation.

The 1020×850 combined summary is about 60 KB. Its fifteen sample bindings and
415,040 display pixels match the inspected package. Evidence is in the character
notes, `tmp/world-seasonal-expansion-summary-check.log` and
`tmp/world-seasonal-expansion-landing-browser.log`.

## Metadata and packaging

The portable package regression failed on an unregistered Autumn idle path,
then passed with the thirty-one exact registry additions. It checks PNGs,
controls and complete animation properties. An independent inventory matches
pristine archive sidecars, local geometry, author metadata and native cycle
definitions. All frames retain 80×80 geometry, Default atlas and vertical origin
54. Hayden, Celine and Ryis's reading phases retain `Middle` horizontally;
Ryis's other eight new strips retain numeric `40.0`.
Evidence: `tmp/world-seasonal-expansion-package-{red,checks}.log` and
`tmp/world-seasonal-expansion-inventory.log`.

Single-frame idles and sitting retain their timing defaults. Walk retains four
frames at `0.15`. Reading retains 3/4/3 phases, including loop durations
`[3.0, 0.1, 3.0, 0.1]`; writing retains 2/4/2 at `0.15`. Ryis's seven-frame
hammer remains at `0.1`, with its native sound. Celine's blink and kiss and the
harvest, water, saw and wipe-brow actions retain their nonuniform durations.
Seated animations retain their native state.

## Verification

Formatting, Clippy, 104 active tests and the release build pass. The three new
character corpus tests, thirty-six retained corpus tests and three GML tests
also pass. Retained tests use expanded extractions; only corpus paths,
ignore-count descriptions and three overall source-count assertions changed.
Existing mask and color assertions remain intact. Evidence:
`tmp/world-seasonal-expansion-final-checks.log`,
`tmp/world-seasonal-expansion-gml-checks.log`,
`tmp/world-seasonal-expansion-retained-{hayden,ryis,celine}-checks.log` and the
character notes.

The combined `generated/characters-world-seasonal-expansion-trial` contains
36 characters, 2,647 sources and 10,588 validated variants. All 6,840 previous
original/variant PNG and metadata files for these three characters remain
identical, as do 19,485 files for the other 33 characters. All 2,860 current
variants for the expanded characters match the inspected standalone outputs.
Previous runtime table rows and controls are unchanged. Prior profile fields,
complete palette sets and existing region objects remain identical. Evidence:
`tmp/world-seasonal-expansion-comparison.json`,
`tmp/world-seasonal-expansion-all-variants.log`,
`tmp/world-seasonal-expansion-input-audit.log` and
`tmp/world-seasonal-expansion-frozen-inputs.sha256`.

Native probes use the actual 222/211/282-row tables alongside Adeline's 282
rows. All 138 direction/frame cases run in five choices: 690 palette
observations, 185 linear completions and forty reading/writing phase
transitions. Paused/fractional state, seating, West reversal, portrait
synchronization, wrapper identity and independent selection pass. Ryis's
hammer sound reaches the simulated audio service; palette changes preserve
sound calls and state. Evidence:
`tmp/world-seasonal-expansion-{hayden,ryis,celine}-runtime.log` and corresponding
`-runtime-inputs.json` files.

These probes run the original handler and NPC `animate` method with simulated
engine services. Factory direction fallbacks, seating and sound fields are
reproduced from source; packs and loop requests are selected directly. They do
not exercise the full NPC factory/state machine, natural schedules,
speaking/background interruptions, automatic outfit changes, separately drawn
props, audible sound or live timing.

A fresh isolated MOMI install passes pixel, metadata and generated-table
validation in `tmp/world-seasonal-expansion-playtest`. Installed archive SHA-256:
`3baed4ac15a7d9b0abb864938395d9d2ee07b938b9ac10548178d95da3ffec5f`.
The source archive and retained `previous.zip` match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/world-seasonal-expansion-install-report.json`,
`tmp/world-seasonal-expansion-install.log` and
`tmp/world-seasonal-expansion-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. The desktop launcher is unchanged.

## Next coverage

Continue Hayden's Autumn blink, sit, eat and drink animations. Celine can start
an Autumn idle/walk pilot, and Ryis can start a Winter idle/walk pilot, following
the established outfit progression.
