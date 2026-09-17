# Summer actions for Hayden and Celine, and Ryis's Autumn pilot

The user accepted the [previous Summer batch](world-summer-expansion.md),
committed as `7b1331d`. This slice adds 28 strips, 77 source frames and 29 native
West mirrors. The user accepted this offline artwork on 2026-09-16.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Summer blink, sit, eat and drink | 11 | 31 | 12 | 203 |
| Ryis | Autumn idle and walk | 6 | 15 | 5 | 184 |
| Celine | Regular Summer blink, sit, eat and drink | 11 | 31 | 12 | 253 |

Hayden now covers seventeen of thirty Summer strips; Celine covers seventeen
of forty-six, including her separate garden outfit in that folder total. Ryis
covers six of thirty-three Autumn strips. Their accepted Spring and Summer
coverage remains intact. The archive-folder inventory is recorded in
`tmp/world-seasonal-actions-remaining.json`.

F8, F10 and Insert retain the same five choices and Vanilla defaults. Runtime
Rust/GML code is unchanged. Character notes describe material boundaries and
every-frame review: [Hayden](hayden-seasonal-actions.md),
[Ryis](ryis-seasonal-actions.md) and [Celine](celine-seasonal-actions.md).

## Offline review

- [Compact five-choice summary](../../generated/world-seasonal-actions-preview/summary.png):
  Hayden eating South frame four, Ryis walking South frame two and Celine
  drinking South frame two.
- [Complete Vanilla/Debug Blue review](../../generated/world-seasonal-actions-preview/index.html):
  all 106 direction/frame cases, with paginated character reviews and individual
  five-choice summaries.

The complete reviews include every new frame and native West mirror. Static
images do not establish natural scheduling, automatic outfit changes, separate
prop drawing or live rendering.

All new source frames were inspected in all four target palettes. Material
checks retain Hayden's hat, shirt and beard, Ryis's gloves, coat and scarf, and
Celine's hair, dress and sandal straps. The complete reviews verify exact
pixels, source/metadata bindings, full visible extents and West reversal.
Chromium checks every page, image and link. The 1020×850 combined summary is
about 59 KB; all fifteen sample bindings and 376,560 display pixels match the
inspected package. Evidence is in the character notes,
`tmp/world-seasonal-actions-summary-check.log` and
`tmp/world-seasonal-actions-landing-browser.log`.

## Metadata and packaging

The portable package regression failed on an unregistered Summer blink path,
then passed after the twenty-eight exact registry additions. It verifies PNGs,
controls and complete animation properties. An independent inventory matches
pristine archive sidecars, local geometry, author metadata and native cycle
definitions. All frames retain 80×80 geometry, the Default atlas and Middle/54
origin. Evidence: `tmp/world-seasonal-actions-package-{red,checks}.log` and
`tmp/world-seasonal-actions-inventory.log`.

Blink retains three frames with `[0.075, 0.125, 0.075]`. Drink and North eating
retain three frames at `1.0`; South/East eating retain five with
`[0.125, 0.15, 0.175, 0.125, 0.6]`. Sitting and idle retain omitted single-frame
defaults; Ryis's walk retains four frames at `0.15`. Native eating and drinking
cycles retain seated state and the random final-frame hold of `[240, 360]`.

## Verification

Formatting, Clippy, 101 active tests and the release build pass. The three new
character corpus tests, twenty-seven retained corpus tests and three GML tests
pass. Retained tests use the expanded extractions; only their corpus paths,
ignore-count descriptions and three overall source-count assertions changed.
Existing mask and color assertions are preserved. Evidence:
`tmp/world-seasonal-actions-final-checks.log`,
`tmp/world-seasonal-actions-retained-{hayden,ryis,celine}-checks.log`,
`tmp/world-seasonal-actions-gml-checks.log` and the character notes.

The combined `generated/characters-world-seasonal-actions-trial` contains
36 characters, 2,572 sources and 10,288 validated variants. All 6,120 previous
original/variant PNG and metadata files for these three characters remain
identical, as do 19,485 files for the other 33 characters. All 2,560 current
variants for the expanded characters match the inspected standalone outputs.
Previous runtime table rows and controls are unchanged. Prior profile fields
and complete palette sets are identical. Evidence:
`tmp/world-seasonal-actions-comparison.json`,
`tmp/world-seasonal-actions-all-variants.log` and
`tmp/world-seasonal-actions-frozen-inputs.sha256`.

Native probes use the actual 203/184/253-row tables alongside Adeline's 282
rows. All 106 direction/frame cases run in five choices: 530 palette
observations, 190 linear completions and eighty eating/drinking final-frame
hold checks. The original random function remains in use; checks observe
persistence below the minimum hold and completion beyond its maximum.
Paused/fractional state, seating, West reversal, portrait synchronization,
wrapper identity and independent selection pass. Evidence:
`tmp/world-seasonal-actions-{hayden,ryis,celine}-runtime.log` and corresponding
`-runtime-inputs.json` files.

The probes run the original handler and NPC `animate` method with simulated
engine services. Factory direction fallbacks, seating and hold fields are
reproduced from source; animation packs and loop requests are selected directly.
They do not exercise the full NPC factory/state machine, natural schedules,
speaking/background interruptions, automatic outfit changes, separately drawn
food/cup props, audible sound or live timing.

A fresh isolated MOMI install passes pixel, metadata and generated-table
validation in `tmp/world-seasonal-actions-playtest`. Installed archive SHA-256:
`79d4e189cba7c4346912244018fccd4a947c274714b58687cec67da26fc058f7`.
The source archive and retained `previous.zip` match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/world-seasonal-actions-install-report.json`,
`tmp/world-seasonal-actions-install.log` and
`tmp/world-seasonal-actions-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. The desktop launcher is unchanged. Artwork, previews,
packages and isolated installs remain ignored.

## Next coverage

Continue Hayden and Celine's regular Summer general actions, sleep and kiss.
Ryis can expand Autumn with blink, sit, eat and drink using the reviewed pilot.
