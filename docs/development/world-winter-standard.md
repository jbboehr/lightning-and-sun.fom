# Winter general actions and idle/walk

The user approved the [Winter action batch](world-winter-actions.md), committed
as `4baa5ce`. This slice adds eleven strips, 41 source frames and seventeen
native West mirrors. The user approved this slice for commit on 2026-09-25.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Winter general actions, sleep and kiss | 5 | 26 | 12 | 268 |
| Celine | Winter idle and walk | 6 | 15 | 5 | 334 |

Hayden now covers 22 of thirty Winter strips. Celine covers six of 33 Winter
strips, retaining her complete Autumn folder. Ryis's complete Winter coverage
is unchanged. F8 and Insert keep their five choices and Vanilla defaults. No
runtime Rust or GML behavior changed. Material decisions and focused checks are
in [Hayden](hayden-winter-standard.md) and [Celine](celine-winter-standard.md).

## Source and metadata

The mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding complete archive. New source pins were added; no
existing hash was refreshed. Generation uses strict hashes, without
`--allow-source-hash-mismatch`.

Independent archive metadata and both author exports agree. All eleven strips
retain 80×80 frames, Default atlas and Middle/54 origin, with their original
frame durations. The current native cycles support both characters' Winter
outfits. Hayden's general actions retain random final-frame holds of `[240,360]`.
Celine's walking retains four frames at 0.15 seconds each; idle retains single
frames and default timing. The synthetic package regression first failed on
Hayden's unregistered Winter action path, then passed with the eleven paths
registered. It checks complete animation properties, frame counts, PNG outputs
and controls. Evidence: `tmp/world-winter-standard-package-{red,checks}.log`,
`tmp/world-winter-standard-inventory.log` and
`tmp/world-winter-standard-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/world-winter-standard-preview/summary.png):
  Hayden's South general action and Celine walking South.
- [Complete Vanilla/Debug Blue review](../../generated/world-winter-standard-preview/index.html):
  all 58 frame/direction cases, paginated by character, plus separate
  five-choice summaries.

Every new source frame was inspected in all four targets. The combined summary
passes ten source/palette bindings, native metadata, complete-art crops and
262,400 exact output pixel comparisons. Character previews cover every source
frame and native West reversal. Chromium checks images, local links and
horizontal overflow on every review page and the combined landing. Evidence:
`tmp/world-winter-standard-summary-check.log`,
`tmp/world-winter-standard-landing-browser.log` and the character preview records.

Static previews do not establish natural NPC schedules, automatic outfit
changes, separately drawn props or live rendering. Extracted artwork, previews,
packages and isolated installs remain ignored.

## Verification

Formatting, Clippy, all 116 active tests and the release build pass through the
pure Nix shell; 111 opt-in tests are ignored by the normal suite. Separately,
both new character corpus tests, both retained `*_world` corpus tests and three
pinned Fabricator GML tests pass. Other historical opt-in corpus tests, the
legacy replacement-install test and live gameplay were not rerun. Evidence:
`tmp/world-winter-standard-final-checks.log`,
`tmp/world-winter-standard-retained-{hayden,celine}.log`,
`tmp/world-winter-standard-gml-checks.log` and the character test logs.

The first full run could not start one temporary installer test runner because
of `Text file busy` (OS error 26). Its focused retry and a fresh full default
suite passed without code changes. The first failure and retry are retained in
`tmp/world-winter-standard-{first-checks,installer-retry}.log`.

Probes execute the current archive's actual animator, sprite-pack constructors
and NPC `animate()` method with the shipped palette wrapper. All 58 cases pass
across five choices: 290 palette observations, eighty linear completions and
forty general-action hold-boundary checks. Palette changes preserve fractional
phase, loop counters, West reversal, portrait selection and wrapper identity.
Engine services and sprite information are simulated; cycles and loop limits
are selected directly. This does not exercise natural schedules, the full NPC
state machine, automatic outfit dispatch, audible sound or live gameplay.
Evidence: `tmp/world-winter-standard-{hayden,celine}-runtime.{gml,log}`.

All 11,112 variants validate across 2,778 sources and 36 characters. The combined
bundle matches all 2,408 standalone variants for these two characters. Their
5,910 previous original/variant PNG and metadata files are byte-identical;
all 21,930 files for the other thirty-four characters are byte-identical,
including reports. Earlier runtime rows and controls, all 591 previous region
objects, color groups and palette mappings are unchanged. The 36 retained test
edits only update fixture paths, ignore descriptions and two source counts.
Evidence: `tmp/world-winter-standard-{comparison.json,input-audit.log}`.

A fresh isolated install passes MOMI v0.16.4 strict lints, required compilation
and installed-pixel/metadata verification. The lab is
`tmp/world-winter-standard-playtest`; its installed archive SHA-256 is
`a64e245f18132be40695c0bb4178c74739e7b5b506897ad1c573471bf3dc0c1c`.
The mounted source and lab's preserved `previous.zip` retain the source hash
above. The live game and desktop launcher were not changed. Evidence:
`tmp/world-winter-standard-install-report.json` and
`tmp/world-winter-standard-source-after.sha256`.

Eight frozen palette/registry input hashes remain unchanged after generation
and installation (`tmp/world-winter-standard-frozen-{inputs.sha256,check.log}`).

## Next coverage

Continue the seasonal plan with Hayden's eight remaining Winter specials and
Celine's Winter blink/sit/eat/drink. That will finish Hayden's Winter folder.
Ryis's next outfit can be selected separately. Folder evidence is
`tmp/world-winter-standard-remaining.json`.
