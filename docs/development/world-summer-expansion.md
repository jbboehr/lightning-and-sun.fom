# Summer pilots for Hayden and Celine, and Ryis's remaining specials

The user accepted the [previous outfit batch](world-outfit-special.md), committed
as `23c4f1d`. This slice adds 23 strips, 67 source frames and 22 native West
mirrors. The user accepted this offline artwork on 2026-09-16.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Regular Summer idle and walk | 6 | 15 | 5 | 192 |
| Ryis | Summer reading, writing, closed-eye sitting, hammer, saw and wipe brow | 11 | 37 | 12 | 178 |
| Celine | Regular Summer idle and walk | 6 | 15 | 5 | 242 |

Ryis now covers his complete 33-strip Summer sprite folder as well as Spring.
Hayden covers six of thirty Summer strips; Celine covers six of forty-six,
including her separate garden outfit in the folder total. Their complete Spring
coverage is retained. The full archive-folder inventory is recorded in
`tmp/world-summer-expansion-remaining.json`.

F8, F10 and Insert retain the same five choices and Vanilla defaults. Runtime
Rust/GML code is unchanged. Prior regions, color groups and target maps are
preserved. Character notes describe the reviewed material boundaries:
[Hayden](hayden-summer-expansion.md), [Ryis](ryis-summer-expansion.md) and
[Celine](celine-summer-expansion.md).

## Offline review

- [Compact five-choice summary](../../generated/world-summer-expansion-preview/summary.png):
  Hayden walking South frame two, Ryis reading South frame one and Celine
  walking South frame two.
- [Complete Vanilla/Debug Blue review](../../generated/world-summer-expansion-preview/index.html):
  all 89 direction/frame cases, with paginated character reviews and individual
  five-choice summaries.

The authors inspected every source frame in all four targets. Hayden's straw hat,
shirt and beard, Ryis's books, tools and gloves, and Celine's hair, dress and
sandal straps retain their original colors. The complete reviews include every
reading/writing phase and native West mirror. Static review does not establish
natural scheduling, automatic outfit changes or live rendering.

Exact preview checks bind every case to the source, palette, metadata and West
reversal. Chromium checks every page, image and link. The combined 1020×850
summary is about 60 KB; its fifteen sample bindings and 409,680 display pixels
match the inspected package. Evidence is in the character notes,
`tmp/world-summer-expansion-summary-check.log` and
`tmp/world-summer-expansion-landing-browser.log`.

## Metadata and packaging

The portable package regression failed on an unregistered Summer idle path,
then passed after adding the twenty-three exact registry paths. It verifies
the PNGs, controls and complete animation properties. The independent inventory
matches pristine archive sidecars, local geometry, author metadata and native
cycle definitions. All frames remain 80×80, in the Default atlas, with vertical
origin 54. Ryis's reading strips retain `Middle` horizontally; his other eight
new strips retain numeric `40.0`. Hayden and Celine retain `Middle`.

Idle and closed-eye sitting retain omitted single-frame timing defaults. Walk
retains four frames at `0.15`. Ryis's reading phases have 3/4/3 frames; the loop
retains `[3.0, 0.1, 3.0, 0.1]`. Writing has 2/4/2 frames at `0.15`; hammer has
seven at `0.1`. Saw and wipe brow retain their nonuniform four- and six-frame
duration arrays. Evidence: `tmp/world-summer-expansion-package-{red,checks}.log`
and `tmp/world-summer-expansion-inventory.log`.

## Verification

Formatting, Clippy, 100 active tests and the release build pass. The three new
character corpus tests, twenty-four retained corpus tests and three GML tests
also pass. Retained tests use the expanded extractions; only their corpus paths,
ignore-count descriptions and three overall source-count assertions changed.
Existing mask and color assertions are preserved. Evidence:
`tmp/world-summer-expansion-final-checks.log`,
`tmp/world-summer-expansion-retained-{hayden,ryis,celine}-checks.log`,
`tmp/world-summer-expansion-gml-checks.log` and the character notes.

The combined `generated/characters-world-summer-expansion-trial` contains
36 characters, 2,544 sources and 10,176 validated variants. All 5,890 previous
original/variant PNG and metadata files for these three characters remain
identical, as do 19,485 files for the other 33 characters. All 2,448 current
variants for the expanded characters match the inspected standalone outputs.
Previous runtime table rows and controls are unchanged. Evidence:
`tmp/world-summer-expansion-comparison.json`,
`tmp/world-summer-expansion-all-variants.log` and
`tmp/world-summer-expansion-frozen-inputs.sha256`.

Native probes use the actual 192/178/242-row tables alongside Adeline's 282
rows. All 89 new direction/frame cases run in five choices: 445 palette
observations, 120 linear completions and forty reading/writing phase
transitions at the original frame boundaries. Paused/fractional state, West
reversal, portrait synchronization, wrapper identity and independent selection
pass. Ryis's hammer sound reaches the simulated audio service; palette changes
preserve native sound calls and state. Evidence:
`tmp/world-summer-expansion-{hayden,ryis,celine}-runtime.log` and the corresponding
`-runtime-inputs.json` files.

These probes run the original handler and NPC `animate` method with simulated
engine services. Factory direction fallbacks, seating and sound fields are
reproduced from source; animation packs and loop requests are selected directly.
The full NPC factory/state machine, natural schedules, speaking/background
interruptions, automatic outfit changes, separate prop drawing, audible sound
and live timing are not exercised.

A fresh isolated MOMI installation passes pixel, metadata and generated-table
validation in `tmp/world-summer-expansion-playtest`. Installed archive SHA-256:
`0879ac5bd0309c7704cb7310b9c4769855f4ac8d535530fa5cae8ae89b4ec23b`.
The source archive and retained `previous.zip` still match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/world-summer-expansion-install-report.json`,
`tmp/world-summer-expansion-install.log` and
`tmp/world-summer-expansion-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. The desktop launcher is unchanged. Game artwork,
previews, packages and isolated installs remain ignored.

## Next coverage

Continue Hayden and Celine's Summer blink, sit, eat and drink animations. Ryis
can move to a small Autumn idle/walk pilot before expanding that outfit.
