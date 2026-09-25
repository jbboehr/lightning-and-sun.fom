# Winter specials and everyday actions

The user approved the [Winter standard batch](world-winter-standard.md), committed
as `dc78309`. This slice adds nineteen strips, 72 source frames and 37 native
West mirrors. The user approved the artwork for commit on 2026-09-25.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Remaining Winter tool, wipebrow and reading specials | 8 | 41 | 25 | 276 |
| Celine | Winter blink, sit, eat and drink | 11 | 31 | 12 | 345 |

Hayden now covers all thirty Winter strips. Celine covers seventeen of 33 Winter
strips, retaining her complete Autumn folder. Ryis's complete Winter coverage
is unchanged. F8 and Insert keep their five choices and Vanilla defaults. No
runtime Rust or GML behavior changed. Material decisions and focused checks are
in [Hayden](hayden-winter-special.md) and [Celine](celine-winter-special.md).

## Source and metadata

The mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding complete archive. New source pins were added; no
existing hash was refreshed. Generation uses strict hashes, without
`--allow-source-hash-mismatch`.

Independent archive metadata and both author exports agree. All nineteen strips
retain 80×80 frames, Default atlas and Middle/54 origin, with their original
frame durations. Current native cycles support both characters' Winter outfits.
Hayden's seated reading is a complex start/loop/end sequence; hammering retains
its native sound hook. Celine's eating and drinking retain seated poses and
random final-frame holds of `[240,360]`. The synthetic package regression first
failed on Hayden's unregistered Winter hammer path, then passed with the nineteen
paths registered. It checks complete animation properties, frame counts, PNG
outputs and controls. Evidence: `tmp/world-winter-special-package-{red,checks}.log`,
`tmp/world-winter-special-inventory.log` and
`tmp/world-winter-special-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/world-winter-special-preview/summary.png):
  Hayden hammering East and Celine drinking South.
- [Complete Vanilla/Debug Blue review](../../generated/world-winter-special-preview/index.html):
  all 109 frame/direction cases, paginated by character, plus separate
  five-choice summaries.

Static previews do not establish natural NPC schedules, automatic outfit
changes, separately drawn props or live rendering. Extracted artwork, previews,
packages and isolated installs remain ignored.

The combined summary passes ten source/palette bindings and 282,880 exact-pixel
comparisons, including raw metadata and complete artwork crops. Character
galleries pass their separate exact-pixel, West-reversal and Chromium checks;
Chromium also loaded the combined landing page and all its local links/images
without horizontal overflow. Evidence is in
`tmp/world-winter-special-summary-check.log` and
`tmp/world-winter-special-landing-browser.log`.

## Verification and installation

Formatting, Clippy with warnings denied, all 117 active tests, and the release
build passed. The normal suite leaves 113 local opt-in tests ignored; seven
were run separately and passed: both new character corpus tests, both retained
world corpus tests and the three GML runtime tests. Their logs are under
`tmp/world-winter-special-`, with author-specific logs linked in the character
notes. Older opt-in tests and live gameplay were not rerun.

The combined build covers 2,797 sources and 11,188 variants across 36
characters. Every variant passed exact recipe validation. All 6,020 previously
accepted Hayden/Celine PNG and metadata files and all 21,930 files belonging to
the other 34 characters remain byte-identical. The 2,484 standalone
Hayden/Celine variants match the combined output. Runtime scripts, controls,
existing region objects and color maps are unchanged. Evidence:
`tmp/world-winter-special-{comparison.json,input-audit.log}`.

Native animator probes use the current archive's animator, cycle definitions
and NPC animation method with simulated engine services. Across five choices
they passed 545 frame/direction observations, 120 linear completions, 80
Celine final-frame hold checks and twenty Hayden reading-phase transitions,
including West mirroring, palette wrapper idempotence, frame phase and loop
counter preservation. These probes do not establish natural NPC scheduling,
audible sound, automatic outfit dispatch or full-engine state transitions.
Evidence: `tmp/world-winter-special-runtime.log` and the character-specific
runtime logs.

MOMI v0.16.4 installation passed in the isolated
`tmp/world-winter-special-playtest` lab, including installed pixel and animation
metadata verification. Installed archive SHA-256:
`bf7cdee0197bb1afd50b75e190324df44887e0dff1dd3a87272f6205c723627c`.
The mounted source and the lab's `previous.zip` both retain the source hash
above. All eight frozen recipe/registry inputs remained unchanged through
generation and installation. Evidence:
`tmp/world-winter-special-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Continue the seasonal plan with Celine's Winter general actions, sleep and kiss,
then her eleven remaining Winter specials. Hayden and Ryis have complete Winter
folders; their next outfits can be selected separately. Folder evidence is
`tmp/world-winter-special-remaining.json`.
