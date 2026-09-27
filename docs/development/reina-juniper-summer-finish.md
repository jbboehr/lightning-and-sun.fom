# Reina and Juniper: completing Summer sprites

The user approved the Summer writing, laugh and smithing batch, committed as
`af76d65`. This slice adds Reina's chopping and three polishing phases, plus
Juniper's spell casting, hair flip and snooze. These nine strips complete both
Summer sprite folders. The user approved this artwork for commit on
2026-09-27, including Juniper's thirteen cuff-edge pixel occurrences.

| Character | New strips | Source frames | West mirrors | Total sources | Summer coverage | Art notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Reina | 4 | 29 | 9 | 176 | 35/35 | [Reina](reina-summer-finish.md) |
| Juniper | 5 | 22 | 0 | 206 | 33/33 | [Juniper](juniper-summer-finish.md) |

The combined 36-character trial has 3,130 sources and 12,520 variants. Home
and Page Down retain the same five choices, starting with Vanilla. Portraits
and supported sprites share each character's palette selector. Later overworld
outfits retain their original sprites. March's coverage is unchanged; his fourteen
Summer injured strips remain. No runtime Rust or GML code changed.

## Source and native metadata

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or pin refresh was used.
All nine strips retain Default atlas, 80×80 frames and numeric offsets
40.0/54.0. Raw sidecars preserve all mixed duration arrays and absent timing
fields on the single-frame spell-casting end.

Chopping faces North and has twenty frames. Polishing is complex with
start/loop/end counts of 3/4/2; its East pack also renders West with the native
flip. Juniper's sources face South: six hair-flip frames, nine snooze frames
and 2/4/1 frames for complex spell casting. Chopping and spell casting retain
their native sound configuration, including detached spell sound. Evidence:
`tmp/world-summer-finish-native-inputs.json` and the two raw metadata exports.

The synthetic package test first rejected unregistered Reina chopping, then
passed after all nine exact paths were registered. It checks complete animation
properties, frame totals, PNG outputs and existing controls. Evidence:
`tmp/world-summer-finish-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-summer-finish-preview/summary.png):
  samples of polishing and spell casting.
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-summer-finish-preview/index.html):
  all 51 source frames and nine native West polishing mirrors.

Juniper has thirteen new cuff-edge pixel occurrences joined to nearby hands
or forearms under the current component-mask rules. They recolor to keep skin
coverage complete, following the earlier compromise; 127 separable warm
jewelry pixels remain protected. Her material notes and gallery identify the
affected spell, hair-flip and snooze frames. Earlier approved masks remain
unchanged.

Static previews cover recoloring; they do not establish natural interactions,
scheduling, outfit changes, pauses, sound, attached effects or full-engine
rendering. Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 151 active tests and the release
build passed. The normal suite leaves 166 local opt-in tests ignored; 27 ran
separately and passed: two new material tests, 22 retained character corpus
tests and three GML tests. The new material tests also passed their missed-skin
and material-spill controls. Older opt-ins and live gameplay were not rerun.
Evidence: `tmp/world-summer-finish-final-checks.log`,
`tmp/world-summer-finish-*-retained-*.log`, `tmp/world-summer-finish-gml.log`
and the individual material notes.

Every combined variant passes exact recipe validation: 12,520 across 36 characters.
All 3,730 earlier original/variant PNG and metadata files for these two
characters remain byte-identical, as do all 27,650 files for the other 34.
The 1,528 standalone variants match the combined outputs. Previous runtime rows
and controls remain exact. Evidence: `tmp/world-summer-finish-comparison.{json,log}`.

The definition audit preserves all 373 earlier region objects, every source
color, color group and target mapping. Sets, portrait-only definitions and the
collection remain unchanged. Exactly nine profile/registry paths are added;
Reina's stylized description now includes complete Summer coverage. All 36 export reports
retain earlier source entries and the current archive hash. Retained tests
change only corpus paths and total counts; previous material assertions remain
intact. Evidence: `tmp/world-summer-finish-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, native
Single packs pass 300 frame/palette observations, 60 complex phase transitions
and 15 linear completions
across all five choices. They check palette binding, wrapper idempotence,
animation phase, cycle counters, West mirroring, portrait synchronization and the complete
start-to-loop-to-end progression, including repeated loops and completion.
Natural interactions, seating, scheduling, pause policy, outfit changes, sound
and full-engine rendering remain unverified.
Evidence: `tmp/world-summer-finish-runtime.log`.

The combined summary checks ten source/palette bindings and 103,200 exact
pixels, raw metadata and complete artwork crops. The two galleries verify
every source frame and all nine native West polishing mirrors with exact
rendered-pixel, metadata and browser checks;
their records are in the individual material notes. Chromium also passes the
combined landing page, summary and local links. The three preview directories
total about 0.97 MiB.
Evidence: `tmp/world-summer-finish-summary-check.log`,
`tmp/world-summer-finish-preview-browser-check.json`,
`tmp/world-summer-finish-preview-sizes.json` and
`tmp/world-summer-finish-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-summer-finish-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`278909aa81f38a98acf192514a209b61d9c38111f5a30572a228fb23dbeeb2df`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eight frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-summer-finish-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Add March's fourteen Summer injured strips, completing his Summer folder.
Reina and Juniper have no remaining Summer PNGs in the current archive;
the complete inventory is `tmp/world-summer-finish-remaining.json`.
