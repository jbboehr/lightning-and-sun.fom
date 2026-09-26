# Reina, Juniper and March: general actions, sleeping and kissing

The user approved the [Spring everyday actions](reina-juniper-march-actions.md),
committed as `08c8d1a`. This slice adds five Spring strips per character:
general action North/South/East, sleep East and kiss East. They contain
26 source frames and twelve native West mirrors each, totaling fifteen strips
and 114 review cases. The user approved this artwork for commit on 2026-09-25.

| Character | Total sources | Spring sprite coverage | Art notes |
| --- | ---: | ---: | --- |
| Reina | 125 | 22/38 | [Reina](reina-spring-standard.md) |
| Juniper | 154 | 22/41 | [Juniper](juniper-spring-standard.md) |
| March | 203 | 22/53 | [March](march-spring-standard.md) |

The combined 36-character trial has 2,966 sources and 11,864 variants. Home,
Page Down and U retain the existing five choices and Vanilla defaults. Portraits
and supported overworld sprites share each character's selector. Remaining
special actions and other outfits retain their original sprites. No runtime
Rust or GML behavior changed.

The new masks reuse existing colors and target roles. Earlier region objects,
source pins, color groups, maps and portrait-only definitions are preserved.
Juniper's three accepted eating-edge exceptions remain documented in the
previous slice; these five strips are inspected separately.

## Source and metadata

The read-only archive remains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All source pins remain strict; no hash refresh or mismatch override was used.
All new strips use 80×80 frames, Default atlas and Middle/54 origin. General
actions have seven frames at `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` seconds;
kissing has four at `[0.15,0.15,0.8,0.15]`; sleeping uses single-frame defaults.
Native action holds and speaking/pause behavior remain unchanged.

The shared synthetic package test first rejected unregistered Reina action
East, then passed after adding all fifteen exact paths. It checks complete
animation properties, frame counts, PNG outputs and existing character/hotkey
bindings. Evidence: `tmp/world-spring-standard-package-{red,checks}.log` and
`tmp/world-spring-standard-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-standard-preview/summary.png):
  South-facing general action frame four for all three characters.
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-standard-preview/index.html):
  all 78 source frames and 36 native West mirrors, grouped by character, with
  separate five-choice summaries.

Static images do not establish animation timing, natural NPC scheduling,
interaction/outfit dispatch, attached props or live rendering. Source art,
generated variants, previews, packages and temporary helpers remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 132 active tests and the release
build passed. The normal suite leaves 139 local opt-in tests ignored; twelve
ran separately and passed: three new material corpus tests, six retained
world/action corpus tests and three GML tests. Character notes record the new
tests and their missed-skin/material-spill controls. Retained tests change only
corpus paths and total source counts; earlier pixel/material assertions remain
intact. Older opt-in tests and live gameplay were not rerun. Shared evidence:
`tmp/world-spring-standard-final-checks.log` and
`tmp/world-spring-standard-{reina-retained,juniper-retained,march-gml-retained}.log`.

All fifteen author-exported raw sidecars match the independent archive read.
Every combined variant passes exact recipe validation: 11,864 variants across
36 characters. All 4,670 earlier Reina/Juniper/March original and variant
PNG/metadata files remain byte-identical, as do all 25,005 files for the other
33 characters. The 1,928 standalone variants match the combined outputs.
Previous runtime rows and controls are unchanged. Evidence:
`tmp/world-spring-standard-metadata-check.log` and
`tmp/world-spring-standard-comparison.{json,log}`.

The definition audit preserves all 467 earlier region objects and every source
color, color group and target mapping. The three sets, collection and
portrait-only recipes are unchanged. Exactly fifteen registry/profile paths
are added. All 36 export reports preserve the earlier source entries and current
archive hash. Evidence: `tmp/world-spring-standard-input-audit.log`.

Native animator and NPC object probe inputs match the mounted archive byte for
byte. With simulated engine services and direct cycle selection, probes pass
570 frame/direction observations, 120 linear completions and 120 action-hold
checks across all five choices. They check West mirroring, wrapper idempotence,
frame phase, cycle counters and portrait synchronization. They do not establish
natural scheduling, interaction/outfit dispatch, speaking-triggered action-to-idle
transitions, pause policy, attached props or full-engine rendering. Evidence:
`tmp/world-spring-standard-runtime.log`.

The combined summary passes fifteen source/palette bindings and 136,800 exact
displayed-pixel checks, including raw metadata and complete artwork bounds.
Each character gallery independently passes exact-pixel, crop, metadata,
West-reversal and Chromium checks. All local links and images load on the
combined landing without horizontal overflow. The four review directories
occupy about 1.73 MiB. Evidence:
`tmp/world-spring-standard-{summary-check,landing-browser}.log`, plus character
records.

MOMI installation passed in the fresh isolated `tmp/world-spring-standard-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`99b10889f21dde2be0aaceb2ba50aacbfb7bcb10e714c2984056122cad8c1cce`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-spring-standard-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Continue Spring shocked reactions and seated reading, then character-specific
work and special actions. Remaining inventories are in
`tmp/world-spring-standard-remaining.json`.
