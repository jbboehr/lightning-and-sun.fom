# Hayden, Ryis and Celine: shocked reactions and seated reading

The accepted [general-action, sleep and kiss batch](spring-world-standard.md)
was committed as `7f6d711`. This parallel slice adds six Spring strips per
character: shocked start/loop/end and seated reading start/loop/end. All 39
source frames face South; these cycles have no native West mirrors. The user
accepted the offline artwork review.

Each character now has 28 Spring overworld strips alongside the existing
portraits. F8, F10 and Insert still select Hayden, Ryis and Celine respectively;
Vanilla remains the default. Celine's garden sprites, other special actions and
other outfits remain outside this batch. Rust and GML runtime code is unchanged;
the shared registry adds eighteen exact source paths.

| Character | Total sources | Variants | New seeds | New skin pixels per target |
| --- | ---: | ---: | ---: | ---: |
| Hayden | 161 | 644 | 224 | 510 |
| Ryis | 137 | 548 | 58 | 700 |
| Celine | 211 | 844 | 178 | 536 |

All earlier region objects, source colors, groups and target mappings are
preserved. Character notes cover the material boundaries and focused tests:
[Hayden](hayden-reactions.md), [Ryis](ryis-reactions.md) and
[Celine](celine-reactions.md). Books, gloves and mouth interiors retain their
original colors. Hayden's shocked loop includes two cheek highlights from the
existing portrait ramp; both are explicitly covered by the corpus test.

## Offline review

- [Small combined summary](../../generated/spring-reactions-preview/summary.png):
  shocked loop in all five choices, about 64 KB.
- [Complete review landing page](../../generated/spring-reactions-preview/index.html):
  each character's sampled five-choice summary and every new Vanilla/Debug Blue
  frame, split into small pages of at most seven cases.

Authors inspected every new frame in all four targets. Integration inspection
also covered Hayden's cheek and book/hand boundaries, Ryis's gloves and blue
book, and Celine's open mouth, raised hands and reading poses. Individual notes
record full preview pixel and browser checks. The combined summary independently
matches all fifteen character/palette/frame bindings, source hashes, metadata
and 468,000 display pixels without clipping or overlap. Chromium decoded the
landing image and all six character summary/review links. Evidence:
`tmp/spring-reactions-summary-check.log` and
`tmp/spring-reactions-landing-browser.log`.

Static reviews do not demonstrate timing, natural schedules, interactions,
outfit changes or separately drawn props.

## Verification

Formatting, Clippy, all 95 active tests and the release build pass in
`tmp/spring-reactions-final-checks.log`. The three new opt-in corpus tests pass
for all four targets. The nine earlier character corpus tests retain their
mask assertions and pass against the expanded local corpora. The three GML
tests also pass. Evidence: the character notes,
`tmp/spring-reactions-retained-{hayden,ryis,celine}-checks.log` and
`tmp/spring-reactions-gml-checks.log`.

The shared package fixture first rejected an unregistered reading path, then
passed after registration. It covers all six phases and thirteen frames per
character. Each shocked phase uses single-frame engine defaults. Reading
start/end have three frames at `0.1`; the four-frame loop uses
`[3.0,0.1,3.0,0.1]`. All sources retain 80×80 frames, Default atlas and Middle/54
origin. Evidence: `tmp/spring-reactions-package-{red,checks}.log` and
`tmp/spring-reactions-inventory.log`.

The combined bundle `generated/characters-spring-reactions-trial` contains
36 characters, 2,441 sources and 9,764 variants. Exact comparison preserves all
4,910 previous original/variant PNG and metadata files for these three
characters and all 19,485 files for the other 33. Earlier table rows and controls
remain identical. All 2,036 variants for the expanded characters equal the
inspected standalone output; every combined variant validates. Evidence:
`tmp/spring-reactions-comparison.json`, `tmp/spring-reactions-all-variants.log`
and `tmp/spring-reactions-frozen-inputs.sha256`.

Three native probes use the original animation handler and NPC `animate` method
with actual 161/137/211-row character tables and Adeline's 282-row table. Each
passes thirteen frame cases, 65 palette observations and forty complex phase
transitions at the actual last frames, including twenty multiframe boundaries.
They also check standing/seated flags, fractional and paused phase, native state,
portrait synchronization and independent Adeline selection. Evidence:
`tmp/spring-reactions-{hayden,ryis,celine}-runtime.log` and corresponding
`-runtime-inputs.json` files. Engine services and relative frame durations are
simulated; collections and loop limits are selected directly. Full NPC state
machines, natural scheduling, automatic outfit dispatch and speaking-triggered
animation changes are not exercised.

A fresh isolated MOMI install in `tmp/spring-reactions-playtest` passes installed
pixel, metadata and table validation. Installed archive SHA-256:
`40b6c7b837889ec357cba2f38920220490b5134073a4400ff75cd138091621ed`.
The read-only source `tmp/momi-lab/assets.bak.zip` and retained `previous.zip`
match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/spring-reactions-install-report.json`,
`tmp/spring-reactions-install.log` and `tmp/spring-reactions-source-after.sha256`.
No preview helper or launcher change is included. An uninstall roundtrip and
live gameplay were not rerun. Game assets, previews and isolated installs remain
ignored.
