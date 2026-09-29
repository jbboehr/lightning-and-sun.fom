# Reina and Juniper: completing Winter sprites

The user approved the Winter writing, laugh and smithing batch, committed as
`653807a`. This slice adds Reina's chopping and three polishing phases, plus
Juniper's spell casting, hair flip and snooze. These nine strips complete both
Winter sprite folders. The user approved the artwork for commit on 2026-09-28.

| Character | New strips | Source frames | West mirrors | Total sources | Winter coverage | Art notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Reina | 4 | 29 | 9 | 246 | 35/35 | [Reina](reina-winter-finish.md) |
| Juniper | 5 | 22 | 0 | 272 | 33/33 | [Juniper](juniper-winter-finish.md) |

The combined 36-character trial has 3,357 sources and 13,428 variants. Home
and Page Down retain the five palette choices and portrait synchronization.
March's coverage is unchanged; his fourteen Winter injured strips remain.
No runtime Rust or GML code changed.

## Sources and materials

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Pins remain strict; no mismatch override or earlier pin refresh was used.
All nine strips retain Default atlas, 80×80 frames and numeric origins
40.0/54.0. Raw sidecars preserve all mixed duration arrays and absent timing
fields on the single-frame spell-casting end.

Chopping faces North and has twenty frames. Polishing is complex with
start/loop/end counts of 3/4/2; its East pack also renders West with the native
flip. Juniper's sources face South: six hair-flip frames, nine snooze frames
and 2/4/1 frames for complex spell casting. Chopping and spell casting retain
their native sound configuration, including detached spell sound. All nine
author sidecars match independent archive reads. Evidence:
`tmp/world-winter-completion-native-inputs.json`, the raw metadata exports and
`tmp/world-winter-completion-{metadata,timing}-check.log`.

Both authors inspected every actual frame across four target palettes. Reina's
Winter scarf covers the nape in four chopping frames, which have no visible
skin and stay unchanged. Juniper's tiny snooze ear and spell-loop forehead
follow the selected palette, while gloves, cuffs, circlet and cosmetics keep
their original colors. There are no new source colors or material exceptions.
See the character notes for literal landmarks and omission/spill controls.

The synthetic package test first rejected unregistered Reina chopping, then
passed after all nine exact paths were registered. It checks complete animation
metadata, frame totals, generated PNGs and existing controls. Evidence:
`tmp/world-winter-completion-{red,package-checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-winter-finish-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-winter-finish-preview/index.html):
  all 51 source frames and nine native West polishing mirrors.

Static previews cover recoloring; they do not establish natural interactions,
schedules, outfit changes, pauses, sound, attached effects or full-engine
rendering. Game-derived images, extracted files and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 181 active tests and the release
build passed. The normal suite leaves 203 local opt-in tests ignored; 51 ran
separately and passed: two new material tests, 46 retained character corpus
tests and three GML tests. The new material tests also passed their missed-skin
and material-spill controls. Other opt-ins and live gameplay were not rerun.
Evidence: `tmp/world-winter-completion-final-checks.log`,
`tmp/world-winter-completion-*-retained-*.log`, `tmp/world-winter-completion-gml.log`
and the individual material notes.

Every combined variant passes exact recipe validation: 13,428 across 36 characters.
All 5,090 earlier original/variant PNG and metadata files for these two
characters remain byte-identical, as do all 28,560 files for the other 34.
The 2,072 standalone variants match the combined outputs. Previous runtime rows
and controls remain exact. Evidence: `tmp/world-winter-completion-comparison.{json,log}`.

The definition audit preserves all 509 earlier region objects, every source
color, color group and target mapping. Sets, portrait-only definitions and the
collection remain unchanged. Exactly nine profile/registry paths are added;
Reina's stylized description now includes complete Winter coverage. All 36 export reports
retain earlier source entries and the current archive hash. Retained tests
change only corpus paths and total counts; previous material assertions remain
intact. Evidence: `tmp/world-winter-completion-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, native
Single packs pass 300 frame/palette observations, 60 complex phase transitions
and 15 linear completions
across all five choices. They check palette binding, wrapper idempotence,
animation phase, cycle counters, West mirroring, portrait synchronization and the complete
start-to-loop-to-end progression, including repeated loops and completion.
Natural interactions, seating, scheduling, pause policy, outfit changes, sound
and full-engine rendering remain unverified.
Evidence: `tmp/world-winter-completion-runtime.log`.

The combined summary checks ten source/palette bindings and 103,200 exact
pixels, raw metadata and complete artwork crops. The two galleries verify
every source frame and all nine native West polishing mirrors with exact
rendered-pixel, metadata and browser checks;
their records are in the individual material notes. Chromium also passes the
combined landing page, summary and local links. The three preview directories
total about 0.97 MiB.
Evidence: `tmp/world-winter-completion-summary-check.log`,
`tmp/world-winter-completion-preview-browser-check.json`,
`tmp/world-winter-completion-preview-sizes.json` and
`tmp/world-winter-completion-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-winter-finish-653807a-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`50a8634ffc97cac747d162e545b9b8d00da33df25b38614419a211d5a6e72138`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eight frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-winter-completion-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

March has fourteen remaining Winter injured strips. Cover those next. Reina and
Juniper have no remaining Winter PNGs in the current archive; the complete
inventory is `tmp/world-winter-completion-remaining.json`.
