# Autumn actions and Winter pilot

The user accepted the [previous seasonal expansion](world-seasonal-expansion.md),
committed as `3e7b949`. This slice adds 23 strips, 61 source frames and 22 native
West mirrors. The user accepted the complete offline artwork on 2026-09-19.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Autumn blink, sit, eat and drink | 11 | 31 | 12 | 233 |
| Ryis | Winter idle and walk North/South/East | 6 | 15 | 5 | 217 |
| Celine | Regular Autumn idle and walk North/South/East | 6 | 15 | 5 | 288 |

Hayden covers seventeen of thirty Autumn strips. Celine covers six of forty-six
Autumn strips, including fourteen separate garden strips in that folder total.
Ryis covers six of thirty-three Winter strips. All three retain complete Spring
and Summer coverage, and Ryis retains complete Autumn coverage. The archive-folder
inventory is recorded in `tmp/world-autumn-actions-remaining.json`.

F8, F10 and Insert retain the same five choices and Vanilla defaults. Runtime
Rust/GML code is unchanged. Character notes describe material boundaries and
every-frame review: [Hayden](hayden-autumn-actions.md), [Ryis](ryis-autumn-actions.md)
and [Celine](celine-autumn-actions.md).

## Offline review

- [Compact five-choice summary](../../generated/world-autumn-actions-preview/summary.png):
  Hayden drinking South frame two, Ryis walking South frame two in Winter, and
  Celine walking South frame two in Autumn.
- [Complete Vanilla/Debug Blue review](../../generated/world-autumn-actions-preview/index.html):
  all 83 direction/frame cases, with paginated character reviews and individual
  five-choice summaries.

Static previews show recoloring and material boundaries. They do not establish
natural scheduling, automatic outfit changes, separately drawn props or live
rendering. Extracted artwork, previews, packages and isolated installs remain
ignored.

## Metadata and packaging

The portable package regression failed on an unregistered Autumn blink path,
then passed with the twenty-three exact registry additions. It checks PNGs,
controls and complete animation properties. All frames retain 80×80 geometry,
Default atlas and Middle/54 origin. Evidence:
`tmp/world-autumn-actions-package-{red,checks}.log`.

Single-frame idles and sitting retain their timing defaults. Walk retains four
frames at `0.15`. Hayden's blink retains `[0.075, 0.125, 0.075]`; drink and North
eating retain three frames at `1.0`; South/East eating retain five frames with
`[0.125, 0.15, 0.175, 0.125, 0.6]`. Eating and drinking retain the native random
final-frame holds of `[240, 360]`. Sitting, eating and drinking remain seated.

## Verification

Formatting, Clippy, all 105 active tests and the release build pass through
`nix-shell --pure`. The normal suite reports 98 opt-in tests ignored. Separately,
the three new character corpus tests, three retained `*_world` corpus tests and
three GML runtime tests pass. The other retained opt-in corpora were not rerun:
their assertions are unchanged, and every prior output was compared directly.
Evidence: `tmp/world-autumn-actions-final-checks.log`,
`tmp/world-autumn-actions-smoke-gml.log` and the character-specific test logs.

The combined package contains 2,670 sources and 10,680 validated variants across
36 characters. All 7,150 prior original/variant PNG and metadata files for these
three characters remain byte-identical; all 19,485 files for the other thirty-three
characters are unchanged. The combined outputs match all 2,952 inspected
standalone variants. Earlier runtime rows, controls, full region objects and
palette sets are unchanged. The 39 retained test edits only update fixture paths,
ignore descriptions and three total-source assertions. Evidence:
`tmp/world-autumn-actions-{comparison.json,input-audit.log,all-variants.log}`.

Independent archive and native-configuration checks agree with all three author
inventories. Probes using the real native animator, packs and NPC animate method
pass all 83 frame/direction cases across five choices: 415 palette observations,
155 linear completions and 40 final-frame hold checks. They preserve frame state,
West reversal, portrait synchronization and independent character selection.
Native random holds are exercised below their minimum and beyond their maximum.
Engine services are simulated, and collections and loop limits are selected
directly; natural schedules, the full NPC state machine, automatic outfit
dispatch, audible sound and live gameplay remain untested. Evidence:
`tmp/world-autumn-actions-{hayden,ryis,celine}-runtime.{log,gml}` and the matching
`runtime-inputs.json` records.

The compact summary is 57,906 bytes. Its fifteen samples pass source/palette
bindings, native metadata, source hashes, crop and 398,880 exact-pixel checks.
Each character's full review covers every new source frame and West mirror;
Chromium decodes all images and verifies page links. The combined landing page
also passes its six character links, image decoding and overflow checks.
Evidence: `tmp/world-autumn-actions-summary-check.log` and
`tmp/world-autumn-actions-landing-browser.log`.

A fresh isolated MOMI install succeeds in `tmp/world-autumn-actions-playtest`,
including installed-pixel and metadata verification. Installed archive SHA-256:
`de56c96f85732ff2094cee8eb1477a7dc6d5eabdbd5d6ce9310e92ee51c0ee60`.
The source archive and preserved `previous.zip` both retain SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The live game and desktop launcher were not changed. Eleven frozen input hashes
remain unchanged after building and installation. Evidence:
`tmp/world-autumn-actions-install-report.json`,
`tmp/world-autumn-actions-source-after.sha256` and
`tmp/world-autumn-actions-frozen-inputs.sha256`.

## Next coverage

Continue Hayden's Autumn general actions, sleep and kiss. Expand Celine's Autumn
and Ryis's Winter blinking, sitting, eating and drinking before their later
standard and special animations.
