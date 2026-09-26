# March: injured Spring sprites

The user approved the [Spring kitchen work, magic and gestures batch](reina-juniper-march-finish.md),
committed as `3294baf`. This slice adds March's fourteen remaining Spring
strips: injured idle and walk in North/South/East, blink in South/East, seated
poses in North/South/East, seated blink in South/East, and a South-facing action.
The user approved this artwork for commit on 2026-09-26. See the [material notes](march-spring-injured-art.md)
for per-frame decisions and the focused corpus test.

The new strips contain 37 source frames and twelve native West mirrors, making
49 review cases. March now has 234 sources and 936 variants; all 53 strips in
his Spring sprite folder are covered. Reina and Juniper's Spring folders remain
complete. The combined 36-character trial has 3,032 sources and 12,128 variants.
U keeps March's portraits and supported sprites on the same five-choice selector,
starting with Vanilla. Overworld sprites in other outfits remain original.
No runtime Rust or GML behavior changed.

## Source and native metadata

The read-only source archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All source pins remain strict; no mismatch override or pin refresh was used.
The fourteen raw sidecars retain 80×80 frames, Default atlas and numeric origin
40/54. Walking has four frames at 0.15 seconds; standing/seated blinks have three
at `[0.075,0.125,0.075]`; idle and seated poses have single-frame defaults.
The seven-frame action retains `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` timing.

All six native cycles are linear. Idle, walk and seated poses support all four
directions; blinks support South and horizontal directions; the action faces
South. West uses the corresponding East pack with native horizontal mirroring.
Seated cycles retain their seated flag. The action's final frame retains its
native `[240,360]` hold interval and speaking fallback to injured idle.
Evidence: `tmp/world-spring-injured-native-inputs.json` and
`tmp/march-spring-injured-author-metadata.json`.

The synthetic package test first rejected unregistered March injured action,
then passed after all fourteen exact paths were added. It checks complete
animation properties, frame counts, PNG outputs and March's existing U binding.
Evidence: `tmp/world-spring-injured-package-{red,checks}.log`.

## Offline review

- [Five-choice summary](../../generated/march-spring-injured-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-spring-injured-preview/blue-review/index.html):
  every source frame and native West mirror, including the action's final pose.

Static images cover recoloring; they do not establish natural injury-state
dispatch, schedules, interaction/outfit changes or full-engine rendering.
Game-derived images, variants, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 140 active tests and the release
build passed. The normal suite leaves 149 local opt-in tests ignored; ten ran
separately and passed: the new material corpus test, six retained March corpus
tests and three GML tests. The new test also passed its missed-finger and
clothing-spill controls. The grouped local run was terminated during its last
world test; that test passed when rerun on its own. Evidence:
`tmp/world-spring-injured-{final-checks,retained-gml,world-recheck}.log` and the
character's material notes. Older opt-in tests and live gameplay were not rerun.

All fourteen author-exported raw sidecars match the independent archive read.
Every combined variant passes exact recipe validation: 12,128 variants across
36 characters. All 2,200 earlier March original and variant PNG/metadata files
remain byte-identical, as do all 28,155 files for the other 35 characters.
The 936 standalone variants match the combined outputs. Previous runtime rows
and controls are unchanged. Evidence:
`tmp/world-spring-injured-metadata-check.log` and
`tmp/world-spring-injured-comparison.{json,log}`.

The definition audit preserves all 220 earlier March region objects and every
source color, color group and target mapping. His set, stylized recipe,
portrait-only definitions and the collection remain unchanged. Exactly fourteen
registry/profile paths are added. All 36 export reports preserve earlier source
entries and the current archive hash. Retained tests change only corpus paths
and total source counts; earlier pixel assertions remain intact. Evidence:
`tmp/world-spring-injured-input-audit.log`.

Native animator and NPC object probe inputs match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, probes pass
245 frame/palette observations, 95 linear completions and ten final-frame hold
boundary checks across all five choices. They check palette binding, wrapper
idempotence, frame phase, cycle counters, completion state, West mirroring and
portrait synchronization. The action holds below the native minimum and
completes above its maximum. Natural injury-state dispatch, schedules,
interaction/outfit changes, pause policy and full-engine rendering are not
established. Evidence: `tmp/world-spring-injured-runtime.log`.

The gallery covers all 49 cases. Exact checks pass 7,996,800 gallery pixels and
326,400 summary pixels, including source/palette bindings, raw metadata, full
artwork bounds and West mirroring. Chromium passes all ten pages, fifty images
and their local links. See the [material notes](march-spring-injured-art.md) for
the preview records.
The review directory totals about 810 KiB. All eight links in the new notes
resolve to existing local files; see `tmp/world-spring-injured-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-spring-injured-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`f09c6e3520ae1eb2cf6d26debe77b77b7f0ce9713ad6afad1bdd75e929a80e52`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-spring-injured-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Begin Summer idle/walk coverage for Reina, Juniper and March, using their
completed Spring profiles and a fresh source inventory. Their other Summer
actions and later outfits remain for subsequent batches.
