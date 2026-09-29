# Reina, Juniper and March: Wedding idle and walking

Following the approved Beach completion (`f96019b`), this pilot adds Wedding
idle and walking North/South/East for all three characters. The eighteen strips
contain 45 source frames and fifteen native West mirrors, giving 60 offline
review cases. The user approved the offline artwork on 2026-09-29.

| Character | Total sources | Wedding folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Reina | 269 | 6/15 | [Reina](reina-wedding-pilot.md) |
| Juniper | 296 | 6/15 | [Juniper](juniper-wedding-pilot.md) |
| March | 389 | 6/15 | [March](march-wedding-pilot.md) |

The combined 36-character trial has 3,438 sources and 13,752 variants. Home,
Page Down and U retain their five choices and Vanilla defaults. All three
characters' complete seasonal and Beach artwork remain intact. Other Wedding
animations retain their original colors. No runtime Rust or GML behavior changed.

Reina's dress and gold accessory edges stay original, including twenty North-view
pixels that reuse a portrait skin shade. March's face, bare hands and exposed
ankles recolor while his suit, cuffs and shoes stay original. Juniper protects
separable gold hairpin, arm-band and anklet borders. Six anklet-edge occurrences
in idle East and walking East frames one/three connect to foot skin and follow
the established full-skin-coverage compromise. Her notes and gallery mark them;
the nineteen earlier Beach exceptions remain unchanged. No new source colors,
groups or target mappings were needed.

## Updated archive and native behavior

The mounted archive has changed since the preceding slice. Its new SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`, replacing
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All 3,420 previously covered images and their raw metadata match the accepted
bundle byte for byte: 6,840 files checked. Existing pins remain unchanged and
strict; no mismatch override was used. Evidence:
`tmp/trio-wedding-pilot-source-update.log`.

The native animator is unchanged. The NPC object changed in eating and
pathfinding code; its `animate()` function remains byte-identical. Probes use
fresh copies from the updated archive. Evidence:
`tmp/trio-wedding-pilot-native-{source.log,object.diff}`.

All eighteen new strips retain 80×80 frames, Default atlas and Middle/54 origin.
Idle poses omit frame count and duration; walking has four frames at `0.15`.
Both linear cycles support North, South and horizontal directions, with South
as default; West mirrors East. Walking retains its native pause fallback to
idle. The synthetic package regression first rejected unregistered Reina
Wedding idle East, then passed after all eighteen paths were registered. It
checks full animation metadata, frame counts, PNG outputs and existing hotkeys.
Evidence: `tmp/trio-wedding-pilot-package-{red,green}.log` and the independent
inventory in `tmp/trio-wedding-pilot-{native-inputs,remaining}.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-wedding-pilot-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/reina-juniper-march-wedding-pilot-preview/index.html):
  all 60 cases, grouped by character, with individual five-choice summaries.

Static previews cover recoloring. Natural schedules, Wedding-event transitions,
automatic outfit changes and live rendering still need gameplay checks.
Game-derived files, generated packages and previews remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 192 active tests and the release
build passed. The standard suite leaves 218 local opt-ins ignored; nine
local checks passed separately: the three new material tests, three retained
world corpus tests and three GML runtime tests. Omission/spill controls failed
at their intended material assertions before the final recipes passed. Other
local opt-ins and live gameplay were not rerun. Evidence:
`tmp/trio-wedding-pilot-{final-checks,gml,retained-reina,retained-juniper,retained-march}.log`
and the character notes.

All 13,752 combined variants pass exact recipe validation. All 3,816 standalone
Reina/Juniper/March variants match the combined output. All 9,360 previous
original/variant PNG and metadata files for these three characters remain
byte-identical, as do all 24,972 files for the other 33 characters. Their 33 export reports change only the archive hash. Existing
runtime rows and hotkeys remain exact. Evidence:
`tmp/trio-wedding-pilot-comparison.{json,log}`.

The definition audit preserves all 936 earlier region objects, source colors,
groups and target mappings. Sets, portraits and the collection remain unchanged.
Exactly eighteen registry/profile paths were added. The 85 retained test edits
change only corpus paths and totals. All 36 export reports preserve earlier
entries and record the updated archive hash. All eighteen author raw sidecars exactly match
independent archive reads. Evidence:
`tmp/trio-wedding-pilot-{input-audit,metadata-check,timing-check}.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 300 frame/palette observations and 120 linear completions
across five choices and supported directions. Checks cover West mirroring,
wrapper idempotence, frame/phase and cycle counters, and portrait synchronization.
Natural schedules, Wedding-event transitions, automatic outfit dispatch and
full-engine rendering were not exercised. Evidence:
`tmp/trio-wedding-pilot-{native-source,runtime}.log`.

Each complete gallery passes exact source/palette/frame, crop and mirrored-pixel
checks and Chromium loading checks; details are in the character notes. The
combined summary passes fifteen source/palette bindings and exact displayed-pixel
comparisons against raw metadata and complete artwork crops. The combined landing
loads all local links/images without horizontal overflow. The four preview
directories total 1,190,640 bytes (about 1163 KiB). Evidence:
`tmp/trio-wedding-pilot-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/trio-wedding-pilot-f96019b-playtest`
copy, including required compilation and installed pixels/animation metadata.
Installed archive SHA-256:
`f28ebe9eee43354f4da086ee923b9c82b9064033f04edd4d430e428b9bf8a2cf`.
The mounted archive and lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/trio-wedding-pilot-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Each Wedding folder has nine strips left: blink East/South, sit and general
action North/South/East, and kiss East. Cover those next to finish all three
Wedding folders. The current inventory is `tmp/trio-wedding-pilot-remaining.json`.
