# Reina, Juniper and March: Beach bathing/swimming

Following the approved Beach action batch (`3f89d89`), this slice adds
bathing/swimming East/South for all three characters. The six strips contain
24 source frames and twelve native West mirrors, giving 36 offline review cases.
The user approved the offline artwork on 2026-09-29.

| Character | Total sources | Beach folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Reina | 260 | 14/17 | [Reina](reina-beach-swim.md) |
| Juniper | 286 | 14/18 | [Juniper](juniper-beach-swim.md) |
| March | 383 | 14/14 — complete | [March](march-beach-swim.md) |

The combined 36-character trial has 3,413 sources and 13,652 variants. Home,
Page Down and U retain their five choices and Vanilla defaults. The three
characters' seasonal and earlier Beach artwork remain intact. Unlisted Beach
animations retain their original colors. No runtime Rust or GML behavior changed.

Only the exposed faces and jaws recolor in these swimming strips. Water, foam,
detached droplets, hair, eyes and transparency remain original. No new palette
aliases or material exceptions were needed. Juniper's nineteen accepted bracelet
edge exceptions in earlier Beach strips remain unchanged.

## Sources and native behavior

The read-only mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
New PNG pins are strict; no earlier pin was refreshed or mismatch override used.
All six strips retain four 80×80 frames, 0.15-second duration per frame, Default
atlas and Middle/54 origin. Each native `bath_swim` cycle is linear, Beach-only,
and supports South and horizontal views with South as its default direction.
West mirrors the East pack.

The synthetic package regression first rejected unregistered Reina Beach
bathing/swimming East, then passed after all six paths were registered. It checks
full animation metadata, frame counts, PNG outputs and existing hotkeys.
Evidence: `tmp/trio-beach-swim-package-{red,green}.log` and independent archive
inventory in `tmp/trio-beach-swim-{native-inputs,remaining}.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-beach-swim-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/reina-juniper-march-beach-swim-preview/index.html):
  all 36 cases, grouped by character, with individual five-choice summaries.

Static previews cover recoloring. Natural schedules, entering/leaving water,
automatic outfit changes and live rendering still require gameplay checks.
Game-derived files, generated packages and previews remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 189 active tests and the release
build passed. The standard suite leaves 213 local opt-ins ignored; nine
local checks passed separately: the three new material tests, three retained
world corpus tests and three GML runtime tests. Omission/spill controls failed
at their intended material assertions before the final recipes passed. Other
local opt-ins and live gameplay were not rerun. Evidence:
`tmp/trio-beach-swim-{final-checks,gml,retained-reina,retained-juniper,retained-march}.log`
and the character notes.

All 13,652 combined variants pass exact recipe validation. All 3,716 standalone
Reina/Juniper/March variants match the combined output. All 9,230 previous
original/variant PNG and metadata files for these three characters remain
byte-identical, as do all 25,005 files for the other 33 characters. Existing
runtime rows and hotkeys remain exact. Evidence:
`tmp/trio-beach-swim-comparison.{json,log}`.

The definition audit preserves all 923 earlier region objects, source colors,
groups and target mappings. Sets, portraits and the collection remain unchanged.
Exactly six registry/profile paths were added. The 80 retained test edits
change only corpus paths and totals. All 36 export reports preserve earlier
entries and archive hashes. All six author raw sidecars exactly match
independent archive reads. Evidence:
`tmp/trio-beach-swim-{input-audit,metadata-check,timing-check}.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 180 frame/palette observations and 45 linear completions
across five choices and supported directions. Checks cover West mirroring,
wrapper idempotence, frame/phase and cycle counters, and portrait synchronization.
Natural schedules, entering/leaving water, automatic outfit dispatch and
full-engine rendering were not exercised. Evidence:
`tmp/trio-beach-swim-{native-source,runtime}.log`.

Each complete gallery passes exact source/palette/frame, crop and mirrored-pixel
checks and Chromium loading checks; details are in the character notes. The
combined summary passes fifteen source/palette bindings and exact displayed-pixel
comparisons against raw metadata and complete artwork crops. The combined landing
loads all local links/images without horizontal overflow. The four preview
directories total 677,428 bytes (about 662 KiB). Evidence:
`tmp/trio-beach-swim-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/trio-beach-swim-3f89d89-playtest`
copy, including required compilation and installed pixels/animation metadata.
Installed archive SHA-256:
`e376ce38f0a4f803632d8c89abd25bbadb0af3d033dce7d8dfce9815051c5205`.
The mounted archive and lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/trio-beach-swim-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Finish Reina's three seated Beach poses and Juniper's four special swimming/spell
strips. March's Beach folder is complete. The current folder inventory is
`tmp/trio-beach-swim-remaining.json`.
