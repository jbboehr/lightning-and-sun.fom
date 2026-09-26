# Reina, Juniper and March's Spring everyday actions

The user approved the [first world pilots](reina-juniper-march-world.md), committed
as `b5a43dc`. This slice adds Spring blinking, sitting, eating and drinking:
eleven strips per character, with 31 source frames and twelve native West
mirrors each. Together that is 33 strips and 129 offline review cases. The user approved this artwork for commit on 2026-09-25.

| Character | Total sources | Spring sprite coverage | Art notes |
| --- | ---: | ---: | --- |
| Reina | 120 | 17/38 | [Reina](reina-spring-actions.md) |
| Juniper | 149 | 17/41 | [Juniper](juniper-spring-actions.md) |
| March | 198 | 17/53 | [March](march-spring-actions.md) |

The combined 36-character trial has 2,951 sources and 11,804 variants. Home,
Page Down and U retain the existing five choices and Vanilla defaults. Each
character's portraits and supported overworld sprites share their selector.
Other actions/outfits remain original. No runtime Rust or GML behavior changed.

Reina reuses the accepted mapping. Juniper and March each append the exact
`#E8B171` drinking-arm shade as a singleton group, mapped to the existing midtone
role. Earlier source colors, groups, target roles, region objects and source
pins are unchanged. Portrait-only definitions remain available and unchanged.

Juniper has three known clothing boundary exceptions: eat South frames two and
three at `[40,46]` and `[41,45]`, and eat East frame five at `[39,44]`. Coordinates
are within each 80×80 frame. These edge pixels share connected source-color
components with exposed skin, so the current component masks recolor them too.
The character record and material test identify these exceptions explicitly;
other separate jewelry and clothing components retain their colors.

## Source and metadata

The mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Source pins remain strict; no hash was refreshed and no mismatch override used.
All strips have 80×80 frames, Default atlas and Middle/54 origin. Blinks have
three frames at `[0.075,0.125,0.075]` seconds. Sitting uses single-frame defaults;
drinking and eating North have three frames at one second. Eating East/South
has five frames at `[0.125,0.15,0.175,0.125,0.6]` seconds. Native seated flags and
eat/drink last-frame holds remain unchanged.

The shared synthetic package test first rejected unregistered Reina blink East,
then passed with all 33 paths registered. It checks complete animation
properties, PNG outputs, frame counts and existing character/hotkey bindings.
All author-exported sidecars match the independent archive read by asset path.
Evidence: `tmp/world-spring-actions-{package-red,package-checks,metadata-check}.log`
and `tmp/world-spring-actions-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-actions-preview/summary.png):
  South-facing drink frame two for all three characters.
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-actions-preview/index.html):
  all 93 source frames and 36 native West mirrors, grouped by character, with
  separate five-choice summaries.

Static previews do not establish natural NPC scheduling, automatic outfit
changes or live rendering. Source art, generated variants, previews, packages
and temporary helpers remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 130 active tests and the release
build passed. The normal suite leaves 136 local opt-in tests ignored; nine ran
separately and passed: the three new character corpus tests, three retained
world/portrait corpus tests and three GML tests. Character notes record the
new tests and their skin-omission/material-spill controls. Older opt-in tests
and live gameplay were not rerun. Shared evidence:
`tmp/world-spring-actions-final-checks.log` and
`tmp/world-spring-actions-{reina-retained,march-retained-early,juniper-gml-retained}.log`.

Retained tests use the expanded local corpora and source counts. March's pilot
assertions keep checking exactly the original two added roles and six original
world strips; the new test checks the appended drinking role and eleven strips.
The first full run exposed one remaining open-ended pilot-role assertion, which
was bounded to those original two roles before the focused and full reruns passed.
No earlier material or pixel assertion was removed.

All 11,804 combined variants pass exact recipe validation. All 4,340 accepted
Reina/Juniper/March original and variant PNG/metadata files remain byte-identical,
as do all 25,005 files for the other 33 characters. All 1,868 standalone variants
match the combined output; earlier runtime rows and controls are unchanged.
The definition audit preserves all 434 earlier region objects and every old
mapping/group entry. Only the two midtone aliases and 33 new registry/profile
entries are added; the collection is unchanged. All 36 export reports preserve
earlier source entries and the current archive hash. Evidence:
`tmp/world-spring-actions-comparison.{json,log}` and
`tmp/world-spring-actions-input-audit.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection,
probes pass 645 frame/direction observations, 225 linear completions and 240
hold checks across all five choices. They check West mirroring, wrapper
idempotence, frame phase and cycle counters. They do not establish natural
scheduling, automatic outfit dispatch, pause policy, attached prop rendering
or full-engine rendering. Evidence: `tmp/world-spring-actions-runtime.log`.

The combined summary passes fifteen source/palette bindings and 118,560 exact
displayed-pixel checks, including raw metadata and complete artwork bounds.
Character galleries independently pass exact-pixel, crop, metadata, West-reversal
and Chromium checks. All local links and images load on the combined landing
without horizontal overflow. The four review directories occupy about 1.94 MiB.
Evidence: `tmp/world-spring-actions-{summary-check,landing-browser}.log`, plus
the character records.

MOMI installation passed in the fresh isolated `tmp/world-spring-actions-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`b0d372b6b8dc94120f753a967e29c14a9ee4aa14fedb527c163840d33646e114`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-spring-actions-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Continue Spring general actions, sleeping and kissing for these three characters,
then their shocked reactions, reading and character-specific work/special actions.
Remaining source inventories are in `tmp/world-spring-actions-remaining.json`.
