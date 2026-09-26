# Reina, Juniper and March: Summer idle and walking

The user approved March's injured Spring batch, committed as `d685aed`.
This slice starts Summer coverage with six idle/walk strips per character:
North, South and East, with native West mirroring. The user approved this artwork for commit on 2026-09-26.
Each character adds fifteen source frames and five West mirrors; the full review
has 60 cases. The individual notes describe the material boundaries:
[Reina](reina-summer-world.md), [Juniper](juniper-summer-world.md) and
[March](march-summer-world.md).

Reina now has 147 sources, Juniper 179 and March 240. The combined 36-character
trial has 3,050 sources and 12,200 variants. Their existing Home, Page Down and U
controls keep portraits and supported sprites synchronized across all five
choices, starting with Vanilla. Other Summer actions and later outfits remain
original. No runtime Rust or GML code changed.

## Source and native metadata

The mounted source archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins are strict; no mismatch override or pin refresh was used.
All eighteen strips retain Default atlas, 80×80 frames and Middle/54 origin.
Idle poses have single-frame defaults; walking uses four frames at 0.15 seconds.
The native idle/walk cycles are linear, support Summer and all four directions,
and use the East pack with horizontal mirroring for West. Native walking pause
fallbacks still point to idle. Evidence: `tmp/world-summer-pilots-native-inputs.json`
and the three `tmp/world-summer-pilots-*-raw.json` sidecar exports.

The synthetic package test first rejected an unregistered Summer idle path,
then passed after all eighteen exact paths were registered. It checks full
animation properties, frame totals, PNG outputs and the three existing controls.
Evidence: `tmp/world-summer-pilots-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-summer-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-summer-preview/index.html):
  all frames and native West mirrors, grouped by character.

These static previews cover recoloring. They do not establish natural movement,
outfit changes, pause handling or full-engine rendering. Game-derived images,
variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 142 active tests and the release
build passed. The normal suite leaves 152 local opt-in tests ignored; 25 ran
separately and passed: three new material tests, nineteen retained character
corpus tests and three GML tests. The material tests also passed their missed-skin
and clothing-spill controls. Older opt-ins and live gameplay were not rerun.
Evidence: `tmp/world-summer-pilots-final-checks.log`,
`tmp/world-summer-pilots-*-retained-*.log`, `tmp/world-summer-pilots-gml.log`
and the individual material notes.

All eighteen author-exported raw sidecars match independent archive reads.
Every combined variant passes exact recipe validation: 12,200 across 36
characters. All 5,480 earlier original and variant PNG/metadata files for these
three characters remain byte-identical, as do all 25,005 files for the other
33 characters. The 2,264 standalone variants match the combined outputs.
Previous runtime rows and controls remain exact. Evidence:
`tmp/world-summer-pilots-metadata-check.log` and
`tmp/world-summer-pilots-comparison.{json,log}`.

The definition audit preserves all 548 earlier region objects, every color,
color group and target mapping. Sets, portrait-only definitions and the
collection are unchanged; Reina's stylized description now mentions Summer.
Exactly eighteen profile/registry paths are added. All 36 export reports retain
their earlier source entries and current archive hash. Retained tests change
only corpus paths and total counts; previous material assertions remain intact.
Evidence: `tmp/world-summer-pilots-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, probes
pass 300 frame/palette observations and 120 linear completions across all five
choices. They check palette binding, wrapper idempotence, animation phase,
cycle counters, completion state, West mirroring and portrait synchronization.
Natural movement, scheduling, pause policy, outfit changes and full-engine
rendering remain unverified. Evidence: `tmp/world-summer-pilots-runtime.log`.

The combined summary checks fifteen source/palette bindings and 136,800 exact
pixels, raw metadata and full artwork crops. The three complete galleries verify
every source frame and West mirror with exact rendered-pixel, metadata and
browser checks, recorded in the individual material notes. Chromium also passes
the combined landing page, its summary and local links. Evidence:
`tmp/world-summer-pilots-summary-check.log` and
`tmp/world-summer-pilots-preview-browser-check.json`.

The combined landing page and three character preview directories total about
1.08 MiB. All twelve links in the new notes resolve to local files.
Evidence: `tmp/world-summer-pilots-preview-sizes.json` and
`tmp/world-summer-pilots-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-summer-pilots-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`b312f324c4917776b640fedb7a12406caaa0e55fc6c771a63a71bc496d2edce7`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-summer-pilots-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Add Summer blinking, sitting, eating and drinking for these three characters:
eleven strips each. The fresh inventory shows 29 remaining Summer strips for
Reina, 27 for Juniper and 38 for March after this slice. Other general actions,
reading and character-specific work follow in later batches.
