# Beach blinks, general actions and kisses

The user approved the [Beach idle/walk pilot](world-beach-pilot.md), committed
as `de86042`. This slice adds blink East/South, general action North/South/East
and kiss East for Hayden, Ryis and Celine. Each adds six strips, 31 source
frames and fourteen native West mirrors: eighteen strips and 135 review cases
together. The user approved this artwork for commit on 2026-09-25.

| Character | Total sources | Beach coverage | Art notes |
| --- | ---: | ---: | --- |
| Hayden | 288 | 12/14 | [Hayden](hayden-beach-actions.md) |
| Ryis | 256 | 12/14 | [Ryis](ryis-beach-actions.md) |
| Celine | 373 | 12/14 | [Celine](celine-beach-actions.md) |

The combined 36-character trial has 2,849 sources and 11,396 variants. Their
complete Spring, Summer, Autumn and Winter sprite coverage remains intact.
F8, F10 and Insert retain five choices and Vanilla defaults. No runtime Rust
or GML behavior changed. The character notes record skin/material boundaries
and focused checks for each recipe.

## Source and metadata

The mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding slice. New source pins are strict; no old hash was
refreshed and generation does not use `--allow-source-hash-mismatch`.

Independent archive metadata and all three author exports agree by asset
path. All eighteen strips retain 80×80 frames, Default atlas and Middle/54
origin. Blinks have three frames with durations `[0.075,0.125,0.075]`; general
actions have seven with `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kisses have four
with `[0.15,0.15,0.8,0.15]`. The native cycles support Beach; general actions
also carry a `[240,360]` last-frame hold and `on_pause_speaking: "idle"`.

The synthetic package test first failed on unregistered Hayden Beach action
East, then passed with all eighteen paths registered. It checks complete
animation properties, frame counts, PNG outputs and existing character
bindings. Evidence: `tmp/world-beach-actions-package-{red,checks,final}.log`,
`tmp/world-beach-actions-{inventory,metadata-check}.log` and
`tmp/world-beach-actions-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/world-beach-actions-preview/summary.png):
  South-facing general-action frame two for each character.
- [Complete Vanilla/Debug Blue review](../../generated/world-beach-actions-preview/index.html):
  all 93 source frames and 42 native West mirrors, paginated by character,
  with separate five-choice summaries.

Static previews do not establish natural NPC scheduling, automatic outfit
changes or live rendering. Game-derived files, packages, previews and temporary
helpers remain ignored.

The combined summary passes fifteen source/palette bindings and 128,960 exact
displayed-pixel comparisons, including raw archive metadata and complete opaque
artwork crops. The landing loads all local links and images in Chromium without
horizontal overflow. Evidence: `tmp/world-beach-actions-summary-check.log` and
`tmp/world-beach-actions-landing-browser.log`. Character records cover their full
galleries, West mirrors and separate summaries. The landing and all three preview
directories together occupy about 2.3 MiB.

## Verification

Formatting, Clippy with warnings denied, all 121 active tests and the release
build passed. The normal suite leaves 121 local opt-in tests ignored; nine ran
separately and passed: three new character corpus tests, three retained world
corpus tests and three GML runtime tests. Material controls exercise omitted
skin and spills into the straw hat, hair and sandals. Celine's final recipe
matches her passing candidate exactly. Older opt-in tests and live gameplay
were not rerun. Shared evidence:
`tmp/world-beach-actions-{final-checks,retained-world,gml-checks}.log`;
character-specific test and control logs are linked in their art notes.

All 11,396 combined variants passed exact recipe validation. All 8,990 accepted
Hayden/Ryis/Celine PNG and metadata files and all 19,485 files belonging to the
other 33 characters remain byte-identical. The 3,668 standalone variants match
the combined output. Existing runtime rows, controls, 899 old region objects
and color maps are unchanged. The 62 retained corpus-test edits only update
paths and totals. Evidence:
`tmp/world-beach-actions-{comparison.json,input-audit.log}`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, probes
passed 675 frame/direction observations, 135 linear completions and 120 final-frame
hold checks across all five choices. They check West mirroring, palette wrapper
idempotence, frame phase and cycle-counter preservation. These probes do not
establish natural scheduling, speaking-state transitions, automatic outfit
dispatch or full-engine behavior. Evidence:
`tmp/world-beach-actions-{native-source,runtime}.log`.

MOMI installation passed in the fresh isolated
`tmp/world-beach-actions-playtest` lab, including required compilation and
installed-pixel/animation-metadata verification. Installed archive SHA-256:
`162819de7d83c8362c743e249dafd8ad4e48eaf8d1e7311f0d3a27a696c77e28`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-beach-actions-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Finish Beach with the two bathing/swimming strips per character. The current
folder inventory is `tmp/world-beach-actions-remaining.json`. Wedding and
Hayden's Shadow sprites remain outside this slice.
