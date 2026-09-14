# Adeline seasonal writing animations

This adds 18 standing and seated writing strips across Adeline's Summer, Autumn,
and Winter outfits. They contain 48 frame occurrences and 36 distinct images.
With the [standard seasonal coverage](overworld-seasonal-standard.md), the
existing `adeline-world-actions-trial.json` set now covers 252 sources:
126 portraits and 126 overworld strips, producing 1,008 variants. All 24 standing/
seated writing strips across the four seasons are included.

F6 keeps portraits and supported world sprites on the same Vanilla, Debug Blue,
Hayden, Ryis, or Seridia choice. The combined 36-character package contains
2,327 sources and 9,308 variants. Shared Rust and GML runtime code is unchanged.

## Sources and masks

The read-only source is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
New paths are under `assets/animations/NPCs/Adeline/Sprites/{Summer,Autumn,Winter}/`,
named `spr_npc_adeline_specialanimation_{summer,autumn,winter}_write{,_sit}_{start,loop,end}_south.png`.
Each outfit adds six strips and 16 frames. Every frame is 80×80 on the Default
atlas with Middle/54 origin. All writing sprites face South; no additional
mirrored directions are registered.

Standing and seated sequences each have two start frames at duration `0.125`,
four loop frames with `[0.1,0.125,0.1,0.3]`, and two end frames with `[0.125,0.1]`.
Source metadata is preserved. The native complex animator owns progression,
looping and ending; seated writing retains its native seated flag. Source
metadata inventory: `tmp/adeline-seasonal-writing-metadata.json`.

The profile appends 248 seeds, selecting 2,110 additional skin pixels per target
while preserving 234 matching collar/material pixels. Both color groups and all
234 preceding regions are unchanged. Each writing frame has 39–50 selected skin
pixels, including fingers and lower thumbs around the boards. Pens, board wood,
clothing and mouth interiors retain their original colors. All 36 distinct
frames were inspected in Vanilla and all four targets; 36 independent source
landmarks and 1,228,800 target pixels passed exact checks.

Frozen profile SHA-256:
`651bbe7c47460f6aa1fc6ef16127c9092a02d5874bb3d2bc8ddd0fec46474a88`.
The full Adeline set changes 436,377 pixels per target. Evidence:
`generated/adeline-seasonal-writing-author/final-audit.json`,
`tmp/adeline-seasonal-writing-art-review.md`, and
`tmp/adeline-seasonal-writing-frozen-inputs.sha256`.

## Offline review

- [Five-palette summary](../../generated/adeline-seasonal-writing-preview/summary.png):
  standing writing, loop frame 2 in each outfit.
- [Complete Vanilla/Debug Blue review](../../generated/adeline-seasonal-writing-preview/blue-review/index.html):
  all 48 South-facing source frames and 96 views on six small pages, one
  standing/seated sequence per outfit and eight cases per page.

The summary uses 6× enlargement; full frames use 10× nearest-neighbor enlargement
with fresh bounds covering the pens, boards and seated poses. Exact comparisons
pass for every placed pixel, all 15 summary samples, source metadata and palette
bindings, with no clipping or invented mirrors. Chromium checked every page,
case, image and link. The complete preview bundle is 1.17 MB. These static views
do not show native animation timing. Evidence:
`tmp/adeline-seasonal-writing-preview-pixel-check.log` and
`tmp/adeline-seasonal-writing-preview-browser-check.json`.

## Verification

Formatting, Clippy, all 87 active tests, the release build, three local world
corpus tests and three Fabricator GML tests passed. The world corpus now covers
126 strips and 391 frame occurrences. The material-boundary test retains its
108 earlier landmarks and adds 20 for writing hands and protected props. Its
test-only outfit lookup now resolves the season after `specialanimation_`, so
the new cases load their actual seasonal artwork. All four targets, alpha,
metadata and identical mask selection are checked.

The package fixture first rejected unregistered
`specialanimation_autumn_write_end_south`, then passed after the 18 exact registry
entries were added. It checks native timing/origins and rejects unreviewed
reading and invented writing directions. Removing Summer standing-write end
seed `[125,48]` from a candidate copy fails the independent thumb landmark at
`[126,49]`; the final profile passes. Evidence:
`tmp/adeline-seasonal-writing-package-{red,green}.log`,
`tmp/adeline-seasonal-writing-missing-thumb-red.log`, and
`tmp/adeline-seasonal-writing-checks.log`.

The combined build preserves 2,340 previous Adeline original/variant PNG and
metadata files and all 20,925 files for the other 35 characters. All 1,008 Adeline
variants match reviewed author outputs. The table retains prior rows, choices
and controls; all 91 standard seasonal strips and all 24 seasonal writing strips
are included. All 9,308 variants passed exact validation. Evidence:
`tmp/adeline-seasonal-writing-build-report.json`,
`tmp/adeline-seasonal-writing-comparison.log`, and
`tmp/adeline-seasonal-writing-all-variants.log`.

The focused native probe passed 48 South/frame cases, 240 palette observations
and 120 start/loop/end transitions at actual last frames across the six seasonal
standing/seated sequences. It uses the original native animator and NPC `animate`
method with the shipped wrapper and actual 252-row table. Checks preserve seated
flags, fractional/paused phase, source/pack state, portrait synchronization and
Hayden's independent choice. Engine lookups/UI are simulated, original metadata
durations use relative units, and packs/loop requests are selected directly.
Natural scheduling, full NPC state machines, automatic outfit dispatch and
speaking/background pause policy are outside the probe. Evidence:
`tmp/adeline-seasonal-writing-runtime-root.log` and
`tmp/adeline-seasonal-writing-runtime-inputs.json`.

A fresh MOMI installation in `tmp/adeline-seasonal-writing-playtest` verified
installed pixels, metadata and the table. Installed archive SHA-256:
`208a756aadbd8e6af632d8e6de320e97f3f20e8b05be7632f07a8acef4c7c3f7`.
The source backup and retained `previous.zip` still match the pristine hash.
Evidence: `tmp/adeline-seasonal-writing-install-report.json` and
`tmp/adeline-seasonal-writing-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. This isolated package has no preview helper; the desktop
launcher remains unchanged on its earlier Zorel copy.

## Remaining coverage

The [next seasonal pass](overworld-seasonal-special.md) adds the remaining
reading, working, thinking, and fainting strips. Other outfits and other
characters' overworld sprites remain original. The user accepted the offline
writing review; native gameplay and natural scheduling remain unverified.
Generated artwork and local game data stay ignored by Git.
