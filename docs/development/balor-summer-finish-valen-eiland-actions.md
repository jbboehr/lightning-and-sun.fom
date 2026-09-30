# Balor's Summer finish and Valen/Eiland's Summer actions

Following the approved Summer expansion (`cc2325c`), this slice adds 23 strips:
99 source frames and 24 native West mirrors, giving 123 review cases. The user approved
the offline artwork on 2026-09-29.

| Character | New strips | Source frames + mirrors | Total sources | Summer coverage | Material notes |
| --- | ---: | ---: | ---: | ---: | --- |
| Balor | 7 | 42 + 0 | 173 | 29/29, complete | [Balor](balor-summer-finish.md) |
| Valen | 5 | 26 + 12 | 154 | 22/34 | [Valen](valen-summer-standard.md) |
| Eiland | 11 | 31 + 12 | 142 | 17/41 | [Eiland](eiland-summer-actions.md) |

The combined 36-character trial contains 3,654 sources and 14,616 variants.
I, O and J keep their five choices, Vanilla defaults, and synchronization
between portraits and supported overworld sprites. Unlisted outfits and actions
remain original. No runtime Rust or GML behavior changed.

All masks reuse existing world skin roles. Balor's hands and face recolor around
his unchanged coin, gem and book. Valen's hands, forearms, neck, face and sandal
skin recolor; clothing, straps, goggles and mouth interiors remain original.
Eiland's face, neck, hands and arms recolor around unchanged gold trim, pink
clothing and mouth interiors. No new material exceptions were needed.

## Sources and native behavior

The read-only mounted archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All old PNG pins remain strict; no mismatch override or pin refresh was used.
New strips retain 80×80 frames, Default atlas and Middle/54 origins. Exact raw
metadata, including duration representation, is preserved.

Balor's seated reading is a South complex cycle with 3/4/3 start/loop/end
frames. Gem inspection is another South complex cycle with 4/7/4 frames;
coin flipping is South linear with 17 frames. There are no West mirrors.

Valen's general actions use linear North/South/East packs with seven frames
at 0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds and a 240–360-tick last-frame hold.
Sleep and kiss are East Single packs; sleep uses single-frame defaults and
kiss has four frames at 0.15/0.15/0.8/0.15 seconds. West mirrors East.

Eiland's blink uses South/East; seated sit/eat/drink use North/South/East.
Blinks have three frames at 0.075/0.125/0.075 seconds. South/East eating has
five at 0.125/0.15/0.175/0.125/0.6; North eating and drinking have three
one-second frames. Sitting uses single-frame defaults. Eating/drinking retain
the native 240–360-tick hold. West mirrors East. Food/cup scene attachments
are separate from these poses and outside the static review.

The shared package test first rejected unregistered Balor Summer coin flip,
then passed with all 23 paths registered. It checks complete native animation
properties, frame counts, PNG outputs and existing hotkeys. Native animator
and NPC object sources match the previous slice byte for byte. Evidence:
`tmp/bve-summer-finish-actions-package-{red,green}.log`,
`tmp/bve-summer-finish-actions-{inventory,native-source}.log` and
`tmp/bve-summer-finish-actions-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/balor-summer-finish-valen-eiland-actions-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/balor-summer-finish-valen-eiland-actions-preview/index.html):
  all 123 cases, grouped by character, with individual five-choice summaries.

The combined index covers the entire batch. Character folders are parts of
that review regardless of timestamps. The summaries contain samples; full
galleries cover every source frame and native West mirror. Static previews
establish recoloring only. Game-derived artwork and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 205 active tests and the release
build passed. The standard suite leaves 248 local opt-ins ignored; 30 local
tests passed separately: three new material tests, twenty-four retained corpus
tests and three GML runtime tests. Six negative controls successfully generate
but fail the intended pixel assertions for omitted skin or material spill.
Other opt-ins and live gameplay were not rerun. Evidence:
`tmp/bve-summer-finish-actions-{final-checks,gml}.log`, the character retained logs
and `tmp/{balor-summer-finish,valen-summer-standard,eiland-summer-actions}-author-{omission,spill}-red.log`.
The long Balor retained run was interrupted during its final world test; that
test was rerun successfully and appended to the same log.

All 14,616 combined variants validate; all 1,876 standalone Balor/Valen/Eiland
variants pass strict recipe validation and match the combined output. All
4,460 earlier original/variant PNG and metadata files for these characters
remain byte-identical, as do all 32,015 files for the other 33 characters.
Previous runtime rows and hotkeys are exact. Evidence:
`tmp/bve-summer-finish-actions-comparison.{json,log}` and character final-output logs.

The definition audit preserves all 446 prior regions, source-color roles, groups
and target mappings. Sets, portrait-only files and the collection are unchanged.
Twenty-four retained tests change only corpus paths and totals. Exactly 23
registry/profile paths were added. All 36 export reports retain earlier entries
and the source archive hash. All 23 author raw sidecars match independent
archive reads. Evidence: `tmp/bve-summer-finish-actions-{input-audit,metadata-check,final-scope}.log`.

With simulated engine services and direct cycle selection, the current native
animator and NPC object pass 615 frame/palette observations, 120 linear
completions, 120 last-frame hold checks and 40 complex-phase transitions across
all five choices. Checks include palette binding, wrapper idempotence, frame
phase, cycle counters, seated flags, completion state and portrait synchronization.
Native directions, including West mirrors, are exercised. Natural schedules,
interaction/outfit dispatch, pause policy, attached effects and full-engine
rendering were not exercised. Evidence:
`tmp/bve-summer-finish-actions-{native-source,runtime}.log`.

Each complete gallery passes frame/palette, crop, pixel and Chromium checks;
character notes record their counts. The combined summary passes fifteen
source/palette bindings and exact displayed-pixel checks against raw metadata
and complete artwork crops. Its landing page loads every local link/image
without horizontal overflow. The four preview directories total
2,054,552 bytes (about 2006 KiB). Evidence:
`tmp/bve-summer-finish-actions-{summary-check,landing-browser}.log`.

MOMI installation passed in the fresh isolated `tmp/bve-summer-finish-actions-cc2325c-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`bec3b3a4cd4be89135506ac6170c57ca89b9d06c8cb1a15c7c6ec1558d6d76ff`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-summer-finish-actions-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Balor can begin his Autumn idle/walk pilot. Valen has twelve Summer strips
remaining: seated reading, standing/seated writing and healing. Eiland has
24 Summer strips remaining, starting with general actions/sleep/kiss, then
reading, writing, magnifying-glass and tool cycles. Current Summer inventories
are in `tmp/bve-summer-finish-actions-remaining.json`.
