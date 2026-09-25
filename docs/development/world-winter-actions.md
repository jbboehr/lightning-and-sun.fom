# Winter actions and Autumn gardening

The user approved the [Winter expansion](world-winter-expansion.md), committed
as `1133994`. This slice adds 25 strips, 73 source frames and 39 native West
mirrors. The user approved this slice for commit on 2026-09-25.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Winter blink, sit, eat and drink | 11 | 31 | 12 | 263 |
| Celine | Complete Autumn garden outfit | 14 | 42 | 27 | 328 |

Celine now covers all 46 Autumn strips, including the fourteen garden strips.
Hayden covers seventeen of thirty Winter strips; Ryis's complete Winter coverage
is retained. F8 and Insert keep their five choices and Vanilla defaults. No
runtime Rust or GML behavior changed. Character material decisions and focused
checks are in [Hayden](hayden-winter-actions.md) and
[Celine](celine-winter-actions.md).

## Source and metadata

The mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding complete archive. New source pins were added; no
existing hash was refreshed. Generation uses strict hashes, without
`--allow-source-hash-mismatch`.

Independent archive metadata and both author exports agree. All 25 strips
retain 80×80 frames, Default atlas and Middle/54 origin, including all native
frame durations. Current native cycle configuration supports Winter for Hayden
and `autumn_garden` for Celine. Hayden's eating and drinking retain seated poses
and random final-frame holds of `[240,360]`. Celine's harvesting retains its ten
variable-duration frames. The synthetic package regression first failed on
Hayden's unregistered Winter blink path, then passed with the 25 exact paths
registered. It checks complete animation properties, source frames, PNG outputs
and controls. Evidence: `tmp/world-winter-actions-package-{red,checks}.log`,
`tmp/world-winter-actions-inventory.log` and
`tmp/world-winter-actions-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/world-winter-actions-preview/summary.png):
  Hayden drinking and Celine watering in her garden outfit.
- [Complete Vanilla/Debug Blue review](../../generated/world-winter-actions-preview/index.html):
  all 112 frame/direction cases, paginated by character, plus separate
  five-choice summaries.

Every new source frame was inspected in all four targets. The combined summary
passes ten source/palette bindings, native metadata, complete-art crops and
262,400 exact output pixel comparisons. Character previews cover every source
frame and native West reversal. Chromium checks images, local links and
horizontal overflow. Evidence: `tmp/world-winter-actions-summary-check.log`,
`tmp/world-winter-actions-landing-browser.log` and the character preview records.

Static previews do not establish natural NPC schedules, automatic outfit
changes, separately drawn props or live rendering. Extracted artwork, previews,
packages and isolated installs remain ignored.

## Verification

Formatting, Clippy, all 115 active tests and the release build pass through the
pure Nix shell; 109 opt-in tests are ignored by the normal suite. Separately,
both new character corpus tests, both retained `*_world` corpus tests and three
pinned Fabricator GML tests pass. Other historical opt-in corpus tests, the
legacy replacement-install test and live gameplay were not rerun. Evidence:
`tmp/world-winter-actions-final-checks.log`,
`tmp/world-winter-actions-retained-{hayden,celine}.log`,
`tmp/world-winter-actions-gml-checks.log` and the character test logs.

Probes execute the current archive's actual animator, sprite-pack constructors
and NPC `animate()` method with the shipped palette wrapper. All 112 cases pass
across five choices: 560 palette observations, 180 linear completions and eighty
eat/drink hold-boundary checks. Palette changes preserve fractional phase, loop
counters, West reversal, portrait selection and wrapper identity. Engine
services and sprite information are simulated; cycles and loop limits are
selected directly. This does not exercise natural schedules, the full NPC state
machine, automatic outfit dispatch, audible sound or live gameplay. Evidence:
`tmp/world-winter-actions-{hayden,celine}-runtime.{gml,log}`.

All 11,068 variants validate across 2,767 sources and 36 characters. The combined
bundle matches all 2,364 standalone variants for these two characters. Their
5,660 previous original/variant PNG and metadata files are byte-identical;
all 21,930 files for the other thirty-four characters are byte-identical,
including reports. Earlier runtime rows and controls, all 566 previous region
objects, color groups and palette mappings are unchanged. The 34 retained test
edits only update fixture paths, ignore descriptions and two source counts.
Evidence: `tmp/world-winter-actions-{comparison.json,input-audit.log}`.

A fresh isolated install passes MOMI v0.16.4 strict lints, required compilation
and installed-pixel/metadata verification. The lab is
`tmp/world-winter-actions-playtest`; its installed archive SHA-256 is
`996ba4cd2b2d1975a2285600c66dec889bc1e37566cddee008b5dbdd161d56c1`.
The mounted source and lab's preserved `previous.zip` retain the source hash
above. The live game and desktop launcher were not changed. Evidence:
`tmp/world-winter-actions-install-report.json` and
`tmp/world-winter-actions-source-after.sha256`.

Eight frozen palette/registry input hashes remain unchanged after generation
and installation (`tmp/world-winter-actions-frozen-{inputs.sha256,check.log}`).

## Next coverage

Continue the seasonal plan with Hayden's Winter general actions, sleep and kiss,
then the remaining Winter specials. Celine's complete Autumn folder is done;
her Winter idle/walk pilot follows. Ryis's next outfit can be selected separately.
Folder evidence is `tmp/world-winter-actions-remaining.json`.
