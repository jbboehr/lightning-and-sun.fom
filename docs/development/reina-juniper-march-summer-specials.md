# Reina, Juniper and March: Summer writing, laugh and smithing

The user approved the Summer seated-reading batch, committed as `7a7ed38`.
This slice adds fourteen strips: Reina's standing and seated writing, Juniper's
laugh, and March's seated work, hammering and brow-wiping. The user approved
this artwork for commit on 2026-09-26, including Juniper's four cuff-edge
pixel occurrences.

| Character | New strips | Source frames | West mirrors | Total sources | Summer coverage | Art notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Reina | 6 | 16 | 0 | 172 | 31/35 | [Reina](reina-summer-specials.md) |
| Juniper | 3 | 5 | 0 | 201 | 28/33 | [Juniper](juniper-summer-specials.md) |
| March | 5 | 23 | 7 | 264 | 30/44 | [March](march-summer-specials.md) |

The combined 36-character trial has 3,121 sources and 12,484 variants. Home,
Page Down and U retain the same five choices, starting with Vanilla. Portraits
and supported sprites share each character's palette selector. Other Summer
actions and later outfits remain original. No runtime Rust or GML code changed.

## Source and native metadata

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or pin refresh was used.
All fourteen strips retain Default atlas, 80×80 frames and vertical offset 54.
Standing writing, laugh, hammer and brow-wiping use numeric horizontal offset
40.0; seated writing and work use Middle. Raw sidecars preserve mixed duration
arrays and absent timing fields on single-frame strips.

Writing and laugh face South. March's seated work faces North, hammer faces
East with native West mirroring, and brow-wiping faces South. The four complex
cycles retain start/loop/end phases and seated flags. Hammer retains its native
sound configuration. Evidence: `tmp/world-summer-specials-native-inputs.json`
and the three `tmp/world-summer-specials-*-raw.json` exports.

The synthetic package test first rejected unregistered Reina writing end,
then passed after all fourteen exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and existing controls.
Evidence: `tmp/world-summer-specials-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-summer-specials-preview/summary.png):
  samples of writing, laughter and brow-wiping.
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-summer-specials-preview/index.html):
  all 44 source frames and seven native West hammer mirrors.

Juniper has four new cuff-edge pixel occurrences inseparable from nearby skin
under the current component-mask rules. They recolor to keep skin coverage
complete, following the earlier compromise. Separable borders remain original.
Her material notes and gallery identify the affected frames; literal assertions
cover the exceptions and protected borders. Earlier approved masks remain
unchanged.

Static previews cover recoloring; they do not establish natural interactions,
seating, scheduling, outfit changes, pauses, sound or full-engine rendering.
Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 150 active tests and the release
build passed. The normal suite leaves 164 local opt-in tests ignored; 37 ran
separately and passed: three new material tests, 31 retained character corpus
tests and three GML tests. The new material tests also passed their missed-skin
and material-spill controls. Older opt-ins and live gameplay were not rerun.
Evidence: `tmp/world-summer-specials-final-checks.log`,
`tmp/world-summer-specials-*-retained-*.log`, `tmp/world-summer-specials-gml.log`
and the individual material notes.

Every combined variant passes exact recipe validation: 12,484 across 36 characters.
All 6,230 earlier original/variant PNG and metadata files for these three
characters remain byte-identical, as do all 25,005 files for the other 33.
The 2,548 standalone variants match the combined outputs. Previous runtime rows
and controls remain exact. Evidence: `tmp/world-summer-specials-comparison.{json,log}`.

The definition audit preserves all 623 earlier region objects, every source
color, color group and target mapping. Sets, portrait-only definitions and the
collection remain unchanged. Exactly fourteen profile/registry paths are added;
Reina's stylized description now includes writing. All 36 export reports
retain earlier source entries and the current archive hash. Retained tests
change only corpus paths and total counts; previous material assertions remain
intact. Evidence: `tmp/world-summer-specials-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, native
Single packs pass 255 frame/palette observations, 80 complex phase transitions
and 15 linear completions
across all five choices. They check palette binding, wrapper idempotence,
animation phase, cycle counters, West mirroring, portrait synchronization and the complete
start-to-loop-to-end progression, including repeated loops and completion.
Natural interactions, seating, scheduling, pause policy, outfit changes, sound
and full-engine rendering remain unverified.
Evidence: `tmp/world-summer-specials-runtime.log`.

The combined summary checks fifteen source/palette bindings and 140,080 exact
pixels, raw metadata and complete artwork crops. The three galleries verify
every source frame and all seven native West hammer mirrors with exact
rendered-pixel, metadata and browser checks;
their records are in the individual material notes. Chromium also passes the
combined landing page, summary and local links. The four preview directories
total about 1.05 MiB.
Evidence: `tmp/world-summer-specials-summary-check.log`,
`tmp/world-summer-specials-preview-browser-check.json`,
`tmp/world-summer-specials-preview-sizes.json` and
`tmp/world-summer-specials-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-summer-specials-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`21400393e779f36c9bdddc8f68c354ccc629c48d800c606a0f38608067f3e817`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-summer-specials-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Finish Reina's Summer kitchen work (chopping and the three polishing phases)
and Juniper's remaining Summer magic and gestures (spell casting, hair flip
and snooze). March has fourteen injured strips left in Summer. The inventory
is `tmp/world-summer-specials-remaining.json`.
