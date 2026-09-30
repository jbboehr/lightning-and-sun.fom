# Balor, Valen and Eiland's Summer expansion

Following the approved Summer-actions/Spring-finale batch (`fe8b473`), this
slice adds Balor's Summer general actions, sleep and kiss; Valen's Summer
everyday actions; and Eiland's Summer idle/walk. The 22 strips contain 72 source
frames and 29 West mirrors, giving 101 review cases. The user approved the offline artwork on 2026-09-29.

| Character | New strips | Source frames + mirrors | Total sources | Summer coverage | Material notes |
| --- | ---: | ---: | ---: | ---: | --- |
| Balor | 5 | 26 + 12 | 166 | 22/29 | [Balor](balor-summer-standard.md) |
| Valen | 11 | 31 + 12 | 149 | 17/34 | [Valen](valen-summer-actions.md) |
| Eiland | 6 | 15 + 5 | 131 | 6/41 | [Eiland](eiland-summer-world.md) |

The combined 36-character trial has 3,631 sources and 14,524 variants. I, O and
J retain their five choices and Vanilla defaults, synchronizing portraits and
supported overworld sprites. Other actions and outfits remain original.
No runtime Rust or GML behavior changed.

All masks use existing skin roles. Balor's kiss jaw edge, hands, forearms and
ankles recolor while his wrist accessories, shirt, belt and shoes remain original.
Valen's exposed arms, fingers and sandal skin recolor; pink sleeves, blue straps,
trousers, goggles and red mouth interiors retain their materials. Eiland's
forearms and hands recolor while his Summer gold trim and pink clothing remain
original. No new material exceptions were needed; character notes record the
full-frame decisions and protected source landmarks.

## Sources and native behavior

The read-only mounted archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All earlier PNG pins remain strict; no prior pin was refreshed or mismatch
override used. All new strips retain 80×80 frames, the Default atlas and
Middle/54 origins. Raw durations and metadata representations are preserved.

Balor's general actions are linear North/South/East packs with seven frames at
0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds and a 240–360-tick last-frame hold.
Sleep and kiss are East Single packs: sleep uses single-frame defaults,
while kiss has four frames at 0.15/0.15/0.8/0.15 seconds. West mirrors East.

Valen's blink cycle has South/East directions; sit/eat/drink have North/South/East.
West mirrors East. Blinks have three frames at 0.075/0.125/0.075 seconds;
South/East eating uses five frames at 0.125/0.15/0.175/0.125/0.6 seconds.
North eating and all drinking use three one-second frames. Sitting uses
single-frame defaults. Sit/eat/drink retain their seated flag; eating/drinking
retain the native 240–360-tick last-frame hold. These strips contain poses;
separate food and cup scene attachments are outside the static review.

Eiland's idle/walk packs are linear in North/South/East, with West mirrored
from East. Idle uses single-frame defaults; walking uses four frames at 0.15
seconds. Natural schedule selection and pause dispatch remain untested here.

The shared synthetic package test first rejected unregistered Balor Summer
action East, then passed after all 22 paths were registered. It checks complete
animation properties, frame counts, PNG outputs and existing hotkeys.
Native animator and NPC object source match the previous slice byte for byte.
Evidence: `tmp/bve-summer-expansion-package-{red,green}.log`,
`tmp/bve-summer-expansion-{inventory,native-source}.log` and
`tmp/bve-summer-expansion-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/balor-valen-eiland-summer-expansion-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/balor-valen-eiland-summer-expansion-preview/index.html):
  all 101 cases, grouped by character, with individual five-choice summaries.

The combined index covers the entire batch. The linked character galleries
are parts of the same review regardless of folder timestamps. The summary
contains samples; the full galleries cover every source frame and native West
mirror. Static previews cover recoloring. Natural schedules, interaction and
outfit dispatch, pause behavior, attached effects and live rendering still
need gameplay checks. Game-derived artwork and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 204 active tests and the release
build passed. The standard suite leaves 245 local opt-ins ignored; 27
local tests passed separately: three new material tests, twenty-one retained corpus
tests and three GML runtime tests. Character notes record the material checks
and omitted-skin/material-spill controls. Other opt-ins and live gameplay were
not rerun. Evidence: `tmp/bve-summer-expansion-{final-checks,gml}.log` and the
character-focused logs.

All 14,524 combined variants pass exact recipe validation; all 1,784 standalone
Balor/Valen/Eiland variants match the combined output. All 4,240 earlier original
and variant PNG/metadata files for these characters remain byte-identical, as do
all 32,015 files for the other 33 characters. Previous runtime rows and hotkeys
remain exact. Evidence: `tmp/bve-summer-expansion-comparison.{json,log}`.

The definition audit preserves all 424 prior regions, source-color roles, groups
and target mappings. Sets, portrait-only files and the collection are unchanged.
The twenty-one retained tests change only corpus paths and totals; their earlier
material assertions remain exact. Exactly 22 registry/profile paths were
added. All 36 export reports retain their earlier source entries and archive
hash. All 22 author raw sidecars match independent archive reads.
Evidence: `tmp/bve-summer-expansion-{input-audit,metadata-check}.log`.

With simulated engine services and direct cycle selection, the current native
animator and NPC object pass 505 frame/palette observations, 155 linear completions and 120 last-frame hold checks across all five choices. They check palette binding, wrapper
idempotence, frame phase, cycle counters, seated flags, completion state and
portrait synchronization. Native directions, including West mirrors, are exercised. Natural schedules, interaction/outfit dispatch, pause policy, attached effects and full-engine rendering were not exercised. Evidence:
`tmp/bve-summer-expansion-{native-source,runtime}.log`.

Each complete gallery passes source/palette/frame, crop, pixel and Chromium
checks; details are in the character notes. The combined summary passes fifteen
source/palette bindings and exact displayed-pixel checks against raw metadata
and complete artwork crops. The combined landing loads all local links/images
without horizontal overflow. The four preview directories total
1,683,932 bytes (about 1644 KiB). Evidence:
`tmp/bve-summer-expansion-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/bve-summer-expansion-fe8b473-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`f2d499bb71e1b0dac807fe6690de9e20a5bbc55ad73a79c6a770e801254bc4ea`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-summer-expansion-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Balor has seven Summer strips left: seated reading, coin flip and gem inspection.
Valen can continue her Summer general actions, sleep and kiss; Eiland can add
Summer everyday actions. Current folder inventories are in
`tmp/bve-summer-expansion-remaining.json`.
