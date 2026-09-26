# Beach idle and walk pilot

The user approved the [Winter completion batch](world-winter-finish.md),
committed as `43670ea`. This slice starts Beach coverage for Hayden, Ryis and
Celine with idle/walk North, South and East. Each character adds six strips,
fifteen source frames and five native West mirrors. Together they give sixty
offline review cases. The user approved the artwork for commit on 2026-09-25.

| Character | Total sources | Beach coverage | Art notes |
| --- | ---: | ---: | --- |
| Hayden | 282 | 6/14 | [Hayden](hayden-beach-pilot.md) |
| Ryis | 250 | 6/14 | [Ryis](ryis-beach-pilot.md) |
| Celine | 367 | 6/14 | [Celine](celine-beach-pilot.md) |

The combined 36-character trial has 2,831 sources and 11,324 variants. All three
retain complete Spring, Summer, Autumn and Winter sprite folders. F8, F10 and
Insert keep their five choices and Vanilla defaults. No runtime Rust or GML
behavior changed. The character notes describe exposed skin, body-hair and
swimwear boundaries and their focused checks.

## Source and metadata

The read-only mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding archive. New source pins are strict; no previous hash
was refreshed and generation does not use `--allow-source-hash-mismatch`.

Independent archive metadata and all three author exports agree by asset path.
All eighteen strips retain 80×80 frames, Default atlas and Middle/54 origin.
Idle uses single-frame defaults; walking has four frames at 0.15 seconds each.
Current native idle/walk cycles include the Beach outfit. The synthetic package
regression first failed on unregistered Hayden Beach idle East, then passed
after registering all eighteen paths. It checks complete animation properties,
frame counts, PNG outputs and the existing character bindings. Evidence:
`tmp/world-beach-pilot-package-{red,checks}.log`,
`tmp/world-beach-pilot-{inventory,metadata-check}.log` and
`tmp/world-beach-pilot-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/world-beach-pilot-preview/summary.png):
  one South-facing walking sample per character.
- [Complete Vanilla/Debug Blue review](../../generated/world-beach-pilot-preview/index.html):
  all sixty cases, paginated by character, with separate five-choice summaries.

Static previews do not establish natural NPC scheduling, automatic outfit
changes or live rendering. Game-derived files, previews, packages and temporary
helpers remain ignored.

The combined summary passes fifteen source/palette bindings and 136,400 exact
displayed-pixel comparisons, including raw archive metadata and complete opaque
artwork crops. Each character gallery passes its exact-pixel, West-reversal and
Chromium checks; the combined landing also loads all local links/images without
horizontal overflow. Evidence: `tmp/world-beach-pilot-summary-check.log` and
`tmp/world-beach-pilot-landing-browser.log`, plus the character records.

## Verification and installation

Formatting, Clippy with warnings denied, all 120 active tests and the release
build passed. The normal suite leaves 118 local opt-in tests ignored; nine were
run separately and passed: three new character corpus tests, three retained
world corpus tests and three GML runtime tests. Material tests exercise omitted
skin and spills into clothing, hair or the straw hat. Celine's focused test
caught a detached thigh-shadow pixel; the corrected recipe passed before the
final combined build and installation. Her retained test was rerun afterward.
Older opt-in tests and live gameplay were not rerun. Shared logs:
`tmp/world-beach-pilot-{final-checks,retained-world,retained-celine-final,gml-checks}.log`;
character-specific evidence is linked in the art notes.

All 11,324 combined variants passed exact recipe validation. All 8,810 accepted
Hayden/Ryis/Celine PNG and metadata files and all 19,485 files belonging to the
other 33 characters remain byte-identical. The 3,596 standalone variants match
the combined output. Existing runtime rows, controls, region objects and color
maps are unchanged. The 59 retained corpus-test edits only update paths and
totals. Evidence: `tmp/world-beach-pilot-{comparison.json,input-audit.log}`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection,
probes passed 300 frame/direction observations and 120 linear completions across
all five choices. They check West mirroring, palette wrapper idempotence, frame
phase and loop-counter preservation. These probes do not establish natural NPC
scheduling, automatic outfit dispatch or full-engine state transitions.
Evidence: `tmp/world-beach-pilot-{native-source,runtime}.log`.

MOMI v0.16.4 installation passed in the fresh isolated
`tmp/world-beach-pilot-playtest` lab, including strict lints, required compilation,
installed pixels and animation metadata verification. Installed archive SHA-256:
`d4b4533b584f91afa82bb60d0c6778bcfe6f6ea7cf6fa9dbc61633f2f7b94b27`.
The mounted archive and lab's `previous.zip` retain the source hash above. All
eleven frozen registry/recipe inputs remained unchanged through final generation
and installation. Evidence:
`tmp/world-beach-pilot-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Continue Beach coverage with blink East/South, general actions North/South/East
and kiss East for these three characters. Each then has two bathing/swimming
strips left to inspect separately. The current folder inventory is
`tmp/world-beach-pilot-remaining.json`. Wedding and Hayden's Shadow sprites
remain outside this slice.
