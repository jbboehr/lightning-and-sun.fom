# Reina and Juniper: Beach completion

Following the approved swimming batch (`2cc74e5`), this slice adds Reina's three
seated poses and Juniper's four special swimming/spell strips. The seven strips
contain nineteen source frames and five native West mirrors, giving 24 offline
review cases. The user approved the offline artwork on 2026-09-29.

| Character | Total sources | Beach folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Reina | 263 | 17/17 — complete | [Reina](reina-beach-finish.md) |
| Juniper | 290 | 18/18 — complete | [Juniper](juniper-beach-finish.md) |

The combined 36-character trial has 3,420 sources and 13,680 variants. Home and
Page Down retain their five choices and Vanilla defaults. March's 14/14 Beach
folder was completed in the preceding slice. No runtime Rust or GML behavior
changed.

Reina's exposed back, arms, hands and folded legs recolor while her swimsuit,
sandal straps, hair and eyes stay original. Juniper's faces, small North-facing
ear areas and raised casting hands recolor; her pink water/foam effects, hair
and jewelry stay original. No new palette roles or material exceptions are
needed. Juniper's nineteen accepted exceptions in earlier Beach strips remain
unchanged.

## Sources and native behavior

The read-only mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
New PNG pins are strict; no earlier pin was refreshed or mismatch override used.
All seven strips retain 80×80 frames, Default atlas and vertical origin 54.
Reina's one-frame poses omit frame count/duration and use horizontal `Middle`.
Juniper's four-frame strips retain duration `0.15` and numeric horizontal `40.0`.

Reina's linear `sit` cycle retains `is_seated = true`. Juniper's `swim_idle` and
`swim_spell_cast` cycles are linear and Beach-only. Sitting and swimming idle
support North, South and horizontal directions; casting supports South only.
All default to South. West mirrors the East pack.

The synthetic package regression first rejected unregistered Reina Beach sit
East, then passed after all seven paths were registered. It checks full animation
metadata, frame counts, PNG outputs and existing hotkeys. Evidence:
`tmp/duo-beach-finish-package-{red,green}.log` and the independent archive
inventory in `tmp/duo-beach-finish-{native-inputs,remaining}.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-beach-finish-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/reina-juniper-beach-finish-preview/index.html):
  all 24 cases, grouped by character, with individual five-choice summaries.

Static previews cover recoloring. Natural schedules, seating placement, spell
events, automatic outfit changes and live rendering still need gameplay checks.
Game-derived files, generated packages and previews remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 190 active tests and the release
build passed. The standard suite leaves 215 local opt-ins ignored; seven
local checks passed separately: the two new material tests, two retained
world corpus tests and three GML runtime tests. Omission/spill controls failed
at their intended material assertions before the final recipes passed. Other
local opt-ins and live gameplay were not rerun. Evidence:
`tmp/duo-beach-finish-{final-checks,gml,retained-reina,retained-juniper}.log`
and the character notes.

All 13,680 combined variants pass exact recipe validation. All 2,212 standalone
Reina/Juniper variants match the combined output. All 5,460 previous
original/variant PNG and metadata files for these two characters remain
byte-identical, as do all 28,840 files for the other 34 characters. Existing
runtime rows and hotkeys remain exact. Evidence:
`tmp/duo-beach-finish-comparison.{json,log}`.

The definition audit preserves all 546 earlier region objects, source colors,
groups and target mappings. Sets, portraits and the collection remain unchanged.
Exactly seven registry/profile paths were added. The 54 retained test edits
change only corpus paths and totals. All 36 export reports preserve earlier
entries and archive hashes. All seven author raw sidecars exactly match
independent archive reads. Evidence:
`tmp/duo-beach-finish-{input-audit,metadata-check,timing-check}.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 120 frame/palette observations and 45 linear completions
across five choices and supported directions. Checks cover West mirroring,
wrapper idempotence, frame/phase and cycle counters, seated flags and portrait synchronization.
Natural schedules, seating placement, spell events, automatic outfit dispatch and
full-engine rendering were not exercised. Evidence:
`tmp/duo-beach-finish-{native-source,runtime}.log`.

Each complete gallery passes exact source/palette/frame, crop and mirrored-pixel
checks and Chromium loading checks; details are in the character notes. The
combined summary passes ten source/palette bindings and exact displayed-pixel
comparisons against raw metadata and complete artwork crops. The combined landing
loads all local links/images without horizontal overflow. The three preview
directories total 469,747 bytes (about 459 KiB). Evidence:
`tmp/duo-beach-finish-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/duo-beach-finish-2cc74e5-playtest`
copy, including required compilation and installed pixels/animation metadata.
Installed archive SHA-256:
`ea9594eeef68b8f20d900302512ec60bb05bcf8759ddfbe159ff7cb4f920beb3`.
The mounted archive and lab's `previous.zip` retain the original source hash.
All eight frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/duo-beach-finish-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Reina, Juniper and March now cover their complete Spring, Summer, Autumn, Winter
and Beach sprite folders. Proposed next slice: their Wedding idle/walk sprites,
following the same pilot-then-completion approach used for Hayden, Ryis and Celine.
