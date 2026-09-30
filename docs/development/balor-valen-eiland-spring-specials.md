# Balor, Valen and Eiland: Spring gestures and writing

Following the approved shocked/reading batch (`21a6537`), this slice adds six
strips per character. Balor gains coin flipping, hair flipping, jumping and the
complete gem-inspection cycle, finishing his Spring sprite folder. Valen and
Eiland gain standing and seated writing start/loop/end phases. The eighteen
strips contain 91 source frames plus 22 native West jump mirrors, giving 113
offline review cases. The user approved the offline artwork on 2026-09-29.

| Character | Source frames + mirrors | Total sources | Spring folder coverage | Material notes |
| --- | ---: | ---: | ---: | --- |
| Balor | 59 + 22 | 144 | 34/34 | [Balor](balor-spring-specials.md) |
| Valen | 16 + 0 | 126 | 34/40 | [Valen](valen-spring-specials.md) |
| Eiland | 16 + 0 | 112 | 34/47 | [Eiland](eiland-spring-specials.md) |

The combined 36-character trial has 3,567 sources and 14,268 variants. I, O and
J retain their five choices and Vanilla defaults, synchronizing portraits and
supported overworld sprites. Other actions and outfits remain original.
No runtime Rust or GML behavior changed.

Existing skin roles cover all new frames. The masks preserve Balor's coin,
gem and glints, and the writing surfaces and quills for Valen and Eiland.
Balor retains 261 trouser pixels that share a portrait skin color; Eiland
retains 31 shared-color trim pixels. Valen needs no new material exclusions.
Character notes record the moving-hand and prop boundaries checked in every
frame and all four target palettes.

## Sources and native behavior

The mounted read-only archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
Source pins remain strict; no prior pin was refreshed or mismatch override used.
All eighteen strips use 80×80 frames and the Default atlas. Balor's jump has
numeric origin 24/54; his other strips and Eiland's use Middle/54. Valen's
writing strips retain numeric 40/54. These raw representations are preserved.

Balor's coin flip, hair flip and jump are linear cycles with 17, 5 and 22
frames. Gem inspection uses 4/7/4-frame start/loop/end phases. Jump uses the
native East Single pack with West mirroring; his other new cycles face South.
Standing and seated writing each use South-only 2/4/2-frame phases, retaining
the seated flag only for seated writing. All raw per-frame durations remain
intact. Native animator and NPC object sources match the previous batch.

The shared synthetic package test first rejected unregistered Balor coin flip,
then passed after all eighteen paths were registered. It checks complete
animation properties, frame counts, PNG outputs and existing hotkeys, including
the jump's different origin. Evidence:
`tmp/bve-spring-specials-package-{red,green}.log`,
`tmp/bve-spring-specials-{inventory,native-source}.log` and
`tmp/bve-spring-specials-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/balor-valen-eiland-spring-specials-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/balor-valen-eiland-spring-specials-preview/index.html):
  all 113 cases, grouped by character, with individual five-choice summaries.

The combined HTML index is the entry point for this entire batch. Individual
character preview folders are linked parts of the same review, regardless of
folder timestamps. The summary PNG contains samples; the linked galleries
cover every source frame and native jump mirror.

Static previews cover recoloring. Natural schedules, interaction and outfit
dispatch, jump movement, pause policy and live rendering still need gameplay
checks. Game-derived files, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 201 active tests and the release
build passed. The standard suite leaves 236 local opt-ins ignored; eighteen
local tests passed separately: three new material tests, twelve retained corpus
tests and three GML runtime tests. Character notes record the material checks
and omitted-skin/material-spill controls. Other opt-ins and live gameplay were
not rerun. Evidence: `tmp/bve-spring-specials-{final-checks,gml}.log` and the
character-focused logs.

All 14,268 combined variants pass exact recipe validation; all 1,528 standalone
Balor/Valen/Eiland variants match the combined output. All 3,640 earlier original
and variant PNG/metadata files for these characters remain byte-identical, as do
all 32,015 files for the other 33 characters. Previous runtime rows and hotkeys
remain exact. Evidence: `tmp/bve-spring-specials-comparison.{json,log}`.

The definition audit preserves all 364 prior regions, source-color roles, groups
and target mappings. Sets, portrait-only files and the collection are unchanged.
The twelve retained tests change only corpus paths and totals; their earlier
material assertions remain exact. Exactly eighteen registry/profile paths were
added. All 36 export reports retain their earlier source entries and archive
hash. All eighteen author raw sidecars match independent archive reads.
Evidence: `tmp/bve-spring-specials-{input-audit,metadata-check}.log`.

With simulated engine services and direct cycle selection, the current native
animator and NPC object pass 565 frame/palette observations, 20 linear completions and 100
start/loop/end transitions across all five choices. They check palette binding, wrapper
idempotence, frame phase, cycle counters, seated flags, completion state and
portrait synchronization. South and native East/West jump views are exercised.
Jump uses its raw 24/54 origin in the sprite-info inputs. Natural schedules, interaction/outfit dispatch, pause policy, jump movement, attached effects and full-engine rendering were not exercised. Evidence:
`tmp/bve-spring-specials-{native-source,runtime}.log`.

Each complete gallery passes source/palette/frame, crop, pixel and Chromium
checks; details are in the character notes. The combined summary passes fifteen
source/palette bindings and exact displayed-pixel checks against raw metadata
and complete artwork crops. The combined landing loads all local links/images
without horizontal overflow. The four preview directories total
2,202,479 bytes (about 2151 KiB). Evidence:
`tmp/bve-spring-specials-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/bve-spring-specials-21a6537-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`2b12606d4dd2caee5918ba7b8ca5d99e4200efdb7c7a9c8f44183da334af855b`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-spring-specials-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue with Valen's healing/charm cycles and Eiland's magnifying-glass,
princely-pose and archaeology animations. Balor is ready for his next outfit.
Remaining archive-derived inventories are in
`tmp/bve-spring-specials-remaining.json`.
