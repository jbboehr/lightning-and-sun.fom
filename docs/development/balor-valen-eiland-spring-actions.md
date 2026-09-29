# Balor, Valen and Eiland: Spring everyday actions

Following the approved Spring idle/walk pilots (`55450c5`), this slice adds
eleven strips per character: blinking East/South, plus sitting, eating and
drinking North/South/East. The thirty-three strips contain 93 source frames
and 36 native West mirrors, giving 129 offline review cases. The user approved
the offline artwork on 2026-09-29.

| Character | Total sources | Spring folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Balor | 127 | 17/34 | [Balor](balor-spring-actions.md) |
| Valen | 109 | 17/40 | [Valen](valen-spring-actions.md) |
| Eiland | 95 | 17/47 | [Eiland](eiland-spring-actions.md) |

The combined 36-character trial has 3,516 sources and 14,064 variants. I, O and
J retain their five choices and Vanilla defaults, synchronizing each character's
portraits and supported overworld sprites. Other actions and outfits remain
original. No runtime Rust or GML behavior changed.

No new source colors, groups or target mappings were needed. Balor's masks
protect 72 trouser pixels that reuse a portrait skin shade. Eiland's masks
protect 86 shared-color uniform and gold-trim pixels. Valen needs no material
exceptions. Mouth interiors stay original; Valen and Eiland's seated North
views have no exposed skin and remain unchanged.

## Sources and native behavior

The mounted read-only archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All reviewed PNG pins remain strict; no earlier hash was refreshed or mismatch
override used. Native animator and NPC object sources match the preceding slice.
All thirty-three strips retain 80×80 frames, Default atlas and Middle/54 origin.

Sitting uses single-frame defaults. Blinks have three frames with durations
`[0.075,0.125,0.075]`; eating East/South has five frames at
`[0.125,0.15,0.175,0.125,0.6]`. Eating North and every drinking strip have three
frames at 1.0 seconds each. All twelve native cycles are linear; eating,
drinking and sitting retain their seated flag. Eating/drinking retain the
`[240,360]` final-frame hold. West mirrors East.

The synthetic package test first rejected unregistered Balor Spring blink East,
then passed after all thirty-three paths were registered. It checks full
animation properties, frame counts, PNG outputs and existing character hotkeys.
Evidence: `tmp/bve-spring-actions-package-{red,green}.log`,
`tmp/bve-spring-actions-inventory.log` and `tmp/bve-spring-actions-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/balor-valen-eiland-spring-actions-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/balor-valen-eiland-spring-actions-preview/index.html):
  all 129 cases, grouped by character, with individual five-choice summaries.

The combined HTML index is the entry point for the entire batch. Individual
character preview folders are linked parts of this same review, regardless
of folder timestamps. The summary PNG contains samples; the linked character
galleries cover every frame and native West mirror.

The game draws food, plates and drink props separately; these previews show
the character sprites. This was checked in the current NPC object's
`setup_eating` and `setup_drinking` functions. Prop setup and rendering were
not exercised by the direct animation probes.

Static previews cover recoloring. Natural schedules, seating transitions,
automatic outfit selection and live rendering still need gameplay checks.
Game-derived files, generated previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 198 active tests and the release
build passed. The standard suite leaves 227 local opt-ins ignored; nine
local tests passed separately: three new material tests, three retained world
tests and three GML runtime tests. Material controls exercise omitted skin and
spills into clothing or mouths; details and focused logs are in the character notes. Other opt-ins and live
gameplay were not rerun. Evidence: `tmp/bve-spring-actions-{final-checks,gml}.log`.

All 14,064 combined variants pass exact recipe validation; all 1,324 standalone
Balor/Valen/Eiland variants match the combined output. All 2,980 earlier original
and variant PNG/metadata files for these characters remain byte-identical, as do
all 32,015 files for the other 33 characters. Previous runtime rows and hotkeys
remain exact. Evidence: `tmp/bve-spring-actions-comparison.{json,log}`.

The definition audit preserves all 298 prior regions, source-color roles, groups
and target mappings. Sets, portrait-only files and the collection are unchanged.
The three retained tests change only corpus paths and totals; their earlier
material assertions remain exact. Exactly thirty-three registry/profile paths
were added. All 36 export reports
retain their earlier source entries and archive hash. All thirty-three author raw
sidecars match independent archive reads. Evidence:
`tmp/bve-spring-actions-{input-audit,metadata-check,timing-check}.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 645 frame/palette observations, 225 linear completions and 240 final-frame hold checks
across five choices and supported directions. They check West mirroring,
wrapper idempotence, frame phase, cycle counters, seated flags and portrait synchronization.
Natural schedules, seating transitions, automatic outfit dispatch and full-engine
rendering were not exercised. Evidence: `tmp/bve-spring-actions-{native-source,runtime}.log`.

Each complete gallery passes source/palette/frame, crop, mirrored-pixel and
Chromium checks; details are in the character notes. The combined summary passes
fifteen source/palette bindings and exact displayed-pixel checks against raw
metadata and complete artwork crops. The combined landing loads all local
links/images without horizontal overflow. The four preview directories total
2,049,718 bytes (about 2002 KiB). Evidence:
`tmp/bve-spring-actions-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/bve-spring-actions-55450c5-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`cb410f06ec2d4cd747f055d000b82dbc80647c88ce61d19ec8820dd49f8cd57a`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-spring-actions-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue with Spring general actions, sleeping and kissing after these material
choices are accepted. The remaining archive-derived inventories are in
`tmp/bve-spring-actions-remaining.json`; later batches can cover reactions,
reading and character-specific work.
