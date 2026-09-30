# Summer actions and Eiland's Spring finale

Following the approved Summer/healing/magnifying batch (`7117091`), this slice
adds Balor's Summer everyday actions, Valen's Summer idle/walk and Eiland's
remaining Spring princely-pose and tool animations. The 24 strips contain
87 source frames and 46 West mirrors, giving 133 review cases. Eiland's Spring
sprite folder is complete, joining Balor and Valen. The user approved the offline artwork on 2026-09-29.

| Character | New strips | Source frames + mirrors | Total sources | Folder coverage | Material notes |
| --- | ---: | ---: | ---: | ---: | --- |
| Balor | 11 | 31 + 12 | 161 | Summer 17/29 | [Balor](balor-summer-actions.md) |
| Valen | 6 | 15 + 5 | 138 | Summer 6/34 | [Valen](valen-summer-world.md) |
| Eiland | 7 | 41 + 29 | 125 | Spring 47/47 | [Eiland](eiland-spring-finish.md) |

The combined 36-character trial has 3,609 sources and 14,436 variants. I, O and
J retain their five choices and Vanilla defaults, synchronizing portraits and
supported overworld sprites. Other actions and outfits remain original.
No runtime Rust or GML behavior changed.

Balor's masks use the existing world skin roles and retain his wrist accessories,
shirt, belt, shoes and red mouth interiors. Valen's exposed arms, hands, ankles
and sandal skin recolor while her pink shirt, tan trousers, blue wristbands and
sandal straps stay original. Eiland's moving hands and face shading recolor;
166 shared-color gold-trim and cape pixels, his rose, tool heads, shafts,
bristles and swing trails stay original. All new masks use existing source
roles. Character notes record the all-frame material decisions and literal
protected landmarks.

## Sources and native behavior

The read-only mounted archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All earlier PNG pins remain strict; no prior pin was refreshed or mismatch
override used. All new strips retain 80×80 frames, the Default atlas and
Middle/54 origins. Raw durations and metadata representations are preserved.

Balor's blink cycle has South/East directions; sit/eat/drink have North/South/East.
West mirrors East. Blinks have three frames at 0.075/0.125/0.075 seconds;
South/East eating uses five frames at 0.125/0.15/0.175/0.125/0.6 seconds.
North eating and all drinking use three one-second frames. Sitting uses
single-frame defaults. Sit/eat/drink retain their seated flag; eating/drinking
retain the native 240–360-tick last-frame hold.

Valen's idle/walk packs are linear in North/South/East, with West mirrored
from East. Idle uses single-frame defaults; walking uses four frames at 0.15
seconds. Eiland's South-facing princely pose is a complex 8/1/3-frame cycle.
His four tool cycles are linear East Single packs with native West mirroring:
axe and pickaxe have six frames each, brush has seven, and trowel has ten.
Their individual timing arrays remain exact. Tool swing trails are included
in the gallery crops.

The shared synthetic package test first rejected unregistered Balor Summer
blink East, then passed after all 24 paths were registered. It checks complete
animation properties, frame counts, PNG outputs and existing hotkeys.
Native animator and NPC object source match the previous slice byte for byte.
Evidence: `tmp/bve-summer-actions-spring-finish-package-{red,green}.log`,
`tmp/bve-summer-actions-spring-finish-{inventory,native-source}.log` and
`tmp/bve-summer-actions-spring-finish-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/balor-valen-summer-eiland-spring-finish-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/balor-valen-summer-eiland-spring-finish-preview/index.html):
  all 133 cases, grouped by character, with individual five-choice summaries.

The combined index covers the entire batch. The linked character galleries
are parts of the same review regardless of folder timestamps. The summary
contains samples; the full galleries cover every source frame and native West
mirror. Static previews cover recoloring. Natural schedules, interaction and
outfit dispatch, pause behavior, attached effects and live rendering still
need gameplay checks. Game-derived artwork and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 203 active tests and the release
build passed. The standard suite leaves 242 local opt-ins ignored; 24
local tests passed separately: three new material tests, eighteen retained corpus
tests and three GML runtime tests. Character notes record the material checks
and omitted-skin/material-spill controls. Other opt-ins and live gameplay were
not rerun. Evidence: `tmp/bve-summer-actions-spring-finish-{final-checks,gml}.log` and the
character-focused logs.

All 14,436 combined variants pass exact recipe validation; all 1,696 standalone
Balor/Valen/Eiland variants match the combined output. All 4,000 earlier original
and variant PNG/metadata files for these characters remain byte-identical, as do
all 32,015 files for the other 33 characters. Previous runtime rows and hotkeys
remain exact. Evidence: `tmp/bve-summer-actions-spring-finish-comparison.{json,log}`.

The definition audit preserves all 400 prior regions, source-color roles, groups
and target mappings. Sets, portrait-only files and the collection are unchanged.
The eighteen retained tests change only corpus paths and totals; their earlier
material assertions remain exact. Exactly 24 registry/profile paths were
added. All 36 export reports retain their earlier source entries and archive
hash. All 24 author raw sidecars match independent archive reads.
Evidence: `tmp/bve-summer-actions-spring-finish-{input-audit,metadata-check}.log`.

With simulated engine services and direct cycle selection, the current native
animator and NPC object pass 665 frame/palette observations, 155 linear completions, 80 last-frame hold checks and 20
start/loop/end transitions across all five choices. They check palette binding, wrapper
idempotence, frame phase, cycle counters, seated flags, completion state and
portrait synchronization. Native directions, including West mirrors, are exercised. Natural schedules, interaction/outfit dispatch, pause policy, attached effects and full-engine rendering were not exercised. Evidence:
`tmp/bve-summer-actions-spring-finish-{native-source,runtime}.log`.

Each complete gallery passes source/palette/frame, crop, pixel and Chromium
checks; details are in the character notes. The combined summary passes fifteen
source/palette bindings and exact displayed-pixel checks against raw metadata
and complete artwork crops. The combined landing loads all local links/images
without horizontal overflow. The four preview directories total
2,093,882 bytes (about 2045 KiB). Evidence:
`tmp/bve-summer-actions-spring-finish-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/bve-summer-actions-spring-finish-7117091-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`bddf61d299cfdfe70c990186bc420bf884f3c0c68c45d6a0e1e7bb5575d8bd1b`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-summer-actions-spring-finish-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Balor has twelve Summer strips left: general actions, sleeping, kissing,
seated reading, coin flip and gem inspection. Valen can continue her Summer
everyday actions, while Eiland can begin Summer. Current folder inventories
are in `tmp/bve-summer-actions-spring-finish-remaining.json`.
