# Autumn general actions and seasonal seated poses

The user accepted the [previous Autumn actions and Winter pilot](world-autumn-actions.md),
committed as `81c1621`. This slice adds 27 strips, 88 source frames and 36 native
West mirrors. The user approved this slice for commit on 2026-09-25.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Autumn general actions, sleep and kiss | 5 | 26 | 12 | 238 |
| Ryis | Winter blink, sit, eat and drink | 11 | 31 | 12 | 228 |
| Celine | Regular Autumn blink, sit, eat and drink | 11 | 31 | 12 | 299 |

Hayden now covers twenty-two of thirty Autumn strips; Ryis covers seventeen of
thirty-three Winter strips. Celine covers seventeen of forty-six Autumn strips,
with fourteen garden strips still outside this batch. All three retain complete
Spring and Summer coverage, and Ryis retains complete Autumn coverage.
The folder inventory is `tmp/world-autumn-standard-remaining.json`.

F8, F10 and Insert retain their five choices and Vanilla defaults. Runtime
Rust/GML code is unchanged. Character material decisions and focused checks are
recorded in [Hayden](hayden-autumn-standard.md), [Ryis](ryis-autumn-standard.md)
and [Celine](celine-autumn-standard.md).

## Offline review

- [Compact five-choice summary](../../generated/world-autumn-standard-preview/summary.png):
  Hayden's South general action frame four and Ryis/Celine drinking South frame two.
- [Complete Vanilla/Debug Blue review](../../generated/world-autumn-standard-preview/index.html):
  all 124 new direction/frame cases, with paginated character reviews and individual
  five-choice summaries.

Static review covers recoloring and material boundaries. It does not establish
natural scheduling, automatic outfit changes, separately drawn props or live
rendering. Extracted artwork, previews, packages and isolated installs remain ignored.

## Metadata and packaging

The package regression failed on the unregistered Hayden Autumn general-action
path, then passed after registering the twenty-seven exact paths. It checks PNGs,
controls and complete animation properties. All strips retain 80×80 frames,
Default atlas and Middle/54 origin. Independent archive metadata, author exports
and native configuration agree. Evidence:
`tmp/world-autumn-standard-package-{red,checks}.log` and
`tmp/world-autumn-standard-inventory.log`.

Hayden's general action has seven frames at
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`; kiss has four at
`[0.15, 0.15, 0.8, 0.15]`; sleep retains its single-frame defaults.
Ryis and Celine retain three-frame blinking at `[0.075, 0.125, 0.075]`,
three-frame drinking and North eating at `1.0`, and five-frame South/East
eating at `[0.125, 0.15, 0.175, 0.125, 0.6]`. Sitting retains single-frame
defaults. Hayden's general action and Ryis/Celine's eating and drinking preserve
native random final-frame holds of `[240, 360]`. Ryis/Celine's sitting, eating
and drinking remain seated.

## Verification

The integration and art checks below were completed on 2026-09-19. Temporary
logs and installation labs are disposable; the current packages, complete
offline reviews and regression baselines were retained during the 2026-09-25
generated-output cleanup. That cleanup preserved retained-file hashes and passed
the Hayden riding test against its trimmed historical baseline. Formatting,
Clippy, all 106 active tests and the release build were rerun successfully before
commit on 2026-09-25 (`tmp/autumn-standard-commit-checks.log`).

Formatting, Clippy, all 106 active tests and the release build pass through
`nix-shell --pure`. The normal suite reports 101 opt-in tests ignored. Separately,
the three new character corpus tests, three retained `*_world` corpus tests and
three GML runtime tests pass. Other retained opt-in corpora were not rerun;
their assertions are preserved and every previous output was compared directly.
Evidence: `tmp/world-autumn-standard-final-checks.log`,
`tmp/world-autumn-standard-retained-{hayden,ryis,celine}-world.log`,
`tmp/world-autumn-standard-gml-checks.log` and the character test logs.

All 10,788 variants validate across 2,697 sources and 36 characters. The 7,380
previous original/variant PNG and metadata files for these three characters are
byte-identical; all 19,485 files for the other thirty-three characters are
unchanged. The combined package matches all 3,060 inspected standalone variants.
Earlier runtime rows, controls, full region objects and palette sets are unchanged.
The 42 retained test edits only update fixture paths, ignore descriptions and
three total-source assertions. Evidence:
`tmp/world-autumn-standard-{comparison.json,input-audit.log,all-variants.log}`.

Probes using the real native animator, packs and NPC animate method pass all
124 direction/frame cases across five choices: 620 palette observations,
190 linear completions and 100 final-frame hold checks. Frame state, West
reversal, portrait synchronization and independent character selection are
preserved. Native random holds are exercised below their minimum and beyond
their maximum. Engine services are simulated, and collections and loop limits
are selected directly. Natural schedules, the full NPC state machine, automatic
outfit dispatch, audible sound and live gameplay remain untested. Evidence:
`tmp/world-autumn-standard-{hayden,ryis,celine}-runtime.{log,gml}` and matching
`runtime-inputs.json` records.

The compact summary is 56,579 bytes. Its fifteen samples pass source/palette
bindings, metadata, source hashes, crop and 341,280 exact-pixel checks. Character
reviews include all new source frames and West mirrors. Chromium verifies their
pages, images and navigation; the combined landing page also passes all six
character links, image decoding and overflow checks. Evidence:
`tmp/world-autumn-standard-summary-check.log`,
`tmp/world-autumn-standard-landing-browser.log` and the character preview logs.

A fresh isolated MOMI install succeeds in `tmp/world-autumn-standard-playtest`,
including installed-pixel and metadata verification. Installed archive SHA-256:
`db25a6d6f336f86071086f47e48a21ccf171ed94b78dc0582a58023be14f80a0`.
The source archive and preserved `previous.zip` both retain SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The live game and desktop launcher were not changed. Eleven frozen input hashes
remain unchanged after building and installation. Evidence:
`tmp/world-autumn-standard-install-report.json`,
`tmp/world-autumn-standard-source-after.sha256` and
`tmp/world-autumn-standard-frozen-inputs.sha256`.

## Next coverage

Finish Hayden's eight Autumn special strips: reading while seated, hammering,
watering, harvesting, tilling and wiping his brow. Add Celine's regular Autumn
and Ryis's Winter general actions, sleep and kiss, then their remaining specials.
Celine's Autumn garden outfit remains a separate batch.
