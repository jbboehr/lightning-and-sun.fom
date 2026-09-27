# March: injured Summer sprites

The user approved the Reina and Juniper Summer completion batch, committed as
`e51400d`. This slice adds March's fourteen remaining Summer strips: injured
idle and walk North/South/East, blink South/East, seated poses North/South/East,
seated blink South/East and a South-facing action. The user approved the artwork
for commit on 2026-09-27. See the [material notes](march-summer-injured-art.md) for the masks and
focused corpus test.

The new strips contain 37 source frames and twelve native West mirrors, making
49 review cases. March now has 278 sources and 1,112 variants; all 44 strips in
his Summer sprite folder are covered. Reina and Juniper's Summer folders remain
complete. The combined 36-character trial has 3,144 sources and 12,576 variants.
U keeps March's portraits and supported sprites on the same five-choice selector,
starting with Vanilla. Later overworld outfits retain their original sprites.
No runtime Rust or GML code changed.

## Source and native metadata

The mounted archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no mismatch override or pin refresh was used.
All fourteen strips retain 80×80 frames, Default atlas and numeric origin
40.0/54.0. Walking has four frames at 0.15 seconds; standing/seated blinks have
three at `[0.075,0.125,0.075]`; idle and seated poses use single-frame defaults.
The seven-frame action retains `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` timing.

All six native cycles are linear. Idle, walk and seated poses support all four
directions; blinks support South and horizontal directions; the action faces
South. West uses the East pack with native horizontal mirroring. Seated cycles
retain their seated flag. The action retains its native `[240,360]` final-frame
hold interval and speaking fallback to injured idle. Evidence:
`tmp/world-summer-injured-native-inputs.json` and
`tmp/march-summer-injured-author-metadata.json`.

The synthetic package test first rejected unregistered March injured action,
then passed after all fourteen exact paths were registered. It checks complete
animation properties, frame totals, PNG outputs and March's existing U binding.
Evidence: `tmp/world-summer-injured-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/march-summer-injured-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-summer-injured-preview/blue-review/index.html):
  every source frame and native West mirror, including the action's final pose.

Static images cover recoloring; they do not establish natural injury-state
dispatch, schedules, interaction/outfit changes or full-engine rendering.
Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 153 active tests and the release
build passed. The normal suite leaves 167 local opt-in tests ignored; 16 ran
separately and passed: the new material corpus test, 12 retained March corpus
tests and three GML tests. The new test passed its missed-skin and material-spill
controls. Older opt-ins and live gameplay were not rerun. Evidence:
`tmp/world-summer-injured-{final-checks,gml}.log`,
`tmp/world-summer-injured-march-retained-*.log` and the material notes.

All fourteen author raw sidecars match independent archive reads. Every combined
variant passes exact recipe validation: 12,576 across 36 characters.
All 2,640 earlier March original/variant PNG and metadata files remain
byte-identical, as do all 28,835 files for the other 35 characters. The 1,112
standalone variants match the combined outputs. Previous runtime rows and
controls remain exact. Evidence: `tmp/world-summer-injured-metadata-check.log`
and `tmp/world-summer-injured-comparison.{json,log}`.

The definition audit preserves all 264 earlier March region objects, every
source color, color group and target mapping. Sets, portrait-only definitions
and the collection remain unchanged. Exactly fourteen profile/registry paths
are added. All 36 export reports retain earlier source entries and the current
archive hash. Retained tests change only corpus paths and total counts;
previous material assertions remain intact. Evidence:
`tmp/world-summer-injured-input-audit.log`.

The native animator and NPC object probe inputs match the mounted archive byte
for byte. Using simulated engine services and direct cycle selection, probes
pass 245 frame/palette observations, 95 linear completions and ten final-frame
hold boundary checks across all five choices. They check palette binding,
wrapper idempotence, animation phase, cycle counters, West mirroring and
portrait synchronization. The action stays on its final frame below the native
minimum and completes above its maximum. Natural injury-state dispatch,
scheduling, pause policy, outfit changes and full-engine rendering remain
unverified. Evidence: `tmp/world-summer-injured-runtime.log`.

The summary and full gallery cover all 37 source frames and twelve native West
mirrors. Exact source/palette, crop, metadata and rendered-pixel checks and the
Chromium checks are recorded in the material notes. The preview directory is
about 0.78 MiB. Local documentation links resolve. Evidence:
`tmp/world-summer-injured-preview-size.json` and
`tmp/world-summer-injured-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-summer-injured-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`4b27d34e0fb87b26acd8b766f6729f032422f59b8965bdd5a0617286fb8efe17`.
The mounted archive and the lab's `previous.zip` retain the original source
hash above. All five frozen registry/recipe inputs stayed unchanged through
generation and installation. Evidence:
`tmp/world-summer-injured-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Reina, Juniper and March now have complete Spring and Summer folders. The next
proposed slice is an Autumn idle/walk pilot for these three characters, using
the same source inspection and complete offline review workflow.
