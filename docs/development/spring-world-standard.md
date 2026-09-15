# Hayden, Ryis and Celine: general action, sleep and kiss

The accepted [blink/sit/eat/drink batch](spring-world-actions.md) was committed
as `db562a3`. This next parallel slice adds five normal Spring strips per
character: general action North/South/East, sleep East and kiss East. The fifteen
strips contain 78 source frames; native West mirroring adds 36 cases. The user
accepted the offline artwork review.

Each character now has 22 shared Spring overworld strips alongside the existing
portraits. F8, F10 and Insert still select Hayden, Ryis and Celine respectively;
Vanilla remains the default. Special actions, Celine's garden sprites and other
overworld outfits remain outside this batch. Rust and GML runtime code is
unchanged; the shared registry adds fifteen exact source paths.

| Character | Total sources | Variants | New seeds | New skin pixels per target |
| --- | ---: | ---: | ---: | ---: |
| Hayden | 155 | 620 | 460 | 1,015 |
| Ryis | 131 | 524 | 107 | 1,211 |
| Celine | 205 | 820 | 266 | 897 |

All earlier region objects, source colors, groups and target mappings are
preserved. Character notes cover the material boundaries and focused tests:
[Hayden](hayden-standard.md), [Ryis](ryis-standard.md) and
[Celine](celine-standard.md). Hayden's kiss includes one `E8B271` cheek highlight
from the existing portrait ramp, covered explicitly by the new corpus test.

## Offline review

- [Small combined summary](../../generated/spring-standard-preview/summary.png):
  South action frame four in all five choices, about 57 KB.
- [Complete review landing page](../../generated/spring-standard-preview/index.html):
  each character's sampled five-choice summary and every new Vanilla/Debug Blue
  frame, including West mirrors. Pages have at most eight cases each.

Authors inspected every new frame in all four targets. Integration inspection
also covered Hayden's cheek and forearm boundaries, Ryis's raised hands and
sleeping pose, and Celine's kissing frames and tiny north-facing hands.
Individual notes record full preview pixel and browser checks. The combined
summary independently matches all fifteen character/palette/frame bindings,
source hashes, metadata and 392,040 display pixels without clipping or overlap.
Chromium decoded the landing image and all six character summary/review links.
Evidence: `tmp/spring-standard-summary-check.log` and
`tmp/spring-standard-landing-browser.json`.

Static reviews do not demonstrate animation timing, interactions, natural
schedules, outfit changes or separately drawn props.

## Verification

Formatting, Clippy, all 94 active tests and the release build pass in
`tmp/spring-standard-final-checks.log`. The three new opt-in corpus tests pass
for all four targets. The six previous idle/walk and blink/sit/eat/drink corpus
tests retain their mask assertions and run against the expanded local corpora;
the three GML tests run alongside them. Evidence:
`tmp/spring-standard-retained-gml-checks.log` and the character notes.

The shared package fixture first rejected the unregistered general-action path,
then passed after registration. It now covers sixteen action strips and 57
frames per character, preserving the earlier eleven-strip assertions and adding
the new native durations: general action has seven frames at
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`, kiss has four at `[0.15,0.15,0.8,0.15]`,
and sleep uses the single-frame defaults. All sources retain 80×80 frames,
Default atlas and Middle/54 origin. Evidence:
`tmp/spring-standard-package-{red,checks}.log` and
`tmp/spring-standard-inventory.log`.

The combined bundle `generated/characters-spring-standard-trial` contains
36 characters, 2,423 sources and 9,692 variants. Exact comparison preserves all
4,760 previous original/variant PNG and metadata files for the expanded
characters and all 19,485 files for the other 33 characters. Earlier table rows
and controls remain identical. All 1,964 variants for the expanded characters
equal the inspected standalone output; every combined variant validates.
Evidence: `tmp/spring-standard-comparison.json`,
`tmp/spring-standard-all-variants.log` and
`tmp/spring-standard-frozen-inputs.sha256`.

Three native probes use the original animation handler and NPC `animate` method
with actual 155/131/205-row character tables and Adeline's 282-row table. Each
passes 38 direction/frame cases and 190 palette observations, an original
`[240,360]` action hold/release, sleep and kiss completion, West reversal,
fractional/paused phase, native state, portrait synchronization and independent
Adeline selection. Evidence: `tmp/spring-standard-{hayden,ryis,celine}-runtime.log`
and corresponding `-runtime-inputs.json` files. Engine services and relative
frame durations are simulated; collections and loop limits are selected
directly. Full NPC state machines, automatic outfit/interaction dispatch and
speaking-triggered action-to-idle transitions are not exercised.

A fresh isolated MOMI install in `tmp/spring-standard-playtest` passes installed
pixel, metadata and table validation. Installed archive SHA-256:
`563271a207fc7adf699fdbfb6513a73784ed2528fca32be4d3f27759d0c42226`.
The read-only source `tmp/momi-lab/assets.bak.zip` and retained `previous.zip`
match pristine SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/spring-standard-install-report.json`,
`tmp/spring-standard-install.log` and `tmp/spring-standard-source-after.sha256`.
No preview helper or launcher change is included. An uninstall roundtrip and
live gameplay were not rerun. Game assets, previews and isolated installs remain
ignored.
