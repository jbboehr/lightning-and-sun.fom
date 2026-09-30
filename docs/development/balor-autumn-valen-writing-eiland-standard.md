# Balor's Autumn pilot and Valen/Eiland's Summer actions

Following the approved Summer finish/actions batch (`6e895c1`), this slice
adds 20 strips: 67 source frames and 17 native West mirrors, giving 84 review
cases. The user approved the offline artwork on 2026-09-29.

| Character | New strips | Source frames + mirrors | Total sources | Current folder coverage | Material notes |
| --- | ---: | ---: | ---: | --- | --- |
| Balor | 6 | 15 + 5 | 179 | Autumn 6/28 | [Balor](balor-autumn-world.md) |
| Valen | 9 | 26 + 0 | 163 | Summer 31/34 | [Valen](valen-summer-writing.md) |
| Eiland | 5 | 26 + 12 | 147 | Summer 22/41 | [Eiland](eiland-summer-standard.md) |

The combined 36-character trial has 3,674 sources and 14,696 variants. I, O
and J retain their five choices, Vanilla defaults and portrait/world palette
synchronization. Unlisted actions/outfits remain original. No runtime Rust or
GML behavior changed.

Existing world skin roles cover all new strips. Balor's face, neck and hands
recolor while his Autumn scarf and clothing remain original. Valen's exposed
skin changes around her unchanged books and pen. Eiland's face, neck and
hands change around his unchanged gold trim and pink clothing. No new source
roles or material exceptions were needed.

## Sources and native behavior

The mounted archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 469 prior PNG pins match fresh exports; no pin refresh or mismatch override
was used. New strips preserve 80×80 frames, the Default atlas, Middle/54
origins and exact raw metadata, including scalar/array duration representation.

Balor's idle/walk cycles are linear North/South/East packs. Idle uses
single-frame defaults; walking uses four 0.15-second frames. West mirrors East.

Valen has three South complex cycles with no West mirrors. Seated reading
uses 3/4/3 start/loop/end frames: starts/ends use scalar 0.1-second timing and
the loop uses 3.0/0.1/3.0/0.1. Standing and seated writing each use 2/4/2
frames: starts use scalar 0.125, loops use 0.1/0.125/0.1/0.3, and ends use
0.125/0.1. Reading and seated writing retain their native seated flag.

Eiland's general actions are linear North/South/East packs with seven frames
at 0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds and a 240–360-tick last-frame hold.
Sleep and kiss are East Single packs; sleep uses single-frame defaults and
kiss has four frames at 0.15/0.15/0.8/0.15 seconds. West mirrors East.

The shared package test first rejected unregistered Balor Autumn idle East,
then passed after all 20 paths were registered. It checks complete animation
properties, source frame counts, output PNGs and unchanged hotkeys. Native
animator and NPC object sources match the previous slice byte for byte.
Evidence: `tmp/bve-autumn-writing-standard-package-{red,green}.log`,
`tmp/bve-autumn-writing-standard-{inventory,native-source}.log` and
`tmp/bve-autumn-writing-standard-native-inputs.json`.

## Offline review

- [Five-choice sample summary](../../generated/balor-autumn-valen-writing-eiland-standard-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/balor-autumn-valen-writing-eiland-standard-preview/index.html):
  all 84 cases and each character's five-choice summary.

The combined index covers the entire batch. Its linked character folders
belong to the same review regardless of timestamps. The summaries contain
samples; the full galleries include every source frame and native West mirror.
Game-derived artwork and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 206 active tests and the release
build passed. The standard suite leaves 251 local opt-ins ignored; 33 local
tests passed separately: three new material tests, twenty-seven retained corpus
tests and three GML runtime tests. Six negative controls generate successfully
but fail the intended pixel assertions for omitted skin or material spill.
Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/bve-autumn-writing-standard-final-checks.log`,
`tmp/bve-autumn-writing-standard-{balor-spring,balor-seasonal,valen,eiland,gml}-local.log`
and the character `author-{omission,spill}-red.log` files.

All 14,696 combined variants validate; all 1,956 standalone Balor/Valen/Eiland
variants pass strict recipe validation and match the combined output. All
4,690 earlier original/variant PNG and metadata files for these characters
remain byte-identical, as do all 32,015 files for the other 33 characters.
Previous runtime rows and hotkeys remain exact. Evidence:
`tmp/bve-autumn-writing-standard-comparison.{json,log}` and character final-output logs.

The definition audit preserves all 469 prior regions, source-color roles, groups
and target mappings. Sets, portrait-only files and the collection are unchanged.
Twenty-seven retained tests change only corpus paths and totals. Exactly 20
registry/profile paths were added. All 36 export reports retain earlier entries
and the source archive hash. All 20 author raw sidecars match independent
archive reads. Evidence: `tmp/bve-autumn-writing-standard-{input-audit,metadata-check,final-scope}.log`.

With simulated engine services and direct cycle selection, the current native
animator and NPC object pass 420 frame/palette observations, 80 linear
completions, 40 last-frame hold checks and 60 complex-phase transitions across
all five choices. Checks include palette binding, wrapper idempotence, frame
phase, cycle counters, seated flags, completion state and portrait synchronization.
Native directions, including West mirrors, are exercised. Natural schedules,
interaction/outfit dispatch, pause policy, attached effects and full-engine
rendering were not exercised. Evidence:
`tmp/bve-autumn-writing-standard-{native-source,runtime}.log`.

Each gallery passes frame/palette, crop, pixel and Chromium checks; character
notes record their counts. The combined summary passes fifteen source/palette
bindings and exact displayed-pixel checks against raw metadata and complete
artwork crops. Its landing page loads every local link/image without horizontal
overflow. The four preview directories total 1,627,001 bytes
(about 1589 KiB). Evidence:
`tmp/bve-autumn-writing-standard-{summary-check,landing-browser}.log`
and character final-output logs.

MOMI installation passed in the fresh isolated `tmp/bve-autumn-writing-standard-6e895c1-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`66aae3d6783781e39b03b2ee888db61ed7f1472a1f7416685aeed31ec0c68613`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-autumn-writing-standard-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Balor can continue Autumn blinking, sitting, eating and drinking. Valen's
three Summer healing strips will complete her Summer folder. Eiland can
continue Summer seated reading and standing/seated writing; after these nine
strips, his magnifying-glass and tool cycles remain. Folder inventories are
in `tmp/bve-autumn-writing-standard-remaining.json`.
