# Balor, Valen and Eiland's first overworld pilots

Following the approved Wedding completion (`189d426`), this slice adds six
Spring idle/walk strips each for Balor, Valen and Eiland. The eighteen strips
contain 45 source frames and fifteen native West mirrors: sixty offline review
cases. The user approved the offline artwork on 2026-09-29.

| Character | Total sources | Spring folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Balor | 116 | 6/34 | [Balor](balor-world.md) |
| Valen | 98 | 6/40 | [Valen](valen-world.md) |
| Eiland | 84 | 6/47 | [Eiland](eiland-world.md) |

The combined 36-character trial has 3,483 sources and 13,932 variants. I, O and J
retain five choices and Vanilla defaults, synchronizing each character's
portraits and supported overworld sprites. Other actions and outfits remain
original. No runtime Rust or GML behavior changed.

Separate world profiles, sets and Debug Blue recipes extend the accepted
portrait definitions. The portrait-only files remain available and unchanged.
The collection switches just these three entries to their new world sets.
Character notes document the new world shades, mapping roles and material
boundaries. Balor's trouser shading and Eiland's gold trim remain protected;
Valen's North views have no exposed skin and stay original.

## Sources and native behavior

The read-only archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All reviewed PNG pins remain strict. No earlier hash was refreshed or mismatch
override used. The native animator and NPC object match the preceding slice.
All eighteen strips retain 80×80 frames, Default atlas and Middle/54 origin.
Idle uses the native single-frame defaults; walking has four frames at 0.15
seconds each. Both native cycles are linear, support Spring and
North/South/East, and default to South. West mirrors East.

The synthetic package test first rejected unregistered Balor Spring idle East,
then passed after all eighteen paths were registered. It checks full animation
properties, frame counts, PNG outputs and the three existing hotkeys. Evidence:
`tmp/bve-world-package-{red,green}.log`, `tmp/bve-world-inventory.log` and
`tmp/bve-world-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/balor-valen-eiland-world-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/balor-valen-eiland-world-preview/index.html):
  all sixty cases, grouped by character, with individual five-choice summaries.

Static previews cover recoloring. Natural schedules, pause/turning policy,
automatic outfit selection and live rendering still need gameplay checks.
Game-derived files, generated previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 197 active tests and the release
build passed. The standard suite leaves 224 local opt-ins ignored; ten
local tests passed separately: three new material tests, four retained portrait
tests and three GML runtime tests. Material controls exercise omitted skin and
spills into clothing; details are in the character notes. Other opt-ins and live
gameplay were not rerun. Evidence: `tmp/bve-world-{final-checks,retained-checks,gml}.log`.

All 13,932 combined variants pass exact recipe validation; all 1,192 standalone
Balor/Valen/Eiland variants match the combined output. All 2,800 earlier original
and variant PNG/metadata files for these characters remain byte-identical, as do
all 32,015 files for the other 33 characters. Previous runtime rows and hotkeys
remain exact. Evidence: `tmp/bve-world-comparison.{json,log}`.

The definition audit preserves all 280 prior regions, source-color roles, groups
and target mapping prefixes in the new world definitions. Portrait-only files
and retained tests are unchanged. Exactly eighteen registry/profile paths were
added, and only three collection preset references changed. All 36 export reports
retain their earlier source entries and archive hash. All eighteen author raw
sidecars match independent archive reads. Evidence:
`tmp/bve-world-{input-audit,metadata-check,timing-check}.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 300 frame/palette observations and 120 linear completions
across five choices and supported directions. They check West mirroring,
wrapper idempotence, frame phase, cycle counters and portrait synchronization.
Natural schedules, pause/turning policy, automatic outfit dispatch and full-engine
rendering were not exercised. Evidence: `tmp/bve-world-{native-source,runtime}.log`.

Each complete gallery passes source/palette/frame, crop, mirrored-pixel and
Chromium checks; details are in the character notes. The combined summary passes
fifteen source/palette bindings and exact displayed-pixel checks against raw
metadata and complete artwork crops. The combined landing loads all local
links/images without horizontal overflow. The four preview directories total
1,156,062 bytes (about 1129 KiB). Evidence:
`tmp/bve-world-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/bve-world-189d426-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`dc9a961624f07d3743f839a42912f7ab9712c7d2343078fc1271183afb78846f`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-world-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

After these material choices are accepted, continue with Spring blinking,
sitting, eating and drinking. The archive-derived remaining inventories are
in `tmp/bve-world-remaining.json`. Later batches can cover the other standard
actions and character-specific work.
