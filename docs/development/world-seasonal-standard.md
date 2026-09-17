# Summer standard animations and Ryis's Autumn actions

The user accepted the [previous seasonal action batch](world-seasonal-actions.md),
committed as `8fd718f`. This slice adds 21 strips, 83 source frames and 36 native
West mirrors. The user accepted this offline artwork on 2026-09-16.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Summer general actions, sleep and kiss | 5 | 26 | 12 | 208 |
| Ryis | Autumn blink, sit, eat and drink | 11 | 31 | 12 | 195 |
| Celine | Regular Summer general actions, sleep and kiss | 5 | 26 | 12 | 258 |

Hayden now covers twenty-two of thirty Summer strips. Celine covers twenty-two
of forty-six, including fourteen separate garden-outfit strips in that folder
total. Both have all standard animations for their regular Summer outfits.
Ryis covers seventeen of thirty-three Autumn strips. Prior accepted coverage
remains intact. The complete archive-folder inventory is recorded in
`tmp/world-seasonal-standard-remaining.json`.

F8, F10 and Insert retain the same five choices and Vanilla defaults. Runtime
Rust/GML code is unchanged. Character notes describe material boundaries and
every-frame review: [Hayden](hayden-seasonal-standard.md),
[Ryis](ryis-seasonal-standard.md) and [Celine](celine-seasonal-standard.md).

## Offline review

- [Compact five-choice summary](../../generated/world-seasonal-standard-preview/summary.png):
  Hayden's general action South frame three, Ryis drinking South frame two
  and Celine kissing East frame two.
- [Complete Vanilla/Debug Blue review](../../generated/world-seasonal-standard-preview/index.html):
  all 119 direction/frame cases, with paginated character reviews and individual
  five-choice summaries.

Every source frame was inspected in all four target palettes. Checks preserve
Hayden's shirt, hat and beard, Ryis's gloves, coat and scarf, and Celine's hair,
dress and sandals. Mouth interiors remain original. Hayden's isolated kissing
cheek shade uses its existing mapping and has an explicit source-art test.
Static images do not establish natural scheduling, automatic outfit changes,
separate prop drawing or live rendering.

Exact checks bind each review case to source pixels, metadata, target palette
and West reversal with full visible extents. Chromium checks every page, image
and link. The 1020×850 combined summary is about 59 KB; its fifteen sample
bindings and 381,240 display pixels match the inspected package. Evidence is in
the character notes, `tmp/world-seasonal-standard-summary-check.log` and
`tmp/world-seasonal-standard-landing-browser.log`.

## Metadata and packaging

The portable package regression failed on an unregistered Summer action path,
then passed after the twenty-one exact registry additions. It checks PNGs,
controls and complete animation properties. An independent inventory matches
pristine archive sidecars, local geometry, author metadata and native cycle
definitions. All frames retain 80×80 geometry, Default atlas and Middle/54
origin. Evidence: `tmp/world-seasonal-standard-package-{red,checks}.log` and
`tmp/world-seasonal-standard-inventory.log`.

General actions retain seven frames with
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`; kiss retains four with
`[0.15, 0.15, 0.8, 0.15]`. Sleep and sitting retain single-frame timing defaults.
Ryis's blink retains `[0.075, 0.125, 0.075]`, drink and North eating retain
three frames at `1.0`, and South/East eating retain five with
`[0.125, 0.15, 0.175, 0.125, 0.6]`. General actions, eating and drinking preserve
native random final-frame holds of `[240, 360]`; sitting/eating/drinking retain
their seated state.

## Verification

Formatting, Clippy, 102 active tests and the release build pass. The three new
character corpus tests, thirty retained corpus tests and three GML tests also
pass. Retained tests use the expanded extractions; only corpus paths,
ignore-count descriptions and three overall source-count assertions changed.
Existing mask and color assertions remain intact. Evidence:
`tmp/world-seasonal-standard-final-checks.log`,
`tmp/world-seasonal-standard-gml-checks.log`,
`tmp/world-seasonal-standard-retained-{hayden,ryis,celine}-checks.log` and the
character notes.

The combined `generated/characters-world-seasonal-standard-trial` contains
36 characters, 2,593 sources and 10,372 validated variants. All 6,400 previous
original/variant PNG and metadata files for these three characters remain
identical, as do 19,485 files for the other 33 characters. All 2,644 current
variants for the expanded characters match the inspected standalone outputs.
Previous runtime table rows and controls are unchanged. Prior profile fields
and complete palette sets are identical. Evidence:
`tmp/world-seasonal-standard-comparison.json`,
`tmp/world-seasonal-standard-all-variants.log` and
`tmp/world-seasonal-standard-frozen-inputs.sha256`.

Native probes use the actual 208/195/258-row tables alongside Adeline's 282
rows. All 119 direction/frame cases run in five choices: 595 palette
observations, 155 linear completions and eighty final-frame hold checks.
The original random function remains in use; holds persist below the minimum
and complete beyond the maximum. Paused/fractional state, seating, West
reversal, portrait synchronization, wrapper identity and independent selection
pass. Evidence: `tmp/world-seasonal-standard-{hayden,ryis,celine}-runtime.log`
and corresponding `-runtime-inputs.json` files.

These probes run the original handler and NPC `animate` method with simulated
engine services. Factory direction fallbacks, seating and hold fields are
reproduced from source; packs and loop requests are selected directly. They do
not exercise the full NPC factory/state machine, natural schedules,
speaking/background interruptions, automatic outfit changes, separately drawn
props, audible sound or live timing.

A fresh isolated MOMI install passes pixel, metadata and generated-table
validation in `tmp/world-seasonal-standard-playtest`. Installed archive SHA-256:
`1b70bdbbedf3d20f6187c30f653680586fb83d41f992f29f138ee213ceccf595`.
The source archive and retained `previous.zip` match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/world-seasonal-standard-install-report.json`,
`tmp/world-seasonal-standard-install.log` and
`tmp/world-seasonal-standard-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. The desktop launcher is unchanged. Artwork, previews,
packages and isolated installs remain ignored.

## Next coverage

Finish Hayden's eight Summer special strips and Celine's ten remaining normal
Summer specials. Celine's fourteen Summer garden strips remain a separate pass.
Ryis can continue with Autumn general actions, sleep and kiss.
