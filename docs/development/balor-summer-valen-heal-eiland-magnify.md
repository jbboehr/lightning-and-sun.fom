# Balor's Summer sprites, Valen's healing and Eiland's magnifying glass

Following the approved Spring gestures/writing batch (`5d672e3`), this slice
adds six strips per character. Balor begins Summer with idle/walk sprites;
Valen's healing and charm cycles finish her Spring sprite folder; Eiland gains
the complete Spring magnifying-glass cycle in its native directions. The
eighteen strips contain 42 source frames and 17 West mirrors, giving 59 review
cases. The user approved the offline artwork on 2026-09-29.

| Character | Source frames + mirrors | Total sources | Folder coverage | Material notes |
| --- | ---: | ---: | ---: | --- |
| Balor | 15 + 5 | 150 | Summer 6/29 | [Balor](balor-summer-world.md) |
| Valen | 15 + 6 | 132 | Spring 40/40 | [Valen](valen-spring-finish.md) |
| Eiland | 12 + 6 | 118 | Spring 40/47 | [Eiland](eiland-spring-magnify.md) |

The combined 36-character trial has 3,585 sources and 14,340 variants. I, O and
J retain their five choices and Vanilla defaults, synchronizing portraits and
supported overworld sprites. Other actions and outfits remain original.
No runtime Rust or GML behavior changed.

All new masks use existing source-color roles. Balor's exposed Summer forearms
and ankles recolor while clothing and wrist accessories retain their materials.
Valen's charm-end hand includes a small `#B65932` shadow at frame-local `[33,48]`,
using an existing portrait role. Her gold goggles and pale cloth stay original.
Eiland's lens, gold rim and 54 shared-color material pixels stay original.
One `#DE8F5D` point at `[43,44]` in magnify-start East frame one is treated as
chest trim: its position at the sash endpoint and the corresponding Summer pose
support that inference. A literal protected landmark and spill control guard
it; the far hand still recolors. Character notes record the complete decisions.
The ignored cross-outfit comparison is
`tmp/bve-summer-heal-magnify-trim-compare.{png,log}`.

## Sources and native behavior

The read-only mounted archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All earlier PNG pins remain strict; no prior pin was refreshed or mismatch
override used. All strips retain 80×80 frames and the Default atlas. Balor
and Eiland use Middle/54 origins; Valen retains numeric 40/54.

Balor's idle/walk packs are linear in North/South/East, with West mirrored
from East. Idle uses single-frame defaults; walking uses four frames at 0.15
seconds. Valen's South-facing charm cycle uses 3/4/2 start/loop/end frames;
healing uses an East Single pack with 1/4/1 frames and native West mirroring.
Eiland's magnify cycle uses a South/East Multi pack with 3/1/2 frames in each
direction and West mirrored from East. South remains its default and North
fallback. All raw durations and metadata representations are preserved.

The shared synthetic package test first rejected unregistered Balor Summer
idle East, then passed after all eighteen paths were registered. It checks
complete animation properties, frame counts, PNG outputs and existing hotkeys.
Native animator and NPC object source match the previous slice byte for byte.
Evidence: `tmp/bve-summer-heal-magnify-package-{red,green}.log`,
`tmp/bve-summer-heal-magnify-{inventory,native-source}.log` and
`tmp/bve-summer-heal-magnify-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/balor-summer-valen-heal-eiland-magnify-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/balor-summer-valen-heal-eiland-magnify-preview/index.html):
  all 59 cases, grouped by character, with individual five-choice summaries.

The combined index covers the entire batch. The linked character galleries
are parts of the same review regardless of folder timestamps. The summary
contains samples; the full galleries cover every source frame and native West
mirror. Static previews cover recoloring. Natural schedules, interaction and
outfit dispatch, pause behavior, attached effects and live rendering still
need gameplay checks. Game-derived artwork and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 202 active tests and the release
build passed. The standard suite leaves 239 local opt-ins ignored; 21
local tests passed separately: three new material tests, fifteen retained corpus
tests and three GML runtime tests. Character notes record the material checks
and omitted-skin/material-spill controls. Other opt-ins and live gameplay were
not rerun. Evidence: `tmp/bve-summer-heal-magnify-{final-checks,gml}.log` and the
character-focused logs.

All 14,340 combined variants pass exact recipe validation; all 1,600 standalone
Balor/Valen/Eiland variants match the combined output. All 3,820 earlier original
and variant PNG/metadata files for these characters remain byte-identical, as do
all 32,015 files for the other 33 characters. Previous runtime rows and hotkeys
remain exact. Evidence: `tmp/bve-summer-heal-magnify-comparison.{json,log}`.

The definition audit preserves all 382 prior regions, source-color roles, groups
and target mappings. Sets, portrait-only files and the collection are unchanged.
The fifteen retained tests change only corpus paths and totals; their earlier
material assertions remain exact. Exactly eighteen registry/profile paths were
added. All 36 export reports retain their earlier source entries and archive
hash. All eighteen author raw sidecars match independent archive reads.
Evidence: `tmp/bve-summer-heal-magnify-{input-audit,metadata-check}.log`.

With simulated engine services and direct cycle selection, the current native
animator and NPC object pass 295 frame/palette observations, 40 linear completions and 120
start/loop/end transitions across all five choices. They check palette binding, wrapper
idempotence, frame phase, cycle counters, seated flags, completion state and
portrait synchronization. Native directions, including West mirrors, are exercised. Natural schedules, interaction/outfit dispatch, pause policy, attached effects and full-engine rendering were not exercised. Evidence:
`tmp/bve-summer-heal-magnify-{native-source,runtime}.log`.

Each complete gallery passes source/palette/frame, crop, pixel and Chromium
checks; details are in the character notes. The combined summary passes fifteen
source/palette bindings and exact displayed-pixel checks against raw metadata
and complete artwork crops. The combined landing loads all local links/images
without horizontal overflow. The four preview directories total
1,166,992 bytes (about 1140 KiB). Evidence:
`tmp/bve-summer-heal-magnify-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/bve-summer-heal-magnify-5d672e3-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`fdbf532919e399f13e29f4f5964dfbfd94a8c711aeccd24f76a7c945787608d4`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-summer-heal-magnify-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Eiland has seven Spring strips left: the three princely-pose phases and four
tool animations. Balor can continue his Summer everyday actions, and Valen
is ready for her next outfit. Current folder inventories are in
`tmp/bve-summer-heal-magnify-remaining.json`.
