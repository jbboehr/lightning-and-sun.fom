# Reina, Juniper and March: Wedding completion

Following the approved Wedding idle/walk pilot (`513e2fa`), this slice adds the
remaining nine strips per character: general actions and sitting North/South/East,
blinking East/South and kissing East. The twenty-seven strips contain 102 source
frames and 45 native West mirrors, giving 147 offline review cases. The user
approved the offline artwork on 2026-09-29.

| Character | Total sources | Wedding folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Reina | 278 | 15/15 — complete | [Reina](reina-wedding-finish.md) |
| Juniper | 305 | 15/15 — complete | [Juniper](juniper-wedding-finish.md) |
| March | 398 | 15/15 — complete | [March](march-wedding-finish.md) |

The combined 36-character trial has 3,465 sources and 13,860 variants. Home,
Page Down and U retain their five choices and Vanilla defaults. All three now
cover their complete Spring, Summer, Autumn, Winter, Beach and Wedding sprite
folders. No runtime Rust or GML behavior changed.
A fresh archive-to-registry inventory confirms all eighteen character/folder
combinations are complete; counts are in `tmp/trio-wedding-finish-coverage.json`.

Reina retains another 32 North-view accessory pixels that share a portrait skin
shade; her four exposed hand pixels in seated North still recolor. March keeps
the established bare-hand/ankle treatment while preserving suit, cuffs and shoes.
Juniper's seated North has no exposed skin and stays original. Fifty new
connected jewelry-edge occurrences follow skin: 46 anklet edges and four cuff
edges, explicitly marked in her notes and gallery. Separable gold edges,
bright gold, clothing and cosmetics remain original. Her six earlier Wedding
exceptions and nineteen Beach exceptions remain unchanged. No new source colors,
groups or target mappings were needed.

## Sources and native behavior

The mounted source remains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
New PNG pins are strict; no earlier pin was refreshed or mismatch override used.
The native animator and NPC object still match the preceding archive exactly.
All twenty-seven strips retain 80×80 frames, Default atlas and Middle/54 origin.
Sitting uses one frame with omitted timing. Actions retain seven frames at
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`, blinks three at `[0.075,0.125,0.075]`,
and kissing four at `[0.15,0.15,0.8,0.15]`.

All twelve native cycles are linear. Actions and sitting support every direction,
blinks South and horizontal views, and kissing horizontal views. West mirrors
East. Sitting retains its seated flag. Actions retain the `[240,360]` final-frame
hold and speaking fallback to idle. The synthetic package test first rejected
unregistered Reina Wedding action East, then passed after all twenty-seven paths
were registered. It checks full metadata, frame counts, PNG outputs and hotkeys.
Evidence: `tmp/trio-wedding-finish-package-{red,green}.log` and independent
inventory in `tmp/trio-wedding-finish-{native-inputs,remaining}.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-wedding-finish-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/reina-juniper-march-wedding-finish-preview/index.html):
  all 147 cases, grouped by character, with individual five-choice summaries.

Static previews cover recoloring. Natural schedules, seating, Wedding-event
transitions, automatic outfit changes and live rendering still need gameplay
checks. Game-derived files, generated packages and previews remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 194 active tests and the release
build passed. The standard suite leaves 221 local opt-ins ignored; nine
local checks passed separately: the three new material tests, three retained
world corpus tests and three GML runtime tests. Omission/spill controls failed
at their intended material assertions before the final recipes passed. Other
local opt-ins and live gameplay were not rerun. Evidence:
`tmp/trio-wedding-finish-{final-checks,gml,retained-reina,retained-juniper,retained-march}.log`
and the character notes.

All 13,860 combined variants pass exact recipe validation. All 3,924 standalone
Reina/Juniper/March variants match the combined output. All 9,540 previous
original/variant PNG and metadata files for these three characters remain
byte-identical, as do all 25,005 files for the other 33 characters. Existing
runtime rows and hotkeys remain exact. Evidence:
`tmp/trio-wedding-finish-comparison.{json,log}`.

The definition audit preserves all 954 earlier region objects, source colors,
groups and target mappings. Sets, portraits and the collection remain unchanged.
Exactly twenty-seven registry/profile paths were added. The 88 retained test edits
change only corpus paths and totals. All 36 export reports preserve earlier
entries and archive hashes. All twenty-seven author raw sidecars exactly match
independent archive reads. Evidence:
`tmp/trio-wedding-finish-{input-audit,metadata-check,timing-check}.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 735 frame/palette observations, 195 linear completions and 120 final-frame hold checks
across five choices and supported directions. Checks cover West mirroring,
wrapper idempotence, frame/phase and cycle counters, seated flags and portrait synchronization.
Natural schedules, seating, speaking/background pauses, Wedding-event transitions, automatic outfit dispatch and
full-engine rendering were not exercised. Evidence:
`tmp/trio-wedding-finish-{native-source,runtime}.log`.

Each complete gallery passes exact source/palette/frame, crop and mirrored-pixel
checks and Chromium loading checks; details are in the character notes. The
combined summary passes fifteen source/palette bindings and exact displayed-pixel
comparisons against raw metadata and complete artwork crops. The combined landing
loads all local links/images without horizontal overflow. The four preview
directories total 2,430,976 bytes (about 2374 KiB). Evidence:
`tmp/trio-wedding-finish-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/trio-wedding-finish-513e2fa-playtest`
copy, including required compilation and installed pixels/animation metadata.
Installed archive SHA-256:
`cd867c80cd4befe237169d6f285973df271a790b9296eeb615da07a559ceef5d`.
The mounted archive and lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/trio-wedding-finish-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

No Wedding strips remain for these three characters. Continue the broader
overworld plan with small Spring idle/walk pilots for the next portrait-covered
characters, establishing their material boundaries before expanding to larger
batches. Balor, Valen and Eiland are the proposed next trio.
All eighteen of their Spring idle/walk paths exist in the current archive;
the proposed source list is `tmp/next-spring-pilots.json`.
