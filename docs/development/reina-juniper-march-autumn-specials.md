# Reina, Juniper and March: Autumn writing, laugh and smithing

The user approved the Autumn seated-reading batch, committed as `f538e1c`.
This slice adds fourteen strips: Reina's standing and seated writing, Juniper's
laugh, and March's seated work, hammering and brow-wiping. The user approved
this artwork on 2026-09-27.

| Character | New strips | Source frames | West mirrors | Total sources | Autumn coverage | Art notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Reina | 6 | 16 | 0 | 207 | 31/35 | [Reina](reina-autumn-specials.md) |
| Juniper | 3 | 5 | 0 | 234 | 28/33 | [Juniper](juniper-autumn-specials.md) |
| March | 5 | 23 | 7 | 308 | 30/47 | [March](march-autumn-specials.md) |

The combined 36-character trial has 3,233 sources and 12,932 variants. Home,
Page Down and U retain the same five choices, starting with Vanilla. Portraits
and supported sprites share each character's palette selector. Other Autumn
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
sound configuration. Fresh metadata matches the inspected seasonal configuration;
all fourteen author sidecars match independent archive reads. Evidence:
`tmp/world-autumn-specials-native-inputs.json`, the three
`tmp/world-autumn-specials-*-raw.json` exports, and
`tmp/world-autumn-specials-{metadata,timing}-check.log`.

The synthetic package test first rejected unregistered Reina writing end,
then passed after all fourteen exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and existing controls.
Evidence: `tmp/world-autumn-specials-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-autumn-specials-preview/summary.png):
  samples of writing, laughter and brow-wiping.
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-autumn-specials-preview/index.html):
  all 44 source frames and seven native West hammer mirrors.

Juniper's laugh introduces two skin aliases: `#B58E45` uses her existing shadow
target and `#E0B572` her existing midtone target. Singleton groups keep them
separate from jewelry; all previous masks and mappings stay intact. Her notes
record the actual keyhole and midriff landmarks.

March's seated-work loop has no exposed skin in this Autumn outfit. It remains
byte-identical in all targets and is included in the full gallery and checks.

Static previews cover recoloring; they do not establish natural interactions,
seating, scheduling, outfit changes, pauses, sound or full-engine rendering.
Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 165 active tests and the release
build passed. The normal suite leaves 182 local opt-in tests ignored; 55 ran
separately and passed: three new material tests, 49 retained character corpus
tests and three GML tests. The new material tests also passed their missed-skin
and material-spill controls. Other opt-ins and live gameplay were not rerun.
Evidence: `tmp/world-autumn-specials-final-checks.log`,
`tmp/world-autumn-specials-*-retained-*.log`, `tmp/world-autumn-specials-gml.log`
and the individual material notes.

Every combined variant passes exact recipe validation: 12,932 across 36 characters.
All 7,350 earlier original/variant PNG and metadata files for these three
characters remain byte-identical, as do all 25,005 files for the other 33.
The 2,996 standalone variants match the combined outputs. Previous runtime rows
and controls remain exact. Evidence: `tmp/world-autumn-specials-comparison.{json,log}`.

The definition audit preserves all 735 earlier region objects and every previous
source color, group and target mapping. Juniper adds exactly two singleton skin
aliases and matching preset/stylized targets, using existing shadow and midtone
roles. Other sets, portrait-only definitions and the collection remain unchanged.
Exactly fourteen profile/registry paths are added. All 36 export reports retain
earlier source entries and the current archive hash. Retained tests change only
corpus paths and total counts, plus exact assertions for the two Juniper aliases;
previous material assertions remain intact. Evidence:
`tmp/world-autumn-specials-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, native
Single packs pass 255 frame/palette observations, 80 complex phase transitions
and 15 linear completions
across all five choices. They check palette binding, wrapper idempotence,
animation phase, cycle counters, West mirroring, portrait synchronization and the complete
start-to-loop-to-end progression, including repeated loops and completion.
Natural interactions, seating, scheduling, pause policy, outfit changes, sound
and full-engine rendering remain unverified.
Evidence: `tmp/world-autumn-specials-runtime.log`.

The combined summary checks fifteen source/palette bindings and 140,080 exact
pixels, raw metadata and complete artwork crops. The three galleries verify
every source frame and all seven native West hammer mirrors with exact
rendered-pixel, metadata and browser checks;
their records are in the individual material notes. Chromium also passes the
combined landing page, summary and local links. The four preview directories
total about 1.06 MiB.
Evidence: `tmp/world-autumn-specials-summary-check.log`,
`tmp/world-autumn-specials-preview-browser-check.json`,
`tmp/world-autumn-specials-preview-sizes.json` and
`tmp/world-autumn-specials-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-autumn-specials-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`28d2cfe7fc06960705f93d61275d1477483fe6f04a7c68d71c5ae01a70228178`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-autumn-specials-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Finish Reina's four kitchen-work strips and Juniper's five magic/gesture strips.
March has seventeen Autumn strips left: three poses and fourteen injured
animations. Keep his remaining work as a separate slice after these completions.
