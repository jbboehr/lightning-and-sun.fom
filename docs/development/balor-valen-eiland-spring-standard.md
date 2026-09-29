# Balor, Valen and Eiland: Spring actions, sleep and kiss

Following the approved everyday-action batch (`c9afefc`), this slice adds five
strips per character: general actions North/South/East, sleeping East and kissing
East. The fifteen strips contain 78 source frames and 36 native West mirrors,
giving 114 offline review cases. The user approved the offline artwork on 2026-09-29.

| Character | Total sources | Spring folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Balor | 132 | 22/34 | [Balor](balor-spring-standard.md) |
| Valen | 114 | 22/40 | [Valen](valen-spring-standard.md) |
| Eiland | 100 | 22/47 | [Eiland](eiland-spring-standard.md) |

The combined 36-character trial has 3,531 sources and 14,124 variants. I, O and
J retain their five choices and Vanilla defaults, synchronizing each character's
portraits and supported overworld sprites. Other actions and outfits remain
original. No runtime Rust or GML behavior changed.

The masks use existing color roles. They preserve 74 shared-color trouser
pixels for Balor and 95 cape, belt and uniform pixels for Eiland. Comparing
Eiland's North action with the same Summer pose identified four exposed
hand-edge pixels beside his Spring cape; these now recolor and have a positive
regression check. Valen needed no new material exceptions. The character notes
record the complete frame counts and material decisions.

## Sources and native behavior

The mounted read-only archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All reviewed PNG pins remain strict; no earlier hash was refreshed or mismatch
override used. The native animator and NPC object match the preceding slice.
All fifteen strips retain 80×80 frames, Default atlas and Middle/54 origin.

Actions have seven frames at `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kissing
has four at `[0.15,0.15,0.8,0.15]`. Sleeping uses single-frame defaults. All
nine native cycles are linear. Actions retain their `[240,360]` final-frame
hold and speaking fallback to idle. Kissing and sleeping use native East
single-direction packs; West mirrors East.

The synthetic package test first rejected unregistered Balor Spring action East,
then passed after all fifteen paths were registered. It checks full animation
properties, frame counts, PNG outputs and existing hotkeys. Evidence:
`tmp/bve-spring-standard-package-{red,green}.log`,
`tmp/bve-spring-standard-inventory.log` and `tmp/bve-spring-standard-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/balor-valen-eiland-spring-standard-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/balor-valen-eiland-spring-standard-preview/index.html):
  all 114 cases, grouped by character, with individual five-choice summaries.

The combined HTML index is the entry point for this entire batch. Individual
character preview folders are linked parts of the same review, regardless of
folder timestamps. The summary PNG contains samples; the linked galleries
cover every frame and native West mirror.

Static previews cover recoloring. Natural schedules, speaking pauses,
sleeping/kissing transitions, automatic outfit selection and live rendering
still need gameplay checks. Game-derived files, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 199 active tests and the release
build passed. The standard suite leaves 230 local opt-ins ignored; twelve
local tests passed separately: three new material tests, six retained corpus
tests and three GML runtime tests. Material controls exercise omitted skin and
spills into clothing or mouths; details and focused logs are in the character notes. Other opt-ins and live
gameplay were not rerun. Evidence: `tmp/bve-spring-standard-{final-checks,gml}.log`.

All 14,124 combined variants pass exact recipe validation; all 1,384 standalone
Balor/Valen/Eiland variants match the combined output. All 3,310 earlier original
and variant PNG/metadata files for these characters remain byte-identical, as do
all 32,015 files for the other 33 characters. Previous runtime rows and hotkeys
remain exact. Evidence: `tmp/bve-spring-standard-comparison.{json,log}`.

The definition audit preserves all 331 prior regions, source-color roles, groups
and target mappings. Sets, portrait-only files and the collection are unchanged.
The six retained tests change only corpus paths and totals; their earlier
material assertions remain exact. Exactly fifteen registry/profile paths
were added. All 36 export reports
retain their earlier source entries and archive hash. All fifteen author raw
sidecars match independent archive reads. Evidence:
`tmp/bve-spring-standard-{input-audit,metadata-check,timing-check}.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 570 frame/palette observations, 120 linear completions and 120 final-frame hold checks
across five choices and supported directions. They check West mirroring,
wrapper idempotence, frame phase, cycle counters, seated flags and portrait synchronization.
Natural schedules, sleeping/kissing transitions, automatic outfit dispatch and full-engine
rendering were not exercised. Evidence: `tmp/bve-spring-standard-{native-source,runtime}.log`.

Each complete gallery passes source/palette/frame, crop, mirrored-pixel and
Chromium checks; details are in the character notes. The combined summary passes
fifteen source/palette bindings and exact displayed-pixel checks against raw
metadata and complete artwork crops. The combined landing loads all local
links/images without horizontal overflow. The four preview directories total
1,893,380 bytes (about 1849 KiB). Evidence:
`tmp/bve-spring-standard-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/bve-spring-standard-c9afefc-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`167e7b4a883d13fc49b72f4c8fdf0e5d673452c6b7ef9a9113fda211ec07f289`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-spring-standard-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue with Spring shocked reactions and seated reading after these material
choices are accepted. Remaining archive-derived inventories are in
`tmp/bve-spring-standard-remaining.json`; later batches can cover character-specific work.
