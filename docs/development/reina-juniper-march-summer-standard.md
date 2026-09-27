# Reina, Juniper and March: Summer actions, sleeping and kissing

The user approved the Summer everyday-action batch, committed as `91767e0`.
This slice adds five Summer strips per character: general actions North/South/East,
sleep East and kiss East. The user approved this artwork for commit on 2026-09-26. Each character adds
26 source frames and twelve native West mirrors, giving 114 review cases.
Material decisions are recorded separately for [Reina](reina-summer-standard.md),
[Juniper](juniper-summer-standard.md) and [March](march-summer-standard.md).

Reina now has 163 sources, Juniper 195 and March 256. The combined 36-character
trial has 3,098 sources and 12,392 variants. Their existing Home, Page Down and U
controls keep portraits and supported sprites synchronized across all five
choices, starting with Vanilla. Other Summer actions and later outfits remain
original. No runtime Rust or GML code changed.

## Source and native metadata

The mounted source archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or pin refresh was used.
All fifteen strips retain Default atlas, 80×80 frames and Middle/54 offset.
General actions have seven frames at `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`;
kissing has four at `[0.15,0.15,0.8,0.15]`; sleeping has single-frame defaults.

All three native cycles are linear. General actions retain the `[240,360]`
final-frame hold and speaking fallback to idle. Sleep and kiss declare only
East in the NPC TOML; the native Single pack still permits West cardinality,
and the NPC object mirrors it horizontally. The review includes those
renderable West views without claiming natural sleep/kiss dispatch.
Evidence: `tmp/world-summer-standard-native-inputs.json` and the three
`tmp/world-summer-standard-*-raw.json` exports.

The synthetic package test first rejected an unregistered Summer action path,
then passed after all fifteen exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and the existing controls.
Evidence: `tmp/world-summer-standard-package-{red,checks}.log`.
All fifteen author raw sidecars match the independent archive reads.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-summer-standard-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-summer-standard-preview/index.html):
  all frames and native West mirrors, grouped by character.

Juniper has fourteen new bracer-edge pixel occurrences inseparable from nearby
skin under the current component-mask rules. They recolor to keep skin coverage
complete, following the earlier compromise. Separable borders remain original.
Her material notes and gallery identify the affected frames; literal assertions
cover both the exceptions and protected borders. Earlier approved masks remain
unchanged.

Static previews cover recoloring; they do not establish natural cycle dispatch,
sleep/kiss interactions, outfit changes, pauses or full-engine rendering.
Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 146 active tests and the release
build passed. The normal suite leaves 158 local opt-in tests ignored; 31 ran
separately and passed: three new material tests, 25 retained character corpus
tests and three GML tests. The new material tests also passed their missed-skin
and material-spill controls. Older opt-ins and live gameplay were not rerun.
Evidence: `tmp/world-summer-standard-final-checks.log`,
`tmp/world-summer-standard-*-retained-*.log`, `tmp/world-summer-standard-gml.log`
and the individual material notes.

Every combined variant passes exact recipe validation: 12,392 across 36 characters.
All 5,990 earlier original/variant PNG and metadata files for these three
characters remain byte-identical, as do all 25,005 files for the other 33.
The 2,456 standalone variants match the combined outputs. Previous runtime rows
and controls remain exact. Evidence: `tmp/world-summer-standard-comparison.{json,log}`.

The definition audit preserves all 599 earlier region objects, every source
color, color group and target mapping. Sets, portrait-only definitions and the
collection remain unchanged. Exactly fifteen profile/registry paths are added;
Reina's stylized description now includes the new Summer actions. All 36 export reports
retain earlier source entries and the current archive hash. Retained tests
change only corpus paths and total counts; previous material assertions remain
intact. Evidence: `tmp/world-summer-standard-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, probes
pass 570 frame/palette observations, 120 linear completions and 120 final-frame
hold boundary checks across all five choices. They check palette binding,
wrapper idempotence, animation phase, cycle counters, completion state, West
mirroring and portrait synchronization. General actions hold below the
native minimum and complete above the maximum. Natural cycle dispatch,
sleep/kiss interactions, scheduling, pause policy, outfit changes and full-engine rendering
remain unverified. Evidence: `tmp/world-summer-standard-runtime.log`.

The combined summary checks fifteen source/palette bindings and 132,000 exact
pixels, raw metadata and complete artwork crops. The three galleries verify
every source frame and native West mirror with exact rendered-pixel, metadata
and browser checks; their records are in the individual material notes.
Chromium also passes the combined landing page, summary and local links.
The four preview directories total about 1.77 MiB.
Evidence: `tmp/world-summer-standard-summary-check.log`,
`tmp/world-summer-standard-preview-browser-check.json`,
`tmp/world-summer-standard-preview-sizes.json` and
`tmp/world-summer-standard-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-summer-standard-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`e9c0febcd194af313ef5d49b6142550d013947aac0b966af29b085b9c963d11f`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-summer-standard-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Add the three Summer seated-reading phases for each character. The fresh
inventory leaves thirteen Summer strips for Reina, eleven for Juniper and 22
for March after this batch. Writing, magic, smithing and March's injured poses
follow in later slices.
