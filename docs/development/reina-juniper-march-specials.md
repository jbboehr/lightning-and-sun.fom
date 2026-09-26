# Reina, Juniper and March: writing, laugh, charm and smithing

The user approved the [shocked and seated-reading batch](reina-juniper-march-reactions.md),
committed as `51a5969`. This slice adds seventeen Spring strips: Reina's standing
and seated writing cycles, Juniper's laugh and charm cycles, and March's seated
work cycle, hammering and brow-wiping. The user approved the offline artwork
for commit on 2026-09-26.

| Character | New strips | Source frames | West mirrors | Total sources | Spring coverage | Art notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Reina | 6 | 16 | 0 | 137 | 34/38 | [Reina](reina-spring-specials.md) |
| Juniper | 6 | 14 | 0 | 166 | 34/41 | [Juniper](juniper-spring-specials.md) |
| March | 5 | 23 | 7 | 214 | 33/53 | [March](march-spring-specials.md) |

The combined 36-character trial has 3,001 sources and 12,004 variants. Home,
Page Down and U retain their existing five choices and Vanilla defaults.
Portraits and supported overworld sprites share each character's selector.
Remaining special actions and other outfits retain their original sprites.
No runtime Rust or GML behavior changed.

Earlier masks, source pins, color groups and target mappings remain unchanged.
Reina's pencil, paper, clipboard and jacket folds retain their original colors;
March's wood, metal, swing trail and sparks do too. Juniper has three additional
laugh-frame bracer-edge pixels connected to the selected wrist component. They
recolor with the wrist, as documented and tested in her character note. Her three
previously accepted eating-edge exceptions remain unchanged.

## Source and metadata

The read-only source archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Strict source pins remain enabled; no hash refresh or mismatch override was used.
All new frames are 80×80 in the Default atlas with vertical origin 54. Standing
writing, laugh/charm, hammer and brow-wiping use numeric horizontal origin 40;
seated writing/work use Middle. The exact raw sidecars, including mixed duration
arrays and default single-frame timing, are retained.

Writing, laugh and charm face South. March's seated work faces North; hammer
faces East with native West mirroring; brow-wiping faces South. The five complex
cycles retain start/loop/end transitions, including seated flags. Hammer retains
its native sound configuration. Evidence:
`tmp/world-spring-specials-native-inputs.json` and the three raw metadata exports.

The synthetic package test first rejected unregistered Reina writing end, then
passed after all seventeen exact paths were registered. It checks complete
animation properties, frame counts, PNG outputs and existing character/hotkey
bindings. Evidence: `tmp/world-spring-specials-package-{red,checks}.log`.

## Offline review

- [Compact five-choice summary](../../../generated/reina-juniper-march-specials-preview/summary.png):
  samples of writing, charm and brow-wiping.
- [Complete Vanilla/Debug Blue review](../../../generated/reina-juniper-march-specials-preview/index.html):
  all 53 source frames and seven native West hammer mirrors, grouped by character.

Each character also has a separate five-choice summary. These static images
cover recoloring, including tools and props; they do not establish natural
schedules, interaction/outfit dispatch, attached effects, sound or live rendering.
Game-derived images, generated variants, previews and packages stay ignored.

## Verification

Formatting, Clippy with warnings denied, all 136 active tests and the release
build passed. The normal suite leaves 145 local opt-in tests ignored; eighteen
ran separately and passed: three new material corpus tests, twelve retained
world/action corpus tests and three GML tests. Character notes record the new
tests and their practical missed-skin/material-spill controls. Retained tests
change only corpus paths and total source counts; earlier pixel assertions stay
intact. Older opt-in tests and live gameplay were not rerun. Evidence:
`tmp/world-spring-specials-final-checks.log` and
`tmp/world-spring-specials-{reina-retained,juniper-retained,march-gml-retained}.log`.

All seventeen author-exported raw sidecars match the independent archive read.
Every combined variant passes exact recipe validation: 12,004 variants across
36 characters. All 5,000 earlier Reina/Juniper/March original and variant
PNG/metadata files remain byte-identical, as do all 25,005 files for the other
33 characters. The 2,068 standalone variants match the combined outputs.
Previous runtime rows and controls are unchanged. Evidence:
`tmp/world-spring-specials-metadata-check.log` and
`tmp/world-spring-specials-comparison.{json,log}`.

The definition audit preserves all 500 earlier region objects and every source
color, color group and target mapping. The three sets, collection and portrait-only
recipes are unchanged. Exactly seventeen registry/profile paths are added.
All 36 export reports preserve earlier source entries and the current archive
hash. Evidence: `tmp/world-spring-specials-input-audit.log`.

Native animator and NPC object probe inputs match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, probes pass
300 frame/palette observations, 100 complex transitions and fifteen linear
completions across all five choices. They check palette binding, wrapper
idempotence, frame phase, cycle counters, completion state, West mirroring and
portrait synchronization. Natural scheduling, interaction/outfit dispatch,
pause policy, sound, attached effects and full-engine rendering are not
established. Evidence: `tmp/world-spring-specials-runtime.log`.

The combined summary passes fifteen source/palette bindings and 140,080 exact
displayed-pixel checks, including raw metadata and complete artwork bounds.
Each character's gallery independently checks source/palette bindings, exact
pixels, crops and metadata. Chromium checks the character galleries and combined
landing; local links and images load without horizontal overflow. Evidence:
`tmp/world-spring-specials-{summary-check,landing-browser}.log`, plus character
records.
The four review directories total about 1.2 MiB.

MOMI installation passed in the fresh isolated `tmp/world-spring-specials-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`72e98f99870896c6f110e42090555bfac9873064cb6ee1883925abc5d4c3c24e`.
The mounted archive and the lab's `previous.zip` retain the source
hash above. All eleven frozen registry/recipe inputs remained unchanged through
generation and installation. Evidence:
`tmp/world-spring-specials-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Reina has chopping and the polishing cycle left in Spring (four strips).
Juniper has pose, hair flip, spell casting, gremlin and snooze left (seven).
March has gestures/poses and injured animations left (twenty).
The remaining source inventory is `tmp/world-spring-specials-remaining.json`.
