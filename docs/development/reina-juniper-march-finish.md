# Reina, Juniper and March: Spring kitchen work, magic and gestures

The user approved the [writing, laugh/charm and smithing batch](reina-juniper-march-specials.md),
committed as `dcb980b`. This slice adds seventeen Spring strips: Reina's chopping
and polishing, Juniper's remaining gestures and spell casting, and March's
gestures and poses. The user approved the offline artwork for commit on
2026-09-26.

| Character | New strips | Source frames | West mirrors | Total sources | Spring coverage | Art notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Reina | 4 | 29 | 9 | 141 | 38/38 | [Reina](reina-spring-finish.md) |
| Juniper | 7 | 29 | 6 | 173 | 41/41 | [Juniper](juniper-spring-finish.md) |
| March | 6 | 13 | 1 | 220 | 39/53 | [March](march-spring-finish.md) |

Reina and Juniper's Spring sprite folders are complete. March's remaining
fourteen strips are injured animations; his normal Spring outfit is complete.
The combined 36-character trial has 3,018 sources and 12,072 variants. Home,
Page Down and U retain their existing five choices and Vanilla defaults.
Portraits and supported overworld sprites share each character's selector.
Overworld sprites in other outfits and March's injured sprites retain their
original appearance.
No runtime Rust or GML behavior changed.

The masks use existing source colors and target roles. Reina's jacket folds,
glass, cloth, knife and chopping effects keep their original appearance.
March's crossed-arm skin changes while his shirt, apron, goggles, expression
accents and mouth interiors stay original. Juniper's moving hair exposes dark
forehead skin above the circlet, which is included. Seven new hair-flip
bracer-edge pixels share components with her wrist/arm and recolor with the
skin; their exact coordinates are documented and tested in her character note.
The six accepted eating/laugh boundary exceptions remain unchanged.

## Source and metadata

The read-only source archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Strict source pins remain enabled; no hash refresh or mismatch override was used.
Every new frame is 80×80 in the Default atlas with numeric origin 40/54.
The raw sidecars retain all mixed duration arrays and single-frame defaults.
Reina's twenty-frame chopping strip includes its full timing sequence.

Polishing and spell casting use start/loop/end cycles; the other native cycles
are linear. Chopping faces North, polishing and gremlin face East with native
West mirrors, and March's pose supports all four directions. Other new gestures
face South. Chopping and spell casting retain their native sound configuration,
including spell casting's detached sound. Evidence:
`tmp/world-spring-finish-native-inputs.json` and the three raw sidecar inventories.

The synthetic package test first rejected unregistered Reina chopping, then
passed after all seventeen exact paths were registered. It checks complete
animation properties, frame counts, PNG outputs and existing character/hotkey
bindings. Evidence: `tmp/world-spring-finish-package-{red,checks}.log`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-finish-preview/summary.png):
  samples of polishing, spell casting and March's grumpy gesture.
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-finish-preview/index.html):
  all 71 source frames and sixteen native West mirrors, grouped by character.

Each character also has a separate five-choice summary. Static images cover
recoloring; they do not establish natural schedules, interaction/outfit dispatch,
attached effects, sound or live rendering. Game-derived images, generated
variants, previews and packages stay ignored.

## Verification

Formatting, Clippy with warnings denied, all 138 active tests and the release
build passed. The normal suite leaves 148 local opt-in tests ignored; twenty-one
ran separately and passed: three new material corpus tests, fifteen retained
world/action corpus tests and three GML tests. Character notes record the new
tests and their practical missed-skin/material-spill controls. Retained tests
change only corpus paths and total source counts; earlier pixel assertions stay
intact. Older opt-in tests and live gameplay were not rerun. Evidence:
`tmp/world-spring-finish-final-checks.log` and
`tmp/world-spring-finish-{reina-retained,juniper-retained,march-gml-retained}.log`.

All seventeen author-exported raw sidecars match the independent archive read.
Every combined variant passes exact recipe validation: 12,072 variants across
36 characters. All 5,170 earlier Reina/Juniper/March original and variant
PNG/metadata files remain byte-identical, as do all 25,005 files for the other
33 characters. The 2,136 standalone variants match the combined outputs.
Previous runtime rows and controls are unchanged. Evidence:
`tmp/world-spring-finish-metadata-check.log` and
`tmp/world-spring-finish-comparison.{json,log}`.

The definition audit preserves all 517 earlier region objects and every source
color, color group and target mapping. The three sets, collection and portrait-only
recipes are unchanged. Exactly seventeen registry/profile paths are added.
All 36 export reports preserve earlier source entries and the current archive
hash. Evidence: `tmp/world-spring-finish-input-audit.log`.

Native animator and NPC object probe inputs match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, probes pass
435 frame/palette observations, forty complex transitions and 65 linear
completions across all five choices. They check palette binding, wrapper
idempotence, frame phase, cycle counters, completion state, West mirroring and
portrait synchronization. Complex transitions exercise each cycle's default
direction; direct frame observations also cover the West polishing mirror.
Natural scheduling, interaction/outfit dispatch, pause policy, sound, attached
effects and full-engine rendering are not established. Evidence:
`tmp/world-spring-finish-runtime.log`.

The combined summary passes fifteen source/palette bindings and 149,200 exact
displayed-pixel checks, including raw metadata and complete artwork bounds.
Each character's gallery independently checks source/palette bindings, exact
pixels, crops, metadata and native mirrors. Chromium checks all character
galleries and the combined landing; local links and images load without
horizontal overflow. Review links in the previous batch's notes were also
corrected to resolve from `docs/development`. Evidence:
`tmp/world-spring-finish-{summary-check,landing-browser}.log`, plus character
records.
The four review directories total about 1.5 MiB. All 24 local links in the new
character/shared notes and corrected prior notes resolve successfully; see
`tmp/world-spring-finish-doc-links.log`.

MOMI installation passed in the fresh isolated `tmp/world-spring-finish-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`b7d6537edad7b1db3196401dff5bf858c0bdbfad7acdf2c3bc8a221812fc9b2b`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-spring-finish-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Finish March's fourteen Spring injured strips: idle and walk in three source
directions, blink in two, seated poses in three, seated blink in two, and one
general action. Then expand Reina, Juniper and March into the other outfits.
The remaining Spring inventory is `tmp/world-spring-finish-remaining.json`.
