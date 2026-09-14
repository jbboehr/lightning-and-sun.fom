# Adeline standard seasonal animations

This adds the remaining 15 standard strips across Adeline's Summer, Autumn,
and Winter outfits: general actions in North/South/East, plus sleep and kiss in
East. The source strips contain 78 frame occurrences and 52 distinct images. With the
[everyday seasonal actions](overworld-seasonal-actions.md), all 91 standard
seasonal strips are included: 25 Spring and 22 in each other season. Shocked
poses have Spring source strips only; no seasonal copies are invented.

The existing `adeline-world-actions-trial.json` set contains 234 sources:
126 portraits and 108 overworld strips, including Spring's 17 special animations.
It produces 936 variants. F6 keeps portraits and supported world sprites on the
same Vanilla, Debug Blue, Hayden, Ryis, or Seridia choice. The combined
36-character package contains 2,309 sources and 9,236 variants. Shared Rust and
GML runtime code is unchanged.

## Sources and masks

The read-only source is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
New paths are under
`assets/animations/NPCs/Adeline/Sprites/{Summer,Autumn,Winter}/`, named
`spr_npc_adeline_{summer,autumn,winter}_{action_north,action_south,action_east,sleep_east,kiss_east}.png`.
Each outfit adds five strips and 26 frames. Every frame is 80×80 on the Default
atlas, with Middle/54 origin. Metadata is preserved.

General actions have seven frames with durations
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss has four frames with
`[0.15,0.15,0.8,0.15]`. Sleep is a single frame with omitted count/duration.
The native animator owns progression, facing and general-action final-frame
holds. West mirrors East artwork. Kiss hearts are separate game effects.
Source metadata inventory: `tmp/adeline-seasonal-standard-metadata.json`.

The masks append 282 seeds, selecting 2,417 additional pixels per target and
preserving 461 matching material pixels. Both color groups and all preceding
219 regions are unchanged. Each new frame contains 4–50 selected skin pixels.
Detached Summer hand pixels, raised fingers, sleep hand contours and shifted
kiss face edges are included; belts, collars, boots, hair and mouth interiors
remain original. The author and independent reviewer inspected all 52 distinct
frames across all five choices. All 39 independent source landmarks and
1,996,800 target pixels passed exact checks.
Evidence: `tmp/adeline-seasonal-standard-art-review-verification.json` and
`tmp/adeline-seasonal-standard-art-review-preservation.json`.

Frozen profile SHA-256:
`f09b615da53f28d676c18307b354ec02195ba310e6c1d5427395b3501dbe963c`.
The complete Adeline set changes 434,267 pixels per target. Recipe and registry
hashes are recorded in `tmp/adeline-seasonal-standard-frozen-inputs.sha256`.

## Offline review

- [Five-palette summary](../../generated/adeline-seasonal-standard-preview/summary.png):
  Action South frame 4 in each outfit.
- [Complete Vanilla/Debug Blue review](../../generated/adeline-seasonal-standard-preview/blue-review/index.html):
  all 78 source frames and 36 mirrored West views, for 114 cases and 228 views.
  Eighteen small pages group action directions, sleep, and kiss, with at most
  eight cases per page.

The summary uses 6× enlargement and the full review uses 10× nearest-neighbor
enlargement. Fresh bounds include the entire visible artwork. Exact checks pass
for all placed pixels, all 15 summary samples, source metadata and West mirrors,
with no clipping. Chromium checked every page, case, image and link. The complete
preview bundle is 2.43 MB. These are static frames; separate kiss-heart effects
and game timing are not displayed. Evidence:
`tmp/adeline-seasonal-standard-preview-pixel-check.log` and
`tmp/adeline-seasonal-standard-preview-browser-check.json`.

## Verification

Formatting, Clippy, all 87 active tests, the release build, three local world
corpus tests and three Fabricator GML tests passed. The world corpus covers
108 strips and 343 frame occurrences. The material-boundary test retains its
88 earlier literal landmarks and adds 20 for the new strips, checking all four
targets, identical mask selection, alpha and native metadata.

The packaging fixture first failed on the unregistered `autumn_action_east`,
then passed after adding the 15 exact registry entries. It checks native timing
and origins and rejects invented directions, nonexistent seasonal shocked
strips and unreviewed special animations. Removing Summer's detached hand seed
`[525,46]` in a candidate copy fails its independent skin landmark; the final
profile passes. Evidence: `tmp/adeline-seasonal-standard-package-{red,green}.log`,
`tmp/adeline-seasonal-standard-missing-hand-red.log`, and
`tmp/adeline-seasonal-standard-checks.log`.

The combined build preserves 2,190 previous Adeline original/variant PNG and
metadata files and all 20,925 files for the other 35 characters. All 936 Adeline
variants match the reviewed author outputs. The generated table retains prior
rows, choices and controls, and the complete 91-strip standard seasonal source
inventory is included. All 9,236 variants passed exact validation. Evidence:
`tmp/adeline-seasonal-standard-build-report.json`,
`tmp/adeline-seasonal-standard-comparison.log`, and
`tmp/adeline-seasonal-standard-all-variants.log`.

The focused Fabricator probe uses the original native animator and NPC `animate`
method with the shipped wrapper and actual 234-row table. It passed 114 new
direction/frame cases, 570 palette observations, three native 240–360 action
hold/release checks and six sleep/kiss completions. West flips, fractional/paused
phase, source/pack state, portrait synchronization and Hayden's independent
choice are preserved. Engine lookups/UI are simulated, metadata durations use
relative units, and the probe selects native collections directly. Full NPC
state machines, natural interactions/outfit changes and speaking/pause policy
are outside this probe. Evidence: `tmp/adeline-seasonal-standard-runtime-root.log`
and `tmp/adeline-seasonal-standard-runtime-inputs.json`.

A fresh MOMI installation in `tmp/adeline-seasonal-standard-playtest` verified
installed pixels, metadata and the generated table. Installed archive SHA-256:
`0749fbb2220644bd020dcfeb89f4b717635436080d20ffd3dbea9123d104278e`.
The source backup and retained `previous.zip` still match the pristine source
hash. Evidence: `tmp/adeline-seasonal-standard-install-report.json` and
`tmp/adeline-seasonal-standard-source-after.sha256`. Live gameplay and an
uninstall roundtrip were not rerun. The isolated package has no preview helper;
the desktop launcher remains unchanged on its earlier Zorel copy.

## Remaining coverage

Seasonal special animations, other outfits, and other characters' overworld
sprites remain original. The user accepted the offline art review; native
gameplay and natural scheduling remain unverified. Generated artwork and local
game data stay ignored by Git.
