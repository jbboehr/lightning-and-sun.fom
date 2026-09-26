# Reina, Juniper and March: shocked reactions and seated reading

The user approved the [general action, sleep and kiss batch](reina-juniper-march-standard.md),
committed as `48d4fc7`. This slice adds six Spring strips per character: the
start, loop and end phases of shocked reactions and seated reading. They contain
thirteen source frames each, totaling eighteen strips and 39 review cases.
Both native cycles only face South; no West mirrors are added. The user approved
the offline artwork for commit on 2026-09-25.

| Character | Total sources | Spring sprite coverage | Art notes |
| --- | ---: | ---: | --- |
| Reina | 131 | 28/38 | [Reina](reina-spring-reactions.md) |
| Juniper | 160 | 28/41 | [Juniper](juniper-spring-reactions.md) |
| March | 209 | 28/53 | [March](march-spring-reactions.md) |

The combined 36-character trial has 2,984 sources and 11,936 variants. Home,
Page Down and U retain the existing five choices and Vanilla defaults. Portraits
and supported overworld sprites share each character's selector. Remaining
special actions and other outfits retain their original sprites. No runtime
Rust or GML behavior changed.

Earlier region objects, source pins, color groups and target mappings remain
unchanged. The new masks distinguish exposed skin from clothing, jewelry and
the books. Character notes record the material decisions. Juniper's accepted
three eating-edge exceptions remain in the earlier slice.

## Source and metadata

The mounted read-only archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All pins remain strict; no hash refresh or mismatch override was used. All new
strips retain 80×80 frames, Default atlas and Middle/54 origin. Each shocked
phase has one frame using native defaults. Reading start/end each have three
frames at 0.1 seconds; the four-frame loop uses `[3.0,0.1,3.0,0.1]` seconds.
Both native cycles use start/loop/end transitions. The reading cycle retains
its seated flag.

The shared synthetic package test first rejected unregistered Reina reading
end, then passed after all eighteen exact paths were added. It checks complete
animation properties, frame counts, PNG outputs and existing character/hotkey
bindings. Evidence: `tmp/world-spring-reactions-package-{red,checks}.log` and
`tmp/world-spring-reactions-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-reactions-preview/summary.png):
  seated-reading loop frame one for all three characters.
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-reactions-preview/index.html):
  all 39 source frames, grouped by character, with separate five-choice summaries.

Static images do not establish animation timing, natural schedules, interaction
and outfit dispatch, attached effects or live rendering. Source art, generated
variants, previews, packages and temporary helpers remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 134 active tests and the release
build passed. The normal suite leaves 142 local opt-in tests ignored; fifteen
ran separately and passed: three new material corpus tests, nine retained
world/action corpus tests and three GML tests. Character notes record the new
tests and their missed-skin/material-spill controls. Retained tests change only
corpus paths and total source counts; earlier pixel/material assertions remain
intact. Older opt-in tests and live gameplay were not rerun. Shared evidence:
`tmp/world-spring-reactions-final-checks.log` and
`tmp/world-spring-reactions-{reina-retained,juniper-retained,march-gml-retained}.log`.

All eighteen author-exported raw sidecars match the independent archive read.
Every combined variant passes exact recipe validation: 11,936 variants across
36 characters. All 4,820 earlier Reina/Juniper/March original and variant
PNG/metadata files remain byte-identical, as do all 25,005 files for the other
33 characters. The 2,000 standalone variants match the combined outputs.
Previous runtime rows and controls are unchanged. Evidence:
`tmp/world-spring-reactions-metadata-check.log` and
`tmp/world-spring-reactions-comparison.{json,log}`.

The definition audit preserves all 482 earlier region objects and every source
color, color group and target mapping. The three sets, collection and
portrait-only recipes are unchanged. Exactly eighteen registry/profile paths
are added. All 36 export reports preserve earlier source entries and the current
archive hash. Evidence: `tmp/world-spring-reactions-input-audit.log`.

Native animator and NPC object probe inputs match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, probes pass
195 frame/palette observations and 120 start/loop/end transitions across all
five choices. They check palette binding, wrapper idempotence, frame phase,
cycle counters, completion state and portrait synchronization. Only South is
exercised for these cycles. Natural scheduling, interaction/outfit dispatch,
pause policy, attached effects and full-engine rendering are not established.
Evidence: `tmp/world-spring-reactions-runtime.log`.

The combined summary passes fifteen source/palette bindings and 142,560 exact
displayed-pixel checks, including raw metadata and complete artwork bounds.
Character galleries independently check source/palette bindings, exact pixels,
crops and metadata. Chromium checks each gallery and the combined landing;
local links and images load without horizontal overflow. The four review
directories occupy about 0.87 MiB. Evidence:
`tmp/world-spring-reactions-{summary-check,landing-browser}.log`, plus character
records.

MOMI installation passed in the fresh isolated `tmp/world-spring-reactions-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`3ead8a169b739e64694e804936b22aeb1ad601d4ce9c9cb6222a288681933a98`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-spring-reactions-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Continue character-specific Spring work and special animations: Reina's
writing and kitchen work, Juniper's gestures and magic, and March's smithing,
poses and injured animations. Remaining inventories are in
`tmp/world-spring-reactions-remaining.json`.
