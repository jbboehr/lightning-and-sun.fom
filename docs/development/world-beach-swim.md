# Beach bathing and swimming

The user approved the [Beach action batch](world-beach-actions.md), committed
as `de4d0f3`. This slice finishes Hayden, Ryis and Celine's Beach sprite folders
with bathing/swimming East and South. Each adds two strips, eight source frames
and four native West mirrors: six strips and 36 review cases together. The user approved this artwork for commit on 2026-09-25.

| Character | Total sources | Beach coverage | Art notes |
| --- | ---: | ---: | --- |
| Hayden | 290 | 14/14 | [Hayden](hayden-beach-swim.md) |
| Ryis | 258 | 14/14 | [Ryis](ryis-beach-swim.md) |
| Celine | 375 | 14/14 | [Celine](celine-beach-swim.md) |

The combined 36-character trial has 2,855 sources and 11,420 variants. These
three characters now cover their complete Spring, Summer, Autumn, Winter and
Beach sprite folders. F8, F10 and Insert retain five choices and Vanilla defaults.
No runtime Rust or GML behavior changed. Character notes describe the exposed
skin and the preserved water effects, hair and eyes.

## Source and metadata

The mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding slice. Source pins remain strict; no old hash was
refreshed and generation does not use `--allow-source-hash-mismatch`.

Independent archive metadata and all three author exports agree by asset path.
All six strips retain four 80×80 frames at 0.15 seconds per frame, Default atlas
and Middle/54 origin. The native `bath_swim` cycle is linear, supports South and
East, defaults to South and uses only the Beach outfit.

The synthetic package test first failed on unregistered Hayden swimming East,
then passed after registration. It checks complete animation properties, frame
counts, PNG outputs and existing character bindings. Evidence:
`tmp/world-beach-swim-package-{red,checks}.log`,
`tmp/world-beach-swim-{inventory,metadata-check}.log` and
`tmp/world-beach-swim-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/world-beach-swim-preview/summary.png):
  South-facing swimming frame two for each character.
- [Complete Vanilla/Debug Blue review](../../generated/world-beach-swim-preview/index.html):
  all 24 source frames and twelve native West mirrors, grouped by character,
  with separate five-choice summaries.

Static previews do not establish natural NPC scheduling, entering or leaving
water, automatic outfit changes or live rendering. Source art, generated
variants, previews, packages and temporary helpers remain ignored.

Swimming heads sit lower in the source frames than standing sprites. Every
preview crop includes the full head and detached splashes. The combined summary
passes fifteen source/palette bindings and 77,600 exact displayed-pixel checks,
including raw archive metadata and complete opaque artwork bounds. All local
links and images load in Chromium without horizontal overflow. Character notes
record their full-gallery checks, exact pixels and West reversals. The combined
landing and all three preview directories occupy about 792 KiB. Evidence:
`tmp/world-beach-swim-{summary-check,landing-browser}.log`.

## Verification

Formatting, Clippy with warnings denied, all 122 active tests and the release
build passed. The normal suite leaves 124 local opt-in tests ignored; nine were
run separately and passed: three new character corpus tests, three retained
world corpus tests and three GML runtime tests. Material controls exercise
missing face/neck components and spills into hair, water or foam. Final Hayden
and Celine recipes match their passing candidates exactly. Older opt-in tests
and live gameplay were not rerun. Shared evidence:
`tmp/world-beach-swim-{final-checks,retained-world,gml-checks}.log`; the character
notes link their focused tests and controls.

All 11,420 combined variants passed exact recipe validation. All 9,170 accepted
Hayden/Ryis/Celine PNG and metadata files and all 19,485 files belonging to the
other 33 characters remain byte-identical. All 3,692 standalone variants match
the combined output. Existing runtime rows, controls, 917 old region objects
and color maps are unchanged. The 65 retained corpus-test edits only update
fixture paths and totals. Evidence:
`tmp/world-beach-swim-{comparison.json,input-audit.log}`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, probes
passed 180 frame/direction observations and 45 linear completions across all five
choices. They check West mirroring, palette wrapper idempotence, frame phase and
cycle-counter preservation. These probes do not establish natural scheduling,
entering/leaving water, outfit dispatch or full-engine rendering. Evidence:
`tmp/world-beach-swim-{native-source,runtime}.log`.

MOMI installation passed in the fresh isolated
`tmp/world-beach-swim-playtest` lab, including required compilation and
installed-pixel/animation-metadata verification. Installed archive SHA-256:
`fb2147dacbfc1cfde925c1964164fcb20889876b6595be1909a49cabaa512bf6`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-beach-swim-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Start Wedding idle/walk for these three characters. Their Wedding folders have
fifteen uncovered strips each: idle, walk, blink, sit, general action and kiss.
Hayden also has 29 Shadow strips left. This inventory is recorded in
`tmp/world-beach-swim-uncovered-sprites.json`; Beach has no remaining strips
in `tmp/world-beach-swim-remaining.json`.
