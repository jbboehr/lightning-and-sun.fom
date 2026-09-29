# Reina, Juniper and March: Beach blinks, actions and kissing

Following the approved Beach idle/walk pilot (`000cf97`), this slice adds six
strips per character: blink East/South, general action North/South/East and kiss
East. The eighteen strips contain 93 source frames and 42 native West mirrors,
giving 135 offline review cases. The user approved the offline artwork on 2026-09-28.

| Character | Total sources | Beach folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Reina | 258 | 12/17 | [Reina](reina-beach-actions.md) |
| Juniper | 284 | 12/18 | [Juniper](juniper-beach-actions.md) |
| March | 381 | 12/14 | [March](march-beach-actions.md) |

The combined 36-character trial has 3,407 sources and 13,628 variants. Home,
Page Down and U retain their five choices and Vanilla defaults. The three
characters' four seasonal folders and Beach idle/walk artwork remain intact.
Other Beach animations retain their original colors. No runtime Rust or GML
behavior changed.

Juniper has five new bracelet-edge pixels connected to skin-colored hand
shadows. They follow the established full-skin-coverage compromise; separable
bracelet borders stay original. Her notes and gallery identify the exact action
frames. The fourteen accepted Beach idle/walk exceptions remain unchanged.

## Sources and native behavior

The read-only mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
New PNG pins are strict; no earlier pin was refreshed or mismatch override used.
All eighteen strips retain 80×80 frames, Default atlas and Middle/54 origin.
Actions have seven frames at `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`, blinks have
three at `[0.075,0.125,0.075]`, and kissing has four at `[0.15,0.15,0.8,0.15]`.

All nine native cycles are linear. Actions support all directions, blinks South
and horizontal views, and kissing horizontal views. West mirrors the East pack.
Actions retain the native `[240,360]` final-frame hold and speaking fallback to
idle. The synthetic package regression first rejected unregistered Reina Beach
action East, then passed after all eighteen paths were registered. It checks
full animation metadata, frame counts, PNG outputs and existing hotkeys.
Evidence: `tmp/trio-beach-actions-package-{red,checks}.log` and the independent
archive inventory in `tmp/trio-beach-actions-{native-inputs,remaining}.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-beach-actions-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/reina-juniper-march-beach-actions-preview/index.html):
  all 135 cases, grouped by character, with individual five-choice summaries.

Static previews cover recoloring. Natural schedules, automatic outfit changes,
interactions and live rendering still require gameplay checks. Game-derived
files, generated packages and previews remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 187 active tests and the release
build passed. The standard suite leaves 210 local opt-ins ignored; nine
local checks passed separately: the three new material tests, three retained
world corpus tests and three GML runtime tests. Omission/spill controls failed
at their intended material assertions before the final recipes passed. Other
local opt-ins and live gameplay were not rerun. Evidence:
`tmp/trio-beach-actions-{final-checks,gml,retained-reina,retained-juniper,retained-march}.log`
and the character notes.

All 13,628 combined variants pass exact recipe validation. All 3,692 standalone
Reina/Juniper/March variants match the combined output. All 9,050 previous
original/variant PNG and metadata files for these three characters remain
byte-identical, as do all 25,005 files for the other 33 characters. Existing
runtime rows and hotkeys remain exact. Evidence:
`tmp/trio-beach-actions-comparison.{json,log}`.

The definition audit preserves all 905 earlier region objects, source colors,
groups and target mappings. Sets, portraits and the collection remain unchanged.
Exactly eighteen registry/profile paths were added. The 77 retained test edits
change only corpus paths and totals. All 36 export reports preserve earlier
entries and archive hashes. All eighteen author raw sidecars exactly match
independent archive reads. Evidence:
`tmp/trio-beach-actions-{input-audit,metadata-check,timing-check}.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, the
native packs pass 675 frame/palette observations, 135 linear completions and
120 final-frame hold checks across five choices and supported directions. Checks cover West mirroring,
wrapper idempotence, frame/phase and cycle counters, and portrait synchronization.
The action holds its final frame below the native minimum and completes above
its maximum hold time. Natural schedules, automatic outfit dispatch, speaking/background pauses and
full-engine rendering were not exercised. Evidence:
`tmp/trio-beach-actions-{native-source,runtime}.log`.

Each complete gallery passes exact source/palette/frame, crop and mirrored-pixel
checks and Chromium loading checks; details are in the character notes. The
combined summary passes fifteen source/palette bindings and exact displayed-pixel
comparisons against raw metadata and complete artwork crops. The combined landing
loads all local links/images without horizontal overflow. The four preview
directories total 2,195,037 bytes (about 2144 KiB). Evidence:
`tmp/trio-beach-actions-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/trio-beach-actions-000cf97-playtest`
copy, including required compilation and installed pixels/animation metadata.
Installed archive SHA-256:
`6f0c653ec2f33294985aa1aa0c9a6167fdbf08bec27b298ca6f2133e4f39725c`.
The mounted archive and lab's `previous.zip` retain the original source hash.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/trio-beach-actions-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue with Beach bathing/swimming East/South for these three characters.
That would finish March's Beach folder. Reina's seated Beach poses and Juniper's
special swimming/spell sprites remain for following slices. The current folder
inventory is `tmp/trio-beach-actions-remaining.json`.
