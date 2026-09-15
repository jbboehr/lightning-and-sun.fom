# Hayden, Ryis and Celine: Spring actions

The three accepted idle/walk pilots now expand in parallel, with one author per
character. Each adds eleven normal Spring blink, sit, eat and drink strips:
33 strips, 93 source frames and 129 direction/frame cases with native West
mirroring. The user accepted the offline artwork review. Celine's accepted pilot was
committed as `557c173` before this expansion.

F8, F10 and Insert still switch Hayden, Ryis and Celine respectively, keeping
portraits and included world sprites together. Each starts on Vanilla and keeps
the same four alternatives. Other actions and outfits, including Celine's
gardening sprites, remain original. Rust and GML runtime code is unchanged.

| Character | Previous sources | Added strips | Total sources | Variants | New skin pixels per target |
| --- | ---: | ---: | ---: | ---: | ---: |
| Hayden | 139 | 11 | 150 | 600 | 1,404 |
| Ryis | 115 | 11 | 126 | 504 | 1,496 |
| Celine | 189 | 11 | 200 | 800 | 1,131 |

The combined trial contains 36 characters, 2,408 sources and 9,632 variants.
All previous region objects, source colors, color groups and target mappings
remain unchanged. Character-specific masks and source evidence are documented
in [Hayden's action pass](hayden-actions.md), [Ryis's action pass](ryis-actions.md)
and [Celine's action pass](celine-actions.md).

## Offline review

- [Combined summary PNG](../../generated/spring-actions-preview/summary.png):
  one drinking pose per character in all five choices, about 54 KB.
- [Complete review landing page](../../generated/spring-actions-preview/index.html):
  links to each character's four-action summary and complete Vanilla/Debug Blue
  review. Every new source frame and native West mirror is included, with at
  most eight cases on each page.

The authors inspected every new source frame in all four target palettes.
Integration inspection also covered the summaries, Hayden's raised-hand/shirt
boundaries, Ryis's eating poses and Celine's open mouths and north-facing hands.
The combined preview independently checks all fifteen character/palette/frame
bindings against the actual combined bundle, source hashes, native metadata,
345,600 exact display pixels and unclipped crops. Chromium decoded the landing
image and all six linked character review/summary destinations. Evidence:
`tmp/spring-actions-summary-check.log` and
`tmp/spring-actions-landing-browser.json`. Individual character notes record
their complete review pixel and browser checks.

These are static sprite reviews. They do not show live animation timing,
natural schedules, outfit changes or separately drawn food/cup overlays.

## Verification

Formatting, Clippy, all 94 active tests and the release build pass in
`tmp/spring-actions-final-checks.log`. The three new opt-in action corpus tests
check all four target palettes and literal skin/material boundaries; their
passing runs and deliberately broken-mask checks are recorded in the character
notes. The existing three idle/walk corpus tests are also run against the
expanded profiles, retaining their original mask assertions, along with the
three GML tests: `tmp/spring-actions-pilot-gml-checks.log`.

The shared synthetic package test first rejected the unregistered new source,
then passed after the 33 registry entries were added. It checks each character's
control, eleven action rows, exact packaged PNG bytes and native directional
timing: three blink frames; implicit single-frame sitting; five South/East eating
frames versus three North frames; and three drinking frames. Evidence:
`tmp/spring-actions-package-red.log`, `tmp/spring-actions-package-checks.log`
and `tests/spring_world_actions.rs`.

The final bundle is `generated/characters-spring-actions-trial`. Exact comparison
preserves all 4,430 prior original/variant PNG and metadata files for the three
expanded characters and all 19,485 files for the other 33 characters. Earlier
table rows and controls remain identical. All 1,904 variants for the expanded
characters equal the standalone author outputs, and all 9,632 combined variants
pass exact validation. Evidence: `tmp/spring-actions-comparison.json` and
`tmp/spring-actions-all-variants.log`. Eleven final build inputs are pinned in
`tmp/spring-actions-frozen-inputs.sha256`.

Three focused probes use the original game's animation handler and NPC
`animate` method with the actual 150/126/200-row character tables and the
282-row Adeline table. Each passes 43 direction/frame cases, 215 matrix palette
observations, twenty collection switches with another 100 observations, twenty
last-frame loop boundaries, ten original `[240,360]` random holds and fifteen
configured blink-to-idle returns. They also check seated flags, West reversal,
fractional/paused phase, native state, portrait synchronization and independent
Adeline selection. Evidence: `tmp/spring-actions-{hayden,ryis,celine}-runtime.log`
and corresponding `-runtime-inputs.json` files.

The probes simulate engine services, select collections directly and use a
small reset adapter for configured blink returns. They do not run automatic
blink scheduling, full NPC state machines, natural seasonal dispatch,
speaking/background pause policy or live gameplay.

A fresh isolated MOMI install in `tmp/spring-actions-playtest` passed installed
pixel, metadata and table validation. Installed archive SHA-256:
`a5448820ee146262cb6f84c211f0ca1e522028fecf7531feae717c32a2dd6bf9`.
The read-only source `tmp/momi-lab/assets.bak.zip` and retained `previous.zip`
match the pristine hash
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Evidence: `tmp/spring-actions-install-report.json`,
`tmp/spring-actions-install.log` and `tmp/spring-actions-source-after.sha256`.
No preview helper was installed. The desktop launcher was not changed;
an uninstall roundtrip and live gameplay were not rerun. All game assets,
generated artwork and isolated installs remain ignored.
