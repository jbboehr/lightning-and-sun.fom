# Hayden's first overworld batch

This is the first small pilot before expanding more characters' overworld masks.
It covers six Spring idle/walk strips, containing 15 source frames. West uses the
native horizontal mirror of East, giving 20 direction/frame cases. F8 selects
Vanilla, Debug Blue, Adeline, Ryis or Seridia for both Hayden's portraits and these
world sprites. Other actions and outfits remain original.

The new `hayden-world-trial.json` profile and set retain all 133 portrait regions
and add six world regions, for 139 sources and 556 recolored variants. The
portrait-only definitions remain available. The combined character set now uses
this extended set and contains 36 characters, 2,363 sources and 9,452 variants.
Shared Rust and GML runtime code is unchanged.

## Source and mask decisions

Sources are under `assets/animations/NPCs/Hayden/Sprites/Spring/`, named
`spr_npc_hayden_spring_{idle,walk}_{north,south,east}.png`. All are 80×80 frames
on the Default atlas with Middle/54 origin. Idle omits frame count/duration and
uses the engine's single-frame defaults; walking has four frames at duration
`0.15`. The original `assets/fiddle/npcs/hayden.toml` confirms linear idle/walk
cycles, North/South/East source directions and native outfit declarations.

The read-only source archive is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Local extraction: `extracted/hayden-world-study`. Source metadata inventory:
`tmp/hayden-world-metadata.json`.

World skin uses `E7B172`, `AB7E3F`, `815A2E` and `523C26`. Each maps to the
corresponding four base portrait target shades. The existing eleven portrait
source colors, three color groups and all earlier region objects are unchanged.
The four new world colors each have a separate color group: the shirt reuses
their brown shades, and joining the colors lets a skin mask cross into the
shirt's side and shoulder shadows during walking. Separate groups and reviewed
component seeds keep those folds original while covering faces, ears, necks,
arms, hands and forearm shading. Head hair, beard, eyes and other colors remain
original. Four connected upper-arm edge pixels in the South walking poses stay
with their same-color arm components.

The six additions use 276 seeds and recolor 668 pixels per target; 197 matching
world-ramp pixels remain excluded. All 15 frames were inspected in Vanilla and
all four targets. Profile SHA-256:
`640223cefdb2386ecfc2c9d1399492307894b7ce84e7e49aa9b5a97c290eb5b8`.
Evidence: `tmp/hayden-world-refinement.json`, the three
`tmp/hayden-world-refined-art-*.png` sheets and
`tmp/hayden-world-frozen-inputs.sha256`.

## Offline review

- [Summary PNG](../../generated/hayden-world-preview/summary.png): three idle
  directions across all five choices, about 46 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-world-preview/blue-review/index.html):
  all 15 source frames plus five West mirrors, on three pages with at most eight
  cases each. The complete preview directory is about 429 KB.

Fresh crop `[29,24,22,34]` contains every visible source pixel, including the
lowest walking boots. Summary samples use 6× enlargement and full cases use 10×.
Exact checks cover 2,992,000 full-review pixels, 403,920 summary pixels, palette
bindings, source metadata and native West reversal, with no clipping. Chromium
decoded every image and checked all three pages, 20 cases and links. Evidence:
`tmp/hayden-world-preview-pixel-check.log` and
`tmp/hayden-world-preview-browser-check.json`. These static images do not show
game timing. The user accepted the offline artwork review.

## Verification

Formatting, Clippy, all 89 active tests, the release build, the local Hayden mask
test and three Fabricator GML tests passed. The local test checks all four
targets, alpha, metadata, common selection and 13 literal skin/material landmarks.
The package fixture first rejected the unregistered Hayden world source, then
passed after the six registry entries were added, checking F8 and native geometry.
Merging the world color groups in an ignored negative-control copy fails the
protected North walking shoulder at `[116,40]`, confirming that a broader mask
cannot pass. Evidence: `tmp/hayden-world-package-red.log`,
`tmp/hayden-world-focused-checks.log`, `tmp/hayden-world-mixed-colors-red.log` and
`tmp/hayden-world-final-checks.log`.

The combined bundle preserves all 1,330 earlier Hayden original/variant PNG and
metadata files and 22,415 files for the other 35 characters. All 556 Hayden
variants match the inspected outputs. Previous table rows and controls are
unchanged, and all 9,452 combined variants pass exact validation. Evidence:
`tmp/hayden-world-comparison.log` and `tmp/hayden-world-all-variants.log`.

The focused native probe uses the original animator and NPC `animate` method
with the actual 139-row Hayden table. It passes 20 direction/frame cases,
100 matrix palette observations and ten Spring idle/walk collection switches
with 50 additional palette observations and ten last-frame loop boundaries.
West facing, fractional/paused phase, native state, portrait synchronization and
Adeline's independent palette selection are checked. Engine services are
simulated and metadata durations use relative units; collection selection is
direct. Natural schedules, automatic outfit changes, full NPC state machines,
speaking/background pause policy and live gameplay were not exercised. Evidence:
`tmp/hayden-world-runtime.log` and `tmp/hayden-world-runtime-inputs.json`.

A fresh MOMI install in `tmp/hayden-world-playtest` passed installed pixel,
metadata and table validation. Installed archive SHA-256:
`fd4098ba40ebf8520f68d3233387a76a8a57c492ecd63811153724a45ee28ba4`.
The source and retained `previous.zip` match the pristine hash. No preview helper
is installed and the desktop launcher is unchanged. An uninstall roundtrip was
not rerun. Evidence: `tmp/hayden-world-install-report.json` and
`tmp/hayden-world-source-after.sha256`.

## Next pilots

Review Hayden first, then make equally small Spring idle/walk batches for Ryis
and Celine one at a time. Once those initial masks are accepted, expand larger
character batches with subagents. Adeline's remaining beach/wedding world sprites
stay queued. Game files, generated artwork and isolated installs remain ignored.
