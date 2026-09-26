# Wedding sprite completion

The user approved the [Wedding idle/walk pilot](world-wedding-pilot.md), committed
as `81b5097`. This slice adds the nine remaining Wedding strips for each of
Hayden, Ryis and Celine: blink East/South, sit and general action North/South/East,
and kiss East. Each contributes 34 source frames and fifteen native West mirrors:
27 strips and 147 offline review cases together. The user approved this artwork for commit on 2026-09-25.

| Character | Total sources | Wedding coverage | Art notes |
| --- | ---: | ---: | --- |
| Hayden | 305 | 15/15 | [Hayden](hayden-wedding-finish.md) |
| Ryis | 273 | 15/15 | [Ryis](ryis-wedding-finish.md) |
| Celine | 390 | 15/15 | [Celine](celine-wedding-finish.md) |

The combined 36-character trial has 2,900 sources and 11,600 variants. These
three now cover their complete Spring, Summer, Autumn, Winter, Beach and Wedding
sprite folders. F8, F10 and Insert retain five choices and Vanilla defaults.
No runtime Rust or GML behavior changed. Character notes record the reviewed
skin/material boundaries.

## Source and metadata

The mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding slice. Source pins remain strict; no previous hash
was refreshed and generation does not use `--allow-source-hash-mismatch`.

Independent archive metadata matches all three author exports by asset path
(`tmp/world-wedding-finish-metadata-check.log`). Every strip retains 80×80 frames,
Default atlas and Middle/54 origin. Sitting uses single-frame defaults. Blinking
has three frames at `[0.075, 0.125, 0.075]`; general actions have seven frames at
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`; kisses have four frames at
`[0.15, 0.15, 0.8, 0.15]`.

Native Wedding cycles support the expected directions and defaults. General
actions retain the `[240, 360]` last-frame hold and speaking-idle fallback;
sitting retains its seated flag. The synthetic package test first failed on
unregistered Hayden Wedding action East, then passed with all 27 paths
registered. It checks complete animation properties, frame counts, PNG outputs
and existing character bindings. Evidence:
`tmp/world-wedding-finish-package-{red,checks}.log`,
`tmp/world-wedding-finish-inventory.log` and
`tmp/world-wedding-finish-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/world-wedding-finish-preview/summary.png):
  South-facing general action frame three for each character.
- [Complete Vanilla/Debug Blue review](../../generated/world-wedding-finish-preview/index.html):
  all 102 source frames and 45 native West mirrors, grouped by character,
  with separate five-choice summaries.

Static previews do not establish Wedding-event behavior, natural NPC scheduling,
automatic outfit changes or live rendering. Source art, generated variants,
previews, packages and temporary helpers remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 124 active tests and the release
build passed. The normal suite leaves 130 local opt-in tests ignored; nine ran
separately and passed: three new character corpus tests, three retained world
corpus tests and three GML runtime tests. Material controls exercise omitted
skin and spills into shoe soles, gloves and neckwear. Hayden and Celine's final
outputs match their passing candidates exactly. Older opt-in tests and live
gameplay were not rerun. Shared evidence:
`tmp/world-wedding-finish-{final-checks,retained-world,gml-checks}.log`; the
character notes link focused tests and controls.

All 11,600 combined variants passed exact recipe validation. All 9,410 accepted
Hayden/Ryis/Celine PNG and metadata files and all 19,485 files belonging to the
other 33 characters remain byte-identical. All 3,872 standalone variants match
the combined output. Existing runtime rows, controls, 941 old region objects
and color maps are unchanged. The 71 retained corpus-test edits only update
fixture paths and totals. Evidence:
`tmp/world-wedding-finish-{comparison.json,input-audit.log}`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, probes
passed 735 frame/direction observations, 195 linear completions and 120 hold
checks across all five choices. They check West mirroring, palette wrapper
idempotence, frame phase and cycle-counter preservation. These probes do not
establish natural scheduling, Wedding-event transitions, automatic outfit
dispatch or full-engine rendering. Evidence:
`tmp/world-wedding-finish-{package-checks,runtime}.log`.

The combined summary passes fifteen source/palette bindings and 133,920 exact
displayed-pixel checks, including raw archive metadata and complete opaque
artwork bounds. Each character gallery passes its exact-pixel, crop, metadata,
West-reversal and Chromium checks. All local links and images load on the
combined landing without horizontal overflow. The four preview directories
occupy about 2.52 MiB. Evidence:
`tmp/world-wedding-finish-{summary-check,landing-browser}.log`, plus character
records.

MOMI installation passed in the fresh isolated
`tmp/world-wedding-finish-playtest` lab, including required compilation and
installed-pixel/animation-metadata verification. Installed archive SHA-256:
`7db1c7bf69411b54b143102b3a1e7d223d96e10e535433535a60ba73c843928e`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-wedding-finish-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

No Wedding sprite strips remain for these three characters. Hayden's 29 Shadow
strips remain outside this slice. The other portrait characters still need their
own overworld coverage, starting with small idle/walk pilots to establish their
material boundaries. Inventory: `tmp/world-wedding-finish-remaining.json`.
