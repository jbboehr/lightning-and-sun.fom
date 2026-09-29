# Reina, Juniper and March: Beach idle and walking

After the approved March Winter completion (`8d631ac`), this slice starts Beach
coverage with idle and walking North, South and East for each character. The
eighteen strips contain 45 source frames and fifteen native West mirrors, giving
60 offline review cases. The user approved the offline artwork on 2026-09-28.

| Character | Total sources | Beach folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Reina | 252 | 6/17 | [Reina](reina-beach-pilot.md) |
| Juniper | 278 | 6/18 | [Juniper](juniper-beach-pilot.md) |
| March | 375 | 6/14 | [March](march-beach-pilot.md) |

The combined 36-character trial now has 3,389 sources and 13,556 variants.
Home, Page Down and U keep their five palette choices and Vanilla defaults.
Their four seasonal sprite folders remain complete. Other Beach animations
retain original colors. No runtime Rust or GML behavior changed.

Juniper retains the earlier full-skin-coverage compromise: fourteen bracelet-edge
pixels in walking North/South share a color and connected component with her
skin, so they recolor together. Separable bracelet borders and gold centers
remain original. Her material note and gallery identify the exact cases.

## Sources and native behavior

The read-only mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
New PNG pins are strict; no prior pin was refreshed and no hash-mismatch override
was used. All eighteen strips use 80×80 frames, Default atlas and Middle/54
origin. Idle has one frame with default timing; walking has four frames at
0.15 seconds each. Native idle/walk cycles include the Beach outfit, and West
mirrors the East pack. Walking retains its speaking/background idle fallback.

The synthetic package regression first rejected unregistered Reina Beach idle
East, then passed after registering the eighteen exact paths. It checks full
animation properties, frame counts, PNG contents and existing hotkeys. Evidence:
`tmp/trio-beach-pilot-package-{red,checks}.log` and the independent archive
inventory in `tmp/trio-beach-pilot-{native-inputs,remaining}.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-beach-pilot-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/reina-juniper-march-beach-pilot-preview/index.html):
  all sixty cases, grouped by character, with individual five-choice summaries.

These static previews cover recoloring. Natural NPC schedules, automatic outfit
changes, interactions and live rendering still require gameplay checks.
Game-derived files, generated packages and previews remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 185 active tests and the release
build passed. The standard suite leaves 207 local opt-ins ignored; nine
local checks passed separately: the three new material tests, three retained
world corpus tests and three GML runtime tests. Omission/spill controls failed
at their intended material assertions before the final recipes passed. Other
local opt-ins and live gameplay were not rerun. Evidence:
`tmp/trio-beach-pilot-{final-checks,gml,retained-reina,retained-juniper,retained-march}.log`
and the character notes.

All 13,556 combined variants pass exact recipe validation. All 3,620 standalone
Reina/Juniper/March variants match the combined output. All 8,870 previous
original/variant PNG and metadata files for these three characters remain
byte-identical, as do all 25,005 files for the other 33 characters. Existing
runtime rows and hotkeys remain exact. Evidence:
`tmp/trio-beach-pilot-comparison.{json,log}`.

The definition audit preserves all 887 earlier region objects, source colors,
groups and target mappings. Sets, portraits and the collection remain unchanged.
Exactly eighteen registry/profile paths were added. The 74 retained test edits
change only corpus paths and totals. All 36 export reports preserve earlier
entries and archive hashes. All eighteen author raw sidecars exactly match
independent archive reads. Evidence:
`tmp/trio-beach-pilot-{input-audit,metadata-check,timing-check}.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 300 frame/palette observations and 120 linear completions
across five choices and supported directions. Checks cover West mirroring,
wrapper idempotence, frame/phase and cycle counters, and portrait synchronization.
Natural schedules, automatic outfit dispatch, speaking/background pauses and
full-engine rendering were not exercised. Evidence:
`tmp/trio-beach-pilot-{native-source,runtime}.log`.

Each complete gallery passes exact source/palette/frame, crop and mirrored-pixel
checks and Chromium loading checks; details are in the character notes. The
combined summary passes fifteen source/palette bindings and exact displayed-pixel
comparisons against raw metadata and complete artwork crops. The combined landing
loads all local links/images without horizontal overflow. The four preview
directories total 1,165,940 bytes (about 1139 KiB). Evidence:
`tmp/trio-beach-pilot-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/trio-beach-pilot-8d631ac-playtest`
copy, including required compilation and installed pixels/animation metadata.
Installed archive SHA-256:
`92de4123ddfd68c9224829bfa267fa14eb322067e2825c0dcef46807850317f4`.
The mounted archive and lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/trio-beach-pilot-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue with Beach blinks East/South, general actions North/South/East and kiss
East for these three characters. Reina's seated Beach poses, bathing/swimming
and Juniper's special swimming/spell sprites remain for later slices. The
current folder inventory is `tmp/trio-beach-pilot-remaining.json`.
