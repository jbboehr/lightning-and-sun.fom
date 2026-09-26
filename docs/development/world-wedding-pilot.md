# Wedding idle and walk pilot

The user approved the [Beach swimming batch](world-beach-swim.md), committed
as `22dc645`. This slice starts Wedding sprite coverage for Hayden, Ryis and
Celine with idle/walk North, South and East. Each adds six strips, fifteen
source frames and five native West mirrors: eighteen strips and sixty offline
review cases together. The user approved this artwork for commit on 2026-09-25.

| Character | Total sources | Wedding coverage | Art notes |
| --- | ---: | ---: | --- |
| Hayden | 296 | 6/15 | [Hayden](hayden-wedding-pilot.md) |
| Ryis | 264 | 6/15 | [Ryis](ryis-wedding-pilot.md) |
| Celine | 381 | 6/15 | [Celine](celine-wedding-pilot.md) |

The combined 36-character trial has 2,873 sources and 11,492 variants. These
three retain complete Spring, Summer, Autumn, Winter and Beach sprite coverage.
F8, F10 and Insert retain five choices and Vanilla defaults. No runtime Rust or
GML behavior changed. Character notes record the Wedding skin/material boundaries.

## Source and metadata

The mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding slice. Source pins remain strict; no previous hash
was refreshed and generation does not use `--allow-source-hash-mismatch`.

Independent archive metadata and all three author exports agree by asset path
(`tmp/world-wedding-pilot-metadata-check.log`). All eighteen strips retain
80×80 frames, Default atlas and Middle/54 origin.
Idle uses single-frame defaults; walking has four frames at 0.15 seconds each.
Both native cycles support the Wedding outfit in North, South and East, with
South as their default direction. Walking retains the native idle fallback for
background and speaking pauses. Native turning during speaking also remains
unchanged.

The synthetic package test first failed on unregistered Hayden Wedding idle
East, then passed with all eighteen paths registered. It checks complete
animation properties, frame counts, PNG outputs and existing character bindings.
Evidence: `tmp/world-wedding-pilot-package-{red,checks}.log`,
`tmp/world-wedding-pilot-inventory.log` and
`tmp/world-wedding-pilot-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/world-wedding-pilot-preview/summary.png):
  South-facing walking frame two for each character.
- [Complete Vanilla/Debug Blue review](../../generated/world-wedding-pilot-preview/index.html):
  all 45 source frames and fifteen native West mirrors, grouped by character,
  with separate five-choice summaries.

Static previews do not establish Wedding-event behavior, natural NPC scheduling,
automatic outfit changes or live rendering. Source art, generated variants,
previews, packages and temporary helpers remain ignored.

The combined summary passes fifteen source/palette bindings and 143,840 exact
displayed-pixel checks, including raw archive metadata and complete opaque
artwork bounds. Each character gallery passes its exact-pixel, crop, metadata,
West-reversal and Chromium checks. All local links and images also load on the
combined landing without horizontal overflow. The four preview directories
occupy about 1.25 MiB. Evidence:
`tmp/world-wedding-pilot-{summary-check,landing-browser}.log`, plus the character
records.

## Verification

Formatting, Clippy with warnings denied, all 123 active tests and the release
build passed. The normal suite leaves 127 local opt-in tests ignored; nine ran
separately and passed: three new character corpus tests, three retained world
corpus tests and three GML runtime tests. Material controls exercise missing
skin and spills into shoes, hair and neckwear. Final Hayden and Celine recipes
match their passing candidates exactly. Older opt-in tests and live gameplay
were not rerun. Shared evidence:
`tmp/world-wedding-pilot-{final-checks,retained-world,gml-checks}.log`; the
character notes link their focused tests and controls.

All 11,492 combined variants passed exact recipe validation. All 9,230 accepted
Hayden/Ryis/Celine PNG and metadata files and all 19,485 files belonging to the
other 33 characters remain byte-identical. All 3,764 standalone variants match
the combined output. Existing runtime rows, controls, 923 old region objects
and color maps are unchanged. The 68 retained corpus-test edits only update
fixture paths and totals. Evidence:
`tmp/world-wedding-pilot-{comparison.json,input-audit.log}`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, probes
passed 300 frame/direction observations and 120 linear completions across all
five choices. They check West mirroring, palette wrapper idempotence, frame phase
and cycle-counter preservation. These probes do not establish natural scheduling,
Wedding-event transitions, automatic outfit dispatch or full-engine rendering.
Evidence: `tmp/world-wedding-pilot-{native-source,runtime}.log`.

MOMI installation passed in the fresh isolated
`tmp/world-wedding-pilot-playtest` lab, including required compilation and
installed-pixel/animation-metadata verification. Installed archive SHA-256:
`25f6c3f901eec60d55fccc72f6123c97d8527894eeae2711e7a32e04812d162f`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-wedding-pilot-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Each Wedding folder has nine strips left: blink East/South, sit and general
action North/South/East, and kiss East. Inventory:
`tmp/world-wedding-pilot-remaining.json`. Hayden's 29 Shadow strips remain
outside this slice.
