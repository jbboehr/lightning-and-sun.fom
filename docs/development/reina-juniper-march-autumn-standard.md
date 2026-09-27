# Reina, Juniper and March: Autumn actions, sleeping and kissing

The user approved the Autumn everyday-action batch, committed as `719325a`.
This slice adds five Autumn strips per character: general actions North/South/East,
sleep East and kiss East. Each character adds 26 source frames and twelve native
West mirrors, giving 114 review cases. The user approved the artwork on 2026-09-27.
Material notes: [Reina](reina-autumn-standard.md), [Juniper](juniper-autumn-standard.md)
and [March](march-autumn-standard.md).

Reina now has 198 sources, Juniper 228 and March 300. The combined 36-character
trial has 3,210 sources and 12,840 variants. Their Home, Page Down and U controls
keep portraits and supported sprites synchronized across all five choices,
starting with Vanilla. Other Autumn actions and later outfits retain original
sprites. No runtime Rust or GML code changed.

## Source and native metadata

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or source-pin refresh was used.
All fifteen strips retain Default atlas, 80×80 frames and Middle/54 origin.
General actions have seven frames at `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`;
kissing has four at `[0.15,0.15,0.8,0.15]`; sleeping has single-frame defaults.

All three native cycles are linear. General actions retain the `[240,360]`
final-frame hold and speaking fallback to idle. Sleep and kiss declare only
East in the NPC TOML. Their native Single pack still permits West cardinality,
and the NPC object mirrors it horizontally. The review includes these renderable
West views without claiming natural sleep/kiss dispatch. Evidence:
`tmp/world-autumn-standard-native-inputs.json` and the three
`tmp/world-autumn-standard-*-raw.json` sidecar exports.

The synthetic package test first rejected an unregistered Autumn action path,
then passed after all fifteen exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and the existing controls.
Evidence: `tmp/world-autumn-standard-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/reina-juniper-march-autumn-standard-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-autumn-standard-preview/index.html):
  every source frame and native West mirror, grouped by character.

Static previews cover recoloring. They do not establish natural cycle dispatch,
sleep/kiss interactions, outfit changes, pauses or full-engine rendering.
Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 160 active tests and the release
build passed. The normal suite leaves 176 local opt-in tests ignored; 49 ran
separately and passed: three new material tests, 43 retained corpus tests
and three GML tests. The material tests passed their missed-skin and material-spill
controls. Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-autumn-standard-{final-checks,gml}.log`,
`tmp/world-autumn-standard-*-retained-*.log` and the individual material notes.

All fifteen author raw sidecars match independent archive reads. Every combined
variant passes exact recipe validation: 12,840 across 36 characters.
All 7,110 earlier original/variant PNG and metadata files for the three
characters remain byte-identical, as do all 25,005 files for the other 33.
The 2,904 standalone variants match the combined outputs. Previous runtime
rows and controls remain exact. Evidence:
`tmp/world-autumn-standard-metadata-check.log` and
`tmp/world-autumn-standard-comparison.{json,log}`.

The definition audit preserves all 711 earlier region objects and every source
color, group and mapping, including Juniper's existing Autumn highlight alias.
All sets, portrait-only definitions and tests, and the collection remain
unchanged. Exactly fifteen profile/registry paths are added. All 36 export
reports retain earlier source entries and the current archive hash. Retained
tests change only corpus paths and total counts; previous material assertions
remain intact. Evidence:
`tmp/world-autumn-standard-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, probes
pass 570 frame/palette observations, 120 linear completions and 120 final-frame
hold boundary checks across all five choices. They check palette binding,
wrapper idempotence, animation phase,
cycle counters, completion state, West mirroring and portrait synchronization.
General actions stay on their final frame below the native minimum hold
and complete above its maximum.
Natural cycle dispatch, sleep/kiss interactions, scheduling, pause policy, outfit changes and full-engine
rendering remain unverified. Evidence: `tmp/world-autumn-standard-runtime.log`.

The combined summary checks fifteen source/palette bindings and 136,800 exact
pixels, raw metadata and complete artwork crops. The three complete galleries
verify every source frame and West mirror with exact rendered-pixel, metadata
and browser checks, recorded in the individual notes. Chromium also passes the
combined landing page, summary and links. The preview directories total about
1.77 MiB. All new documentation links resolve. Evidence:
`tmp/world-autumn-standard-summary-check.log`,
`tmp/world-autumn-standard-preview-browser-check.json`,
`tmp/world-autumn-standard-preview-sizes.json` and
`tmp/world-autumn-standard-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-autumn-standard-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`f14ba304374b8d386a9144f7b49700a20e8a12dc15675dbfbbaaa01658d5eaf9`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All eleven frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-autumn-standard-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue the seasonal plan with the three Autumn seated-reading phases for each
character. The fresh inventory leaves thirteen Autumn strips for Reina, eleven
for Juniper and 25 for March. Writing, magic, smithing and March's injured poses
follow in later slices.
